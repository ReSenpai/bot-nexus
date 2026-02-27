use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Входные данные для регистрации.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "mypassword123")]
    pub password: String,
}

/// Входные данные для логина (авторизации).
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "mypassword123")]
    pub password: String,
}

/// Ответ на успешную авторизацию / регистрацию.
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    /// JWT-токен для авторизации последующих запросов.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
}
