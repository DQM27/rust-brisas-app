// src-tauri/src/commands/supabase_commands.rs

use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::keyring_manager;
use crate::supabase::SupabaseClient;
use crate::SupabaseState;

#[tauri::command]
pub async fn test_supabase_connection(
    supabase_state: State<'_, Arc<RwLock<SupabaseState>>>,
) -> Result<String, String> {
    let state = supabase_state.read().await;
    
    match &state.client {
        Some(client) => {
            // Probar query simple
            match sqlx::query("SELECT 1")
                .fetch_one(client.pool())
                .await 
            {
                Ok(_) => Ok("Conexión exitosa a Supabase ✅".to_string()),
                Err(e) => Err(format!("Error en la consulta: {}", e)),
            }
        }
        None => Err("Cliente de Supabase no inicializado. Configura las credenciales primero.".to_string()),
    }
}

#[tauri::command]
pub async fn get_supabase_config() -> Result<serde_json::Value, String> {
    match keyring_manager::load_credentials() {
        Ok(creds) => Ok(serde_json::json!({
            "url": creds.url,
            "has_credentials": true,
        })),
        Err(_) => Ok(serde_json::json!({
            "url": "",
            "has_credentials": false,
        })),
    }
}

/// Reinicia la conexión de Supabase después de guardar credenciales
#[tauri::command]
pub async fn reinitialize_supabase(
    supabase_state: State<'_, Arc<RwLock<SupabaseState>>>,
) -> Result<String, String> {
    let creds = keyring_manager::load_credentials()
        .map_err(|e| format!("Error cargando credenciales: {}", e))?;
    
    let client = SupabaseClient::new(&creds)
        .await
        .map_err(|e| format!("Error conectando a Supabase: {}", e))?;
    
    let mut state = supabase_state.write().await;
    state.client = Some(client);
    
    Ok("Cliente de Supabase reinicializado exitosamente ✅".to_string())
}