#[cfg(test)]
mod tests {
    use crate::app::create_router;
    use crate::state::AppState;

    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use tower::ServiceExt;

    /// Вспомогательная функция: создаёт AppState с реальной БД для тестов.
    ///
    /// Использует ту же DATABASE_URL из .env.
    async fn test_app_state() -> AppState {
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

    /// Удаляем тестового пользователя после теста, чтобы тесты были изолированы.
    async fn cleanup_user(pool: &PgPool, email: &str) {
        sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
            .execute(pool)
            .await
            .ok();
    }

    /// POST /auth/register с валидным JSON должен вернуть 201 и JSON с полем "token".
    #[tokio::test]
    async fn register_returns_201_with_token() {
        let state = test_app_state().await;
        let pool = state.db.clone();

        cleanup_user(&pool, "test@example.com").await;

        let app = create_router().with_state(state);
        
        let request = Request::builder()
            .method("POST")
            .uri("/auth/register")
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(
                serde_json::json!({
                    "email": "test@example.com",
                    "password": "password123"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Проверяем статус 201 Created.
        assert_eq!(response.status(), StatusCode::CREATED);

        // Проверяем, что в теле есть поле "token".
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert!(body.get("token").is_some(), "Response should contain 'token' field");

        cleanup_user(&pool, "test@example.com").await;
    }

    /// Повторная регистрация с тем же email → 409 Conflict.
    #[tokio::test]
    async fn register_duplicate_email_returns_409() {
        let state = test_app_state().await;
        let pool = state.db.clone();

        cleanup_user(&pool, "duplicate@example.com").await;

        let app = create_router().with_state(state.clone());

        let body = serde_json::json!({
            "email": "duplicate@example.com",
            "password": "password123"
        })
        .to_string();

        // Первая регистрация — должна пройти.
        let req1 = Request::builder()
            .method("POST")
            .uri("/auth/register")
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(body.clone()))
            .unwrap();

        let resp1 = app.oneshot(req1).await.unwrap();
        assert_eq!(resp1.status(), StatusCode::CREATED);

        // Вторая регистрация с тем же email — должна вернуть 409.
        // Нужен новый роутер, т.к. oneshot потребляет Router.
        let app2 = create_router().with_state(state);

        let req2 = Request::builder()
            .method("POST")
            .uri("/auth/register")
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(body))
            .unwrap();

        let resp2 = app2.oneshot(req2).await.unwrap();
        assert_eq!(resp2.status(), StatusCode::CONFLICT);

        cleanup_user(&pool, "duplicate@example.com").await;
    }
}
