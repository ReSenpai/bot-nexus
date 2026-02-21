/// Интеграционные тесты для GET /health.
mod common;

use bot_nexus::app::create_router;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_returns_200_ok() {
    let state = common::test_app_state().await;
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
