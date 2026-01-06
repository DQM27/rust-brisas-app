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

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            memory: 19456, // ~19 MB
            iterations: 2,
            parallelism: 1,
            secret: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllCredentials {
    pub argon2: Argon2Params,
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
// ARGON2 PARAMS
// ==========================================

pub fn store_argon2_params(params: &Argon2Params) -> KeyringResult<()> {
    store_value(KEY_ARGON2_MEMORY, &params.memory.to_string())?;
    store_value(KEY_ARGON2_ITERATIONS, &params.iterations.to_string())?;
    store_value(KEY_ARGON2_PARALLELISM, &params.parallelism.to_string())?;

    // El secreto solo se guarda si no está vacío, o se sobrescribe si ya existe.
    // Para simplificar, guardamos siempre.
    store_value(KEY_PASSWORD_SECRET, &params.secret)?;
    Ok(())
}

pub fn get_argon2_params() -> Argon2Params {
    let memory = retrieve_value(KEY_ARGON2_MEMORY).and_then(|v| v.parse().ok()).unwrap_or(19456);
    let iterations =
        retrieve_value(KEY_ARGON2_ITERATIONS).and_then(|v| v.parse().ok()).unwrap_or(2);
    let parallelism =
        retrieve_value(KEY_ARGON2_PARALLELISM).and_then(|v| v.parse().ok()).unwrap_or(1);
    let secret = retrieve_value(KEY_PASSWORD_SECRET).unwrap_or_default();

    Argon2Params { memory, iterations, parallelism, secret }
}

pub fn has_argon2_secret() -> bool {
    retrieve_value(KEY_PASSWORD_SECRET).map(|s| !s.is_empty()).unwrap_or(false)
}

pub fn delete_argon2_params() -> KeyringResult<()> {
    let _ = delete_value(KEY_ARGON2_MEMORY);
    let _ = delete_value(KEY_ARGON2_ITERATIONS);
    let _ = delete_value(KEY_ARGON2_PARALLELISM);
    let _ = delete_value(KEY_PASSWORD_SECRET);
    Ok(())
}

// ==========================================
// ALL CREDENTIALS
// ==========================================

pub fn get_all_credentials() -> AllCredentials {
    AllCredentials { argon2: get_argon2_params() }
}

pub fn is_fully_configured() -> bool {
    has_argon2_secret()
}

// ==========================================
// CREDENTIAL STATUS (para UI)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialStatus {
    pub argon2_configured: bool,
    pub fully_configured: bool,
}

pub fn get_credential_status() -> CredentialStatus {
    let argon2_configured = has_argon2_secret();
    CredentialStatus { argon2_configured, fully_configured: argon2_configured }
}

// ==========================================
// GENERAR SECRET ALEATORIO
// ==========================================

pub fn generate_random_secret() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes)
}

// ==========================================
// PUBLIC API FOR COMMANDS
// ==========================================

pub fn save_secret(key: &str, value: &str) -> KeyringResult<()> {
    store_value(key, value)
}

pub fn get_secret(key: &str) -> Option<String> {
    retrieve_value(key)
}

pub fn delete_secret(key: &str) -> KeyringResult<()> {
    delete_value(key)
}
