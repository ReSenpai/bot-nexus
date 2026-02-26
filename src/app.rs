use axum::Router;
use crate::routes;
use crate::state::AppState;

/// Создаёт и возвращает основной Router приложения.
pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(routes::health::router())
        .merge(routes::auth::router())
        .merge(routes::lists::router())
        .merge(routes::tasks::router())
}
