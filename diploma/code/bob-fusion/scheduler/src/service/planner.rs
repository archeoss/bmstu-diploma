use crate::error::AppError;
use crate::prelude::*;
use crate::rest::api_router_v1;
use crate::{grpc::grpc_service, openapi_doc};
use axum::response::{IntoResponse, Response};
use axum::routing::IntoMakeService;
use bob_fusion_core::config::ConfigExt;
use bob_fusion_core::{config::LoggerExt, services::HybridMakeService};
use cli::Config;
use hyper::server::conn::AddrIncoming;
use hyper::{Body, StatusCode};
use thiserror::Error;
use tonic::transport::server::Routes;
use tower::ServiceBuilder;
use tracing_appender::non_blocking::WorkerGuard;

pub mod prelude {}

pub struct SchedulerServer {
    config: cli::Config,
    logger_guards: Vec<WorkerGuard>,
    services: HybridMakeService<IntoMakeService<Router<(), Body>>, Routes>, // server: axum::Server<AddrIncoming, HybridMakeService<(), ()>>,
}

impl SchedulerServer {
    pub fn from_config(config: cli::Config) -> Result<Self, AppError> {
        let logger = &config.logger;

        let logger_guards = logger.init_logger().unwrap();
        tracing::info!("Logger: {logger:?}");

        let addr = config.address;
        tracing::info!("Listening on {addr}");

        let app = router(&config);
        #[cfg(all(feature = "swagger", debug_assertions))]
        let app = app.merge(openapi_doc());

        let grpc = grpc_service();
        // let service = MultiplexService::new(app, grpc);

        let services = HybridMakeService::new(app.clone().into_make_service(), grpc);

        Ok(Self {
            config,
            logger_guards,
            services, // server,
        })
    }

    pub async fn start(self) -> Result<(), AppError> {
        axum::Server::bind(&self.config.address)
            .serve(self.services)
            .await
            .change_context(AppError::StartUpError)
            .attach_printable("Failed to start axum server")
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
fn router(config: &Config) -> Router {
    let router = Router::new();

    let router = router.nest(
        ApiV1::to_path(),
        api_router_v1()
            .expect("couldn't get API routes")
            .layer(ServiceBuilder::new().layer(config.get_cors_configuration())),
    );

    #[cfg(feature = "otlp-exporter")]
    let router = {
        // let state = init_metrics();
        router
            // .merge(bob_fusion::config::metrics_routes())
            .route(
                "/metrics",
                axum::routing::get(|| async {
                    autometrics::prometheus_exporter::encode_to_string()
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
                }),
            )
    };

    router
}
