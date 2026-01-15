/// Puertos de Entrada: Resiliencia y Mantenimiento de Datos.
///
/// Este módulo expone comandos para la gestión de copias de seguridad
/// y la preparación de restauraciones atómicas.
use crate::config::manager::save_config;
use crate::config::settings::AppConfigState;
use crate::domain::backup_entry::BackupEntryResponse;
use crate::domain::errors::BackupError;
use crate::services::backup;
use chrono::Local;
use log::{error, info, warn};
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

/// [Comando Tauri] Realiza un backup automático al directorio configurado.
#[command]
pub async fn backup_database_auto(
    config: State<'_, AppConfigState>,
) -> Result<String, BackupError> {
    let backup_dir = get_backup_directory(&config)?;

    // Generar nombre de archivo con timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("brisas_backup_{timestamp}.surql");
    let destination = backup_dir.join(&filename);
    let destination_str = destination.to_string_lossy().to_string();

    info!("📦 Iniciando respaldo automático a: {destination_str}");

    // Ejecutar backup
    backup_database(destination_str.clone()).await?;

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

    info!("✅ Backup automático completado: {filename}");
    Ok(filename)
}

/// [Comando Tauri] Lista todos los backups disponibles en el directorio de backups.
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

        // Solo archivos .surql, .db, .sqlite, .bak
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !["surql", "db", "sqlite", "bak"].contains(&ext_str.as_str()) {
                continue;
            }
        } else {
            continue;
        }

        let metadata = entry
            .metadata()
            .map_err(|e| BackupError::IO(format!("Error obteniendo metadata: {e}")))?;

        if !metadata.is_file() {
            continue;
        }

        let nombre = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();

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

        backups.push(BackupEntryResponse {
            nombre,
            ruta: path.to_string_lossy().to_string(),
            tamano: metadata.len(),
            fecha_creacion,
            dias_antiguedad,
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

/// [Comando Tauri] Restaura desde un backup automático.
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

    // Usar la lógica de restore existente
    let db_path = {
        let config_guard = config
            .read()
            .map_err(|e| BackupError::IO(format!("Error al leer configuración: {e}")))?;

        crate::config::manager::get_database_path_static(&config_guard)
    };

    let restore_path = backup::get_restore_path(&db_path);

    info!("📦 Copiando backup a área de preparación: {}", restore_path.display());

    // Asegurar que el destino esté limpio
    if restore_path.exists() {
        if restore_path.is_dir() {
            let _ = fs::remove_dir_all(&restore_path);
        } else {
            let _ = fs::remove_file(&restore_path);
        }
    }

    backup::copy_recursive(&source_path, &restore_path).map_err(|e| {
        error!("Error al preparar staging de restauración: {e}");
        BackupError::IO(format!("Fallo al copiar datos a staging: {e}"))
    })?;

    info!("✅ Protocolo listo. El sistema se restaurará en el próximo reinicio.");
    Ok(())
}

/// [Comando Tauri] Limpia backups antiguos según la política de retención.
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

        // Solo procesar archivos de backup
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !["surql", "db", "sqlite", "bak"].contains(&ext_str.as_str()) {
                continue;
            }
        } else {
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
#[cfg(test)]
mod integration_tests {
    // use super::*;
    // use crate::services::surrealdb_service::{init_surrealdb, SurrealDbConfig};
    // use std::fs;

    // TODO: Habilitar test cuando se resuelva el error de runtime:
    // `STATUS_ENTRYPOINT_NOT_FOUND` (0xc0000139) en Windows al ejecutar tests de SurrealDB.
    // Parece ser un conflicto de DLLs en el entorno de pruebas vs ejecución normal.
    //
    // #[tokio::test]
    // async fn test_backup_database_demo() {
    //     // 1. Setup - Usar DB Demo (aislada)
    //     // Nota: Init es global (OnceCell), así que esto solo funciona si es el primer test
    //     // o si la configuración coincide. Para `cargo test` suele ser suficiente.
    //     let config = SurrealDbConfig::demo();
    //     let service = init_surrealdb(config.clone());

    //     // Conectar (ignorar error si ya estaba conectado)
    //     let _ = service.connect().await;

    //     // 2. Preparar ruta de prueba
    //     let mut backup_path = std::env::temp_dir();
    //     backup_path.push(format!("test_backup_{}.surql", chrono::Utc::now().timestamp()));
    //     let backup_path_str = backup_path.to_string_lossy().to_string();

    //     // Limpiar previo por si acaso
    //     if backup_path.exists() {
    //         let _ = fs::remove_file(&backup_path);
    //     }

    //     // 3. Ejecutar comando (debe crear el archivo)
    //     let result = backup_database(backup_path_str.clone()).await;

    //     // 4. Validaciones
    //     match result {
    //         Ok(_) => {
    //             assert!(backup_path.exists(), "El archivo de backup debería haberse creado");

    //             let metadata = fs::metadata(&backup_path).unwrap();
    //             assert!(metadata.len() > 0, "El archivo de backup no debería estar vacío");

    //             // Cleanup solo si fue exitoso (para dejar evidencia si falla)
    //             let _ = fs::remove_file(backup_path);
    //         }
    //         Err(e) => {
    //             panic!("El comando backup_database falló: {:?}", e);
    //         }
    //     }
    // }
}
