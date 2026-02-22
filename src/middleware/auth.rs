use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::state::AppState;

/// Middleware для проверки service-token.
///
/// Алгоритм:
/// 1. Извлекаем заголовок `Authorization` из запроса.
/// 2. Проверяем формат: `Bearer <token>`.
/// 3. Сравниваем `<token>` с `state.service_token`.
/// 4. Если совпадает — `next.run(request)` (пропускаем запрос дальше).
/// 5. Если нет — возвращаем `401 Unauthorized`.
///
/// ## Почему `from_fn_with_state`?
/// axum позволяет создавать middleware из обычной async-функции.
/// `from_fn_with_state` — вариант, который умеет извлекать `State<AppState>`,
/// чтобы получить доступ к `service_token`.
pub async fn require_service_token(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Извлекаем заголовок Authorization
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    // 2. Проверяем наличие и формат
    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            // 3. Извлекаем сам токен (после "Bearer ")
            let token = &header["Bearer ".len()..];

            // 4. Сравниваем с ожидаемым
            if token == state.service_token {
                // Токен верный — пропускаем запрос дальше по цепочке
                Ok(next.run(request).await)
            } else {
                // Токен неверный
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        // Заголовок отсутствует или формат неправильный
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
