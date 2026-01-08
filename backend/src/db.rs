use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::error::AppResult;

pub async fn create_pool(database_url: &str) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    tracing::info!("Database connection pool established");

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> AppResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;

    tracing::info!("Database migrations completed");

    Ok(())
}
