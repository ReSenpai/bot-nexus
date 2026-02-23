use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Запрос на создание/обновление списка.
#[derive(Debug, Deserialize)]
pub struct CreateListRequest {
    pub title: String,
}

/// Запрос на обновление названия списка.
#[derive(Debug, Deserialize)]
pub struct UpdateListRequest {
    pub title: String,
}

/// Ответ со списком — то, что видит клиент.
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub id: Uuid,
    pub title: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
