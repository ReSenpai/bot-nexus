use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::dto::lists::{CreateListRequest, ListResponse, UpdateListRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::services;
use crate::state::AppState;

/// POST /lists — создать новый TODO-лист.
pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Json(body): Json<CreateListRequest>,
) -> Result<(StatusCode, Json<ListResponse>), AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let list = services::lists::create_list(&state.db, user_id, &body.title).await?;

    Ok((StatusCode::CREATED, Json(list)))
}

/// GET /lists — получить все списки текущего пользователя.
pub async fn get_all(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Vec<ListResponse>>, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let lists = services::lists::get_all_lists(&state.db, user_id).await?;

    Ok(Json(lists))
}

/// GET /lists/:id — получить один список по ID.
///
/// `Path(list_id)` — axum extractor, который парсит параметр из URL.
pub async fn get_one(
    State(state): State<AppState>,
    user: AuthUser,
    Path(list_id): Path<Uuid>,
) -> Result<Json<ListResponse>, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let list = services::lists::get_list(&state.db, list_id, user_id).await?;

    Ok(Json(list))
}

/// PUT /lists/:id — обновить название списка.
pub async fn update(
    State(state): State<AppState>,
    user: AuthUser,
    Path(list_id): Path<Uuid>,
    Json(body): Json<UpdateListRequest>,
) -> Result<Json<ListResponse>, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let list = services::lists::update_list(&state.db, list_id, user_id, &body.title).await?;

    Ok(Json(list))
}

/// DELETE /lists/:id — удалить список.
///
/// Возвращает 204 No Content — тело ответа пустое.
pub async fn delete(
    State(state): State<AppState>,
    user: AuthUser,
    Path(list_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    services::lists::delete_list(&state.db, list_id, user_id).await?;

    // 204 No Content — стандартный ответ при успешном удалении.
    Ok(StatusCode::NO_CONTENT)
}
