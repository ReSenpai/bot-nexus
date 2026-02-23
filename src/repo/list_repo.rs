use sqlx::PgPool;
use uuid::Uuid;

use crate::models::todo_list::TodoList;

/// Создаёт новый TODO-лист в БД.
pub async fn create(pool: &PgPool, user_id: Uuid, title: &str) -> Result<TodoList, sqlx::Error> {
    let list = sqlx::query_as::<_, TodoList>(
        "INSERT INTO todo_lists (user_id, title) VALUES ($1, $2) RETURNING *",
    )
    .bind(user_id)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(list)
}

/// Возвращает все списки конкретного пользователя.
pub async fn find_all_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<TodoList>, sqlx::Error> {
    let lists = sqlx::query_as::<_, TodoList>(
        "SELECT * FROM todo_lists WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(lists)
}

/// Возвращает один список по ID, только если он принадлежит пользователю.
pub async fn find_by_id(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<Option<TodoList>, sqlx::Error> {
    let list = sqlx::query_as::<_, TodoList>(
        "SELECT * FROM todo_lists WHERE id = $1 AND user_id = $2",
    )
    .bind(list_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(list)
}

/// Обновляет название списка. Возвращает обновлённый список.
pub async fn update(
    pool: &PgPool,
    list_id: Uuid,
    user_id: Uuid,
    title: &str,
) -> Result<Option<TodoList>, sqlx::Error> {
    let list = sqlx::query_as::<_, TodoList>(
        "UPDATE todo_lists SET title = $1, updated_at = now() WHERE id = $2 AND user_id = $3 RETURNING *",
    )
    .bind(title)
    .bind(list_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(list)
}

/// Удаляет список по ID (только если он принадлежит пользователю).
pub async fn delete(pool: &PgPool, list_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM todo_lists WHERE id = $1 AND user_id = $2")
        .bind(list_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    // `rows_affected()` — сколько строк было удалено. 0 = не нашли.
    Ok(result.rows_affected() > 0)
}
