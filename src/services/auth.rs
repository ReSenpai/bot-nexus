use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::repo::user_repo;

/// Claims — содержимое JWT-токена.
///
/// JWT состоит из трёх частей: header.payload.signature
/// `Claims` — это payload.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject — идентификатор пользователя (UUID в виде строки).
    sub: String,
    /// Expiration — время истечения токена (Unix timestamp).
    exp: usize,
}

/// Регистрация нового пользователя.
///
/// Алгоритм:
/// 1. Проверяем, не занят ли email
/// 2. Хэшируем пароль через argon2
/// 3. Сохраняем пользователя в БД
/// 4. Генерируем JWT-токен
///
/// Возвращает JWT-токен или ошибку.
pub async fn register(
    pool: &PgPool,
    jwt_secret: &str,
    email: &str,
    password: &str,
) -> Result<String, AppError> {
    let existing = user_repo::find_by_email(pool, email).await?;
    if existing.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    //    Хэшируем пароль.
    //    Argon2 — один из лучших алгоритмов хэширования паролей.
    //    Salt (соль) генерируется случайно для каждого пароля —
    //    это защищает от rainbow table атак.
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::Validation("Failed to hash password".to_string()))?
        .to_string();

    let user = user_repo::create_user(pool, email, &password_hash).await?;

    //    Генерируем JWT-токен.
    let token = create_jwt(&user.id.to_string(), jwt_secret)?;

    Ok(token)
}

/// Создаёт JWT-токен для пользователя.
///
/// Токен действителен 24 часа. Содержит `sub` (user_id) и `exp` (expiration).
/// Подписывается секретным ключом (HMAC-SHA256).
fn create_jwt(user_id: &str, secret: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    // `encode` подписывает claims секретным ключом и возвращает строку
    // вида "eyJhbGciOi..."
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::Validation("Failed to create token".to_string()))?;

    Ok(token)
}
