use axum::{Router, routing::get};
use crate::handlers;
use crate::state::AppState;

/// Суб-роутер для health-check.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handlers::health::health_check))
}
