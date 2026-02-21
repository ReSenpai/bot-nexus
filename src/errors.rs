use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Единый тип ошибки приложения.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Ресурс не найден (404).
    #[error("{0}")]
    NotFound(String),

    /// Конфликт — например, email уже занят (409).
    #[error("{0}")]
    Conflict(String),

    /// Невалидные данные от клиента (422).
    #[error("{0}")]
    Validation(String),

    /// Неверные учётные данные — логин/пароль (401).
    #[error("Invalid credentials")]
    Unauthorized,

    /// Внутренняя ошибка сервера (500).
    #[error("Internal server error")]
    Internal(#[from] sqlx::Error),
}

/// Реализация `IntoResponse` — это то, что позволяет возвращать
/// `AppError` из handler'ов axum.
///
/// axum вызовет этот метод автоматически, когда handler вернёт `Err(AppError)`.
/// Мы конвертируем каждый вариант ошибки в HTTP-ответ с:
/// - правильным статус-кодом
/// - JSON-телом вида `{ "error": "сообщение" }`
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // Для Internal ошибок НЕ показываем детали клиенту (безопасность).
        // Но логируем их на сервере для отладки.
        let message = match &self {
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                "Internal server error".to_string()
            }
            other => other.to_string(),
        };

        // Собираем ответ: (StatusCode, Json) → axum превратит в HTTP-ответ.
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
