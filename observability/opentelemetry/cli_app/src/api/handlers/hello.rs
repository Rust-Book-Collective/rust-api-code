use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    get,
    path = "/hello",
    tag = "hello",
    responses(
        (status = 200, description = "Hello World", body = String),
    ),
)]
#[instrument(skip(state))]
pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    tracing::info!("Hello world!");
    Ok(format!(
        "\nHello world! Using configuration from {}\n\n",
        state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("[nowhere]".to_string())
    ))
}
