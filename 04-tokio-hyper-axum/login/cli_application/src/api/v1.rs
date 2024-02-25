use super::handlers;
use crate::state::ApplicationState;
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};
use std::sync::Arc;
use crate::api::middleware::auth::auth;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/posts",
            post(handlers::posts::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/posts",
            get(handlers::posts::list).with_state(state.clone()),
        )
        .route(
            "/posts/:slug",
            get(handlers::posts::get).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            put(handlers::posts::update).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            delete(handlers::posts::delete).with_state(state.clone()),
        )
        .route("/login", post(handlers::login::login).with_state(state))
}
