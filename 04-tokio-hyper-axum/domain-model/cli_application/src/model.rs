use chrono::{DateTime, Utc};

#[derive(Copy, Clone)]
pub enum UserStatus {
    Active = 1,
    Blocked = 2,
}

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone)]
pub enum PostStatus {
    Draft = 1,
    Published = 2,
}
#[derive(Clone)]
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
