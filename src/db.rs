use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::str::FromStr;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://app.db".to_string());

        let options = SqliteConnectOptions::from_str(&database_url)?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Database { pool })
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}
