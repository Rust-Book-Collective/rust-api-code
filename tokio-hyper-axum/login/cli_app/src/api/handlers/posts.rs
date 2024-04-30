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

pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.create_post(payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn update(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.update_post(id, payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostsResponse>, AppError> {
    let posts = state.post_service.get_all_posts().await?;

    let response = ListPostsResponse { data: posts };

    Ok(Json(response))
}

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

pub async fn delete(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<()>, AppError> {
    state.post_service.delete_post(id).await?;

    Ok(Json(()))
}
