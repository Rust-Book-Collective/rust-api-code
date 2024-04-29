use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub status: String,
    pub token: String,
}
