use crate::api::errors::AppError;
use crate::api::request::login::LoginRequest;
use crate::api::response::login::LoginResponse;
use crate::api::response::TokenClaims;
use crate::model::validate_password;
use crate::services::user::UserService;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = match state.user_service.get_user_by_name(&payload.username).await {
        Ok(user) => user,
        Err(_) => {
            return Err(AppError::from((
                StatusCode::UNAUTHORIZED,
                anyhow::anyhow!("Invalid username or password"),
            )))
        }
    };

    if validate_password(&payload.password, &user.password).is_err() {
        return Err(AppError::from((
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("Invalid username or password"),
        )));
    }

    let secret = state
        .settings
        .load()
        .token_secret
        .clone()
        .unwrap_or("secret".to_string());
    let timeout = state.settings.load().token_timeout_seconds.unwrap_or(3600);

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(timeout)).timestamp() as usize;
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
