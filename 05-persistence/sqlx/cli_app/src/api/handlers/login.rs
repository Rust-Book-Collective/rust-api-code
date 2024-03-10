use crate::api::errors::AppError;
use crate::api::request::login::LoginRequest;
use crate::api::response::login::LoginResponse;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let secret = state
        .settings
        .load()
        .token_secret
        .clone()
        .unwrap_or("secret".to_string());
    let timeout = state.settings.load().token_timeout_seconds.unwrap_or(3600);

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp =
        (now + chrono::Duration::try_seconds(timeout).unwrap_or_default()).timestamp() as usize;
    let claims = TokenClaims {
        sub: payload.username,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or("".to_string());

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };

    Ok(Json(response))
}
