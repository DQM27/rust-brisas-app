use crate::config::AppConfig;
use crate::db::DbPool;
use crate::domain::errors::ConfigError;
use crate::services::backup;
use log::info;
use std::fs;
use tauri::{command, State};

/// Crea una copia de seguridad de la base de datos usando VACUUM INTO
#[command]
pub async fn backup_database(
    pool_state: State<'_, DbPool>,
    destination_path: String,
) -> Result<(), ConfigError> {
    info!("Iniciando backup manual a: {}", destination_path);

    // Usar SQL directo para el backup en caliente
    // VACUUM INTO es la forma segura en SQLite de copiar mientras está en uso
    let query = format!("VACUUM INTO '{}'", destination_path);

    // Verificar si el archivo destino ya existe y borrarlo (VACUUM INTO falla si existe)
    let dest_path = std::path::Path::new(&destination_path);
    if dest_path.exists() {
        fs::remove_file(dest_path)
            .map_err(|e| ConfigError::Io(format!("No se pudo sobrescribir el archivo: {}", e)))?;
    }

    let pool = pool_state.0.read().await;
    sqlx::query(&query).execute(&*pool).await.map_err(ConfigError::Database)?;

    info!("Backup completado exitosamente");
    Ok(())
}

/// Prepara la restauración de una base de datos (copia a .restore y pide reinicio)
#[command]
pub async fn restore_database(
    config: State<'_, AppConfig>,
    source_path: String,
) -> Result<(), ConfigError> {
    info!("Preparando restauración desde: {}", source_path);

    let db_path = crate::config::manager::get_database_path(&config);
    let restore_path = backup::get_restore_path(&db_path);

    // Validar origen
    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        return Err(ConfigError::Message("El archivo de origen no existe".to_string()));
    }

    // Copiar archivo fuente a *.restore
    info!("Copiando a área de staging: {}", restore_path.display());
    fs::copy(source, &restore_path).map_err(|e| {
        ConfigError::Io(format!("Error al preparar archivo de restauración: {}", e))
    })?;

    info!("Archivo de restauración listo. Se requiere reinicio.");
    Ok(())
}
