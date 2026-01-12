// ==========================================
// src/services/keyring_service.rs
// ==========================================
// Servicio para almacenar credenciales de forma segura
// usando el keyring del sistema operativo via `keyring` crate v3.

use crate::domain::errors::KeyringError;
use keyring::Entry;
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "brisas-app";
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
    #[serde(rename = "argon2_configured")]
    pub has_argon2_secret: bool,
    pub fully_configured: bool,
}

pub type KeyringResult<T> = Result<T, KeyringError>;

// ==========================================
// IMPLEMENTACIÓN UNIFICADA (Windows & General)
// ==========================================

/// Obtiene una entrada segura del llavero para una clave dada.
fn get_entry(key: &str) -> KeyringResult<Entry> {
    Entry::new(SERVICE_NAME, key)
        .map_err(|e| KeyringError::Message(format!("Error creando entrada de keyring: {e}")))
}

/// Almacena un valor secreto.
fn store_value(key: &str, value: &str) -> KeyringResult<()> {
    log::info!("🔐 Guardando en Keyring: key='{key}'");
    let entry = get_entry(key)?;
    match entry.set_password(value) {
        Ok(()) => {
            log::info!("✅ Guardado exitoso en Keyring: {key}");
            Ok(())
        }
        Err(e) => {
            log::error!("❌ Fallo al guardar en Keyring '{key}': {e}");
            Err(KeyringError::StoreError(format!("Error guardando '{key}': {e}")))
        }
    }
}

/// Recupera un valor secreto. Retorna None si no existe o falla.
fn retrieve_value(key: &str) -> Option<String> {
    match get_entry(key) {
        Ok(entry) => match entry.get_password() {
            Ok(pwd) => {
                log::info!("🔓 Recuperado de Keyring: {key}");
                Some(pwd)
            }
            Err(keyring::Error::NoEntry) => {
                log::warn!("ℹ️ Keyring entry not found: {key}");
                None
            }
            Err(e) => {
                log::error!("⚠️ Error leyendo Keyring '{key}': {e}");
                None
            }
        },
        Err(e) => {
            log::error!("❌ Error accediendo a keyring para '{key}': {e}");
            None
        }
    }
}

/// Elimina un valor secreto.
fn delete_value(key: &str) -> KeyringResult<()> {
    log::info!("🗑️ Intentando borrar de Keyring: {key}");
    let entry = get_entry(key)?;
    match entry.delete_credential() {
        Ok(()) => {
            log::info!("✅ Borrado exitoso: {key}");
            Ok(())
        }
        Err(keyring::Error::NoEntry) => {
            log::info!("ℹ️ No había nada que borrar para: {key}");
            Ok(())
        }
        Err(e) => Err(KeyringError::DeleteError(format!("Error borrando '{key}': {e}"))),
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
            Ok(()) => log::info!("✅ Secreto guardado en archivo de respaldo: {}", path.display()),
            Err(e) => {
                log::error!("❌ Fallo total: Ni Keyring ni Archivo funcionaron. {e}");
                return Err(KeyringError::StoreError(format!("Fallo persistencia total: {e}")));
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

    // VALORES POR DEFECTO FIJOS
    Argon2Params { memory: 19456, iterations: 2, parallelism: 1, secret }
}

/// Elimina la configuración de Argon2 (el secreto).
pub fn delete_argon2_params() -> KeyringResult<()> {
    delete_value(KEY_PASSWORD_SECRET)?;
    Ok(())
}

/// Verifica si existe un secreto de Argon2 configurado.
pub fn has_argon2_secret() -> bool {
    !get_argon2_params().secret.is_empty()
}

/// Obtiene el estado actual de las credenciales.
pub fn get_credential_status() -> CredentialStatus {
    let has_secret = has_argon2_secret();
    CredentialStatus { has_argon2_secret: has_secret, fully_configured: has_secret }
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

// ==========================================
// SEGURIDAD: Master Key Export/Import
// ==========================================

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterKeyFile {
    pub version: u32,
    pub created_at: String,
    pub salt: String,       // Para derivación Argon2 de la contraseña
    pub nonce: String,      // Para ChaCha20
    pub ciphertext: String, // Pepper cifrado (Hex)
    pub checksum: String,   // SHA256 del pepper original (Hex)
}

/// Deriva una llave de cifrado de 32 bytes a partir de una contraseña humana.
fn derive_key_from_password(password: &str, salt_str: &str) -> Result<Key, KeyringError> {
    let salt = SaltString::from_b64(salt_str)
        .map_err(|e| KeyringError::Message(format!("Salt inválido: {e}")))?;

    let mut key_buffer = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key_buffer)
        .map_err(|e| KeyringError::Message(format!("Error derivando llave: {e}")))?;

    Ok(*Key::from_slice(&key_buffer))
}

/// Exporta el Master Key (Pepper) actual a un archivo cifrado con contraseña.
pub fn export_master_key(file_path: PathBuf, password: &str) -> KeyringResult<()> {
    if password.len() < 8 {
        return Err(KeyringError::Message(
            "La contraseña debe tener al menos 8 caracteres".to_string(),
        ));
    }

    // 1. Obtener el Pepper actual
    let params = get_argon2_params();
    if params.secret.is_empty() {
        return Err(KeyringError::RetrieveError(
            "No hay un secreto maestro configurado para exportar".to_string(),
        ));
    }
    let pepper_bytes = params.secret.as_bytes();

    // 2. Generar Salt y Nonce
    let salt = SaltString::generate(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    // 3. Derivar llave de cifrado
    let key = derive_key_from_password(password, salt.as_str())?;
    let cipher = ChaCha20Poly1305::new(&key);

    // 4. Cifrar el Pepper
    let ciphertext = cipher
        .encrypt(&nonce, pepper_bytes)
        .map_err(|e| KeyringError::StoreError(format!("Error cifrando: {e}")))?;

    // 5. Calcular Checksum (SHA256 del pepper original)
    let mut hasher = Sha256::new();
    hasher.update(pepper_bytes);
    let checksum = hex::encode(hasher.finalize());

    // 6. Crear estructura y guardar JSON
    let master_key_file = MasterKeyFile {
        version: 1,
        created_at: chrono::Utc::now().to_rfc3339(),
        salt: salt.as_str().to_string(),
        nonce: hex::encode(nonce),
        ciphertext: hex::encode(ciphertext),
        checksum,
    };

    let json = serde_json::to_string_pretty(&master_key_file)
        .map_err(|e| KeyringError::StoreError(format!("Error serializando JSON: {e}")))?;

    std::fs::write(&file_path, json)
        .map_err(|e| KeyringError::StoreError(format!("Error escribiendo archivo: {e}")))?;

    log::info!("✅ Master Key exportada exitosamente a: {:?}", file_path);
    Ok(())
}

/// Importa un Master Key desde un archivo cifrado.
pub fn import_master_key(file_path: PathBuf, password: &str) -> KeyringResult<()> {
    // 1. Leer archivo
    let json = std::fs::read_to_string(&file_path)
        .map_err(|e| KeyringError::RetrieveError(format!("No se puede leer el archivo: {e}")))?;

    let master_file: MasterKeyFile = serde_json::from_str(&json)
        .map_err(|e| KeyringError::Message(format!("Formato de archivo inválido: {e}")))?;

    // 2. Derivar llave de descifrado
    let key = derive_key_from_password(password, &master_file.salt)?;
    let cipher = ChaCha20Poly1305::new(&key);

    // 3. Decodificar componentes
    let nonce_bytes = hex::decode(&master_file.nonce)
        .map_err(|e| KeyringError::Message(format!("Nonce inválido: {e}")))?;
    let ciphertext_bytes = hex::decode(&master_file.ciphertext)
        .map_err(|e| KeyringError::Message(format!("Ciphertext inválido: {e}")))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4. Descifrar Pepper
    let decrypted_bytes = cipher.decrypt(nonce, ciphertext_bytes.as_ref()).map_err(|_| {
        KeyringError::Message("Contraseña incorrecta o archivo corrupto".to_string())
    })?;

    let pepper = String::from_utf8(decrypted_bytes)
        .map_err(|e| KeyringError::Message(format!("Pepper no es UTF-8 válido: {e}")))?;

    // 5. Validar Checksum
    let mut hasher = Sha256::new();
    hasher.update(pepper.as_bytes());
    let calculated_checksum = hex::encode(hasher.finalize());

    if calculated_checksum != master_file.checksum {
        return Err(KeyringError::Message(
            "Integridad fallida: El checksum no coincide".to_string(),
        ));
    }

    // 6. Guardar en el sistema actual
    let params = Argon2Params { memory: 19456, iterations: 2, parallelism: 1, secret: pepper };

    store_argon2_params(&params)?;
    log::info!("✅ Master Key importada y configurada exitosamente.");
    Ok(())
}
