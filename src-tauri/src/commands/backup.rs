/// Puertos de Entrada: Resiliencia y Mantenimiento de Datos.
///
/// Este módulo expone comandos para la gestión de copias de seguridad
/// y la preparación de restauraciones atómicas.
use crate::config::AppConfig;
use crate::domain::errors::BackupError;
use crate::services::backup;
use log::{error, info};
use tauri::{command, State};

/// Realiza una copia de seguridad manual de la base de datos activa.
///
/// **Nota**: Actualmente devuelve error ya que SurrealDB requiere
/// un proceso de exportación específico para hot-backups.
#[command]
pub async fn backup_database(_destination_path: String) -> Result<(), BackupError> {
    info!("Backup manual solicitado (Pendiente de implementación para SurrealDB)");
    Err(BackupError::IO(
        "Funcionalidad de exportación manual no habilitada en esta versión".to_string(),
    ))
}

/// Prepara el sistema para una restauración de base de datos desde un archivo/directorio externo.
///
/// La restauración efectiva NO ocurre inmediatamente. Este comando coloca los datos
/// en un área de "staging" y el sistema los aplicará automáticamente en el próximo arranque.
///
/// # Arguments
///
/// * `config` - Estado de la configuración de la aplicación.
/// * `source_path` - Ruta absoluta al backup a restaurar.
///
/// # Errors
///
/// * `BackupError::NotFound`: Si el archivo de origen no existe.
/// * `BackupError::IO`: Si falla la copia al área de preparación.
#[command]
pub async fn restore_database(
    config: State<'_, AppConfig>,
    source_path: String,
) -> Result<(), BackupError> {
    info!("🔄 Preparando protocolo de restauración desde: {}", source_path);

    let db_path = crate::config::manager::get_database_path(&config);
    let restore_path = backup::get_restore_path(&db_path);

    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        error!("Fallo en restauración: Origen inexistente en {}", source_path);
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
        error!("Error al preparar staging de restauración: {}", e);
        BackupError::IO(format!("Fallo al copiar datos a staging: {}", e))
    })?;

    info!("✅ Protocolo listo. El sistema se restaurará en el próximo reinicio.");
    Ok(())
}
