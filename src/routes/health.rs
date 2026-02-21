#[cfg(test)]
mod tests {
    use crate::app::create_router;
    use crate::state::AppState;

    use tower::ServiceExt;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use sqlx::PgPool;

    /// Вспомогательная: создаёт AppState для тестов.
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

    #[tokio::test]
    async fn health_check_returns_200_ok() {
        let state = test_app_state().await;
        let app = create_router().with_state(state);

        let request = Request::builder()
            .uri("/health")
            .body(axum::body::Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body_string, "ok");
    }
}
