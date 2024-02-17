use crate::state::ApplicationState;
use axum::Router;
use std::sync::Arc;

pub mod errors;
mod handlers;
pub mod response;
mod v1;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}
