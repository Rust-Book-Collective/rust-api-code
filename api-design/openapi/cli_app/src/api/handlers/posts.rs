use crate::api::errors::AppError;
use crate::api::response::posts::ListPostsResponse;
use crate::api::response::posts::SinglePostResponse;
use crate::api::response::TokenClaims;
use crate::services::post::PostService;
use crate::services::post::{CreatePostRequest, UpdatePostRequest};
use crate::state::ApplicationState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/posts",
    tag = "posts",
    request_body = CreatePostRequest,
    responses(
        (status = 200, description = "Post create", body = SinglePostResponse),
    ),
)]
pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.create_post(payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

#[utoipa::path(
    put,
    path = "/posts/{id}",
    params(
        ("id" = i64, Path, description = "ID of the post"),
    ),
    tag = "posts",
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updates", body = SinglePostResponse),
    ),
)]
pub async fn update(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.update_post(id, payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/posts",
    tag = "posts",
    responses(
        (status = 200, description = "List of posts", body = ListPostsResponse),
    ),
)]
pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostsResponse>, AppError> {
    let posts = state.post_service.get_all_posts().await?;

    let response = ListPostsResponse { data: posts };

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/posts/{slug}",
    tag = "posts",
    params(
        ("slug" = String, Path, description = "Slug of the post"),
    ),
    responses(
        (status = 200, description = "Post", body = SinglePostResponse),
    ),
)]
pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(slug): Path<String>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.get_post_by_slug(&slug).await;

    match post {
        Ok(post) => {
            let response = SinglePostResponse { data: post };

            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((StatusCode::NOT_FOUND, e))),
    }
}

#[utoipa::path(
    delete,
    path = "/posts/{id}",
    tag = "posts",
    params(
        ("id" = i64, Path, description = "ID of the post"),
    ),
    responses(
        (status = 200, description = "Post deleted"),
    ),
)]
pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<()>, AppError> {
    state.post_service.delete_post(id).await?;

    Ok(Json(()))
}
