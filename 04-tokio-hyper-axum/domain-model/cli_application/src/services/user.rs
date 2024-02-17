use crate::model::{User, UserStatus};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::Mutex;

trait UserService {
    async fn get_user_by_id(&self, id: i64) -> Option<User>;
    async fn get_user_by_name(&self, name: &str) -> Option<User>;
    async fn create_user(&mut self, req: CreateUserRequest) -> anyhow::Result<User>;
    async fn update_user(&mut self, req: UpdateUserRequest) -> anyhow::Result<User>;
    async fn delete_user(&mut self, id: i64) -> anyhow::Result<()>;
}

pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}

pub struct UpdateUserRequest {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub last_login: Option<DateTime<Utc>>,
}

pub struct InMemoryUserStore {
    pub counter: i64,
    pub items: HashMap<i64, User>,
}
pub struct InMemoryUserService {
    data: Mutex<InMemoryUserStore>,
}

impl Default for InMemoryUserService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryUserStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

impl UserService for InMemoryUserService {
    async fn get_user_by_id(&self, id: i64) -> Option<User> {
        let data = self.data.lock().await;
        data.items.get(&id).map(|user| (*user).clone())
    }

    async fn get_user_by_name(&self, name: &str) -> Option<User> {
        let data = self.data.lock().await;
        for (_id, user) in data.items.iter() {
            if user.username == name {
                return Some(user.clone());
            }
        }

        None
    }

    async fn create_user(&mut self, req: CreateUserRequest) -> anyhow::Result<User> {
        let mut data = self.data.lock().await;
        data.counter += 1;
        let ts = chrono::offset::Utc::now();
        let user = User {
            id: data.counter,
            username: req.username,
            password: req.password,
            status: req.status,
            created: ts,
            updated: ts,
            last_login: None,
        };

        data.items.insert(user.id, user);

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("User not found: {}", data.counter)
            }
            Some(user) => Ok(user.clone()),
        }
    }

    async fn update_user(&mut self, req: UpdateUserRequest) -> anyhow::Result<User> {
        let mut data = self.data.lock().await;
        let user = data
            .items
            .get_mut(&req.id)
            .ok_or(anyhow::anyhow!("User not found: {}", req.id))?;

        user.username = req.username;
        user.password = req.password;
        user.status = req.status;
        user.last_login = req.last_login;

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("User not found: {}", req.id)
            }
            Some(user) => Ok(user.clone()),
        }
    }

    async fn delete_user(&mut self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            None => {
                anyhow::bail!("User not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }
}
