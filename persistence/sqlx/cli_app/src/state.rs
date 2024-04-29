use crate::services::post::InMemoryPostService;
use crate::services::user::MySQLUserService;
use crate::settings::Settings;
use arc_swap::ArcSwap;
use sqlx::MySqlPool;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub user_service: Arc<MySQLUserService>,
    pub post_service: Arc<InMemoryPostService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, pool: MySqlPool) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            user_service: Arc::new(MySQLUserService::new(pool)),
            post_service: Arc::new(InMemoryPostService::default()),
        })
    }
}
