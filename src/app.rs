use axum::Router;
use crate::middleware::auth::require_service_token;
use crate::routes;
use crate::state::AppState;

/// Собирает основной Router из суб-роутеров.
///
/// Принимает `AppState`, чтобы middleware мог использовать state
/// (например, `service_token` для аутентификации).
///
/// Архитектура:
/// - Публичные роуты (health) — без middleware.
/// - Защищённые роуты (bots, users, subscriptions) — с `require_service_token`.
///
/// `.route_layer()` vs `.layer()`:
/// - `.layer()` — применяется ко ВСЕМ запросам, даже к 404.
/// - `.route_layer()` — только к запросам, которые совпали с роутом.
///   Если путь не найден, middleware НЕ вызывается → стандартный 404.
pub fn create_router(state: AppState) -> Router {
    // Защищённые роуты — требуют service-token
    let protected = Router::new()
        .merge(routes::bots::router())
        // По мере добавления:
        // .merge(routes::users::router())
        // .merge(routes::subscriptions::router())
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            require_service_token,
        ));

    Router::new()
        // Публичные роуты — без middleware
        .merge(routes::health::router())
        // Защищённые роуты
        .merge(protected)
        // Финализируем: передаём state всем handler'ам
        .with_state(state)
}
