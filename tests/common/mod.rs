/// Общие утилиты для интеграционных тестов.
use http_body_util::BodyExt;
use axum::http::{Request};
use tower::ServiceExt;
use sqlx::PgPool;

use todo_api::app::create_router;
use todo_api::state::AppState;

/// Создаёт AppState с реальной БД для тестов.
///
/// Использует ту же DATABASE_URL из .env.
/// Каждый тест получает свой пул соединений.
pub async fn test_app_state() -> AppState {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let db = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test DB");

    AppState {
        db,
        jwt_secret: "test-secret-key".to_string(),
    }
}

#[allow(dead_code)]
/// Регистрирует пользователя и возвращает JWT-токен.
pub async fn get_auth_token(state: &todo_api::state::AppState, email: &str) -> String {
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

#[allow(dead_code)]
/// Удаляет тестового пользователя по email.
pub async fn cleanup_user(pool: &PgPool, email: &str) {
    sqlx::query("DELETE FROM users WHERE email = $1")
        .bind(email)
        .execute(pool)
        .await
        .ok();
}
