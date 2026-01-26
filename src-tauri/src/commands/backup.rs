/// Puertos de Entrada: Resiliencia y Mantenimiento de Datos.
///
/// Este módulo expone comandos para la gestión de copias de seguridad
/// y la preparación de restauraciones atómicas.
use crate::commands::security_commands::{decrypt_data, encrypt_data};
use crate::config::manager::save_config;
use crate::config::settings::AppConfigState;
use crate::domain::backup_entry::BackupEntryResponse;
use crate::domain::errors::BackupError;
use crate::services::backup;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use chrono::Local;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use tauri::{command, State};

// --------------------------------------------------------------------------
// UTILIDADES
// --------------------------------------------------------------------------

/// Obtiene el directorio de backups automáticos.
/// Por defecto usa %LOCALAPPDATA%/Brisas/backups/
fn get_backup_directory(config: &AppConfigState) -> Result<PathBuf, BackupError> {
    let config_guard =
        config.read().map_err(|e| BackupError::IO(format!("Error al leer configuración: {e}")))?;

    if let Some(ref dir) = config_guard.backup.directorio {
        return Ok(PathBuf::from(dir));
    }

    // Directorio por defecto
    let backup_dir = dirs::data_local_dir()
        .ok_or_else(|| BackupError::IO("No se pudo obtener directorio local".to_string()))?
        .join("Brisas")
        .join("backups");

    // Crear directorio si no existe
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)
            .map_err(|e| BackupError::IO(format!("Error al crear directorio de backups: {e}")))?;
    }

    Ok(backup_dir)
}

/// Estructura para backups portables (con contraseña).
#[derive(Serialize, Deserialize, Debug)]
struct PortableBackupFile {
    pub version: u32,
    pub created_at: String,
    pub salt: String,       // Para derivación Argon2 de la contraseña
    pub nonce: String,      // Para ChaCha20
    pub ciphertext: String, // Datos encriptados (Hex)
    pub checksum: String,   // SHA256 de los datos originales (Hex)
}

/// Determina el tipo de encriptación basado en la extensión del archivo.
fn get_encryption_type(filename: &str) -> String {
    if filename.ends_with(".surql.enc") {
        "local".to_string()
    } else if filename.ends_with(".surql.penc") {
        "portable".to_string()
    } else {
        "none".to_string()
    }
}

/// Deriva una llave de cifrado de 32 bytes a partir de una contraseña.
fn derive_key_from_password(password: &str, salt_str: &str) -> Result<Key, BackupError> {
    let salt = SaltString::from_b64(salt_str)
        .map_err(|e| BackupError::IO(format!("Salt inválido: {e}")))?;

    let mut key_buffer = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key_buffer)
        .map_err(|e| BackupError::IO(format!("Error derivando llave: {e}")))?;

    Ok(*Key::from_slice(&key_buffer))
}

/// Helper: Exporta la DB a un buffer en memoria (sin escribir a disco).
async fn export_database_to_buffer() -> Result<Vec<u8>, BackupError> {
    use futures::TryStreamExt;

    let db = crate::services::surrealdb_service::get_db().await.map_err(|e| {
        error!("No se pudo obtener conexión a DB: {e}");
        BackupError::IO(format!("Error de conexión: {e}"))
    })?;

    let mut stream = db.export(()).await.map_err(|e| {
        error!("Error al exportar: {e}");
        BackupError::IO(format!("Error exportando base de datos: {e}"))
    })?;

    let mut buffer = Vec::new();
    while let Some(chunk) = stream
        .try_next()
        .await
        .map_err(|e| BackupError::IO(format!("Error leyendo exportación: {e}")))?
    {
        buffer.extend_from_slice(&chunk);
    }

    Ok(buffer)
}

/// Helper: Desencripta un backup local (.surql.enc) usando la Master Key.
fn decrypt_local_backup(encrypted_data: &[u8]) -> Result<Vec<u8>, BackupError> {
    decrypt_data(encrypted_data)
        .map_err(|e| BackupError::IO(format!("Error desencriptando backup local: {e}")))
}

/// Helper: Desencripta un backup portable (.surql.penc) usando contraseña.
fn decrypt_portable_backup(file_content: &str, password: &str) -> Result<Vec<u8>, BackupError> {
    // 1. Parsear el JSON
    let portable_file: PortableBackupFile = serde_json::from_str(file_content)
        .map_err(|e| BackupError::IO(format!("Formato de backup portable inválido: {e}")))?;

    // 2. Derivar llave de descifrado
    let key = derive_key_from_password(password, &portable_file.salt)?;
    let cipher = ChaCha20Poly1305::new(&key);

    // 3. Decodificar componentes
    let nonce_bytes = hex::decode(&portable_file.nonce)
        .map_err(|e| BackupError::IO(format!("Nonce inválido: {e}")))?;
    let ciphertext_bytes = hex::decode(&portable_file.ciphertext)
        .map_err(|e| BackupError::IO(format!("Ciphertext inválido: {e}")))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4. Descifrar
    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext_bytes.as_ref())
        .map_err(|_| BackupError::IO("Contraseña incorrecta o archivo corrupto".to_string()))?;

    // 5. Validar Checksum
    let mut hasher = Sha256::new();
    hasher.update(&decrypted_bytes);
    let calculated_checksum = hex::encode(hasher.finalize());

    if calculated_checksum != portable_file.checksum {
        return Err(BackupError::IO("Integridad fallida: El checksum no coincide".to_string()));
    }

    Ok(decrypted_bytes)
}

/// Helper: Lee y desencripta un backup según su tipo.
/// Para backups portables, requiere password. Para locales, usa Master Key.
fn read_and_decrypt_backup(
    source_path: &std::path::Path,
    password: Option<&str>,
) -> Result<Vec<u8>, BackupError> {
    let filename =
        source_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();

    let encryption_type = get_encryption_type(&filename);

    match encryption_type.as_str() {
        "local" => {
            // Backup encriptado con Master Key
            let encrypted_data = fs::read(source_path)
                .map_err(|e| BackupError::IO(format!("Error leyendo backup: {e}")))?;
            decrypt_local_backup(&encrypted_data)
        }
        "portable" => {
            // Backup encriptado con contraseña
            let password = password.ok_or_else(|| {
                BackupError::IO("Se requiere contraseña para restaurar backup portable".to_string())
            })?;
            let file_content = fs::read_to_string(source_path)
                .map_err(|e| BackupError::IO(format!("Error leyendo backup: {e}")))?;
            decrypt_portable_backup(&file_content, password)
        }
        _ => {
            // Backup legacy sin encriptación - leer directo
            fs::read(source_path).map_err(|e| BackupError::IO(format!("Error leyendo backup: {e}")))
        }
    }
}
// --------------------------------------------------------------------------
// COMANDOS DE MANTENIMIENTO
// --------------------------------------------------------------------------

/// [Comando Tauri] Realiza una copia de seguridad manual de la base de datos activa.
///
/// Ejecuta el comando `EXPORT FILE` de `SurrealDB` para generar un script SQL
/// con la estructura y los datos actuales.
///
/// # Argumentos
/// * `destination_path` - Ruta absoluta donde se guardará el archivo .surql.
///
/// # Retorno
/// Retorna `Ok(())` si la exportación es exitosa.
#[command]
pub async fn backup_database(destination_path: String) -> Result<(), BackupError> {
    use futures::TryStreamExt;
    use tokio::io::AsyncWriteExt;

    info!("📦 Iniciando respaldo manual de base de datos a: {destination_path}");

    // 1. Obtener cliente de BD
    let db = crate::services::surrealdb_service::get_db().await.map_err(|e| {
        error!("No se pudo obtener conexión a DB para respaldo: {e}");
        BackupError::IO(format!("Error de conexión al motor de base de datos: {e}"))
    })?;

    // 2. Crear directorio padre si no existe
    let path = std::path::Path::new(&destination_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| BackupError::IO(format!("Error al crear directorio: {e}")))?;
        }
    }

    // 3. Usar el método export() del SDK para obtener un stream de bytes
    info!("⚙️ Ejecutando exportación via SDK...");

    // Exportar sin argumento retorna un stream
    let mut stream = db.export(()).await.map_err(|e| {
        error!("Error al iniciar exportación: {e}");
        BackupError::IO(format!("Error al exportar base de datos: {e}"))
    })?;

    // 4. Escribir el stream a un archivo
    let mut file = tokio::fs::File::create(&destination_path).await.map_err(|e| {
        error!("Error al crear archivo de backup: {e}");
        BackupError::IO(format!("Error al crear archivo: {e}"))
    })?;

    while let Some(chunk) = stream
        .try_next()
        .await
        .map_err(|e| BackupError::IO(format!("Error leyendo datos de exportación: {e}")))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|e| BackupError::IO(format!("Error escribiendo archivo: {e}")))?;
    }

    file.flush()
        .await
        .map_err(|e| BackupError::IO(format!("Error al finalizar escritura: {e}")))?;

    info!("✅ Respaldo completado exitosamente en: {destination_path}");
    Ok(())
}

/// [Comando Tauri] Realiza un backup automático ENCRIPTADO al directorio configurado.
/// Usa la Master Key del sistema (sin contraseña adicional).
#[command]
pub async fn backup_database_auto(
    config: State<'_, AppConfigState>,
) -> Result<String, BackupError> {
    let backup_dir = get_backup_directory(&config)?;

    // Generar nombre de archivo con timestamp (extensión .surql.enc para encriptado local)
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("brisas_backup_{timestamp}.surql.enc");
    let destination = backup_dir.join(&filename);

    info!("📦 Iniciando respaldo automático encriptado a: {}", destination.display());

    // 1. Exportar DB a buffer en memoria
    let plain_data = export_database_to_buffer().await?;

    // 2. Encriptar con Master Key
    let encrypted_data = encrypt_data(&plain_data)
        .map_err(|e| BackupError::IO(format!("Error al encriptar backup: {e}")))?;

    // 3. Escribir archivo encriptado
    fs::write(&destination, &encrypted_data)
        .map_err(|e| BackupError::IO(format!("Error al escribir backup encriptado: {e}")))?;

    // Actualizar último backup en configuración
    {
        let mut config_guard = config
            .write()
            .map_err(|e| BackupError::IO(format!("Error al escribir configuración: {e}")))?;

        config_guard.backup.ultimo_backup = Some(Local::now().to_rfc3339());

        // Guardar configuración
        let config_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("./config"))
            .join("Brisas")
            .join("brisas.toml");

        save_config(&config_guard, &config_path).map_err(|e| {
            BackupError::IO(format!("Error al guardar config de último backup: {e}"))
        })?;
    }

    info!("✅ Backup automático encriptado completado: {filename}");
    Ok(filename)
}

/// [Comando Tauri] Crea un backup PORTABLE encriptado con contraseña.
/// Puede ser restaurado en cualquier máquina que tenga la contraseña.
#[command]
pub async fn backup_database_portable(
    config: State<'_, AppConfigState>,
    password: String,
) -> Result<String, BackupError> {
    if password.len() < 8 {
        return Err(BackupError::IO("La contraseña debe tener al menos 8 caracteres".to_string()));
    }

    let backup_dir = get_backup_directory(&config)?;

    // Generar nombre de archivo con timestamp (.surql.penc = portable encrypted)
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("brisas_backup_{timestamp}.surql.penc");
    let destination = backup_dir.join(&filename);

    info!("📦 Iniciando respaldo portable encriptado a: {}", destination.display());

    // 1. Exportar DB a buffer en memoria
    let plain_data = export_database_to_buffer().await?;

    // 2. Generar Salt y Nonce
    let salt = SaltString::generate(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    // 3. Derivar llave de cifrado de la contraseña
    let key = derive_key_from_password(&password, salt.as_str())?;
    let cipher = ChaCha20Poly1305::new(&key);

    // 4. Cifrar los datos
    let ciphertext = cipher
        .encrypt(&nonce, plain_data.as_ref())
        .map_err(|e| BackupError::IO(format!("Error cifrando backup: {e}")))?;

    // 5. Calcular Checksum (SHA256 de los datos originales)
    let mut hasher = Sha256::new();
    hasher.update(&plain_data);
    let checksum = hex::encode(hasher.finalize());

    // 6. Crear estructura y guardar JSON
    let portable_file = PortableBackupFile {
        version: 1,
        created_at: chrono::Utc::now().to_rfc3339(),
        salt: salt.as_str().to_string(),
        nonce: hex::encode(nonce),
        ciphertext: hex::encode(ciphertext),
        checksum,
    };

    let json = serde_json::to_string_pretty(&portable_file)
        .map_err(|e| BackupError::IO(format!("Error serializando backup portable: {e}")))?;

    fs::write(&destination, json)
        .map_err(|e| BackupError::IO(format!("Error escribiendo backup portable: {e}")))?;

    info!("✅ Backup portable encriptado completado: {filename}");
    Ok(filename)
}

/// [Comando Tauri] Lista todos los backups disponibles en el directorio de backups.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
#[command]
pub async fn list_backups(
    config: State<'_, AppConfigState>,
) -> Result<Vec<BackupEntryResponse>, BackupError> {
    let backup_dir = get_backup_directory(&config)?;

    if !backup_dir.exists() {
        return Ok(vec![]);
    }

    let mut backups = Vec::new();
    let today = Local::now().date_naive();

    for entry in fs::read_dir(&backup_dir)
        .map_err(|e| BackupError::IO(format!("Error al leer directorio de backups: {e}")))?
    {
        let entry = entry.map_err(|e| BackupError::IO(format!("Error leyendo entrada: {e}")))?;
        let path = entry.path();
        let filename =
            path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();

        // Verificar extensiones válidas (incluyendo encriptados)
        let lower = filename.to_lowercase();
        let is_valid = lower.ends_with(".surql")
            || lower.ends_with(".surql.enc")
            || lower.ends_with(".surql.penc")
            || lower.ends_with(".db")
            || lower.ends_with(".sqlite")
            || lower.ends_with(".bak");

        if !is_valid {
            continue;
        }

        let metadata = entry
            .metadata()
            .map_err(|e| BackupError::IO(format!("Error obteniendo metadata: {e}")))?;

        if !metadata.is_file() {
            continue;
        }

        let fecha_creacion = metadata.created().or_else(|_| metadata.modified()).map_or_else(
            |_| "Desconocida".to_string(),
            |t| chrono::DateTime::<Local>::from(t).to_rfc3339(),
        );

        // Calcular días de antigüedad
        let dias_antiguedad = if let Ok(created) = metadata.created() {
            let created_date = chrono::DateTime::<Local>::from(created).date_naive();
            (today - created_date).num_days().try_into().unwrap_or(0)
        } else {
            0
        };

        // Determinar tipo de encriptación
        let encryption_type = get_encryption_type(&filename);

        backups.push(BackupEntryResponse {
            nombre: filename,
            ruta: path.to_string_lossy().to_string(),
            tamano: metadata.len(),
            fecha_creacion,
            dias_antiguedad,
            encryption_type,
        });
    }

    // Ordenar por fecha (más reciente primero)
    backups.sort_by(|a, b| b.fecha_creacion.cmp(&a.fecha_creacion));

    info!("📋 Listados {} backups", backups.len());
    Ok(backups)
}

/// [Comando Tauri] Elimina un backup específico.
#[command]
pub async fn delete_backup(
    config: State<'_, AppConfigState>,
    filename: String,
) -> Result<(), BackupError> {
    let backup_dir = get_backup_directory(&config)?;
    let file_path = backup_dir.join(&filename);

    if !file_path.exists() {
        return Err(BackupError::NotFound(filename));
    }

    // Verificar que el archivo está dentro del directorio de backups (seguridad)
    if !file_path.starts_with(&backup_dir) {
        return Err(BackupError::IO("Ruta de archivo inválida".to_string()));
    }

    fs::remove_file(&file_path)
        .map_err(|e| BackupError::IO(format!("Error al eliminar backup: {e}")))?;

    info!("🗑️ Backup eliminado: {filename}");
    Ok(())
}

/// [Comando Tauri] Restaura desde un backup automático (local).
/// Automáticamente desencripta backups .surql.enc usando la Master Key.
#[command]
pub async fn restore_from_auto_backup(
    config: State<'_, AppConfigState>,
    filename: String,
) -> Result<(), BackupError> {
    let backup_dir = get_backup_directory(&config)?;
    let source_path = backup_dir.join(&filename);

    if !source_path.exists() {
        return Err(BackupError::NotFound(filename));
    }

    // Verificar que no sea portable (requiere contraseña)
    if filename.ends_with(".surql.penc") {
        return Err(BackupError::IO(
            "Los backups portables requieren contraseña. Use restore_portable_backup.".to_string(),
        ));
    }

    let db_path = {
        let config_guard = config
            .read()
            .map_err(|e| BackupError::IO(format!("Error al leer configuración: {e}")))?;

        crate::config::manager::get_database_path_static(&config_guard)
    };

    let restore_path = backup::get_restore_path(&db_path);

    info!("📦 Preparando restauración desde: {}", source_path.display());

    // Asegurar que el destino esté limpio
    if restore_path.exists() {
        if restore_path.is_dir() {
            let _ = fs::remove_dir_all(&restore_path);
        } else {
            let _ = fs::remove_file(&restore_path);
        }
    }

    // Desencriptar si es necesario y escribir al staging
    let decrypted_data = read_and_decrypt_backup(&source_path, None)?;

    // Escribir datos desencriptados al área de staging
    fs::write(&restore_path, &decrypted_data)
        .map_err(|e| BackupError::IO(format!("Error escribiendo a staging: {e}")))?;

    info!("✅ Protocolo listo. El sistema se restaurará en el próximo reinicio.");
    Ok(())
}

/// [Comando Tauri] Restaura desde un backup portable (requiere contraseña).
#[command]
pub async fn restore_portable_backup(
    config: State<'_, AppConfigState>,
    filename: String,
    password: String,
) -> Result<(), BackupError> {
    let backup_dir = get_backup_directory(&config)?;
    let source_path = backup_dir.join(&filename);

    if !source_path.exists() {
        return Err(BackupError::NotFound(filename));
    }

    if !filename.ends_with(".surql.penc") {
        return Err(BackupError::IO(
            "Este comando es solo para backups portables (.surql.penc)".to_string(),
        ));
    }

    let db_path = {
        let config_guard = config
            .read()
            .map_err(|e| BackupError::IO(format!("Error al leer configuración: {e}")))?;

        crate::config::manager::get_database_path_static(&config_guard)
    };

    let restore_path = backup::get_restore_path(&db_path);

    info!("📦 Preparando restauración portable desde: {}", source_path.display());

    // Asegurar que el destino esté limpio
    if restore_path.exists() {
        if restore_path.is_dir() {
            let _ = fs::remove_dir_all(&restore_path);
        } else {
            let _ = fs::remove_file(&restore_path);
        }
    }

    // Desencriptar con contraseña
    let decrypted_data = read_and_decrypt_backup(&source_path, Some(&password))?;

    // Escribir datos desencriptados al área de staging
    fs::write(&restore_path, &decrypted_data)
        .map_err(|e| BackupError::IO(format!("Error escribiendo a staging: {e}")))?;

    info!("✅ Protocolo listo. El sistema se restaurará en el próximo reinicio.");
    Ok(())
}

/// [Comando Tauri] Limpia backups antiguos según la política de retención.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
#[command]
pub async fn cleanup_old_backups(config: State<'_, AppConfigState>) -> Result<u32, BackupError> {
    let backup_dir = get_backup_directory(&config)?;

    let dias_retencion = {
        let config_guard = config
            .read()
            .map_err(|e| BackupError::IO(format!("Error al leer configuración: {e}")))?;
        config_guard.backup.dias_retencion
    };

    if !backup_dir.exists() {
        return Ok(0);
    }

    let today = Local::now().date_naive();
    let mut deleted_count = 0;

    for entry in fs::read_dir(&backup_dir)
        .map_err(|e| BackupError::IO(format!("Error al leer directorio: {e}")))?
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let filename =
            path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();

        // Solo procesar archivos de backup (incluyendo encriptados)
        let lower = filename.to_lowercase();
        let is_valid = lower.ends_with(".surql")
            || lower.ends_with(".surql.enc")
            || lower.ends_with(".surql.penc")
            || lower.ends_with(".db")
            || lower.ends_with(".sqlite")
            || lower.ends_with(".bak");

        if !is_valid {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if !metadata.is_file() {
            continue;
        }

        // Calcular antigüedad
        let dias_antiguedad = if let Ok(created) = metadata.created() {
            let created_date = chrono::DateTime::<Local>::from(created).date_naive();
            (today - created_date).num_days().try_into().unwrap_or(0)
        } else {
            continue;
        };

        // Eliminar si excede retención
        if dias_antiguedad > dias_retencion && fs::remove_file(&path).is_ok() {
            deleted_count += 1;
            warn!(
                "🗑️ Backup antiguo eliminado: {} ({} días)",
                path.file_name().unwrap_or_default().to_string_lossy(),
                dias_antiguedad
            );
        }
    }

    if deleted_count > 0 {
        info!("🧹 Limpieza completada: {deleted_count} backups antiguos eliminados");
    }

    Ok(deleted_count)
}

/// [Comando Tauri] Prepara el sistema para una restauración de base de datos.
///
/// La restauración efectiva NO ocurre inmediatamente. Este comando coloca los datos
/// en un área de "staging" y el sistema los aplicará automáticamente en el próximo arranque.
///
/// # Argumentos
/// * `config` - Estado de la configuración de la aplicación.
/// * `source_path` - Ruta absoluta al backup a restaurar.
///
/// # Retorno
/// Retorna `Ok(())` si la preparación fue exitosa. Entrega `BackupError::NotFound`
/// si el origen no existe o `BackupError::IO` si falla la copia.
#[command]
pub async fn restore_database(
    config: State<'_, AppConfigState>,
    source_path: String,
) -> Result<(), BackupError> {
    info!("🔄 Preparando protocolo de restauración desde: {source_path}");

    let db_path = {
        let config_guard = config
            .read()
            .map_err(|e| BackupError::IO(format!("Error al leer configuración: {e}")))?;

        crate::config::manager::get_database_path_static(&config_guard)
    };

    let restore_path = backup::get_restore_path(&db_path);

    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        error!("Fallo en restauración: Origen inexistente en {source_path}");
        return Err(BackupError::NotFound(source_path));
    }

    info!("📦 Copiando datos al área de preparación: {}", restore_path.display());

    // Asegurar que el destino esté limpio si es una restauración nueva
    if restore_path.exists() {
        if restore_path.is_dir() {
            let _ = std::fs::remove_dir_all(&restore_path);
        } else {
            let _ = std::fs::remove_file(&restore_path);
        }
    }

    backup::copy_recursive(source, &restore_path).map_err(|e| {
        error!("Error al preparar staging de restauración: {e}");
        BackupError::IO(format!("Fallo al copiar datos a staging: {e}"))
    })?;

    info!("✅ Protocolo listo. El sistema se restaurará en el próximo reinicio.");
    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS DE INTEGRACIÓN
// --------------------------------------------------------------------------
