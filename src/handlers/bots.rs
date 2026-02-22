use axum::Json;

/// Заглушка: возвращает пустой JSON-массив.
/// Полноценная реализация будет при работе над Bot CRUD.
pub async fn list_bots() -> Json<Vec<()>> {
    Json(vec![])
}
