use crate::model::{Post, PostStatus};
use serde::Deserialize;
use sqlx::MySqlPool;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[allow(async_fn_in_trait)]
pub trait PostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post>;
    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post>;
    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post>;
    async fn delete_post(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub author_id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
}

pub struct InMemoryPostStore {
    pub counter: i64,
    pub items: HashMap<i64, Post>,
}
pub struct InMemoryPostService {
    data: Mutex<InMemoryPostStore>,
}

impl Default for InMemoryPostService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryPostStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

impl PostService for InMemoryPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        let data = self.data.lock().await;
        Ok(data.items.values().map(|post| (*post).clone()).collect())
    }

    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let data = self.data.lock().await;

        match data.items.get(&id) {
            Some(post) => Ok((*post).clone()),
            None => anyhow::bail!("Post not found: {}", id),
        }
    }

    async fn get_post_by_slug(&self, slug: &str) -> anyhow::Result<Post> {
        let data = self.data.lock().await;
        for (_id, post) in data.items.iter() {
            if post.slug == slug {
                return Ok(post.clone());
            }
        }

        anyhow::bail!("Post not found: {}", slug)
    }

    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        data.counter += 1;
        let ts = chrono::offset::Utc::now();
        let post = Post {
            id: data.counter,
            author_id: req.author_id,
            slug: req.slug,
            title: req.title,
            content: req.content,
            status: req.status,
            created: ts,
            updated: ts,
        };

        data.items.insert(post.id, post);

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", data.counter)
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        let post = data
            .items
            .get_mut(&id)
            .ok_or(anyhow::anyhow!("Post not found: {}", id))?;

        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.status = req.status;

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", id)
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            None => {
                anyhow::bail!("Post not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }
}

pub struct MySQLPostService {
    pub pool: MySqlPool,
}

impl MySQLPostService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl PostService for MySQLPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        todo!()
    }

    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let res = sqlx::query!(
            r#"
                SELECT id, author_id, slug, title, content, status, created, updated
                FROM posts
                WHERE id = ?
            "#,
            id
        );

        res.fetch_one(&self.pool)
            .await
            .map(|row| Post {
                id: row.id as i64,
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
                author_id: row.author_id as i64,
                slug: row.slug,
                title: row.title,
                content: row.content,
                status: PostStatus::from(row.status),
            })
            .map_err(|e| anyhow::anyhow!(e).context(format!("Failed to get post by id: {}", id)))
    }

    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post> {
        let res = sqlx::query!(
            r#"
                SELECT id, author_id, slug, title, content, status, created, updated
                FROM posts
                WHERE slug = ?
            "#,
            name
        );

        res.fetch_one(&self.pool)
            .await
            .map(|row| Post {
                id: row.id as i64,
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
                author_id: row.author_id as i64,
                slug: row.slug,
                title: row.title,
                content: row.content,
                status: PostStatus::from(row.status),
            })
            .map_err(|e| {
                anyhow::anyhow!(e).context(format!("Failed to get post by slug: {}", name))
            })
    }

    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post> {
        let res = sqlx::query!(
            r#"
                INSERT INTO posts (author_id, slug, title, content, status, created, updated)
                VALUES (?, ?, ?, ?, ?, NOW(), NOW())
            "#,
            req.author_id,
            req.slug,
            req.title,
            req.content,
            i32::from(req.status)
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        let id: i64 = res
            .try_into()
            .or_else(|_| anyhow::bail!("Failed to convert post id"))?;

        self.get_post_by_id(id).await
    }

    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post> {
        let res = sqlx::query!(
            r#"
                UPDATE posts
                SET slug = ?, title = ?, content = ?, status = ?, updated = NOW()
                WHERE id = ?
            "#,
            req.slug,
            req.title,
            req.content,
            i32::from(req.status),
            id
        )
        .execute(&self.pool)
        .await?;

        if res.rows_affected() == 0 {
            anyhow::bail!("Post not found: {}", id)
        }

        let user = self.get_post_by_id(id).await?;

        Ok(user)
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                DELETE FROM posts
                WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
