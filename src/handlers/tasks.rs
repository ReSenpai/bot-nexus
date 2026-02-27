use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::dto::tasks::{CreateTaskRequest, TaskResponse, UpdateTaskRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::services;
use crate::state::AppState;

/// POST /lists/{list_id}/tasks — создать задачу в списке.
#[utoipa::path(
    post,
    path = "/lists/{list_id}/tasks",
    tag = "Tasks",
    security(("bearer_auth" = [])),
    params(
        ("list_id" = Uuid, Path, description = "UUID списка")
    ),
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Задача создана", body = TaskResponse),
        (status = 401, description = "Не авторизован", body = crate::dto::ErrorResponse),
        (status = 404, description = "Список не найден", body = crate::dto::ErrorResponse)
    )
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Path(list_id): Path<Uuid>,
    Json(body): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<TaskResponse>), AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let task = services::tasks::create_task(&state.db, list_id, user_id, &body.title).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

/// GET /lists/{list_id}/tasks — все задачи списка.
#[utoipa::path(
    get,
    path = "/lists/{list_id}/tasks",
    tag = "Tasks",
    security(("bearer_auth" = [])),
    params(
        ("list_id" = Uuid, Path, description = "UUID списка")
    ),
    responses(
        (status = 200, description = "Массив задач", body = Vec<TaskResponse>),
        (status = 401, description = "Не авторизован", body = crate::dto::ErrorResponse),
        (status = 404, description = "Список не найден", body = crate::dto::ErrorResponse)
    )
)]
pub async fn get_all(
    State(state): State<AppState>,
    user: AuthUser,
    Path(list_id): Path<Uuid>,
) -> Result<Json<Vec<TaskResponse>>, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let tasks = services::tasks::get_all_tasks(&state.db, list_id, user_id).await?;

    Ok(Json(tasks))
}

/// GET /lists/{list_id}/tasks/{task_id} — одна задача.
#[utoipa::path(
    get,
    path = "/lists/{list_id}/tasks/{task_id}",
    tag = "Tasks",
    security(("bearer_auth" = [])),
    params(
        ("list_id" = Uuid, Path, description = "UUID списка"),
        ("task_id" = Uuid, Path, description = "UUID задачи")
    ),
    responses(
        (status = 200, description = "Найденная задача", body = TaskResponse),
        (status = 401, description = "Не авторизован", body = crate::dto::ErrorResponse),
        (status = 404, description = "Задача не найдена", body = crate::dto::ErrorResponse)
    )
)]
pub async fn get_one(
    State(state): State<AppState>,
    user: AuthUser,
    Path((list_id, task_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<TaskResponse>, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let task = services::tasks::get_task(&state.db, list_id, user_id, task_id).await?;

    Ok(Json(task))
}

/// PUT /lists/{list_id}/tasks/{task_id} — обновить задачу.
#[utoipa::path(
    put,
    path = "/lists/{list_id}/tasks/{task_id}",
    tag = "Tasks",
    security(("bearer_auth" = [])),
    params(
        ("list_id" = Uuid, Path, description = "UUID списка"),
        ("task_id" = Uuid, Path, description = "UUID задачи")
    ),
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "Обновлённая задача", body = TaskResponse),
        (status = 401, description = "Не авторизован", body = crate::dto::ErrorResponse),
        (status = 404, description = "Задача не найдена", body = crate::dto::ErrorResponse)
    )
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthUser,
    Path((list_id, task_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    let task = services::tasks::update_task(
        &state.db, list_id, user_id, task_id, &body.title, &body.status,
    ).await?;

    Ok(Json(task))
}

/// DELETE /lists/{list_id}/tasks/{task_id} — удалить задачу.
#[utoipa::path(
    delete,
    path = "/lists/{list_id}/tasks/{task_id}",
    tag = "Tasks",
    security(("bearer_auth" = [])),
    params(
        ("list_id" = Uuid, Path, description = "UUID списка"),
        ("task_id" = Uuid, Path, description = "UUID задачи")
    ),
    responses(
        (status = 204, description = "Задача удалена"),
        (status = 401, description = "Не авторизован", body = crate::dto::ErrorResponse),
        (status = 404, description = "Задача не найдена", body = crate::dto::ErrorResponse)
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthUser,
    Path((list_id, task_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let user_id: Uuid = user.user_id.parse()
        .map_err(|_| AppError::Validation("Invalid user ID in token".to_string()))?;

    services::tasks::delete_task(&state.db, list_id, user_id, task_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
