use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse
};
use axum::body::Body;

use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::api::errors::AppError;

pub async fn auth(
    State(state): State<Arc<ApplicationState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            auth_value
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });

    let token = token.ok_or_else(|| {
        AppError::from((StatusCode::UNAUTHORIZED, anyhow::anyhow!("Missing bearer token")))
    })?;

    let secret = state.settings.load().token_secret.clone().unwrap_or("secret".to_string());

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
        .map_err(|_| {
            AppError::from((StatusCode::UNAUTHORIZED, anyhow::anyhow!("Invalid bearer token")))
        })?
        .claims;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}