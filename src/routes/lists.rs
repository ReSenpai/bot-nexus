use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::handlers;
use crate::state::AppState;

/// Суб-роутер для TODO-листов.
pub fn router() -> Router<AppState> {
    Router::new()
        // POST /lists — создать список
        .route("/lists", post(handlers::lists::create))
        // GET /lists — все списки пользователя
        .route("/lists", get(handlers::lists::get_all))
        // GET /lists/:id — один список по ID
        .route("/lists/{id}", get(handlers::lists::get_one))
        // PUT /lists/:id — обновить список
        .route("/lists/{id}", put(handlers::lists::update))
        // DELETE /lists/:id — удалить список
        .route("/lists/{id}", delete(handlers::lists::delete))
}
