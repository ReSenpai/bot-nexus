use sqlx::PgPool;

/// Общее состояние приложения, доступное во всех handler'ах.
#[derive(Clone)]
pub struct AppState {
    /// Пул соединений к PostgreSQL.
    pub db: PgPool,
    /// Секретный ключ для подписи JWT-токенов.
    pub jwt_secret: String,
}
