#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

#[cfg(all(feature = "swagger", debug_assertions))]
use axum::routing::get;

#[allow(unused_imports)]
use prelude::*;

pub mod error;
pub mod grpc;
pub mod rest;
pub mod service;

pub struct ApiV1;

impl<'a> ApiVersion<'a> for ApiV1 {
    fn to_path() -> &'a str {
        "/api/v1"
    }
}

#[cfg_attr(all(feature = "swagger", debug_assertions), derive(OpenApi))]
#[cfg_attr(all(feature = "swagger", debug_assertions), openapi(
    paths(
        rest::hello,
    ),
    components(
        schemas(
            // Some Structs,
        )
    ),
    tags(
        (name = "bob-fusion", description = "BobFusion API")
    )
))]
pub struct ApiDoc;

/// Generate openapi documentation for the project
///
/// # Panics
///
/// Panics if `OpenAPI` couldn't be converted into YAML format
#[cfg(all(feature = "swagger", debug_assertions))]
#[allow(clippy::expect_used)]
pub fn openapi_doc() -> Router {
    use utoipa_rapidoc::RapiDoc;
    use utoipa_redoc::{Redoc, Servable};
    use utoipa_swagger_ui::SwaggerUi;

    /* Swagger-only routes */
    tracing::info!("Swagger ui available at /swagger-ui");

    /* Mount Swagger ui */
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .route(
            "/api-docs/openapi.yaml",
            get(|| async {
                ApiDoc::openapi()
                    .to_yaml()
                    .expect("Couldn't produce .yaml API scheme")
            }),
        )
    // Alternative to above
    // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
}

pub mod prelude {
    pub use crate::{ApiDoc, ApiV1};
    #[cfg(feature = "otlp-exporter")]
    pub use autometrics::{
        autometrics,
        objectives::{Objective, ObjectiveLatency, ObjectivePercentile},
    };
    pub use axum::{Extension, Json, Router};
    #[cfg(feature = "otlp-exporter")]
    pub use bob_fusion_core::config::API_SLO;
    pub use bob_fusion_core::router::ApiVersion;
    pub use error_stack::{Context, Report, Result, ResultExt};
    pub use hyper::{Method, Request, StatusCode};
    pub use serde::{Deserialize, Serialize};
    pub use std::marker::PhantomData;
    pub use thiserror::Error;
    #[cfg(all(feature = "swagger", debug_assertions))]
    pub use utoipa::{
        openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
        IntoParams, Modify, OpenApi, PartialSchema, ToSchema,
    };
}

pub mod main {
    pub mod prelude {
        #[cfg(all(feature = "swagger", debug_assertions))]
        pub use crate::openapi_doc;
        pub use crate::{error::AppError, grpc::grpc_service, rest::api_router_v1, ApiV1};
        pub use axum::Router;
        pub use bob_fusion_core::{
            config::{ConfigExt, LoggerExt},
            router::{ApiGeneric, ApiVersion},
            services::HybridMakeService,
        };
        pub use cli::{Config, Parser};
        pub use error_stack::{Result, ResultExt};
        pub use hyper::StatusCode;
        pub use tower::ServiceBuilder;
    }
}
