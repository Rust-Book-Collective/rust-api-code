use crate::services::post::InMemoryPostService;
use crate::services::user::InMemoryUserService;
use crate::settings::Settings;
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub user_service: Arc<InMemoryUserService>,
    pub post_service: Arc<InMemoryPostService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            user_service: Arc::new(InMemoryUserService::default()),
            post_service: Arc::new(InMemoryPostService::default()),
        })
    }
}
