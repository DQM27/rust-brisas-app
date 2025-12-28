use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::OnceLock;

// Clave para almacenar la master key de encriptación
const MASTER_KEY_NAME: &str = "encryption_master_key";

// Singleton para mantener la llave en memoria y no pedirla al OS a cada rato
static MASTER_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Obtiene la master key usando el mismo sistema que Argon2 (keyring_linux para Linux)
pub fn get_master_key() -> Result<&'static [u8; 32], String> {
    if let Some(key) = MASTER_KEY.get() {
        return Ok(key);
    }

    // Usar el servicio de keyring apropiado según la plataforma
    #[cfg(target_os = "linux")]
    {
        use crate::services::keyring_linux;

        // Intentar obtener del keyring
        if let Some(hex_key) = keyring_linux::retrieve_secret(MASTER_KEY_NAME) {
            if let Ok(bytes) = hex::decode(hex_key.trim()) {
                if bytes.len() == 32 {
                    let mut key_arr = [0u8; 32];
                    key_arr.copy_from_slice(&bytes);
                    let _ = MASTER_KEY.set(key_arr);
                    log::info!("🔑 Master key cargada desde keyring (secret-tool)");
                    return Ok(MASTER_KEY.get().unwrap());
                }
            }
        }

        // No existe, crear nueva llave
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        log::info!("🔑 Generando nueva master key");

        // Guardar en keyring usando secret-tool
        let hex_key = hex::encode(key);
        if let Err(e) = keyring_linux::store_secret(MASTER_KEY_NAME, &hex_key) {
            log::error!("❌ Error guardando master key en keyring: {}", e);
            return Err(format!("No se pudo guardar master key: {}", e));
        }
        log::info!("🔑 Master key guardada en keyring (secret-tool)");

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
                    log::info!("🔑 Master key cargada desde Credential Manager");
                    return Ok(MASTER_KEY.get().unwrap());
                }
            }
        }

        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let hex_key = hex::encode(key);
        keyring_windows::store_secret(MASTER_KEY_NAME, &hex_key)
            .map_err(|e| format!("No se pudo guardar master key: {}", e))?;
        log::info!("🔑 Master key guardada en Credential Manager");
        let _ = MASTER_KEY.set(key);
        return Ok(MASTER_KEY.get().unwrap());
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
        Err("Plataforma no soportada".to_string())
    }
}

// Funciones de ayuda para encriptar/desencriptar
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};

pub fn encrypt_data(data: &[u8]) -> Result<Vec<u8>, String> {
    let key = get_master_key()?;
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(&nonce, data).map_err(|e| e.to_string())?;

    // Prepend nonce to ciphertext
    let mut result = nonce.to_vec();
    result.extend(ciphertext);

    Ok(result)
}

pub fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
    let key = get_master_key()?;
    let cipher = ChaCha20Poly1305::new(key.into());

    if encrypted_data.len() < 12 {
        return Err("Data too short".to_string());
    }

    let nonce = Nonce::from_slice(&encrypted_data[0..12]);
    let ciphertext = &encrypted_data[12..];

    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;

    Ok(plaintext)
}
