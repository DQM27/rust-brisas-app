// src/services/avatar_service.rs
//! Servicio para gestión de avatares encriptados
//!
//! Este servicio maneja el almacenamiento seguro de fotos de usuario
//! utilizando encriptación ChaCha20Poly1305 con llave maestra del OS Keyring.

use crate::commands::security_commands::{decrypt_data, encrypt_data};
use base64::{engine::general_purpose::STANDARD, Engine};
use image::{DynamicImage, ImageReader};
use std::fs;
use std::path::PathBuf;

/// Directorio donde se guardan los avatares encriptados
const AVATAR_DIR: &str = "secure_avatars";

/// Tamaño máximo del avatar (256x256 px)
const MAX_AVATAR_SIZE: u32 = 256;

/// Obtiene la ruta base para avatares
fn get_avatar_base_path() -> Result<PathBuf, String> {
    let data_dir = dirs::data_local_dir()
        .ok_or_else(|| "No se pudo obtener directorio de datos".to_string())?;

    let avatar_path = data_dir.join("com.brisas.app").join(AVATAR_DIR);

    // Crear directorio si no existe
    if !avatar_path.exists() {
        fs::create_dir_all(&avatar_path).map_err(|e| format!("Error creando directorio: {e}"))?;
    }

    Ok(avatar_path)
}

/// Obtiene la ruta del archivo de avatar encriptado para un usuario
fn get_avatar_file_path(user_id: &str) -> Result<PathBuf, String> {
    let base = get_avatar_base_path()?;
    // Sanitizar user_id: quitar prefijo "user:" y caracteres problemáticos
    // SurrealDB usa formato "user:⟨uuid⟩" o "user:uuid"
    let clean_id = user_id
        .trim_start_matches("user:")
        .replace(['⟨', '⟩', '<', '>', '/', '\\', '.', ':', '*', '?', '"', '|'], "");

    let file_path = base.join(format!("{clean_id}.enc"));
    log::debug!("Avatar path for user '{}': {:?}", user_id, file_path);
    Ok(file_path)
}

/// Procesa una imagen: la redimensiona y la convierte a WebP
fn process_image(image_path: &str) -> Result<Vec<u8>, String> {
    // Leer imagen desde el path
    let img: DynamicImage = ImageReader::open(image_path)
        .map_err(|e| format!("Error abriendo imagen: {e}"))?
        .decode()
        .map_err(|e| format!("Error decodificando imagen: {e}"))?;

    // Redimensionar si es necesario (mantiene aspect ratio)
    let resized = if img.width() > MAX_AVATAR_SIZE || img.height() > MAX_AVATAR_SIZE {
        img.thumbnail(MAX_AVATAR_SIZE, MAX_AVATAR_SIZE)
    } else {
        img
    };

    // Convertir a RGBA8 para WebP
    let rgba_image = resized.to_rgba8();

    // Usar buffer para codificar a WebP (lossless en image 0.25)
    let mut webp_buffer: Vec<u8> = Vec::new();
    {
        let mut cursor = std::io::Cursor::new(&mut webp_buffer);
        let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut cursor);
        encoder
            .encode(
                rgba_image.as_raw(),
                rgba_image.width(),
                rgba_image.height(),
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|e| format!("Error codificando WebP: {e}"))?;
    }

    Ok(webp_buffer)
}

/// Sube un avatar: lo procesa, encripta y guarda
///
/// # Arguments
/// * `user_id` - ID del usuario
/// * `file_path` - Ruta al archivo de imagen original
///
/// # Returns
/// * `Ok(String)` - ID de referencia del archivo guardado
/// * `Err(String)` - Error de procesamiento o encriptación
pub fn upload_avatar(user_id: &str, file_path: &str) -> Result<String, String> {
    log::info!("📸 Subiendo avatar para usuario: {}", user_id);

    // 1. Procesar imagen (resize + WebP)
    let image_data = process_image(file_path)?;
    log::info!("   ✓ Imagen procesada: {} bytes", image_data.len());

    // 2. Encriptar con ChaCha20Poly1305
    let encrypted = encrypt_data(&image_data)?;
    log::info!("   ✓ Encriptado: {} bytes", encrypted.len());

    // 3. Guardar archivo encriptado
    let avatar_path = get_avatar_file_path(user_id)?;
    fs::write(&avatar_path, &encrypted).map_err(|e| format!("Error guardando archivo: {e}"))?;
    log::info!("   ✓ Guardado en: {:?}", avatar_path);

    Ok(user_id.to_string())
}

/// Obtiene un avatar desencriptado en formato Base64
///
/// # Arguments
/// * `user_id` - ID del usuario
///
/// # Returns
/// * `Ok(String)` - Imagen WebP en Base64
/// * `Err(String)` - Error si no existe o no se puede desencriptar
pub fn get_avatar(user_id: &str) -> Result<String, String> {
    let avatar_path = get_avatar_file_path(user_id)?;

    // Leer archivo encriptado
    let encrypted = fs::read(&avatar_path)
        .map_err(|_| format!("No se encontró avatar para usuario: {user_id}"))?;

    // Desencriptar
    let decrypted = decrypt_data(&encrypted)?;

    // Convertir a Base64
    let b64 = STANDARD.encode(&decrypted);

    Ok(b64)
}

/// Elimina el avatar de un usuario
///
/// # Arguments
/// * `user_id` - ID del usuario
pub fn delete_avatar(user_id: &str) -> Result<(), String> {
    let avatar_path = get_avatar_file_path(user_id)?;
    if avatar_path.exists() {
        fs::remove_file(&avatar_path).map_err(|e| format!("Error eliminando avatar: {e}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_user_id() {
        let path = get_avatar_file_path("user/../../../etc/passwd").unwrap();
        assert!(path.to_string_lossy().contains("user____________etc_passwd"));
    }
}
