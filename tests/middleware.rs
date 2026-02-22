/// Интеграционные тесты для Service Token middleware.
///
/// Middleware проверяет заголовок Authorization: Bearer <token>.
/// Публичные эндпоинты (/health) работают без токена.
/// Защищённые эндпоинты без токена возвращают 401.

mod common;

use bot_nexus::app::create_router;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

// ==================== PUBLIC ENDPOINTS ====================

/// GET /health должен работать БЕЗ токена.
/// Health-check — публичный эндпоинт, иначе мониторинг не сможет проверять сервис.
#[tokio::test]
async fn health_is_public_no_token_needed() {
    let state = common::test_app_state().await;
    let app = create_router(state);

    let request = Request::builder()
        .uri("/health")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

// ==================== PROTECTED ENDPOINTS ====================

/// Запрос к защищённому эндпоинту БЕЗ заголовка Authorization → 401.
#[tokio::test]
async fn protected_route_without_token_returns_401() {
    let state = common::test_app_state().await;
    let app = create_router(state);

    let request = Request::builder()
        .method("GET")
        .uri("/bots")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Запрос с НЕВЕРНЫМ токеном → 401.
#[tokio::test]
async fn protected_route_with_wrong_token_returns_401() {
    let state = common::test_app_state().await;
    let app = create_router(state);

    let request = Request::builder()
        .method("GET")
        .uri("/bots")
        .header("Authorization", "Bearer wrong-token")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Запрос с ПРАВИЛЬНЫМ токеном → проходит (не 401).
#[tokio::test]
async fn protected_route_with_valid_token_passes() {
    let state = common::test_app_state().await;
    let app = create_router(state);

    // "test-service-token" — токен, заданный в common::test_app_state()
    let request = Request::builder()
        .method("GET")
        .uri("/bots")
        .header("Authorization", "Bearer test-service-token")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Может быть 200 (пустой список) или любой другой, но НЕ 401.
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}
