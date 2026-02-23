mod common;

use axum::http::{Request, StatusCode};
use axum::routing::get;
use axum::{Json, Router};
use http_body_util::BodyExt;
use tower::ServiceExt;

use todo_api::middleware::auth::AuthUser;
use todo_api::state::AppState;

/// Тестовый handler, который требует авторизации.
async fn protected_handler(user: AuthUser) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "user_id": user.user_id }))
}

/// Создаёт тестовый роутер с одним защищённым эндпоинтом.
fn test_router(state: AppState) -> Router {
    Router::new()
        .route("/protected", get(protected_handler))
        .with_state(state)
}

// ==================== TEST: Нет токена → 401 ====================

#[tokio::test]
async fn request_without_token_returns_401() {
    let state = common::test_app_state().await;
    let app = test_router(state);

    // Отправляем запрос БЕЗ заголовка Authorization.
    let request = Request::builder()
        .method("GET")
        .uri("/protected")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Ожидаем 401 Unauthorized.
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ==================== TEST: Невалидный токен → 401 ====================

#[tokio::test]
async fn request_with_invalid_token_returns_401() {
    let state = common::test_app_state().await;
    let app = test_router(state);

    // Отправляем запрос с мусорным токеном.
    let request = Request::builder()
        .method("GET")
        .uri("/protected")
        .header("Authorization", "Bearer invalid.token.here")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ==================== TEST: Валидный токен → 200 ====================

#[tokio::test]
async fn request_with_valid_token_returns_200_and_user_id() {
    let state = common::test_app_state().await;
    let pool = state.db.clone();
    let email = "middleware_test@example.com";

    // Подготовка: создаём пользователя и получаем токен через API.
    common::cleanup_user(&pool, email).await;

    let app = Router::new()
        .merge(todo_api::routes::auth::router())
        .route("/protected", get(protected_handler))
        .with_state(state.clone());

    // Регистрируем пользователя, чтобы получить валидный токен.
    let register_req = Request::builder()
        .method("POST")
        .uri("/auth/register")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::json!({
                "email": email,
                "password": "password123"
            })
            .to_string(),
        ))
        .unwrap();

    let register_resp = app.clone().oneshot(register_req).await.unwrap();
    assert_eq!(register_resp.status(), StatusCode::CREATED);

    // Извлекаем токен из ответа регистрации.
    let body_bytes = register_resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let token = body["token"].as_str().expect("token must be a string");

    // Теперь идём на защищённый эндпоинт с валидным токеном.
    let protected_req = Request::builder()
        .method("GET")
        .uri("/protected")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let protected_resp = app.oneshot(protected_req).await.unwrap();

    // Ожидаем 200 OK.
    assert_eq!(protected_resp.status(), StatusCode::OK);

    // Ответ должен содержать user_id.
    let body_bytes = protected_resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(
        body.get("user_id").is_some(),
        "Response should contain 'user_id' field"
    );

    // Cleanup.
    common::cleanup_user(&pool, email).await;
}
