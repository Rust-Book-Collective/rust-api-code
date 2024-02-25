use crate::state::ApplicationState;
use axum::Router;
use std::sync::Arc;

pub mod errors;
mod handlers;
pub mod response;
pub mod request;
mod v1;
pub mod middleware;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}
