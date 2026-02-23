use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Доменная модель TODO-листа.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TodoList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
