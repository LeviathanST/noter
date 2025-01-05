use std::sync::Arc;

use async_std::task::block_on;
use bevy::prelude::Resource;
use color_eyre::Result;
use config::{Config, File};
use sqlx::SqlitePool;

use crate::views::KeyConfig;

#[derive(PartialEq, Default)]
pub enum Page {
    #[default]
    Todo,
}
#[derive(Resource)]
pub struct GlobalResource {
    pub pool: Arc<SqlitePool>,
    pub key_config: KeyConfig,
    pub page: Page,
}

impl Default for GlobalResource {
    fn default() -> Self {
        let pool = setup_db().unwrap();
        let config_file = File::with_name("config");
        let key_config = Config::builder()
            .add_source(config_file.required(true))
            .build()
            .unwrap();

        let key_config = key_config
            .try_deserialize::<KeyConfig>()
            .map_err(|err| print!("{:?}", err))
            .unwrap();
        Self {
            pool: pool.into(),
            key_config,
            page: Page::default(),
        }
    }
}

fn setup_db() -> Result<SqlitePool> {
    let pool = block_on(async move { SqlitePool::connect("sqlite:schema.db").await })?;
    Ok(pool)
}
