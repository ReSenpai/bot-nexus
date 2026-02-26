/// Интеграционные тесты для CRUD задач (tasks) внутри TODO-листов.
mod common;

use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use todo_api::app::create_router;


// ==================== POST /lists/:list_id/tasks ====================

#[tokio::test]
async fn create_task_returns_201() {
    let state = common::test_app_state().await;
    let email = "tasks_create@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = common::get_auth_token(&state, email).await;
    let list_id = common::create_list(&state, &token).await;

    let app = create_router().with_state(state.clone());

    // Создаём задачу в списке.
    let req = Request::builder()
        .method("POST")
        .uri(format!("/lists/{}/tasks", list_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "Buy milk" }).to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["title"], "Buy milk");
    // Статус по умолчанию — "todo".
    assert_eq!(body["status"], "todo");
    assert!(body.get("id").is_some(), "Response should contain 'id'");

    common::cleanup_user(&state.db, email).await;
}

#[tokio::test]
async fn create_task_in_foreign_list_returns_404() {
    let state = common::test_app_state().await;

    // Пользователь A создаёт список.
    let email_a = "tasks_foreign_a@example.com";
    common::cleanup_user(&state.db, email_a).await;
    let token_a = common::get_auth_token(&state, email_a).await;
    let list_id = common::create_list(&state, &token_a).await;

    // Пользователь B пытается создать задачу в чужом списке.
    let email_b = "tasks_foreign_b@example.com";
    common::cleanup_user(&state.db, email_b).await;
    let token_b = common::get_auth_token(&state, email_b).await;

    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri(format!("/lists/{}/tasks", list_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token_b))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "Hack attempt" }).to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    // Должен быть 404 — список "не найден" для пользователя B.
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    common::cleanup_user(&state.db, email_a).await;
    common::cleanup_user(&state.db, email_b).await;
}


// ==================== GET /lists/:list_id/tasks ====================

#[tokio::test]
async fn get_tasks_returns_200_with_array() {
    let state = common::test_app_state().await;
    let email = "tasks_getall@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = common::get_auth_token(&state, email).await;
    let list_id = common::create_list(&state, &token).await;

    // Создаём две задачи.
    for title in &["Task A", "Task B"] {
        let app = create_router().with_state(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri(format!("/lists/{}/tasks", list_id))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(axum::body::Body::from(
                serde_json::json!({ "title": title }).to_string(),
            ))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    // Получаем все задачи списка.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("GET")
        .uri(format!("/lists/{}/tasks", list_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let tasks = body.as_array().expect("Response should be an array");
    assert!(tasks.len() >= 2, "Should have at least 2 tasks");

    common::cleanup_user(&state.db, email).await;
}


// ==================== GET /lists/:list_id/tasks/:task_id ====================

#[tokio::test]
async fn get_task_by_id_returns_200() {
    let state = common::test_app_state().await;
    let email = "tasks_getone@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = common::get_auth_token(&state, email).await;
    let list_id = common::create_list(&state, &token).await;

    // Создаём задачу.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri(format!("/lists/{}/tasks", list_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "Read docs" }).to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let task_id = created["id"].as_str().unwrap();

    // Получаем задачу по ID.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("GET")
        .uri(format!("/lists/{}/tasks/{}", list_id, task_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["title"], "Read docs");
    assert_eq!(body["status"], "todo");

    common::cleanup_user(&state.db, email).await;
}


// ==================== PUT /lists/:list_id/tasks/:task_id ====================

#[tokio::test]
async fn update_task_title_returns_200() {
    let state = common::test_app_state().await;
    let email = "tasks_update_title@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = common::get_auth_token(&state, email).await;
    let list_id = common::create_list(&state, &token).await;

    // Создаём задачу.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri(format!("/lists/{}/tasks", list_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "Old task" }).to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let task_id = created["id"].as_str().unwrap();

    // Обновляем title и status.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/lists/{}/tasks/{}", list_id, task_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({
                "title": "Updated task",
                "status": "in_progress"
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["title"], "Updated task");
    assert_eq!(body["status"], "in_progress");

    common::cleanup_user(&state.db, email).await;
}

#[tokio::test]
async fn update_task_status_to_done() {
    let state = common::test_app_state().await;
    let email = "tasks_status_done@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = common::get_auth_token(&state, email).await;
    let list_id = common::create_list(&state, &token).await;

    // Создаём задачу.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri(format!("/lists/{}/tasks", list_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "Finish project" }).to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let task_id = created["id"].as_str().unwrap();

    // Переводим статус в "done".
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/lists/{}/tasks/{}", list_id, task_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({
                "title": "Finish project",
                "status": "done"
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "done");

    common::cleanup_user(&state.db, email).await;
}


// ==================== DELETE /lists/:list_id/tasks/:task_id ====================

#[tokio::test]
async fn delete_task_returns_204() {
    let state = common::test_app_state().await;
    let email = "tasks_delete@example.com";
    common::cleanup_user(&state.db, email).await;
    let token = common::get_auth_token(&state, email).await;
    let list_id = common::create_list(&state, &token).await;

    // Создаём задачу.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("POST")
        .uri(format!("/lists/{}/tasks", list_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::from(
            serde_json::json!({ "title": "To delete" }).to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let task_id = created["id"].as_str().unwrap();

    // Удаляем задачу.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/lists/{}/tasks/{}", list_id, task_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Проверяем, что задача больше не существует.
    let app = create_router().with_state(state.clone());
    let req = Request::builder()
        .method("GET")
        .uri(format!("/lists/{}/tasks/{}", list_id, task_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    common::cleanup_user(&state.db, email).await;
}
