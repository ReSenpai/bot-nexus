use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Запрос на создание задачи.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskRequest {
    #[schema(example = "Купить молоко")]
    pub title: String,
}

/// Запрос на обновление задачи (title + status).
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTaskRequest {
    #[schema(example = "Купить молоко и хлеб")]
    pub title: String,
    /// Новый статус: `todo`, `in_progress` или `done`.
    #[schema(example = "in_progress")]
    pub status: String,
}

/// Ответ с задачей — то, что видит клиент.
#[derive(Debug, Serialize, ToSchema)]
pub struct TaskResponse {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
