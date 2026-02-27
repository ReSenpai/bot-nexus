use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Запрос на создание списка.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateListRequest {
    #[schema(example = "Покупки")]
    pub title: String,
}

/// Запрос на обновление названия списка.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateListRequest {
    #[schema(example = "Покупки на неделю")]
    pub title: String,
}

/// Ответ со списком — то, что видит клиент.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListResponse {
    pub id: Uuid,
    pub title: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
