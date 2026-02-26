use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::handlers;
use crate::state::AppState;

/// Суб-роутер для задач внутри TODO-листов.
///
/// Все маршруты вложены в /lists/:list_id/tasks — задача всегда
/// принадлежит конкретному списку.
pub fn router() -> Router<AppState> {
    Router::new()
        // POST /lists/:list_id/tasks — создать задачу
        .route("/lists/{list_id}/tasks", post(handlers::tasks::create))
        // GET /lists/:list_id/tasks — все задачи списка
        .route("/lists/{list_id}/tasks", get(handlers::tasks::get_all))
        // GET /lists/:list_id/tasks/:task_id — одна задача
        .route("/lists/{list_id}/tasks/{task_id}", get(handlers::tasks::get_one))
        // PUT /lists/:list_id/tasks/:task_id — обновить задачу
        .route("/lists/{list_id}/tasks/{task_id}", put(handlers::tasks::update))
        // DELETE /lists/:list_id/tasks/:task_id — удалить задачу
        .route("/lists/{list_id}/tasks/{task_id}", delete(handlers::tasks::delete))
}
