use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::filter::EnvFilter;

use todo_api::app;
use todo_api::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Создаём пул соединений к PostgreSQL (до 5 соединений одновременно).
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Health-check запрос к БД — убеждаемся, что она отвечает.
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Failed to execute health-check query");

    tracing::info!("Database connection established and healthy.");

    // Автоматически применяем миграции при старте.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Database migrations applied successfully.");

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-key".to_string());

    let app_state = AppState {
        db: pool,
        jwt_secret,
    };

    // Создаём роутер и передаём ему state.
    let app = app::create_router().with_state(app_state);

    // Определяем адрес, на котором будет слушать сервер.
    let addr = "0.0.0.0:3000";
    tracing::info!("Server starting on {}", addr);

    // Запускаем HTTP-сервер.
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
