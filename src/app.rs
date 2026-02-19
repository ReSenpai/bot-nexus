use axum::{Router, routing::get};
use crate::handlers;

/// Создаёт Router приложения со всеми маршрутами.
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health::health_check))
}
