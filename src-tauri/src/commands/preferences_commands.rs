use crate::db::preferences_queries;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn get_user_preferences(
    user_id: String,
    category: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<preferences_queries::UserPreferenceRow>, String> {
    preferences_queries::get_user_preferences(&pool, &user_id, &category)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_user_preference(
    user_id: String,
    category: String,
    key: String,
    value: String,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    preferences_queries::set_user_preference(&pool, &user_id, &category, &key, &value)
        .await
        .map_err(|e| e.to_string())
}
