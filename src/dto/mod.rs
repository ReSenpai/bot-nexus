pub mod auth;
pub mod lists;
pub mod tasks;

use serde::Serialize;
use utoipa::ToSchema;

/// Стандартный формат ответа с ошибкой (для Swagger-документации).
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "List not found")]
    pub error: String,
}
