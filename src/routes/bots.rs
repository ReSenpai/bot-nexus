use axum::{routing::get, Router};
use crate::handlers;
use crate::state::AppState;

/// Суб-роутер для эндпоинтов /bots.
///
/// Пока содержит заглушку GET /bots (вернёт пустой список).
/// Middleware для проверки токена применяется НЕ здесь,
/// а в `app.rs` — чтобы все защищённые роуты были в одном месте.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/bots", get(handlers::bots::list_bots))
}
