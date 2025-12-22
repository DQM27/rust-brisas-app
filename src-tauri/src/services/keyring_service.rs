// ==========================================
// src/services/keyring_service.rs
// ==========================================
// Servicio para almacenar credenciales de forma segura
// usando el keyring del sistema operativo:
// - Windows: Credential Manager (nativo)
// - macOS: Keychain (librería keyring)
// - Linux: Secret Service (secret-tool nativo)

#[cfg(target_os = "macos")]
use keyring::Entry;

use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
const SERVICE_NAME: &str = "brisas-app";

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
// FUNCIONES AUXILIARES
// ==========================================

// ==========================================
// IMPLEMENTACIÓN LINUX (secret-tool nativo)
// ==========================================
#[cfg(target_os = "linux")]
fn store_value(key: &str, value: &str) -> Result<(), String> {
    use crate::services::keyring_linux;
    keyring_linux::store_secret(key, value)
}

#[cfg(target_os = "linux")]
fn retrieve_value(key: &str) -> Option<String> {
    use crate::services::keyring_linux;
    keyring_linux::retrieve_secret(key)
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
fn delete_value(key: &str) -> Result<(), String> {
    use crate::services::keyring_linux;
    keyring_linux::delete_secret(key)
}

// ==========================================
// IMPLEMENTACIÓN WINDOWS (Credential Manager nativo)
// ==========================================
#[cfg(target_os = "windows")]
fn store_value(key: &str, value: &str) -> Result<(), String> {
    use crate::services::keyring_windows;
    keyring_windows::store_secret(key, value)
}

#[cfg(target_os = "windows")]
fn retrieve_value(key: &str) -> Option<String> {
    use crate::services::keyring_windows;
    keyring_windows::retrieve_secret(key)
}

// ==========================================
// IMPLEMENTACIÓN MACOS (librería keyring)
// ==========================================
#[cfg(target_os = "macos")]
fn get_entry(key: &str) -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, key)
        .map_err(|e| format!("Error creando entrada keyring para '{}': {}", key, e))
}

#[cfg(target_os = "macos")]
fn store_value(key: &str, value: &str) -> Result<(), String> {
    let entry = get_entry(key)?;
    entry
        .set_password(value)
        .map_err(|e| format!("Error almacenando '{}': {}", key, e))
}

#[cfg(target_os = "macos")]
fn retrieve_value(key: &str) -> Option<String> {
    get_entry(key)
        .ok()
        .and_then(|entry| entry.get_password().ok())
}

// ==========================================
// ARGON2 PARAMS
// ==========================================

pub fn store_argon2_params(params: &Argon2Params) -> Result<(), String> {
    store_value(KEY_ARGON2_MEMORY, &params.memory.to_string())?;
    store_value(KEY_ARGON2_ITERATIONS, &params.iterations.to_string())?;
    store_value(KEY_ARGON2_PARALLELISM, &params.parallelism.to_string())?;
    store_value(KEY_PASSWORD_SECRET, &params.secret)?;
    Ok(())
}

pub fn get_argon2_params() -> Argon2Params {
    let memory = retrieve_value(KEY_ARGON2_MEMORY)
        .and_then(|v| v.parse().ok())
        .unwrap_or(19456);
    let iterations = retrieve_value(KEY_ARGON2_ITERATIONS)
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);
    let parallelism = retrieve_value(KEY_ARGON2_PARALLELISM)
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let secret = retrieve_value(KEY_PASSWORD_SECRET).unwrap_or_default();

    Argon2Params {
        memory,
        iterations,
        parallelism,
        secret,
    }
}

pub fn has_argon2_secret() -> bool {
    retrieve_value(KEY_PASSWORD_SECRET)
        .map(|s| !s.is_empty())
        .unwrap_or(false)
}

// ==========================================
// ALL CREDENTIALS
// ==========================================

pub fn get_all_credentials() -> AllCredentials {
    AllCredentials {
        argon2: get_argon2_params(),
    }
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

    CredentialStatus {
        argon2_configured,
        fully_configured: argon2_configured,
    }
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
