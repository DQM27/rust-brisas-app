// src-tauri/src/commands/supabase_commands.rs

use tauri::State;
use crate::config::AppConfig;
use crate::supabase::SupabaseClient;

#[tauri::command]
pub async fn test_supabase_connection(
    config: State<'_, AppConfig>,
) -> Result<String, String> {
    match SupabaseClient::new(&config).await {
        Ok(_) => Ok("Conexión exitosa a Supabase".to_string()),
        Err(e) => Err(format!("Error al conectar: {}", e)),
    }
}

#[tauri::command]
pub async fn get_supabase_config(
    config: State<'_, AppConfig>,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "url": config.supabase.url,
        "has_anon_key": !config.supabase.anon_key.is_empty(),
    }))
}