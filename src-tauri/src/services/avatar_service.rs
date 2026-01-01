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

/// Proceso de "Ingesta" de Avatar: Procesa, cifra y ofusca el archivo en disco.
///
/// Pasos de seguridad:
/// 1. Procesamiento: Estandarización a WebP.
/// 2. Cifrado: Aplicación de ChaCha20Poly1305.
/// 3. Ofuscación: Nombre de archivo basado en UUID para evitar trazabilidad externa.
///
/// # Arguments
/// * `user_id` - ID del usuario
/// * `file_path` - Ruta al archivo de imagen original
///
/// # Returns
/// * `Ok(String)` - ID de referencia del archivo guardado
/// * `Err(String)` - Error de procesamiento o encriptación
pub async fn upload_avatar(user_id: &str, file_path: &str) -> Result<String, String> {
    log::info!("📸 Iniciando proceso de subida de avatar seguro para: {}", user_id);

    // 1. Procesar imagen (resize + WebP)
    let image_data = process_image(file_path)?;
    log::info!("   ✓ Imagen procesada: {} bytes", image_data.len());

    // 2. Encriptar con ChaCha20Poly1305
    let encrypted = encrypt_data(&image_data)?;
    log::info!("   ✓ Encriptado: {} bytes", encrypted.len());

    // 3. Generar UUID para el archivo (nombre ofuscado)
    let file_uuid = uuid::Uuid::new_v4().to_string();

    // 4. Guardar archivo encriptado
    let base_path = get_avatar_base_path()?;
    let avatar_file_path = base_path.join(format!("{}.enc", file_uuid));
    fs::write(&avatar_file_path, &encrypted)
        .map_err(|e| format!("Error al escribir activo cifrado en disco: {e}"))?;
    log::info!("   ✓ Guardado en: {:?}", avatar_file_path);

    // 5. Actualizar avatar_path en la DB
    update_user_avatar_path(user_id, &file_uuid).await?;
    log::info!("   ✓ DB actualizada con avatar_path: {}", file_uuid);

    Ok(file_uuid)
}

/// Persiste la referencia del activo en la ficha del usuario.
async fn update_user_avatar_path(user_id: &str, avatar_uuid: &str) -> Result<(), String> {
    use crate::services::surrealdb_service::get_db;

    let db = get_db().await.map_err(|e| e.to_string())?;

    // Parsear user_id a RecordId
    let clean_id = user_id.trim_start_matches("user:").replace(['⟨', '⟩', '<', '>'], "");

    let user_record = surrealdb::RecordId::from_table_key("user", &clean_id);

    db.query("UPDATE $id SET avatar_path = $avatar_path, updated_at = time::now()")
        .bind(("id", user_record))
        .bind(("avatar_path", avatar_uuid.to_string()))
        .await
        .map_err(|e| format!("Fallo al actualizar referencia de avatar en DB: {e}"))?;

    Ok(())
}

/// Recuperación de Identidad: Desencripta el activo y lo entrega para visualización en UI.
///
/// # Arguments
/// * `user_id` - ID del usuario
///
/// # Returns
/// * `Ok(String)` - Imagen WebP en Base64
/// * `Err(String)` - Error si no existe o no se puede desencriptar
pub async fn get_avatar(user_id: &str) -> Result<String, String> {
    // 1. Obtener avatar_path de la DB
    let avatar_uuid = get_user_avatar_path(user_id).await?;

    if avatar_uuid.is_empty() {
        return Err(format!("El usuario {} no tiene un avatar configurado", user_id));
    }

    // 2. Construir la ruta del archivo
    let base_path = get_avatar_base_path()?;
    let avatar_file_path = base_path.join(format!("{}.enc", avatar_uuid));

    // 3. Leer archivo encriptado
    let encrypted = fs::read(&avatar_file_path).map_err(|_| {
        format!("Activo no encontrado: el archivo {} ha desaparecido del disco", avatar_uuid)
    })?;

    // 4. Desencriptar
    let decrypted = decrypt_data(&encrypted)?;

    // 5. Convertir a Base64
    let b64 = STANDARD.encode(&decrypted);

    Ok(b64)
}

/// Obtiene el avatar_path del usuario desde la DB
async fn get_user_avatar_path(user_id: &str) -> Result<String, String> {
    use crate::services::surrealdb_service::get_db;
    use serde::Deserialize;

    let db = get_db().await.map_err(|e| e.to_string())?;

    // Parsear user_id a RecordId
    let clean_id = user_id.trim_start_matches("user:").replace(['⟨', '⟩', '<', '>'], "");

    let user_record = surrealdb::RecordId::from_table_key("user", &clean_id);

    #[derive(Deserialize)]
    struct AvatarPath {
        avatar_path: Option<String>,
    }

    let mut result = db
        .query("SELECT avatar_path FROM $id")
        .bind(("id", user_record))
        .await
        .map_err(|e| format!("Error de consulta en DB: {e}"))?;

    let row: Option<AvatarPath> = result.take(0).map_err(|e| e.to_string())?;

    Ok(row.and_then(|r| r.avatar_path).unwrap_or_default())
}

/// Borrado Seguro: Elimina el archivo cifrado y limpia la referencia en DB.
///
/// # Arguments
/// * `user_id` - ID del usuario
pub async fn delete_avatar(user_id: &str) -> Result<(), String> {
    // Obtener avatar_path de la DB
    let avatar_uuid = get_user_avatar_path(user_id).await?;

    if !avatar_uuid.is_empty() {
        let base_path = get_avatar_base_path()?;
        let avatar_file_path = base_path.join(format!("{}.enc", avatar_uuid));

        if avatar_file_path.exists() {
            fs::remove_file(&avatar_file_path)
                .map_err(|e| format!("Error al eliminar archivo de avatar: {e}"))?;
        }

        // Limpiar avatar_path en DB
        update_user_avatar_path(user_id, "").await?;
    }

    Ok(())
}
