use super::handlers;
use crate::api::middleware::auth::auth;
use crate::state::ApplicationState;
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};
use std::sync::Arc;

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
            "/posts/:id",
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

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello::hello,
        handlers::login::login,
        handlers::posts::create,
        handlers::posts::update,
        handlers::posts::delete,
        handlers::posts::list,
        handlers::posts::get,
    ),
    components(
        schemas(
            crate::api::request::login::LoginRequest,
            crate::api::response::login::LoginResponse,
            crate::services::post::CreatePostRequest,
            crate::services::post::UpdatePostRequest,
            crate::api::response::posts::ListPostsResponse,
            crate::api::response::posts::SinglePostResponse,
            crate::model::Post,
            crate::model::PostStatus,
        ),
    ),
    tags(
        (name = "hello", description = "Hello"),
        (name = "login", description = "Login"),
        (name = "posts", description = "Posts"),
    ),
    servers(
        (url = "/v1", description = "Local server"),
    ),
)]
pub struct ApiDoc;
