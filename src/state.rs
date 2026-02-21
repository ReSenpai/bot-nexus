use sqlx::PgPool;

/// Общее состояние приложения, доступное во всех handler'ах.
///
/// Передаётся в каждый handler через `State<AppState>`.
/// Содержит всё, что нужно handler'ам: пул БД, секреты, конфиг.
#[derive(Clone)]
pub struct AppState {
    /// Пул соединений к PostgreSQL.
    pub db: PgPool,
    /// Service token — ключ, которым боты авторизуются к этому API.
    pub service_token: String,
}
