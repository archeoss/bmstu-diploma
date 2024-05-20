#[allow(unused_imports)]
use crate::prelude::*;
pub use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use std::result::Result;
use tonic::{transport::server::Routes, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[must_use]
pub fn grpc_service() -> Routes {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();
    tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(GreeterServer::new(MyGreeter::default()))
        .into_service()
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
#[cfg_attr(feature = "otlp-exporter", autometrics(objective = API_SLO))]
impl Greeter for MyGreeter {
    #[tracing::instrument]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        tracing::info!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
