use crate::model::{Post, PostStatus};
use std::collections::HashMap;
use tokio::sync::Mutex;

trait PostService {
    async fn get_post_by_id(&self, id: i64) -> Option<Post>;
    async fn get_post_by_slug(&self, name: &str) -> Option<Post>;
    async fn create_post(&mut self, req: CreatePostRequest) -> anyhow::Result<Post>;
    async fn update_post(&mut self, req: UpdatePostRequest) -> anyhow::Result<Post>;
    async fn delete_post(&mut self, id: i64) -> anyhow::Result<()>;
}

pub struct CreatePostRequest {
    pub author_id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
}

pub struct UpdatePostRequest {
    pub id: i64,
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
    async fn get_post_by_id(&self, id: i64) -> Option<Post> {
        let data = self.data.lock().await;
        data.items.get(&id).map(|post| (*post).clone())
    }

    async fn get_post_by_slug(&self, slug: &str) -> Option<Post> {
        let data = self.data.lock().await;
        for (_id, post) in data.items.iter() {
            if post.slug == slug {
                return Some(post.clone());
            }
        }

        None
    }

    async fn create_post(&mut self, req: CreatePostRequest) -> anyhow::Result<Post> {
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

    async fn update_post(&mut self, req: UpdatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        let post = data
            .items
            .get_mut(&req.id)
            .ok_or(anyhow::anyhow!("Post not found: {}", req.id))?;

        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.status = req.status;

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", req.id)
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn delete_post(&mut self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            None => {
                anyhow::bail!("Post not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }
}
