use crate::prelude::*;
use cli::{Config, LoggerConfig};
use file_rotate::{suffix::AppendTimestamp, ContentLimit, FileRotate};
use tower_http::cors::CorsLayer;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{filter::LevelFilter, prelude::*, util::SubscriberInitExt};

#[cfg(feature = "otlp-exporter")]
use autometrics::objectives::{Objective, ObjectiveLatency, ObjectivePercentile};
#[cfg(feature = "otlp-exporter")]
use opentelemetry::KeyValue;
#[cfg(feature = "otlp-exporter")]
use opentelemetry_otlp::WithExportConfig;
#[cfg(feature = "otlp-exporter")]
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
#[cfg(feature = "otlp-exporter")]
use tracing_opentelemetry::OpenTelemetryLayer;

#[cfg(feature = "otlp-exporter")]
pub const API_SLO: Objective = Objective::new("api")
    // We expect 99.9% of all requests to succeed.
    .success_rate(ObjectivePercentile::P99_9)
    // We expect 99% of all latencies to be below 250ms.
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

#[allow(clippy::module_name_repetitions)]
pub trait ConfigExt {
    /// Return either very permissive [`CORS`](`CorsLayer`) configuration
    /// or empty one based on `cors_allow_all` field
    fn get_cors_configuration(&self) -> CorsLayer;
}

pub trait LoggerExt {
    /// Initialize logger.
    ///
    /// Returns [`WorkerGuard`]s for off-thread writers.
    /// Should not be dropped.
    ///
    /// # Errors
    ///
    /// Function returns error if `init_file_rotate` fails
    fn init_logger(&self) -> Result<Vec<WorkerGuard>, LoggerError>;

    /// Returns [`std:io::Write`] object that rotates files on write
    ///
    /// # Errors
    ///
    /// Function returns error if `log_file` is not specified
    fn init_file_rotate(&self) -> Result<FileRotate<AppendTimestamp>, LoggerError>;

    /// Returns non-blocking file writer
    ///
    /// Also returns [`WorkerGuard`] for off-thread writing.
    /// Should not be dropped.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file logger configuration is empty, file logging
    /// is disabled or logs filename is not specified
    fn non_blocking_file_writer(&self) -> Result<(NonBlocking, WorkerGuard), LoggerError>;

    /// Returns non-blocking stdout writer
    ///
    /// Also returns [`WorkerGuard`] for off-thread writing.
    /// Should not be dropped.
    ///
    /// # Errors
    ///
    /// This function will return an error if the stdout logger configuration is empty or stdout logging
    /// is disabled
    fn non_blocking_stdout_writer(&self) -> Result<(NonBlocking, WorkerGuard), LoggerError>;

    /// Init OTLP exporter
    #[cfg(feature = "otlp-exporter")]
    fn init_metrics<
        S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
    >(
        &self,
    ) -> Result<OpenTelemetryLayer<S, opentelemetry_sdk::trace::Tracer>, LoggerError>;
}

impl ConfigExt for Config {
    fn get_cors_configuration(&self) -> CorsLayer {
        self.cors_allow_all
            .then_some(CorsLayer::very_permissive())
            .unwrap_or_default()
    }
}

impl LoggerExt for LoggerConfig {
    fn init_logger(&self) -> Result<Vec<WorkerGuard>, LoggerError> {
        let mut guards = Vec::with_capacity(2);

        let file_writer = disable_on_error(self.non_blocking_file_writer())?;
        let stdout_writer = disable_on_error(self.non_blocking_stdout_writer())?;

        let mut layers_iter =
            [file_writer, stdout_writer]
                .into_iter()
                .flatten()
                .map(|(writer, guard)| {
                    guards.push(guard);
                    tracing_subscriber::fmt::layer()
                        .with_writer(writer)
                        .with_filter(LevelFilter::from_level(self.trace_level))
                });

        if let Some(first_layer) = layers_iter.next() {
            let layers = layers_iter.fold(first_layer.boxed(), |layer, next_layer| {
                layer.and_then(next_layer).boxed()
            });
            #[cfg(feature = "otlp-exporter")]
            let layers = layers.and_then(
                self.init_metrics()
                    .change_context(LoggerError::OLTPInitFailed)?,
            );
            tracing_subscriber::registry().with(layers).init();
        };

        Ok(guards)
    }

    fn init_file_rotate(&self) -> Result<FileRotate<AppendTimestamp>, LoggerError> {
        let config = self.file.as_ref().ok_or(LoggerError::EmptyConfig)?;
        let log_file = config.log_file.as_ref().ok_or(LoggerError::NoFileName)?;
        if log_file.as_os_str().is_empty() {
            return Err(LoggerError::NoFileName.into());
        }

        Ok(FileRotate::new(
            log_file,
            AppendTimestamp::default(file_rotate::suffix::FileLimit::MaxFiles(config.log_amount)),
            ContentLimit::BytesSurpassed(config.log_size),
            file_rotate::compression::Compression::OnRotate(1),
            None,
        ))
    }

    fn non_blocking_file_writer(&self) -> Result<(NonBlocking, WorkerGuard), LoggerError> {
        self.file.as_ref().map_or_else(
            || Err(LoggerError::EmptyConfig.into()),
            |config| {
                if config.enabled {
                    Ok(tracing_appender::non_blocking(self.init_file_rotate()?))
                } else {
                    Err(LoggerError::NotEnabled.into())
                }
            },
        )
    }

    fn non_blocking_stdout_writer(&self) -> Result<(NonBlocking, WorkerGuard), LoggerError> {
        self.stdout.as_ref().map_or_else(
            || Err(LoggerError::EmptyConfig.into()),
            |config| {
                if config.enabled {
                    Ok(tracing_appender::non_blocking(std::io::stdout()))
                } else {
                    Err(LoggerError::NotEnabled.into())
                }
            },
        )
    }

    #[cfg(feature = "otlp-exporter")]
    fn init_metrics<
        S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
    >(
        &self,
    ) -> Result<OpenTelemetryLayer<S, opentelemetry_sdk::trace::Tracer>, LoggerError> {
        autometrics::prometheus_exporter::init();
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint("http://localhost:4317"),
            )
            .with_trace_config(
                sdktrace::config()
                    .with_resource(Resource::new(vec![KeyValue::new("service.name", "bob")])),
            )
            .install_batch(runtime::Tokio)
            .change_context(LoggerError::OLTPInitFailed)?;

        // let export_config = ExportConfig {
        //     endpoint: "http://localhost:4317".to_string(),
        //     timeout: Duration::from_secs(3),
        //     protocol: opentelemetry_otlp::Protocol::HttpBinary,
        // };

        // let meter = opentelemetry_otlp::new_pipeline()
        //     .metrics(opentelemetry_sdk::runtime::Tokio)
        //     .with_exporter(
        //         opentelemetry_otlp::new_exporter()
        //             .tonic()
        //             .with_export_config(export_config),
        //         // can also config it using with_* functions like the tracing part above.
        //     )
        //     .with_resource(Resource::new(vec![KeyValue::new(
        //         "service.name",
        //         "example",
        //     )]))
        //     .with_period(Duration::from_secs(3))
        //     .with_timeout(Duration::from_secs(10))
        //     .with_aggregation_selector(DefaultAggregationSelector::new())
        //     .with_temporality_selector(DefaultTemporalitySelector::new())
        //     .build()
        //     .unwrap();
        // // let metrics = init_metrics();
        // let opentelemetry_metrics = MetricsLayer::new(meter);
        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        // tracing_subscriber::registry()
        //     .with(
        //         tracing_subscriber::filter::EnvFilter::from_default_env()
        //             .add_directive(trace_level.into()),
        //     )
        //     .with(opentelemetry)
        //     // .with(opentelemetry_metrics)
        //     // .with(sentry::integrations::tracing::layer())
        //     // .with(EnvFilter::from_default_env().add_directive(trace_level.into()))
        //     .with(tracing_subscriber::fmt::layer())
        //     .try_init()
        //     .change_context(LoggerError::OLTPInitFailed)
        // subscriber.init();
        Ok(opentelemetry)
    }
}

// #[cfg(feature = "otlp-exporter")]
// pub fn init_metrics() -> AppState {
//     let registry = Registry::new();
//     (&registry)
//         .register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
//         .unwrap();
//     (&registry)
//         .register(Box::new(HTTP_CONNECTED_SSE_CLIENTS.clone()))
//         .unwrap();
//     (&registry)
//         .register(Box::new(HTTP_RESPONSE_TIME_SECONDS.clone()))
//         .unwrap();
//     // let exporter = opentelemetry_prometheus::exporter()
//     //     .with_registry(registry)
//     //     .build()
//     //     .unwrap();
//     // // let provider = MeterProviderBuilder::default()
//     // //     .with_reader(exporter)
//     // //     .build();
//     // // // let meter = provider.m
//     // // // let state = Arc::new(Metrics { exporter });
//     // // //
//     // // // state
//     // // // (exporter, provider)
//     // // provider
//     // exporter
//     let registry = prometheus::Registry::new();
//     let exporter = opentelemetry_prometheus::exporter()
//         .with_registry(registry.clone())
//         .build()
//         .unwrap();
//
//     let res = Resource::from_detectors(
//         Duration::from_secs(0),
//         vec![
//             Box::new(SdkProvidedResourceDetector),
//             Box::new(EnvResourceDetector::new()),
//             Box::new(TelemetryResourceDetector),
//             Box::new(OsResourceDetector),
//             Box::new(ProcessResourceDetector),
//         ],
//     )
//     .merge(&mut Resource::new(
//         vec![
//             KeyValue::new(SERVICE_NAME, "BOB"),
//             KeyValue::new(TELEMETRY_SDK_VERSION, "latest"),
//         ]
//         .into_iter(),
//     ));
//
//     let provider = MeterProviderBuilder::default()
//         .with_resource(res)
//         .with_reader(exporter)
//         .with_view(
//             new_view(
//                 Instrument::new().name("histogram_*"),
//                 Stream::new().aggregation(Aggregation::ExplicitBucketHistogram {
//                     boundaries: vec![
//                         0.0, 5.0, 10.0, 25.0, 50.0, 75.0, 100.0, 250.0, 500.0, 1000.0,
//                     ],
//                     record_min_max: true,
//                 }),
//             )
//             .unwrap(),
//         )
//         .build();
//
//     let meter = provider.meter("BOB");
//     let state = AppState {
//         registry,
//         provider,
//         http_counter: meter
//             .u64_counter("http_requests_total")
//             .with_description("Total number of HTTP requests made.")
//             .init(),
//         client_counter: meter
//             .i64_up_down_counter("http_connected_clients")
//             .with_description("Total number of Clients.")
//             .init(),
//         http_body_gauge: meter
//             .u64_histogram("http_response_size")
//             .with_unit(Unit::new("By"))
//             .with_description("The metrics HTTP response sizes in bytes.")
//             .init(),
//         http_req_histogram: meter
//             .f64_histogram("http_request_duration")
//             .with_unit(Unit::new("ms"))
//             .with_description("The HTTP request latencies in milliseconds.")
//             .init(),
//     };
//
//     state
// }
//
// #[cfg(feature = "otlp-exporter")]
// pub async fn metrics_routes() -> Router<()> {
//     // let app_state = Metrics { exporter };
//     //
//     Router::new()
//         // .with_state(app_state)
//         //
//         .route("/metrics", get(exporter_handler))
//     // .route(
//     //     "/metrics",
//     //     get(|| async { autometrics::prometheus_exporter::encode_http_response() }),
//     // )
// }
// //
// #[cfg(feature = "otlp-exporter")]
// // // #[tracing::instrument]
// pub async fn exporter_handler() -> autometrics::prometheus_exporter::PrometheusResponse {
//     //     //     let mut buffer = Vec::new();
//     //     //     let encoder = TextEncoder::new();
//     //     //     encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
//     //     //     encoder.encode(&state.gather(), &mut buffer).unwrap();
//     //     //     let metrics = String::from_utf8(buffer).unwrap();
//     //     //     metrics
//     autometrics::prometheus_exporter::encode_http_response()
// }

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("Empty logger configuration")]
    EmptyConfig,
    #[error("No filename specified")]
    NoFileName,
    #[error("This logger is not enabled")]
    NotEnabled,
    #[error("OLTP init failed")]
    OLTPInitFailed,
}

/// Consume some errors to produce empty logger
fn disable_on_error(
    logger: Result<(NonBlocking, WorkerGuard), LoggerError>,
) -> Result<Option<(NonBlocking, WorkerGuard)>, LoggerError> {
    Ok(match logger {
        Ok(writer) => Some(writer),
        Err(e) => match e.current_context() {
            LoggerError::NotEnabled | LoggerError::EmptyConfig => None,
            _ => return Err(e),
        },
    })
}
