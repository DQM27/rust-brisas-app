// ==========================================
// src/commands/keyring_commands.rs
// ==========================================
// Comandos Tauri para gestión segura de credenciales

use crate::config::{save_config, AppConfig};
use crate::services::keyring_service::{self, Argon2Params, CredentialStatus, SmtpCredentials};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::{command, State};

// ==========================================
// DTOs PARA COMANDOS
// ==========================================

#[derive(Debug, Deserialize)]
pub struct SetupCredentialsInput {
    pub smtp: SmtpCredentials,
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
pub fn is_app_configured(config: State<'_, AppConfig>) -> bool {
    config.setup.is_configured && keyring_service::is_fully_configured()
}

/// Verifica si necesita ejecutar el wizard de configuración
#[command]
pub fn needs_setup(config: State<'_, AppConfig>) -> bool {
    !config.setup.is_configured || !keyring_service::is_fully_configured()
}

// ==========================================
// COMANDOS DE SETUP INICIAL
// ==========================================

/// Configura todas las credenciales en el primer uso
#[command]
pub fn setup_credentials(
    input: SetupCredentialsInput,
    config: State<'_, AppConfig>,
) -> Result<SetupResult, String> {
    // 1. Guardar credenciales SMTP
    keyring_service::store_smtp_credentials(&input.smtp)?;

    // 2. Guardar parámetros de Argon2
    keyring_service::store_argon2_params(&input.argon2)?;

    // 3. Actualizar configuración en TOML
    let mut updated_config = config.inner().clone();

    // Actualizar datos de setup
    updated_config.setup.is_configured = true;
    updated_config.setup.configured_at = Some(Utc::now().to_rfc3339());
    updated_config.setup.configured_version = Some(env!("CARGO_PKG_VERSION").to_string());

    // Actualizar datos de terminal
    updated_config.terminal.nombre = input.terminal_name;
    updated_config.terminal.ubicacion = input.terminal_location;

    // Guardar config actualizada
    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&updated_config, &config_path)
        .map_err(|e| format!("Error guardando configuración: {}", e))?;

    Ok(SetupResult {
        success: true,
        message: "Configuración inicial completada correctamente".to_string(),
    })
}

// ==========================================
// COMANDOS SMTP (Solo admin)
// ==========================================

/// Obtiene credenciales SMTP (sin la contraseña completa por seguridad)
#[command]
pub fn get_smtp_config() -> Option<SmtpCredentialsSafe> {
    keyring_service::get_smtp_credentials().map(|creds| SmtpCredentialsSafe {
        host: creds.host,
        port: creds.port,
        user: creds.user,
        has_password: true,
        feedback_email: creds.feedback_email,
    })
}

#[derive(Debug, Serialize)]
pub struct SmtpCredentialsSafe {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub has_password: bool,
    pub feedback_email: String,
}

/// Actualiza credenciales SMTP
#[command]
pub fn update_smtp_credentials(creds: SmtpCredentials) -> Result<(), String> {
    keyring_service::store_smtp_credentials(&creds)
}

/// Prueba la conexión SMTP con credenciales guardadas
#[command]
pub async fn test_smtp_connection() -> Result<String, String> {
    let creds =
        keyring_service::get_smtp_credentials().ok_or("No hay credenciales SMTP configuradas")?;

    test_smtp_with_credentials(creds).await
}

/// Prueba la conexión SMTP con credenciales proporcionadas (para el wizard)
#[command]
pub async fn test_smtp_connection_with_creds(creds: SmtpCredentials) -> Result<String, String> {
    test_smtp_with_credentials(creds).await
}

/// Función interna para probar conexión SMTP
async fn test_smtp_with_credentials(creds: SmtpCredentials) -> Result<String, String> {
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::SmtpTransport;

    let smtp_creds = Credentials::new(creds.user.clone(), creds.password.clone());

    let mailer = SmtpTransport::relay(&creds.host)
        .map_err(|e| format!("Error conectando: {}", e))?
        .port(creds.port)
        .credentials(smtp_creds)
        .build();

    // Probar conexión
    mailer
        .test_connection()
        .map_err(|e| format!("Error de conexión SMTP: {}", e))?;

    Ok("Conexión SMTP exitosa".to_string())
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
pub fn update_argon2_params(params: Argon2Params) -> Result<(), String> {
    keyring_service::store_argon2_params(&params)
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
pub fn test_keyring() -> Result<String, String> {
    use keyring::Entry;

    let service = "test-brisas-diagnostic";
    let username = "test-user";
    let test_value = "test-password-123";

    let mut results = Vec::new();

    // 1. Crear entrada
    results.push("1. Creando entrada en keyring...".to_string());
    let entry = match Entry::new(service, username) {
        Ok(e) => {
            results.push("   ✓ Entrada creada correctamente".to_string());
            e
        }
        Err(e) => {
            results.push(format!("   ✗ Error creando entrada: {}", e));
            return Ok(results.join("\n"));
        }
    };

    // 2. Guardar contraseña
    results.push("2. Guardando contraseña...".to_string());
    match entry.set_password(test_value) {
        Ok(_) => results.push("   ✓ Contraseña guardada correctamente".to_string()),
        Err(e) => {
            results.push(format!("   ✗ Error guardando contraseña: {}", e));
            results.push(format!("   Detalles: {:?}", e));
            return Ok(results.join("\n"));
        }
    }

    // 3. Recuperar contraseña
    results.push("3. Recuperando contraseña...".to_string());
    match entry.get_password() {
        Ok(password) => {
            results.push(format!("   ✓ Contraseña recuperada: {}", password));
            if password == test_value {
                results.push("   ✓ La contraseña coincide!".to_string());
            } else {
                results.push(format!(
                    "   ✗ La contraseña NO coincide! Esperado: {}, Obtenido: {}",
                    test_value, password
                ));
            }
        }
        Err(e) => {
            results.push(format!("   ✗ Error recuperando contraseña: {}", e));
            return Ok(results.join("\n"));
        }
    }

    // 4. Eliminar contraseña
    results.push("4. Eliminando contraseña...".to_string());
    match entry.delete_credential() {
        Ok(_) => results.push("   ✓ Contraseña eliminada correctamente".to_string()),
        Err(e) => {
            results.push(format!("   ✗ Error eliminando contraseña: {}", e));
        }
    }

    // 5. Verificar eliminación
    results.push("5. Verificando eliminación...".to_string());
    match entry.get_password() {
        Ok(password) => {
            results.push(format!("   ✗ La contraseña aún existe: {}", password));
        }
        Err(_) => {
            results.push("   ✓ La contraseña fue eliminada correctamente".to_string());
        }
    }

    results.push("\n✓ Test completado exitosamente".to_string());
    Ok(results.join("\n"))
}

/// Resetea todas las credenciales (usar con cuidado)
#[command]
pub fn reset_all_credentials(confirm: bool, config: State<'_, AppConfig>) -> Result<(), String> {
    if !confirm {
        return Err("Debes confirmar la operación".to_string());
    }

    // Eliminar credenciales
    let _ = keyring_service::delete_smtp_credentials();

    // Actualizar estado de configuración
    let mut updated_config = config.inner().clone();
    updated_config.setup.is_configured = false;
    updated_config.setup.configured_at = None;
    updated_config.setup.configured_version = None;

    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&updated_config, &config_path)
        .map_err(|e| format!("Error guardando configuración: {}", e))?;

    Ok(())
}
