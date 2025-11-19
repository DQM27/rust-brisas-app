// src-tauri/src/commands/keyring_commands.rs

use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::keyring_manager::{
    save_credentials, load_credentials, delete_credentials,
    credentials_exist, get_keyring_info, SupabaseCredentials
};
use crate::SupabaseState;
use crate::supabase::SupabaseClient;

#[tauri::command]
pub async fn keyring_save(
    url: String,
    anon_key: String,
    db_password: String,
    supabase_state: State<'_, Arc<RwLock<SupabaseState>>>,
) -> Result<String, String> {
    let creds = SupabaseCredentials {
        url,
        anon_key,
        db_password,
    };
    
    // Guardar en keyring
    save_credentials(&creds)?;
    
    // Intentar inicializar cliente de Supabase
    match SupabaseClient::new(&creds).await {
        Ok(client) => {
            let mut state = supabase_state.write().await;
            state.client = Some(client);
            Ok(format!("✅ Credenciales guardadas en {} y cliente inicializado", get_keyring_info()))
        }
        Err(e) => {
            Ok(format!("✅ Credenciales guardadas en {} pero no se pudo conectar: {}", get_keyring_info(), e))
        }
    }
}

// ... resto de comandos igual
#[tauri::command]
pub fn keyring_load() -> Result<SupabaseCredentials, String> {
    load_credentials()
}

#[tauri::command]
pub fn keyring_delete() -> Result<String, String> {
    delete_credentials()?;
    Ok(format!("✅ Credenciales eliminadas de {}", get_keyring_info()))
}

#[tauri::command]
pub fn keyring_check() -> Result<bool, String> {
    Ok(credentials_exist())
}

#[tauri::command]
pub fn keyring_info() -> Result<String, String> {
    Ok(get_keyring_info())
}