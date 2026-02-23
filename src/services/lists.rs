use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::lists::ListResponse;
use crate::errors::AppError;
use crate::repo::list_repo;

/// Создаёт новый TODO-лист.
///
/// Service layer принимает примитивные типы — UUID пользователя и title.
/// Repo возвращает полную модель, service конвертирует в DTO для ответа.
pub async fn create_list(
    pool: &PgPool,
    user_id: Uuid,
    title: &str,
) -> Result<ListResponse, AppError> {
    let list = list_repo::create(pool, user_id, title).await?;

    Ok(ListResponse {
        id: list.id,
        title: list.title,
        created_at: list.created_at,
        updated_at: list.updated_at,
    })
}

/// Возвращает все TODO-листы пользователя.
pub async fn get_all_lists(pool: &PgPool, user_id: Uuid) -> Result<Vec<ListResponse>, AppError> {
    let lists = list_repo::find_all_by_user(pool, user_id).await?;

    // Конвертируем Vec<TodoList> → Vec<ListResponse>.
    // `.into_iter().map().collect()` — идиоматичный Rust для трансформации коллекций.
    let response = lists
        .into_iter()
        .map(|list| ListResponse {
            id: list.id,
            title: list.title,
            created_at: list.created_at,
            updated_at: list.updated_at,
        })
        .collect();

    Ok(response)
}

/// Возвращает один список по ID.
///
/// Если список не найден или принадлежит другому пользователю → 404.
pub async fn get_list(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<ListResponse, AppError> {
    let list = list_repo::find_by_id(pool, list_id, user_id)
        .await?
        .ok_or(AppError::NotFound("List not found".to_string()))?;

    Ok(ListResponse {
        id: list.id,
        title: list.title,
        created_at: list.created_at,
        updated_at: list.updated_at,
    })
}

/// Обновляет название списка.
///
/// Если список не найден → 404.
pub async fn update_list(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
    title: &str,
) -> Result<ListResponse, AppError> {
    let list = list_repo::update(pool, list_id, user_id, title)
        .await?
        .ok_or(AppError::NotFound("List not found".to_string()))?;

    Ok(ListResponse {
        id: list.id,
        title: list.title,
        created_at: list.created_at,
        updated_at: list.updated_at,
    })
}

/// Удаляет список.
///
/// Если список не найден → 404.
pub async fn delete_list(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let deleted = list_repo::delete(pool, list_id, user_id).await?;

    if !deleted {
        return Err(AppError::NotFound("List not found".to_string()));
    }

    Ok(())
}
