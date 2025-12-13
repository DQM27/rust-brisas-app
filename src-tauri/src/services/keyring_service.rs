// ==========================================
// src/services/keyring_service.rs
// ==========================================
// Servicio para almacenar credenciales de forma segura
// usando el keyring del sistema operativo:
// - Windows: Credential Manager
// - macOS: Keychain
// - Linux: Secret Service (GNOME Keyring, KWallet)

#[cfg(not(target_os = "linux"))]
use keyring::Entry;

use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "brisas-app";

// ==========================================
// CONSTANTES PARA CLAVES
// ==========================================

// SMTP
const KEY_SMTP_HOST: &str = "smtp_host";
const KEY_SMTP_PORT: &str = "smtp_port";
const KEY_SMTP_USER: &str = "smtp_user";
const KEY_SMTP_PASSWORD: &str = "smtp_password";
const KEY_FEEDBACK_EMAIL: &str = "feedback_email";

// Argon2
const KEY_ARGON2_MEMORY: &str = "argon2_memory";
const KEY_ARGON2_ITERATIONS: &str = "argon2_iterations";
const KEY_ARGON2_PARALLELISM: &str = "argon2_parallelism";
const KEY_PASSWORD_SECRET: &str = "password_secret";

// SQLite
const KEY_SQLITE_PASSWORD: &str = "sqlite_password";

// ==========================================
// DTOs
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpCredentials {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub feedback_email: String,
}

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
            memory: 19456,      // ~19 MB
            iterations: 2,
            parallelism: 1,
            secret: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllCredentials {
    pub smtp: Option<SmtpCredentials>,
    pub argon2: Argon2Params,
    pub sqlite_password: Option<String>,
}

// ==========================================
// FUNCIONES AUXILIARES
// ==========================================

// Implementación para Linux usando secret-tool directamente
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
fn delete_value(key: &str) -> Result<(), String> {
    use crate::services::keyring_linux;
    keyring_linux::delete_secret(key)
}

// Implementación para Windows/macOS usando la librería keyring
#[cfg(not(target_os = "linux"))]
fn get_entry(key: &str) -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, key)
        .map_err(|e| format!("Error creando entrada keyring para '{}': {}", key, e))
}

#[cfg(not(target_os = "linux"))]
fn store_value(key: &str, value: &str) -> Result<(), String> {
    eprintln!("[KEYRING DEBUG] Intentando guardar: service='{}', key='{}', value_len={}", SERVICE_NAME, key, value.len());
    let entry = get_entry(key)?;
    entry.set_password(value)
        .map_err(|e| format!("Error almacenando '{}': {}", key, e))
}

#[cfg(not(target_os = "linux"))]
fn retrieve_value(key: &str) -> Option<String> {
    get_entry(key)
        .ok()
        .and_then(|entry| entry.get_password().ok())
}

#[cfg(not(target_os = "linux"))]
fn delete_value(key: &str) -> Result<(), String> {
    let entry = get_entry(key)?;
    entry.delete_credential()
        .map_err(|e| format!("Error eliminando '{}': {}", key, e))
}

// ==========================================
// SMTP CREDENTIALS
// ==========================================

pub fn store_smtp_credentials(creds: &SmtpCredentials) -> Result<(), String> {
    store_value(KEY_SMTP_HOST, &creds.host)?;
    store_value(KEY_SMTP_PORT, &creds.port.to_string())?;
    store_value(KEY_SMTP_USER, &creds.user)?;
    store_value(KEY_SMTP_PASSWORD, &creds.password)?;
    store_value(KEY_FEEDBACK_EMAIL, &creds.feedback_email)?;
    Ok(())
}

pub fn get_smtp_credentials() -> Option<SmtpCredentials> {
    let host = retrieve_value(KEY_SMTP_HOST)?;
    let port_str = retrieve_value(KEY_SMTP_PORT)?;
    let port = port_str.parse().ok()?;
    let user = retrieve_value(KEY_SMTP_USER)?;
    let password = retrieve_value(KEY_SMTP_PASSWORD)?;
    let feedback_email = retrieve_value(KEY_FEEDBACK_EMAIL)?;

    Some(SmtpCredentials {
        host,
        port,
        user,
        password,
        feedback_email,
    })
}

pub fn has_smtp_credentials() -> bool {
    retrieve_value(KEY_SMTP_HOST).is_some()
        && retrieve_value(KEY_SMTP_USER).is_some()
        && retrieve_value(KEY_SMTP_PASSWORD).is_some()
}

pub fn delete_smtp_credentials() -> Result<(), String> {
    let _ = delete_value(KEY_SMTP_HOST);
    let _ = delete_value(KEY_SMTP_PORT);
    let _ = delete_value(KEY_SMTP_USER);
    let _ = delete_value(KEY_SMTP_PASSWORD);
    let _ = delete_value(KEY_FEEDBACK_EMAIL);
    Ok(())
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
// SQLITE PASSWORD
// ==========================================

pub fn store_sqlite_password(password: &str) -> Result<(), String> {
    store_value(KEY_SQLITE_PASSWORD, password)
}

pub fn get_sqlite_password() -> Option<String> {
    retrieve_value(KEY_SQLITE_PASSWORD)
}

pub fn has_sqlite_password() -> bool {
    retrieve_value(KEY_SQLITE_PASSWORD).is_some()
}

pub fn delete_sqlite_password() -> Result<(), String> {
    delete_value(KEY_SQLITE_PASSWORD)
}

// ==========================================
// ALL CREDENTIALS
// ==========================================

pub fn get_all_credentials() -> AllCredentials {
    AllCredentials {
        smtp: get_smtp_credentials(),
        argon2: get_argon2_params(),
        sqlite_password: get_sqlite_password(),
    }
}

pub fn is_fully_configured() -> bool {
    has_smtp_credentials() && has_argon2_secret()
}

// ==========================================
// CREDENTIAL STATUS (para UI)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialStatus {
    pub smtp_configured: bool,
    pub argon2_configured: bool,
    pub sqlite_configured: bool,
    pub fully_configured: bool,
}

pub fn get_credential_status() -> CredentialStatus {
    let smtp_configured = has_smtp_credentials();
    let argon2_configured = has_argon2_secret();
    let sqlite_configured = has_sqlite_password();

    CredentialStatus {
        smtp_configured,
        argon2_configured,
        sqlite_configured,
        fully_configured: smtp_configured && argon2_configured,
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
