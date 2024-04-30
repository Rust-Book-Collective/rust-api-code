use anyhow::anyhow;
use argon2::Argon2;
use chrono::{DateTime, Utc};
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize)]
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

pub fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!(e.to_string()))?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_e| anyhow!("Failed to verify password"))
}

pub fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    if let Ok(hash) = argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash.to_string())
    } else {
        Err(anyhow!("Failed to hash password"))
    }
}
