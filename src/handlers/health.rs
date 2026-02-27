/// GET /health — проверка жизни сервиса.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Сервис работает", body = String, example = json!("ok"))
    )
)]
pub async fn health_check() -> &'static str {
    "ok"
}
