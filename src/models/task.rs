use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Доменная модель задачи внутри TODO-листа.
///
/// Статусы: "todo" → "in_progress" → "done".
/// CHECK-constraint в БД гарантирует валидность значений.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
