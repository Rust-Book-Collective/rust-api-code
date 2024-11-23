use crate::model::Post;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SinglePostResponse {
    pub data: Post,
}

#[derive(Serialize, ToSchema)]
pub struct ListPostsResponse {
    pub data: Vec<Post>,
}
