use serde::{Deserialize, Serialize};

/// Входные данные для регистрации.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

/// Входные данные для логина (авторизации).
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Ответ на успешную авторизацию / регистрацию.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}
