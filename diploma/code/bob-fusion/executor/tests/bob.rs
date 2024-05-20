#![allow(clippy::unwrap_used)]

pub const AUTH_NAME: &str = "admin";
pub const AUTH_PASS: &str = "password";

use futures::{stream::FuturesUnordered, StreamExt};
use std::fmt::Display;
use testcontainers::{
    core::{Mount, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, Image, ImageArgs,
};

pub struct BobNodes {
    pub volumes: Vec<Mount>,
}

impl BobNodes {
    #[must_use]
    pub fn status(&self, port: impl Display) -> String {
        format!("http://localhost:{port}/status",)
    }
}

impl Default for BobNodes {
    fn default() -> Self {
        Self {
            volumes: vec![
                ("/tmp".to_string(), "/tmp".to_string()),
                (
                    format!("{}/bob-config/bob/configs", env!("CARGO_MANIFEST_DIR")),
                    "/bob/configs".to_string(),
                ),
            ]
            .into_iter()
            .map(|volume| Mount::bind_mount(volume.0, volume.1))
            .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeArgs {
    args: Vec<String>,
}

impl Default for NodeArgs {
    fn default() -> Self {
        Self {
            args: vec![
                "cluster.yaml".to_string(),
                "node_0.yaml".to_string(),
                "--init_folders".to_string(),
            ],
        }
    }
}

impl ImageArgs for NodeArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        self.args.into_iterator()
    }
}

impl Image for BobNodes {
    type Args = NodeArgs;

    fn name(&self) -> String {
        String::from("qoollo/bob")
    }

    fn tag(&self) -> String {
        String::from("2.1.0.alpha.12")
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout("Disk is ready")]
    }

    fn mounts(&self) -> Box<dyn Iterator<Item = &Mount> + '_> {
        Box::new(self.volumes.iter())
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![8000]
    }
}

pub async fn start_bob() -> ContainerAsync<BobNodes> {
    let container = BobNodes::default().start().await;
    setup_db(container.get_host_port_ipv4(8000).await.to_string()).await;
    container
}

async fn setup_db(port: String) {
    let mut futures: FuturesUnordered<_> = (0..100)
        .map(|key| {
            let port = port.clone();
            tokio::spawn(async move {
                let client = reqwest::ClientBuilder::new().build().unwrap();
                client
                    .execute(
                        client
                            .post(format!("http://localhost:{port}/data/{key}"))
                            .body(hyper::body::Bytes::from(format!("Test data {key}")))
                            .basic_auth(AUTH_NAME, Some(AUTH_PASS))
                            .build()
                            .unwrap(),
                    )
                    .await
                    .unwrap()
            })
        })
        .collect();
    while let Some(res) = futures.next().await {
        assert!(res.is_ok());
        assert!(res.unwrap().status().is_success());
    }
}

#[tokio::test]
async fn is_writable() {
    start_bob().await;
}

#[tokio::test]
async fn bob_started() {
    let container = BobNodes::default().start().await;
    let url = container
        .image()
        .status(container.get_host_port_ipv4(8000).await.to_string());
    let response = reqwest::get(url).await.unwrap();

    assert!(response.status().is_success());
}
