use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::services::auth::validate_jwt;
use crate::state::AppState;

/// Extractor для авторизованного пользователя.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
}

/// Ошибка авторизации
pub struct AuthError(String);

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": self.0 }));
        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}

/// Реализация extractor'а для axum.
///
/// Алгоритм:
/// 1. Берём заголовок `Authorization`
/// 2. Проверяем формат `Bearer <token>`
/// 3. Валидируем JWT через `validate_jwt()`
/// 4. Возвращаем `AuthUser` с user_id из claims
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AuthError("Missing Authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AuthError("Invalid Authorization header format".to_string()))?;

        let claims = validate_jwt(token, &state.jwt_secret)
            .map_err(|_| AuthError("Invalid or expired token".to_string()))?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
