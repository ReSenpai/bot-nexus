/// Интеграционные тесты для CRUD TODO-листов.
///
/// Все эндпоинты требуют авторизации (AuthUser).
/// Формат: POST/GET/PUT/DELETE /lists
mod common;

use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use todo_api::app::create_router;

/// Регистрирует пользователя и возвращает JWT-токен.
async fn get_auth_token(state: &todo_api::state::AppState, email: &str) -> String {
    let app = create_router().with_state(state.clone());

    let req = Request::builder()
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

    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    body["token"].as_str().unwrap().to_string()
}

// ==================== POST /lists ====================

#[tokio::test]
async fn create_list_returns_201() {
    let state = common::test_app_state().await;
    let email = "lists_create@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = get_auth_token(&state, email).await;

    let app = create_router().with_state(state.clone());

    // Создаём новый TODO-лист.
    let req = Request::builder()
        .method("POST")
        .uri("/lists")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "My List" }).to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Ответ должен содержать id и title.
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["title"], "My List");
    assert!(body.get("id").is_some(), "Response should contain 'id'");

    common::cleanup_user(&state.db, email).await;
}

#[tokio::test]
async fn create_list_without_token_returns_401() {
    let state = common::test_app_state().await;
    let app = create_router().with_state(state);

    // Запрос без токена — должен быть 401.
    let req = Request::builder()
        .method("POST")
        .uri("/lists")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "My List" }).to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ==================== GET /lists ====================

#[tokio::test]
async fn get_lists_returns_200_with_array() {
    let state = common::test_app_state().await;
    let email = "lists_getall@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = get_auth_token(&state, email).await;

    // Сначала создаём два списка.
    for title in &["Work", "Personal"] {
        let app = create_router().with_state(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/lists")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(axum::body::Body::from(
                serde_json::json!({ "title": title }).to_string(),
            ))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    // Теперь получаем все списки.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("GET")
        .uri("/lists")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    // Должен быть массив минимум из 2 элементов.
    let lists = body.as_array().expect("Response should be an array");
    assert!(lists.len() >= 2, "Should have at least 2 lists");

    common::cleanup_user(&state.db, email).await;
}

// ==================== GET /lists/:id ====================

#[tokio::test]
async fn get_list_by_id_returns_200() {
    let state = common::test_app_state().await;
    let email = "lists_getone@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = get_auth_token(&state, email).await;

    // Создаём список.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri("/lists")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "Groceries" }).to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let list_id = created["id"].as_str().unwrap();

    // Получаем этот список по ID.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("GET")
        .uri(format!("/lists/{}", list_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["title"], "Groceries");
    assert_eq!(body["id"], list_id);

    common::cleanup_user(&state.db, email).await;
}

#[tokio::test]
async fn get_nonexistent_list_returns_404() {
    let state = common::test_app_state().await;
    let email = "lists_404@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = get_auth_token(&state, email).await;

    let app = create_router().with_state(state.clone());

    // Используем случайный UUID — такого списка нет.
    let fake_id = uuid::Uuid::new_v4();
    let req = Request::builder()
        .method("GET")
        .uri(format!("/lists/{}", fake_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    common::cleanup_user(&state.db, email).await;
}

// ==================== PUT /lists/:id ====================

#[tokio::test]
async fn update_list_returns_200() {
    let state = common::test_app_state().await;
    let email = "lists_update@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = get_auth_token(&state, email).await;

    // Создаём список.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri("/lists")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "Old Title" }).to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let list_id = created["id"].as_str().unwrap();

    // Обновляем название.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/lists/{}", list_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "New Title" }).to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["title"], "New Title");

    common::cleanup_user(&state.db, email).await;
}

// ==================== DELETE /lists/:id ====================

#[tokio::test]
async fn delete_list_returns_204() {
    let state = common::test_app_state().await;
    let email = "lists_delete@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = get_auth_token(&state, email).await;

    // Создаём список.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri("/lists")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "To Delete" }).to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let list_id = created["id"].as_str().unwrap();

    // Удаляем список.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/lists/{}", list_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Проверяем, что список больше не найден.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("GET")
        .uri(format!("/lists/{}", list_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    common::cleanup_user(&state.db, email).await;
}
