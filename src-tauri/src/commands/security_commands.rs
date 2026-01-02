/// Núcleo de Seguridad: Criptografía y Gestión de Llaves Maestras (Cipher Core).
///
/// Este submódulo gestiona la persistencia de la 'Master Key' del sistema,
/// integrándose con los llaveros nativos (Keyring) de cada sistema operativo
/// (Windows Credential Manager, Linux Secret-tool, macOS Keychain) para
/// garantizar que los datos sensibles (Avatares, etc.) permanezcan seguros.
use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::OnceLock;

/// Identificador único para localizar la llave en el almacén seguro del sistema operativo.
const MASTER_KEY_NAME: &str = "encryption_master_key";

/// Memoria Caché de Seguridad: Mantiene la llave descifrada en memoria RAM durante el
/// tiempo de ejecución para optimizar las operaciones criptográficas reactivas.
static MASTER_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Protocolo de Recuperación: Establece un bridge con el Keyring nativo. Si la llave no existe,
/// genera una nueva con entropía de grado militar y la guarda de forma persistente.
pub fn get_master_key() -> Result<&'static [u8; 32], String> {
    if let Some(key) = MASTER_KEY.get() {
        return Ok(key);
    }

    // Adaptador Multiplataforma para Almacenamiento Seguro
    #[cfg(target_os = "linux")]
    {
        use crate::services::keyring_linux;

        if let Some(hex_key) = keyring_linux::retrieve_secret(MASTER_KEY_NAME) {
            if let Ok(bytes) = hex::decode(hex_key.trim()) {
                if bytes.len() == 32 {
                    let mut key_arr = [0u8; 32];
                    key_arr.copy_from_slice(&bytes);
                    let _ = MASTER_KEY.set(key_arr);
                    log::info!("🔑 Llave Maestra cargada desde Llavero de Linux (secret-tool)");
                    return Ok(MASTER_KEY.get().unwrap());
                }
            }
        }

        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        log::info!("🔑 Iniciando generación de Llave Maestra por primera vez");

        let hex_key = hex::encode(key);
        if let Err(e) = keyring_linux::store_secret(MASTER_KEY_NAME, &hex_key) {
            log::error!("❌ Error crítico al persistir llave en el llavero: {}", e);
            return Err(format!("Fallo de seguridad en el almacenamiento: {}", e));
        }
        log::info!("🔑 Llave Maestra persistida en Llavero de Linux");

        let _ = MASTER_KEY.set(key);
        return Ok(MASTER_KEY.get().unwrap());
    }

    #[cfg(target_os = "windows")]
    {
        use crate::services::keyring_windows;

        if let Some(hex_key) = keyring_windows::retrieve_secret(MASTER_KEY_NAME) {
            if let Ok(bytes) = hex::decode(&hex_key) {
                if bytes.len() == 32 {
                    let mut key_arr = [0u8; 32];
                    key_arr.copy_from_slice(&bytes);
                    let _ = MASTER_KEY.set(key_arr);
                    log::info!("🔑 Llave Maestra cargada desde Windows Credential Manager");
                    return Ok(MASTER_KEY.get().unwrap());
                }
            }
        }

        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let hex_key = hex::encode(key);
        keyring_windows::store_secret(MASTER_KEY_NAME, &hex_key)
            .map_err(|e| format!("Error al blindar la llave maestra: {e}"))?;
        log::info!("🔑 Llave Maestra generada y blindada en Windows");
        let _ = MASTER_KEY.set(key);
        Ok(MASTER_KEY.get().unwrap())
    }

    #[cfg(target_os = "macos")]
    {
        use keyring::Entry;

        let entry = Entry::new("BrisasApp", MASTER_KEY_NAME).map_err(|e| e.to_string())?;

        if let Ok(hex_key) = entry.get_password() {
            if let Ok(bytes) = hex::decode(&hex_key) {
                if bytes.len() == 32 {
                    let mut key_arr = [0u8; 32];
                    key_arr.copy_from_slice(&bytes);
                    let _ = MASTER_KEY.set(key_arr);
                    return Ok(MASTER_KEY.get().unwrap());
                }
            }
        }

        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let hex_key = hex::encode(key);
        entry.set_password(&hex_key).map_err(|e| e.to_string())?;
        let _ = MASTER_KEY.set(key);
        return Ok(MASTER_KEY.get().unwrap());
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Err("Entorno de ejecución no compatible con los estándares de seguridad requeridos"
            .to_string())
    }
}

// Motores Criptográficos: Implementan algoritmos de alto desempeño
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};

/// Encripta bloques de datos (Ej: Fotos de trabajadores) usando ChaCha20-Poly1305.
pub fn encrypt_data(data: &[u8]) -> Result<Vec<u8>, String> {
    let key = get_master_key()?;
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, data).map_err(|e| e.to_string())?;

    let mut result = nonce.to_vec();
    result.extend(ciphertext);

    Ok(result)
}

/// Descifra los bloques de datos tras validar su autenticidad.
pub fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
    let key = get_master_key()?;
    let cipher = ChaCha20Poly1305::new(key.into());

    if encrypted_data.len() < 12 {
        return Err("Payload de seguridad corrupto o incompleto".to_string());
    }

    let nonce = Nonce::from_slice(&encrypted_data[0..12]);
    let ciphertext = &encrypted_data[12..];

    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;

    Ok(plaintext)
}
