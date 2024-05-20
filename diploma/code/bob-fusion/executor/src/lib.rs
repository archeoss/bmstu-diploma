#![allow(async_fn_in_trait)]

pub mod prelude {
    pub use error_stack::{Context, Report, Result, ResultExt};
    pub use thiserror::Error;
}

pub mod connector;
pub mod executor;
pub mod service;
