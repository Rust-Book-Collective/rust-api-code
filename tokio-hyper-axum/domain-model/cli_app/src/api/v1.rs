use super::handlers;
use crate::state::ApplicationState;
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/posts",
            post(handlers::posts::create).with_state(state.clone()),
        )
        .route(
            "/posts",
            get(handlers::posts::list).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            get(handlers::posts::get).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            put(handlers::posts::update).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            delete(handlers::posts::delete).with_state(state),
        )
}
