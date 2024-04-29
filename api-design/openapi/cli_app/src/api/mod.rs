use crate::state::ApplicationState;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod errors;
mod handlers;
pub mod middleware;
pub mod request;
pub mod response;
mod v1;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url(
            "/v1/api-docs/openapi.json",
            crate::api::v1::ApiDoc::openapi(),
        ))
        .nest("/v1", v1::configure(state))
}
