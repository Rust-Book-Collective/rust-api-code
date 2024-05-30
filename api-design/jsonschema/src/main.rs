use chrono::{DateTime, Utc};
use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
pub enum PostStatus {
    Draft = 1,
    Published = 2,
}

#[derive(JsonSchema)]
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

fn main() {
    let schema = schema_for!(Post);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
