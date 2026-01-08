mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;

use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "expense_tracker_backend=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    db::run_migrations(&pool).await?;

    let state = AppState {
        pool,
        config: Arc::new(config.clone()),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = routes::create_router(state).layer(cors);

    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;

    tracing::info!("Server listening on {}", config.server_address());

    axum::serve(listener, app).await?;

    Ok(())
}
