use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Запрос на создание задачи.
/// Статус не указывается — по умолчанию "todo" (задаётся в БД).
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
}

/// Запрос на обновление задачи (title + status).
/// Оба поля обязательны — PUT заменяет ресурс целиком.
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: String,
    pub status: String,
}

/// Ответ с задачей — то, что видит клиент.
#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
