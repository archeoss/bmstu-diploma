use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use thiserror::Error;

/// Server start up errors
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Server initialization failed")]
    InitializationError,
    #[error("Server start up failed")]
    StartUpError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}
