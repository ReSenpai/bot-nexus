/// Общие утилиты для интеграционных тестов.

use rust_notest_api::state::AppState;
use sqlx::PgPool;

/// Создаёт AppState с реальной БД для тестов.
///
/// Использует ту же DATABASE_URL из .env.
/// Каждый тест получает свой пул соединений.
pub async fn test_app_state() -> AppState {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let db = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test DB");

    AppState {
        db,
        jwt_secret: "test-secret-key".to_string(),
    }
}

/// Удаляет тестового пользователя по email.
///
/// Вызывается до/после тестов для изоляции.
pub async fn cleanup_user(pool: &PgPool, email: &str) {
    sqlx::query("DELETE FROM users WHERE email = $1")
        .bind(email)
        .execute(pool)
        .await
        .ok();
}
