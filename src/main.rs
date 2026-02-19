use sqlx::{postgres::PgPoolOptions};
use std::{env};
use tracing_subscriber::filter::EnvFilter;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");
    
    sqlx::query("SELECT 1")
    .execute(&pool)
    .await
    .expect("Failed to execute health-check query");

    tracing::info!("Database connection established and healthy.");

    let app_state = AppState { db: pool };
}
