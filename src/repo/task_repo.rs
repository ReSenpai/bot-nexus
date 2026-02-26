use sqlx::PgPool;
use uuid::Uuid;

use crate::models::task::Task;

/// Создаёт задачу в указанном списке. Статус по умолчанию — "todo" (DEFAULT в БД).
pub async fn create(pool: &PgPool, list_id: Uuid, title: &str) -> Result<Task, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (list_id, title) VALUES ($1, $2) RETURNING *",
    )
    .bind(list_id)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(task)
}

/// Возвращает все задачи конкретного списка.
pub async fn find_all_by_list(pool: &PgPool, list_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE list_id = $1 ORDER BY created_at ASC",
    )
    .bind(list_id)
    .fetch_all(pool)
    .await?;

    Ok(tasks)
}

/// Возвращает одну задачу по ID внутри конкретного списка.
pub async fn find_by_id(
    pool: &PgPool,
    list_id: Uuid,
    task_id: Uuid,
) -> Result<Option<Task>, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1 AND list_id = $2",
    )
    .bind(task_id)
    .bind(list_id)
    .fetch_optional(pool)
    .await?;

    Ok(task)
}

/// Обновляет задачу (title + status). Возвращает обновлённую задачу.
pub async fn update(
    pool: &PgPool,
    list_id: Uuid,
    task_id: Uuid,
    title: &str,
    status: &str,
) -> Result<Option<Task>, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET title = $1, status = $2, updated_at = now() \
         WHERE id = $3 AND list_id = $4 RETURNING *",
    )
    .bind(title)
    .bind(status)
    .bind(task_id)
    .bind(list_id)
    .fetch_optional(pool)
    .await?;

    Ok(task)
}

/// Удаляет задачу по ID внутри списка.
pub async fn delete(pool: &PgPool, list_id: Uuid, task_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND list_id = $2")
        .bind(task_id)
        .bind(list_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
