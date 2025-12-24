// ==========================================
// src/commands/keyring_commands.rs
// ==========================================
// Comandos Tauri para gestión segura de credenciales

use crate::config::save_config;
use crate::config::settings::AppConfigState;
use crate::domain::errors::KeyringError;
use crate::services::keyring_service::{self, Argon2Params, CredentialStatus};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::{command, State};

// ==========================================
// DTOs PARA COMANDOS
// ==========================================

#[derive(Debug, Deserialize)]
pub struct SetupCredentialsInput {
    pub argon2: Argon2Params,
    pub terminal_name: String,
    pub terminal_location: String,
}

#[derive(Debug, Serialize)]
pub struct SetupResult {
    pub success: bool,
    pub message: String,
}

// ==========================================
// COMANDOS DE ESTADO
// ==========================================

/// Obtiene el estado de configuración de credenciales
#[command]
pub fn get_credential_status() -> CredentialStatus {
    keyring_service::get_credential_status()
}

/// Verifica si la app está completamente configurada
#[command]
pub fn is_app_configured(config: State<'_, AppConfigState>) -> bool {
    // Necesitamos un read lock porque AppConfigState es RwLock
    if let Ok(guard) = config.read() {
        guard.setup.is_configured && keyring_service::is_fully_configured()
    } else {
        false // Si falla el lock, asumimos no configurado
    }
}

/// Verifica si necesita ejecutar el wizard de configuración
#[command]
pub fn needs_setup(config: State<'_, AppConfigState>) -> bool {
    if let Ok(guard) = config.read() {
        !guard.setup.is_configured || !keyring_service::is_fully_configured()
    } else {
        true // Si falla el lock, asumimos setup necesario
    }
}

// ==========================================
// COMANDOS DE SETUP INICIAL
// ==========================================

/// Configura todas las credenciales en el primer uso
#[command]
pub async fn setup_credentials(
    input: SetupCredentialsInput,
    pool: State<'_, crate::db::DbPool>,
    config: State<'_, crate::config::settings::AppConfigState>,
) -> Result<SetupResult, KeyringError> {
    // 1. Manejo inteligente del secreto Argon2
    // Si ya existe un secreto en el Keyring, lo REUTILIZAMOS para no perder acceso a datos
    // previos si solo se borró el TOML pero no las llaves de Windows.
    let mut final_argon2 = input.argon2.clone();
    if keyring_service::has_argon2_secret() {
        let existing = keyring_service::get_argon2_params();
        if !existing.secret.is_empty() {
            log::info!("🔐 Detectado secreto existente en Keyring. Reutilizando para mantener compatibilidad.");
            final_argon2.secret = existing.secret;
        }
    }

    // 2. Guardar parámetros (reutilizando secreto si existía)
    keyring_service::store_argon2_params(&final_argon2)
        .map_err(|e| KeyringError::StoreError(e.to_string()))?;

    // 3. Actualizar configuración en TOML
    {
        let mut config_guard = config.write().map_err(|e| {
            KeyringError::Message(format!("Error escribiendo configuración: {}", e))
        })?;

        config_guard.setup.is_configured = true;
        config_guard.setup.configured_at = Some(Utc::now().to_rfc3339());
        config_guard.setup.configured_version = Some(env!("CARGO_PKG_VERSION").to_string());

        config_guard.terminal.nombre = input.terminal_name;
        config_guard.terminal.ubicacion = input.terminal_location;

        let config_path = if let Some(data_dir) = dirs::data_local_dir() {
            data_dir.join("Brisas").join("brisas.toml")
        } else {
            std::path::PathBuf::from("brisas.toml")
        };

        crate::config::save_config(&config_guard, &config_path)
            .map_err(|e| KeyringError::Message(format!("Error guardando configuración: {}", e)))?;
    }

    // 4. 🔥 IMPORTANTE: Disparar el SEED ahora que la llave es segura
    log::info!("🌱 Configuración completada. Iniciando sembrado de base de datos...");
    let db_pool = pool.0.read().await;
    if let Err(e) = crate::config::seed::seed_db(&db_pool).await {
        log::error!("❌ Error al sembrar base de datos tras setup: {}", e);
        // No fallamos el comando completo porque la config ya se guardó, pero avisamos.
    }

    Ok(SetupResult {
        success: true,
        message: "Configuración inicial completada correctamente".to_string(),
    })
}

// ==========================================
// COMANDOS ARGON2 (Solo admin)
// ==========================================

/// Obtiene parámetros de Argon2 (sin el secret)
#[command]
pub fn get_argon2_config() -> Argon2ParamsSafe {
    let params = keyring_service::get_argon2_params();
    Argon2ParamsSafe {
        memory: params.memory,
        iterations: params.iterations,
        parallelism: params.parallelism,
        has_secret: !params.secret.is_empty(),
    }
}

#[derive(Debug, Serialize)]
pub struct Argon2ParamsSafe {
    pub memory: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub has_secret: bool,
}

/// Actualiza parámetros de Argon2
#[command]
pub fn update_argon2_params(params: Argon2Params) -> Result<(), KeyringError> {
    keyring_service::store_argon2_params(&params)
        .map_err(|e| KeyringError::StoreError(e.to_string()))
}

/// Genera un nuevo secret aleatorio para Argon2
#[command]
pub fn generate_argon2_secret() -> String {
    keyring_service::generate_random_secret()
}

// ==========================================
// COMANDOS DE UTILIDAD
// ==========================================

/// Genera un secret aleatorio para usar en configuración
#[command]
pub fn generate_random_secret() -> String {
    keyring_service::generate_random_secret()
}

/// Comando de diagnóstico para probar el keyring
#[command]
pub fn test_keyring() -> Result<String, KeyringError> {
    // Implementación multiplataforma usando el keyring_service
    #[cfg(target_os = "linux")]
    use crate::services::keyring_linux as keyring_impl;

    #[cfg(target_os = "windows")]
    use crate::services::keyring_windows as keyring_impl;

    #[cfg(target_os = "macos")]
    use crate::services::keyring_service as keyring_impl;

    let test_key = "test-brisas-diagnostic";
    let test_value = "test-password-123";
    let mut results = Vec::new();

    // Obtener información del OS
    results.push(format!("Sistema operativo: {}", std::env::consts::OS));
    results.push("".to_string());

    // 1. Guardar credencial
    results.push("1. Guardando credencial de prueba...".to_string());
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    match keyring_impl::store_secret(test_key, test_value) {
        Ok(_) => results.push("   ✓ Credencial guardada correctamente".to_string()),
        Err(e) => {
            results.push(format!("   ✗ Error guardando credencial: {}", e));
            return Ok(results.join("\n"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        use keyring::Entry;
        let entry = Entry::new("test-brisas-diagnostic", test_key)
            .map_err(|e| format!("Error creando entrada: {}", e))?;
        match entry.set_password(test_value) {
            Ok(_) => results.push("   ✓ Credencial guardada correctamente".to_string()),
            Err(e) => {
                results.push(format!("   ✗ Error guardando credencial: {}", e));
                return Ok(results.join("\n"));
            }
        }
    }

    // 2. Recuperar credencial
    results.push("2. Recuperando credencial...".to_string());
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    match keyring_impl::retrieve_secret(test_key) {
        Some(password) => {
            results.push(format!("   ✓ Credencial recuperada: {}", password));
            if password == test_value {
                results.push("   ✓ La credencial coincide!".to_string());
            } else {
                results.push(format!(
                    "   ✗ La credencial NO coincide! Esperado: {}, Obtenido: {}",
                    test_value, password
                ));
            }
        }
        None => {
            results.push("   ✗ Error recuperando credencial".to_string());
            return Ok(results.join("\n"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        use keyring::Entry;
        let entry = Entry::new("test-brisas-diagnostic", test_key)
            .map_err(|e| format!("Error creando entrada: {}", e))?;
        match entry.get_password() {
            Ok(password) => {
                results.push(format!("   ✓ Credencial recuperada: {}", password));
                if password == test_value {
                    results.push("   ✓ La credencial coincide!".to_string());
                } else {
                    results.push(format!(
                        "   ✗ La credencial NO coincide! Esperado: {}, Obtenido: {}",
                        test_value, password
                    ));
                }
            }
            Err(e) => {
                results.push(format!("   ✗ Error recuperando credencial: {}", e));
                return Ok(results.join("\n"));
            }
        }
    }

    // 3. Eliminar credencial
    results.push("3. Eliminando credencial...".to_string());
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    match keyring_impl::delete_secret(test_key) {
        Ok(_) => results.push("   ✓ Credencial eliminada correctamente".to_string()),
        Err(e) => {
            results.push(format!("   ✗ Error eliminando credencial: {}", e));
        }
    }

    #[cfg(target_os = "macos")]
    {
        use keyring::Entry;
        let entry = Entry::new("test-brisas-diagnostic", test_key)
            .map_err(|e| format!("Error creando entrada: {}", e))?;
        match entry.delete_credential() {
            Ok(_) => results.push("   ✓ Credencial eliminada correctamente".to_string()),
            Err(e) => {
                results.push(format!("   ✗ Error eliminando credencial: {}", e));
            }
        }
    }

    // 4. Verificar eliminación
    results.push("4. Verificando eliminación...".to_string());
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    match keyring_impl::retrieve_secret(test_key) {
        Some(password) => {
            results.push(format!("   ✗ La credencial aún existe: {}", password));
        }
        None => {
            results.push("   ✓ La credencial fue eliminada correctamente".to_string());
        }
    }

    #[cfg(target_os = "macos")]
    {
        use keyring::Entry;
        let entry = Entry::new("test-brisas-diagnostic", test_key)
            .map_err(|e| format!("Error creando entrada: {}", e))?;
        match entry.get_password() {
            Ok(password) => {
                results.push(format!("   ✗ La credencial aún existe: {}", password));
            }
            Err(_) => {
                results.push("   ✓ La credencial fue eliminada correctamente".to_string());
            }
        }
    }

    results.push("".to_string());
    results.push("✓ Test completado exitosamente".to_string());
    Ok(results.join("\n"))
}

/// Resetea todas las credenciales (usar con cuidado)
#[command]
pub fn reset_all_credentials(
    confirm: bool,
    config: State<'_, AppConfigState>,
) -> Result<(), KeyringError> {
    if !confirm {
        return Err(KeyringError::Message("Debes confirmar la operación".to_string()));
    }

    // Eliminar credenciales del Keyring del OS
    let _ = keyring_service::delete_argon2_params();

    // Actualizar estado de configuración con write lock
    let mut config_guard = config
        .write()
        .map_err(|e| KeyringError::Message(format!("Error escribiendo configuración: {}", e)))?;

    config_guard.setup.is_configured = false;
    config_guard.setup.configured_at = None;
    config_guard.setup.configured_version = None;

    // También borrar info de terminal para reset completo
    config_guard.terminal.nombre = String::new();
    config_guard.terminal.ubicacion = String::new();

    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("brisas.toml")
    };

    save_config(&config_guard, &config_path)
        .map_err(|e| KeyringError::Message(format!("Error guardando configuración: {}", e)))?;

    Ok(())
}
