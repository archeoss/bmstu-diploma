pub mod multiplex;
pub use multiplex::HybridMakeService;

pub mod prelude {
    pub use crate::prelude::*;
    pub use hyper::{body::HttpBody, service::Service, Body, HeaderMap, Request, Version};
    pub use pin_project::pin_project;
    pub use std::{error::Error, future::Future, pin::Pin, task::Poll};
}
