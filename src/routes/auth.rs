use axum::{Router, routing::post};
use crate::handlers;
use crate::state::AppState;

/// Суб-роутер для авторизации.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
}
