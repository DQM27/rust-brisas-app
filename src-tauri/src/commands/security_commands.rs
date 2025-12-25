use keyring::Entry;
use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::OnceLock;

// Nombre del servicio y usuario para el Credential Manager de Windows
const SERVICE_NAME: &str = "BrisasApp";
const USER_NAME: &str = "MasterKey";

// Singleton para mantener la llave en memoria y no pedirla al OS a cada rato
static MASTER_KEY: OnceLock<[u8; 32]> = OnceLock::new();

pub fn get_master_key() -> Result<&'static [u8; 32], String> {
    if let Some(key) = MASTER_KEY.get() {
        return Ok(key);
    }

    // Intentar obtener del OS Keyring
    let entry = Entry::new(SERVICE_NAME, USER_NAME).map_err(|e| e.to_string())?;

    match entry.get_password() {
        Ok(password_hex) => {
            // Decodificar hex a bytes
            let bytes =
                hex::decode(password_hex).map_err(|_| "Error decoding master key".to_string())?;
            if bytes.len() != 32 {
                return Err("Invalid master key length".to_string());
            }
            let mut key_arr = [0u8; 32];
            key_arr.copy_from_slice(&bytes);
            let _ = MASTER_KEY.set(key_arr);
        }
        Err(_) => {
            // No existe, crear nueva
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);

            // Guardar en hex para que sea string compatible
            let hex_key = hex::encode(key);
            entry.set_password(&hex_key).map_err(|e| e.to_string())?;

            let _ = MASTER_KEY.set(key);
        }
    }

    Ok(MASTER_KEY.get().unwrap())
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
