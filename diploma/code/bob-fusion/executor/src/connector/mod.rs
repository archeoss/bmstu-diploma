pub mod prelude {
    pub use crate::prelude::*;
    pub use tonic::Request;
}

use std::{
    marker::{PhantomData, PhantomPinned},
    time::UNIX_EPOCH,
};

use prelude::*;

pub mod api {
    tonic::include_proto!("bob_storage");
}

#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("todo")]
    GetError,
    #[error("todo")]
    PutError,
    #[error("todo")]
    DeleteError,
    #[error("todo")]
    ExistError,
}

use api::bob_api_client::BobApiClient;
use tonic::transport::Channel;

pub trait Connector<Key: Into<api::BlobKey>> {
    type Error;
    type Data;
    async fn get(&mut self, key: Key) -> Result<Self::Data, Self::Error>;
    async fn put(&mut self, key: Key, data: Self::Data) -> Result<(), Self::Error>;
    async fn delete(&mut self, key: Key) -> Result<(), Self::Error>;
    async fn exist(&mut self, key: Vec<Key>) -> Result<Vec<bool>, Self::Error>;
}

#[derive(Clone, Default)]
pub struct DBConnector<Client> {
    client: Client,
    // key: PhantomData<Key>,
}

impl<Client> DBConnector<Client> {
    pub const fn new(client: Client) -> Self {
        Self {
            client,
            // key: PhantomData,
        }
    }
}

impl<Key: Into<api::BlobKey> + Send> Connector<Key> for DBConnector<BobApiClient<Channel>> {
    type Data = Vec<u8>;
    // type Key = Vec<u8>;
    type Error = ConnectorError;

    async fn get(&mut self, key: Key) -> Result<Self::Data, Self::Error> {
        let req = Request::new(api::GetRequest {
            key: Some(key.into()),
            options: Some(api::GetOptions {
                force_node: true,
                source: api::GetSource::Normal.into(),
            }),
        });
        Ok(self
            .client
            .get(req)
            .await
            .change_context(ConnectorError::GetError)?
            .map(|blob| blob.data)
            .into_inner())
    }

    async fn put(&mut self, key: Key, data: Self::Data) -> Result<(), Self::Error> {
        let req = Request::new(api::PutRequest {
            key: Some(key.into()),
            data: Some(api::Blob {
                data,
                meta: Some(api::BlobMeta {
                    timestamp: std::time::SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .change_context(ConnectorError::PutError)?
                        .as_secs(),
                }),
            }),
            options: Some(api::PutOptions {
                remote_nodes: vec![],
                force_node: true,
                overwrite: true,
            }),
        });

        self.client
            .put(req)
            .await
            .change_context(ConnectorError::PutError)?
            .into_inner()
            .error
            .map_or(Ok(()), |err_code| {
                Err(ConnectorError::PutError)
                    // .attach_printable(err_code.desc.clone())
                    .attach(err_code)
            })
    }

    async fn delete(&mut self, key: Key) -> Result<(), Self::Error> {
        let req = Request::new(api::DeleteRequest {
            key: Some(key.into()),
            meta: Some(api::BlobMeta {
                timestamp: std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .change_context(ConnectorError::PutError)?
                    .as_secs(),
            }),
            options: Some(api::DeleteOptions {
                force_alien_nodes: vec![],
                force_node: true,
                is_alien: false,
            }),
        });

        self.client
            .delete(req)
            .await
            .change_context(ConnectorError::DeleteError)?
            .into_inner()
            .error
            .map_or(Ok(()), |err_code| {
                Err(ConnectorError::DeleteError)
                    // .attach_printable(err_code.desc.clone())
                    .attach(err_code)
            })
    }

    async fn exist(&mut self, keys: Vec<Key>) -> Result<Vec<bool>, Self::Error> {
        let req = Request::new(api::ExistRequest {
            keys: keys.into_iter().map(Into::into).collect(),
            options: Some(api::GetOptions {
                force_node: true,
                source: api::GetSource::Normal.into(),
            }),
        });

        Ok(self
            .client
            .exist(req)
            .await
            .change_context(ConnectorError::DeleteError)?
            .into_inner()
            .exist)
    }
}
