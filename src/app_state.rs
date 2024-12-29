use std::sync::Arc;

use anyhow::Result;
use config::{Config, File};
use sqlx::SqlitePool;
use widgetui::State;

use crate::views::KeyConfig;

#[derive(PartialEq, Default)]
pub enum Page {
    #[default]
    Todo,
}
#[derive(State)]
pub struct AppState {
    pub pool: Arc<SqlitePool>,
    pub key_config: KeyConfig,
    pub page: Page,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePool::connect("sqlite:schema.db").await?;
        let config_file = File::with_name("config");
        let key_config = Config::builder()
            .add_source(config_file.required(true))
            .build()
            .unwrap();

        let key_config = key_config
            .try_deserialize::<KeyConfig>()
            .map_err(|err| print!("{:?}", err))
            .unwrap();
        Ok(Self {
            pool: pool.into(),
            key_config,
            page: Page::default(),
        })
    }
}
