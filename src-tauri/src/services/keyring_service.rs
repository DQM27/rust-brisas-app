// ==========================================
// src/services/keyring_service.rs
// ==========================================
// Servicio para almacenar credenciales de forma segura
// usando el keyring del sistema operativo via `keyring` crate v3.

use keyring::Entry;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SERVICE_NAME: &str = "brisas-app";

// ==========================================
// ERROR TYPES
// ==========================================

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("Error de acceso al llavero: {0}")]
    AccessError(String),

    #[error("Error de almacenamiento: {0}")]
    StorageError(String),

    #[error("Error de recuperación: {0}")]
    RetrievalError(String),

    #[error("Error de eliminación: {0}")]
    DeletionError(String),

    #[error("Plataforma no soportada")]
    UnsupportedPlatform,
}

pub type KeyringResult<T> = Result<T, KeyringError>;

// ==========================================
// CONSTANTES PARA CLAVES
// ==========================================

// Argon2
// Argon2
// CAMBIO IMPORTANTE: Cambiamos el nombre de la clave para evitar conflictos con versiones previas
const KEY_PASSWORD_SECRET: &str = "brisas_argon2_secret_v1";

// ==========================================
// DTOs
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argon2Params {
    pub memory: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialStatus {
    pub has_argon2_config: bool,
    pub has_argon2_secret: bool,
}

// ==========================================
// IMPLEMENTACIÓN UNIFICADA (Windows & General)
// ==========================================

/// Obtiene una entrada segura del llavero para una clave dada.
fn get_entry(key: &str) -> KeyringResult<Entry> {
    Entry::new(SERVICE_NAME, key)
        .map_err(|e| KeyringError::AccessError(format!("Error creando entrada de keyring: {}", e)))
}

/// Almacena un valor secreto.
fn store_value(key: &str, value: &str) -> KeyringResult<()> {
    log::info!("🔐 Guardando en Keyring: key='{}'", key);
    let entry = get_entry(key)?;
    match entry.set_password(value) {
        Ok(_) => {
            log::info!("✅ Guardado exitoso en Keyring: {}", key);
            Ok(())
        }
        Err(e) => {
            log::error!("❌ Fallo al guardar en Keyring '{}': {}", key, e);
            Err(KeyringError::StorageError(format!("Error guardando '{}': {}", key, e)))
        }
    }
}

/// Recupera un valor secreto. Retorna None si no existe o falla.
fn retrieve_value(key: &str) -> Option<String> {
    match get_entry(key) {
        Ok(entry) => match entry.get_password() {
            Ok(pwd) => {
                log::info!("🔓 Recuperado de Keyring: {}", key);
                Some(pwd)
            }
            Err(keyring::Error::NoEntry) => {
                log::warn!("ℹ️ Keyring entry not found: {}", key);
                None
            }
            Err(e) => {
                log::error!("⚠️ Error leyendo Keyring '{}': {}", key, e);
                None
            }
        },
        Err(e) => {
            log::error!("❌ Error accediendo a keyring para '{}': {}", key, e);
            None
        }
    }
}

/// Elimina un valor secreto.
fn delete_value(key: &str) -> KeyringResult<()> {
    log::info!("🗑️ Intentando borrar de Keyring: {}", key);
    let entry = get_entry(key)?;
    match entry.delete_credential() {
        Ok(_) => {
            log::info!("✅ Borrado exitoso: {}", key);
            Ok(())
        }
        Err(keyring::Error::NoEntry) => {
            log::info!("ℹ️ No había nada que borrar para: {}", key);
            Ok(())
        }
        Err(e) => Err(KeyringError::DeletionError(format!("Error borrando '{}': {}", key, e))),
    }
}

// ==========================================
// API PÚBLICA (Fachada del Servicio)
// ==========================================

/// Genera un secreto aleatorio seguro (ahora en HEX para máxima compatibilidad).
pub fn generate_random_secret() -> String {
    use rand::Rng;
    let random_bytes: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
    hex::encode(&random_bytes)
}

/// Helper para obtener ruta de archivo de respaldo
fn get_fallback_path() -> std::path::PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("Brisas");
    std::fs::create_dir_all(&path).ok();
    path.push(".credentials");
    path
}

/// Guarda SOLO el secreto de Argon2.
/// Estrategia: Keyring (Prioridad) -> Archivo Local (Fallback)
pub fn store_argon2_params(params: &Argon2Params) -> KeyringResult<()> {
    // 1. Intentar Keyring
    let keyring_attempt = store_value(KEY_PASSWORD_SECRET, &params.secret);

    // 2. Verificar Keyring
    let mut keyring_verified = false;
    if keyring_attempt.is_ok() {
        // Pequeña pausa para dar tiempo al OS
        std::thread::sleep(std::time::Duration::from_millis(100));
        if retrieve_value(KEY_PASSWORD_SECRET).is_some() {
            keyring_verified = true;
            log::info!("✅ Keyring verificado correctamente.");
        } else {
            log::warn!("⚠️ Keyring reportó éxito pero falló verificación de lectura.");
        }
    }

    // 3. Si Keyring falló, usar Archivo
    if !keyring_verified {
        log::warn!("⚠️ Usando almacenamiento en archivo (.credentials) como fallback.");
        let path = get_fallback_path();
        // Guardamos el secret directamente (en un escenario real debería ir cifrado,
        // pero aquí la prioridad es que funcione ante fallo de Keyring)
        match std::fs::write(&path, &params.secret) {
            Ok(_) => log::info!("✅ Secreto guardado en archivo de respaldo: {:?}", path),
            Err(e) => {
                log::error!("❌ Fallo total: Ni Keyring ni Archivo funcionaron. {}", e);
                return Err(KeyringError::StorageError(format!("Fallo persistencia total: {}", e)));
            }
        }
    }

    Ok(())
}

/// Recupera los parámetros de Argon2.
/// Estrategia: Keyring -> Archivo -> Default
pub fn get_argon2_params() -> Argon2Params {
    // 1. Intentar Keyring
    let secret = if let Some(s) = retrieve_value(KEY_PASSWORD_SECRET) {
        s
    } else {
        // 2. Intentar Archivo Fallback
        let path = get_fallback_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(s) => {
                    log::info!("🔓 Secreto recuperado desde archivo de respaldo");
                    s.trim().to_string()
                }
                Err(_) => String::new(),
            }
        } else {
            String::new()
        }
    };

    // VALORES POR DEFECTO FIJOS (Simplificación solicitada)
    // Memory: 19MB | Iterations: 2 | Parallelism: 1
    Argon2Params { memory: 19456, iterations: 2, parallelism: 1, secret }
}

/// Elimina la configuración de Argon2 (el secreto).
pub fn delete_argon2_params() -> KeyringResult<()> {
    delete_value(KEY_PASSWORD_SECRET)?;
    Ok(())
}

/// Verifica si existe un secreto de Argon2 configurado.
pub fn has_argon2_secret() -> bool {
    retrieve_value(KEY_PASSWORD_SECRET).is_some()
}

/// Obtiene el estado actual de las credenciales.
pub fn get_credential_status() -> CredentialStatus {
    CredentialStatus {
        has_argon2_config: true, // Siempre true por defaults
        has_argon2_secret: has_argon2_secret(),
    }
}

/// Verifica si la configuración de seguridad está completa.
pub fn is_fully_configured() -> bool {
    has_argon2_secret()
}

// Helper methods for generic secret storage (used by security_commands directly)
pub fn save_secret(key: &str, value: &str) -> KeyringResult<()> {
    store_value(key, value)
}

pub fn get_secret(key: &str) -> Option<String> {
    retrieve_value(key)
}

pub fn delete_secret(key: &str) -> KeyringResult<()> {
    delete_value(key)
}
