// src-tauri/src/keyring_manager.rs

use keyring::Entry;
use serde::{Deserialize, Serialize};

/// Credenciales de Supabase
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupabaseCredentials {
    pub url: String,
    pub anon_key: String,
    pub db_password: String,
}

const SERVICE_NAME: &str = "brisas-app";
const ACCOUNT_NAME: &str = "supabase";

/// Guarda credenciales en el keyring del OS
pub fn save_credentials(creds: &SupabaseCredentials) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Error creando entry en keyring: {}", e))?;
    
    let json = serde_json::to_string(creds)
        .map_err(|e| format!("Error serializando credenciales: {}", e))?;
    
    entry.set_password(&json)
        .map_err(|e| format!("Error guardando en keyring: {}", e))?;
    
    println!("✅ Credenciales guardadas en keyring del OS");
    Ok(())
}

/// Lee credenciales del keyring del OS
pub fn load_credentials() -> Result<SupabaseCredentials, String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Error creando entry en keyring: {}", e))?;
    
    let json = entry.get_password()
        .map_err(|e| format!("Error leyendo del keyring: {}", e))?;
    
    let creds: SupabaseCredentials = serde_json::from_str(&json)
        .map_err(|e| format!("Error deserializando credenciales: {}", e))?;
    
    println!("✅ Credenciales leídas del keyring del OS");
    Ok(creds)
}

/// Elimina credenciales del keyring
pub fn delete_credentials() -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Error creando entry en keyring: {}", e))?;
    
    entry.delete_credential()
        .map_err(|e| format!("Error eliminando del keyring: {}", e))?;
    
    println!("✅ Credenciales eliminadas del keyring");
    Ok(())
}

/// Verifica si existen credenciales guardadas
pub fn credentials_exist() -> bool {
    match Entry::new(SERVICE_NAME, ACCOUNT_NAME) {
        Ok(entry) => entry.get_password().is_ok(),
        Err(_) => false,
    }
}

/// Obtiene información sobre el backend del keyring
pub fn get_keyring_info() -> String {
    #[cfg(target_os = "windows")]
    return "Windows Credential Manager".to_string();
    
    #[cfg(target_os = "macos")]
    return "macOS Keychain".to_string();
    
    #[cfg(target_os = "linux")]
    return "Linux Secret Service (GNOME Keyring/KWallet)".to_string();
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "Sistema no soportado".to_string();
}