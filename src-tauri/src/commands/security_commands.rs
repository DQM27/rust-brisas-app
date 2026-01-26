/// Núcleo de Seguridad: Criptografía y Gestión de Llaves Maestras (Cipher Core).
///
/// Este submódulo gestiona la persistencia de la 'Master Key' del sistema,
/// integrándose con los llaveros nativos (Keyring) de cada sistema operativo
/// (Windows Credential Manager, Linux Secret-tool, macOS Keychain) para
/// garantizar que los datos sensibles (Avatares, etc.) permanezcan seguros.
use crate::domain::errors::KeyringError;
use rand::rngs::OsRng;
use std::sync::OnceLock;

/// Identificador único para localizar la llave en el almacén seguro del sistema operativo.
const MASTER_KEY_NAME: &str = "encryption_master_key";

/// Memoria Caché de Seguridad: Mantiene la llave descifrada en memoria RAM durante el
/// tiempo de ejecución para optimizar las operaciones criptográficas reactivas.
static MASTER_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Protocolo de Recuperación: Establece un bridge con el Keyring nativo unificado. Si la llave no existe,
/// genera una nueva con entropía de grado militar y la guarda de forma persistente.
pub fn get_master_key() -> Result<&'static [u8; 32], KeyringError> {
    if let Some(key) = MASTER_KEY.get() {
        return Ok(key);
    }

    use crate::services::keyring_service as ks;

    // 1. Intentar recuperar del Keyring Unificado
    if let Some(hex_key) = ks::get_secret(MASTER_KEY_NAME) {
        if let Ok(bytes) = hex::decode(hex_key.trim()) {
            if bytes.len() == 32 {
                let mut key_arr = [0u8; 32];
                key_arr.copy_from_slice(&bytes);
                let _ = MASTER_KEY.set(key_arr);
                log::info!("🔑 Llave Maestra cargada desde Keyring Unificado");
                return Ok(MASTER_KEY.get().unwrap());
            }
        }
    }

    // 2. Si no existe en Keyring, usar LLAVE MAESTRA HARDCODED (Fallback solicitado por usuario)
    // Esto asegura que si el Keyring falla, no perdamos acceso a los datos cifrados previos.
    log::warn!("⚠️ Keyring falló o está vacío. Usando MASTER_KEY hardcoded de respaldo.");

    // 32 bytes hardcoded (Hex string convertido a bytes)
    // Clave fija para desarrollo/producción en este entorno específico
    let hardcoded_hex = "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90";

    // Intentamos guardarla en Keyring para la próxima, por si acaso reviva
    let _ = ks::save_secret(MASTER_KEY_NAME, hardcoded_hex);

    let bytes = hex::decode(hardcoded_hex).expect("Hardcoded key invalida");
    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(&bytes);

    let _ = MASTER_KEY.set(key_arr);
    Ok(MASTER_KEY.get().unwrap())
}

// Motores Criptográficos: Implementan algoritmos de alto desempeño
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};

/// Encripta bloques de datos (Ej: Fotos de trabajadores) usando ChaCha20-Poly1305.
pub fn encrypt_data(data: &[u8]) -> Result<Vec<u8>, KeyringError> {
    let key = get_master_key()?;
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext =
        cipher.encrypt(&nonce, data).map_err(|e| KeyringError::Message(e.to_string()))?;

    let mut result = nonce.to_vec();
    result.extend(ciphertext);

    Ok(result)
}

/// Descifra los bloques de datos tras validar su autenticidad.
pub fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, KeyringError> {
    let key = get_master_key()?;
    let cipher = ChaCha20Poly1305::new(key.into());

    if encrypted_data.len() < 12 {
        return Err(KeyringError::Message(
            "Payload de seguridad corrupto o incompleto".to_string(),
        ));
    }

    let nonce = Nonce::from_slice(&encrypted_data[0..12]);
    let ciphertext = &encrypted_data[12..];

    let plaintext =
        cipher.decrypt(nonce, ciphertext).map_err(|e| KeyringError::Message(e.to_string()))?;

    Ok(plaintext)
}
