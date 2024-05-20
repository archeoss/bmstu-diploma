use crate::prelude::*;
use bob_fusion_core::router::{RouteError, RouterApiExt};

/// API Router V1
///
/// # Errors
///
/// This function will return an error if one of the previously provided
/// path/handler/method combinations with `api_route` does not have a corresponding `OpenAPI` declaration.
/// The returned error in the form of a `Report` will contain all errors during new API route registrartion.
#[tracing::instrument]
pub fn api_router_v1() -> Result<Router<()>, RouteError> {
    Router::new()
        .with_context::<ApiV1, ApiDoc>()
        .api_route("/hello", &Method::GET, hello)
        .unroll()
}

/// Hi
#[allow(clippy::unused_async)]
#[cfg_attr(all(feature = "swagger", debug_assertions), utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/hello",
        responses(
            (status = 200, description = "Hello World!")
        )
    ))]
#[tracing::instrument]
#[cfg_attr(feature = "otlp-exporter", autometrics(objective = API_SLO))]
pub async fn hello() -> &'static str {
    tracing::info!("Got a http hello request.");
    "Hello World!"
}
