/// Servicio: Gestión de Avatares Seguros.
///
/// Orquestador para el cifrado, almacenamiento y recuperación de fotos de perfil.
/// Combina operaciones de sistema de archivos (encriptados) y base de datos (referencias).
///
/// Responsabilidades:
/// - Ingesta segura de imágenes (Resize -> WebP -> `ChaCha20Poly1305` -> Disk).
/// - Recuperación desencriptada bajo demanda (Disk -> Decrypt -> Base64).
/// - Eliminación segura (Shredding lógico).
use crate::commands::security_commands::{decrypt_data, encrypt_data};
use crate::db::surrealdb_user_queries as db;
use crate::domain::errors::UserError;
use base64::{engine::general_purpose::STANDARD, Engine};
use image::{DynamicImage, ImageReader};
use std::fs;
use std::path::PathBuf;

/// Directorio donde se guardan los avatares encriptados
const AVATAR_DIR: &str = "secure_avatars";

/// Tamaño máximo del avatar (256x256 px)
const MAX_AVATAR_SIZE: u32 = 256;

/// Obtiene la ruta base para avatares
fn get_avatar_base_path() -> Result<PathBuf, UserError> {
    let data_dir = dirs::data_local_dir()
        .ok_or_else(|| UserError::IO("No se pudo obtener directorio de datos".to_string()))?;

    let avatar_path = data_dir.join("com.brisas.app").join(AVATAR_DIR);

    // Crear directorio si no existe
    if !avatar_path.exists() {
        fs::create_dir_all(&avatar_path)
            .map_err(|e| UserError::IO(format!("Error creando directorio: {e}")))?;
    }

    Ok(avatar_path)
}

/// Procesa una imagen: la redimensiona y la convierte a WebP
fn process_image(image_path: &str) -> Result<Vec<u8>, UserError> {
    // Leer imagen desde el path
    let img: DynamicImage = ImageReader::open(image_path)
        .map_err(|e| UserError::IO(format!("Error abriendo imagen: {e}")))?
        .decode()
        .map_err(|e| UserError::Validation(format!("Error decodificando imagen: {e}")))?;

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
            .map_err(|e| UserError::Internal(format!("Error codificando WebP: {e}")))?;
    }

    Ok(webp_buffer)
}

/// Proceso de "Ingesta" de Avatar: Procesa, cifra y ofusca el archivo en disco.
///
/// # Arguments
///
/// * `user_id` - ID del usuario propietario.
/// * `file_path` - Ruta absoluta al archivo de imagen original.
///
/// # Returns
///
/// Retorna el UUID del archivo guardado o error.
///
/// # Errors
///
/// * `UserError::IO`: Fallo en disco.
/// * `UserError::Validation`: Imagen inválida.
/// * `UserError::Database`: Fallo al actualizar usuario.
pub async fn upload_avatar(user_id: &str, file_path: &str) -> Result<String, UserError> {
    log::info!("📸 Iniciando subida segura de avatar para: {user_id}");

    // 1. Procesar imagen (resize + WebP)
    let image_data = process_image(file_path)?;
    log::info!("   ✓ Procesado: {} bytes", image_data.len());

    // 2. Encriptar con ChaCha20Poly1305
    let encrypted = encrypt_data(&image_data)
        .map_err(|e| UserError::Internal(format!("Error de encriptación: {e}")))?;

    // 3. Generar UUID para el archivo (nombre ofuscado)
    let file_uuid = uuid::Uuid::now_v7().to_string();

    // 4. Guardar archivo encriptado
    let base_path = get_avatar_base_path()?;
    let avatar_file_path = base_path.join(format!("{file_uuid}.enc"));
    fs::write(&avatar_file_path, &encrypted)
        .map_err(|e| UserError::IO(format!("Error escribiendo archivo ofuscado: {e}")))?;

    // 5. Actualizar avatar_path en la DB
    if let Err(e) = db::update_avatar_path(user_id, &file_uuid).await {
        // Rollback manual: eliminar archivo si falla DB
        let _ = fs::remove_file(&avatar_file_path);
        return Err(UserError::Database(e.to_string()));
    }

    log::info!("   ✓ Avatar asegurado exitosamente: {file_uuid}");
    Ok(file_uuid)
}

/// Recuperación de Identidad: Desencripta el activo y lo entrega para visualización en UI.
///
/// # Arguments
///
/// * `user_id` - ID del usuario.
///
/// # Returns
///
/// Retorna string Base64 de la imagen WebP lista para `<img src="...">`.
pub async fn get_avatar(user_id: &str) -> Result<String, UserError> {
    // 1. Obtener avatar_path de la DB
    let avatar_uuid =
        db::get_avatar_path(user_id).await.map_err(|e| UserError::Database(e.to_string()))?;

    if avatar_uuid.is_empty() {
        return Err(UserError::NotFound); // O manejar como Ok("") si UI lo prefiere vacio
    }

    // 2. Construir la ruta del archivo
    let base_path = get_avatar_base_path()?;
    let avatar_file_path = base_path.join(format!("{avatar_uuid}.enc"));

    // 3. Leer archivo encriptado
    let encrypted = fs::read(&avatar_file_path).map_err(|_| {
        UserError::IO(format!("Archivo de avatar {avatar_uuid} no encontrado en disco"))
    })?;

    // 4. Desencriptar
    let decrypted = decrypt_data(&encrypted)
        .map_err(|e| UserError::Internal(format!("Error desencriptando avatar: {e}")))?;

    // 5. Convertir a Base64
    let b64 = STANDARD.encode(&decrypted);

    Ok(b64)
}

/// Borrado Seguro: Elimina el archivo cifrado y limpia la referencia en DB.
///
/// # Arguments
///
/// * `user_id` - ID del usuario.
pub async fn delete_avatar(user_id: &str) -> Result<(), UserError> {
    // Obtener avatar_path de la DB
    let avatar_uuid =
        db::get_avatar_path(user_id).await.map_err(|e| UserError::Database(e.to_string()))?;

    if !avatar_uuid.is_empty() {
        let base_path = get_avatar_base_path()?;
        let avatar_file_path = base_path.join(format!("{avatar_uuid}.enc"));

        if avatar_file_path.exists() {
            fs::remove_file(&avatar_file_path)
                .map_err(|e| UserError::IO(format!("Error eliminando archivo: {e}")))?;
        }

        // Limpiar avatar_path en DB
        db::update_avatar_path(user_id, "")
            .await
            .map_err(|e| UserError::Database(e.to_string()))?;
    }

    Ok(())
}

