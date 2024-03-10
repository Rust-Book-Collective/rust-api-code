use crate::model::{User, UserStatus};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[allow(async_fn_in_trait)]
pub trait UserService {
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User>;
    async fn get_user_by_name(&self, name: &str) -> anyhow::Result<User>;
    async fn create_user(&mut self, req: CreateUserRequest) -> anyhow::Result<User>;
    async fn update_user(&mut self, id: i64, req: UpdateUserRequest) -> anyhow::Result<User>;
    async fn delete_user(&mut self, id: i64) -> anyhow::Result<()>;
}

pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}

pub struct UpdateUserRequest {
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
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let data = self.data.lock().await;
        match data.items.get(&id) {
            Some(user) => Ok((*user).clone()),
            None => anyhow::bail!("User not found: {}", id),
        }
    }

    async fn get_user_by_name(&self, name: &str) -> anyhow::Result<User> {
        let data = self.data.lock().await;
        for (_id, user) in data.items.iter() {
            if user.username == name {
                return Ok(user.clone());
            }
        }

        anyhow::bail!("User not found: {}", name)
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

    async fn update_user(&mut self, id: i64, req: UpdateUserRequest) -> anyhow::Result<User> {
        let mut data = self.data.lock().await;
        let user = data
            .items
            .get_mut(&id)
            .ok_or(anyhow::anyhow!("User not found: {}", id))?;

        user.username = req.username;
        user.password = req.password;
        user.status = req.status;
        user.last_login = req.last_login;

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("User not found: {}", id)
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

pub struct MySQLUserService {
    pub pool: MySqlPool,
}

impl MySQLUserService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl UserService for MySQLUserService {
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            WHERE id = ?
            "#,
            id
        );

        res.fetch_one(&self.pool)
            .await
            .map(|row| User {
                id: row.id as i64,
                username: row.username,
                password: row.password,
                status: UserStatus::from(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
                last_login: row.last_login,
            })
            .map_err(|e| anyhow::anyhow!(e).context(format!("Failed to get user by id: {}", id)))
    }

    async fn get_user_by_name(&self, name: &str) -> anyhow::Result<User> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            WHERE username = ?
            "#,
            name
        );

        res.fetch_one(&self.pool)
            .await
            .map(|row| User {
                id: row.id as i64,
                username: row.username,
                password: row.password,
                status: UserStatus::from(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
                last_login: row.last_login,
            })
            .map_err(|e| {
                anyhow::anyhow!(e).context(format!("Failed to get user by name: {}", name))
            })
    }

    async fn create_user(&mut self, req: CreateUserRequest) -> anyhow::Result<User> {
        todo!()
    }

    async fn update_user(&mut self, id: i64, req: UpdateUserRequest) -> anyhow::Result<User> {
        todo!()
    }

    async fn delete_user(&mut self, id: i64) -> anyhow::Result<()> {
        todo!()
    }
}
