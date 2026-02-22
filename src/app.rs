use axum::Router;
use crate::routes;
use crate::state::AppState;

/// Собирает основной Router из суб-роутеров.
///
/// По мере добавления модулей (users, bots, subscriptions)
/// просто дописываем `.merge(routes::xxx::router())`.
pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(routes::health::router())
}
