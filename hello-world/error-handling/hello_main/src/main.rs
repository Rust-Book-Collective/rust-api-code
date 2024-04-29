use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

struct AppError(anyhow::Error);

// This allows ? to automatically convert anyhow::Error to AppError
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.0)).into_response()
    }
}

async fn hello_json() -> Result<(StatusCode, Json<Response>), AppError> {
    let response = Response {
        message: generate_message().context("failed to generate message")?,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Generates the hello world message.
fn generate_message() -> anyhow::Result<&'static str> {
    if rand::random() {
        anyhow::bail!("no message for you");
    }
    Ok("Hello, world!")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello_json))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
