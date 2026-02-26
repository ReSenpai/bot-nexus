use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::tasks::TaskResponse;
use crate::errors::AppError;
use crate::repo::{list_repo, task_repo};

/// Конвертирует доменную модель Task в TaskResponse (DTO).
fn to_response(task: crate::models::task::Task) -> TaskResponse {
    TaskResponse {
        id: task.id,
        list_id: task.list_id,
        title: task.title,
        status: task.status,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }
}

/// Проверяет, что список принадлежит пользователю.
/// Возвращает 404, если список не найден или чужой.
async fn verify_list_ownership(pool: &PgPool, list_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    list_repo::find_by_id(pool, list_id, user_id)
        .await?
        .ok_or(AppError::NotFound("List not found".to_string()))?;
    Ok(())
}

/// Создаёт задачу в указанном списке.
/// Сначала проверяем, что список принадлежит пользователю (авторизация на уровне данных).
pub async fn create_task(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
    title: &str,
) -> Result<TaskResponse, AppError> {
    verify_list_ownership(pool, list_id, user_id).await?;

    let task = task_repo::create(pool, list_id, title).await?;
    Ok(to_response(task))
}

/// Возвращает все задачи списка.
pub async fn get_all_tasks(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<TaskResponse>, AppError> {
    verify_list_ownership(pool, list_id, user_id).await?;

    let tasks = task_repo::find_all_by_list(pool, list_id).await?;
    let response = tasks.into_iter().map(to_response).collect();
    Ok(response)
}

/// Возвращает одну задачу по ID.
pub async fn get_task(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<TaskResponse, AppError> {
    verify_list_ownership(pool, list_id, user_id).await?;

    let task = task_repo::find_by_id(pool, list_id, task_id)
        .await?
        .ok_or(AppError::NotFound("Task not found".to_string()))?;

    Ok(to_response(task))
}

/// Обновляет задачу (title + status).
pub async fn update_task(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
    task_id: Uuid,
    title: &str,
    status: &str,
) -> Result<TaskResponse, AppError> {
    verify_list_ownership(pool, list_id, user_id).await?;

    let task = task_repo::update(pool, list_id, task_id, title, status)
        .await?
        .ok_or(AppError::NotFound("Task not found".to_string()))?;

    Ok(to_response(task))
}

/// Удаляет задачу.
pub async fn delete_task(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<(), AppError> {
    verify_list_ownership(pool, list_id, user_id).await?;

    let deleted = task_repo::delete(pool, list_id, task_id).await?;

    if !deleted {
        return Err(AppError::NotFound("Task not found".to_string()));
    }

    Ok(())
}
