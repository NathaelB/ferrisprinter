use anyhow::{Context, Ok, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Postgres {
    pub pool: Arc<PgPool>,
}

impl Postgres {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .context("Failed to connect to the Postgres database")?;

        info!("Connected to the Postgres database");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}
