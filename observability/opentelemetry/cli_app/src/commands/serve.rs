use crate::settings::Settings;
use crate::state::ApplicationState;
use clap::{value_parser, Arg, ArgMatches, Command};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

use crate::settings::OtlpTarget;
use opentelemetry::trace::{TraceError, TracerProvider};
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, Tracer};
use opentelemetry_sdk::{runtime, trace, Resource};
use std::collections::HashMap;

pub const COMMAND_NAME: &str = "serve";

pub fn init_tracer(otlp_target: &OtlpTarget) -> Result<Tracer, TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let otlp_endpoint = otlp_target.address.as_str();

    let mut builder = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(otlp_endpoint);

    if let Some(authorization) = &otlp_target.authorization {
        let mut headers = HashMap::new();
        headers.insert(String::from("Authorization"), authorization.clone());
        builder = builder.with_headers(headers);
    };

    let exporter = builder.build()?;

    let tracer_provider = trace::TracerProvider::builder()
        .with_batch_exporter(exporter, runtime::Tokio)
        .with_config(
            trace::Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "sample_application",
                )])),
        )
        .build();

    Ok(tracer_provider.tracer("sample_application"))
}

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    let port: u16 = *matches.get_one("port").unwrap_or(&8080);

    start_tokio(port, settings)?;

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let telemetry_layer = if let Some(otlp_target) = settings.logging.otlp_target.clone() {
                let tracer = init_tracer(&otlp_target)?;
                Some(tracing_opentelemetry::layer().with_tracer(tracer))
            } else {
                None
            };

            let stdout_log = tracing_subscriber::fmt::layer().with_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or(tracing_subscriber::EnvFilter::new("info")),
            );

            let subscriber = tracing_subscriber::registry()
                .with(telemetry_layer)
                .with(stdout_log);

            subscriber.init();

            let db_url = settings
                .database
                .url
                .clone()
                .expect("Database URL is not set");
            let pool = sqlx::MySqlPool::connect(&db_url).await?;

            let state = Arc::new(ApplicationState::new(settings, pool)?);
            let router = crate::api::configure(state).layer(TraceLayer::new_for_http());

            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router.into_make_service()).await?;

            Ok::<(), anyhow::Error>(())
        })?;

    Ok(())
}
