use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::dto::auth::{AuthResponse, RegisterRequest};
use crate::errors::AppError;
use crate::services;
use crate::state::AppState;

/// Handler для POST /auth/register.
///
/// Как axum разбирает аргументы handler'а:
/// - `State(state)` — извлекает `AppState` из роутера (пул БД, JWT secret).
/// - `Json(body)` — парсит JSON-тело запроса в `RegisterRequest`.
///   Если JSON невалидный, axum сам вернёт 400 Bad Request.
///
/// Возвращаемый тип:
/// - `Result<(StatusCode, Json<AuthResponse>), AppError>`
/// - Успех: 201 + JSON с токеном
/// - Ошибка: AppError → автоматически в HTTP-ответ через IntoResponse
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    // Вызываем сервис — вся бизнес-логика там.
    // Handler — только "мост" между HTTP и бизнес-логикой.
    let token = services::auth::register(&state.db, &state.jwt_secret, &body.email, &body.password)
        .await?;

    // 201 Created — стандартный код для успешного создания ресурса.
    Ok((StatusCode::CREATED, Json(AuthResponse { token })))
}
