use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::dto::auth::{AuthResponse, LoginRequest, RegisterRequest};
use crate::errors::AppError;
use crate::services;
use crate::state::AppState;

/// Handler для POST /auth/register.
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let token = services::auth::register(&state.db, &state.jwt_secret, &body.email, &body.password)
        .await?;

    // 201 Created — стандартный код для успешного создания ресурса.
    Ok((StatusCode::CREATED, Json(AuthResponse { token })))
}

/// Handler для POST /auth/login.
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let token = services::auth::login(&state.db, &state.jwt_secret, &body.email, &body.password)
        .await?;

    // 200 OK — возвращается автоматически для Json<T> без явного StatusCode.
    Ok(Json(AuthResponse { token }))
}
