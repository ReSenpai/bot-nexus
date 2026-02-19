#[cfg(test)]
mod tests {
    use crate::app::create_router;

    // tower::ServiceExt даёт метод `.oneshot()` — он позволяет отправить
    // один HTTP-запрос в роутер без реального сервера.
    use tower::ServiceExt;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn health_check_returns_200_ok() {
        let app = create_router();

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
