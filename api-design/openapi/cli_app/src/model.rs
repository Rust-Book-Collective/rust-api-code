use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Copy, Clone, Serialize, Deserialize, ToSchema)]
pub enum UserStatus {
    Active = 1,
    Blocked = 2,
}

impl From<i32> for UserStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => UserStatus::Active,
            2 => UserStatus::Blocked,
            _ => UserStatus::Active,
        }
    }
}

impl From<UserStatus> for i32 {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Active => 1,
            UserStatus::Blocked => 2,
        }
    }
}

#[derive(Clone, Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Serialize, Deserialize, ToSchema)]
pub enum PostStatus {
    Draft = 1,
    Published = 2,
}

impl From<i32> for PostStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => PostStatus::Draft,
            2 => PostStatus::Published,
            _ => PostStatus::Draft,
        }
    }
}

impl From<PostStatus> for i32 {
    fn from(value: PostStatus) -> Self {
        match value {
            PostStatus::Draft => 1,
            PostStatus::Published => 2,
        }
    }
}

#[derive(Clone, Serialize, ToSchema)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
