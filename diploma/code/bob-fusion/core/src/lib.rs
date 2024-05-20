#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

#[allow(unused_imports)]
use prelude::*;

pub mod config;
pub mod router;
pub mod services;
pub mod types;

pub mod prelude {
    pub use axum::{
        response::{IntoResponse, Response},
        Router,
    };
    pub use error_stack::{Context, Report, Result, ResultExt};
    pub use hyper::{body::HttpBody, Method};
    pub use serde::{Deserialize, Serialize};
    pub use std::marker::PhantomData;
    pub use thiserror::Error;
    #[cfg(all(feature = "swagger", debug_assertions))]
    pub use utoipa::{OpenApi, PartialSchema, ToSchema};
}
