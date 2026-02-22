use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Контракты на клонирование, сериализацию и десериализацию для структуры `User`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    /// Хэш пароля (argon2). НЕ сам пароль — мы никогда не храним пароли в открытом виде.
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}
