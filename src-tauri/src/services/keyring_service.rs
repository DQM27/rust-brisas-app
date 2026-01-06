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
const KEY_ARGON2_MEMORY: &str = "argon2_memory";
const KEY_ARGON2_ITERATIONS: &str = "argon2_iterations";
const KEY_ARGON2_PARALLELISM: &str = "argon2_parallelism";
const KEY_PASSWORD_SECRET: &str = "password_secret";

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
    let entry = get_entry(key)?;
    entry
        .set_password(value)
        .map_err(|e| KeyringError::StorageError(format!("Error guardando '{}': {}", key, e)))
}

/// Recupera un valor secreto. Retorna None si no existe o falla.
fn retrieve_value(key: &str) -> Option<String> {
    match get_entry(key) {
        Ok(entry) => match entry.get_password() {
            Ok(pwd) => Some(pwd),
            Err(_) => None,
        },
        Err(e) => {
            log::warn!("Error accediendo a keyring para '{}': {}", key, e);
            None
        }
    }
}

/// Elimina un valor secreto.
fn delete_value(key: &str) -> KeyringResult<()> {
    let entry = get_entry(key)?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(KeyringError::DeletionError(format!("Error borrando '{}': {}", key, e))),
    }
}

// ==========================================
// API PÚBLICA (Fachada del Servicio)
// ==========================================

/// Genera un secreto aleatorio seguro (fines de utilidad).
pub fn generate_random_secret() -> String {
    use rand::Rng;
    let random_bytes: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.encode(&random_bytes)
}

/// Guarda la configuración completa de Argon2.
pub fn store_argon2_params(params: &Argon2Params) -> KeyringResult<()> {
    store_value(KEY_ARGON2_MEMORY, &params.memory.to_string())?;
    store_value(KEY_ARGON2_ITERATIONS, &params.iterations.to_string())?;
    store_value(KEY_ARGON2_PARALLELISM, &params.parallelism.to_string())?;
    store_value(KEY_PASSWORD_SECRET, &params.secret)?;
    Ok(())
}

/// Recupera la configuración de Argon2.
/// Si falla la recuperación de algún valor, retorna defaults seguros.
pub fn get_argon2_params() -> Argon2Params {
    let memory = retrieve_value(KEY_ARGON2_MEMORY).and_then(|v| v.parse().ok()).unwrap_or(19456);

    let iterations =
        retrieve_value(KEY_ARGON2_ITERATIONS).and_then(|v| v.parse().ok()).unwrap_or(2);

    let parallelism =
        retrieve_value(KEY_ARGON2_PARALLELISM).and_then(|v| v.parse().ok()).unwrap_or(1);

    let secret = retrieve_value(KEY_PASSWORD_SECRET).unwrap_or_default();

    Argon2Params { memory, iterations, parallelism, secret }
}

/// Elimina toda la configuración de Argon2.
pub fn delete_argon2_params() -> KeyringResult<()> {
    delete_value(KEY_ARGON2_MEMORY)?;
    delete_value(KEY_ARGON2_ITERATIONS)?;
    delete_value(KEY_ARGON2_PARALLELISM)?;
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
        has_argon2_config: retrieve_value(KEY_ARGON2_MEMORY).is_some(),
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
