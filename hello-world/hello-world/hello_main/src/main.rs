use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello_json() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Hello, world!",
    };

    (StatusCode::OK, Json(response))
}

#[allow(dead_code)]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
