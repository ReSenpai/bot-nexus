use axum::{Router, routing::{get, post}};
use crate::handlers;
use crate::state::AppState;

/// Создаёт Router приложения со всеми маршрутами.
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Health-check 
        .route("/health", get(handlers::health::health_check))
        // Регистрация — POST /auth/register
        .route("/auth/register", post(handlers::auth::register))
}
