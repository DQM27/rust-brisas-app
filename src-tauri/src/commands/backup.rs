use crate::config::AppConfig;
use crate::domain::errors::ConfigError;
use crate::services::backup;
use log::info;
use std::fs;
use tauri::{command, State};

#[command]
pub async fn backup_database(_destination_path: String) -> Result<(), ConfigError> {
    info!("Backup manual solicitado (Stub SurrealDB)");
    Err(ConfigError::Database("No implementado para SurrealDB aún".to_string()))
}

#[command]
pub async fn restore_database(
    config: State<'_, AppConfig>,
    source_path: String,
) -> Result<(), ConfigError> {
    info!("Preparando restauración desde: {}", source_path);

    let db_path = crate::config::manager::get_database_path(&config);
    let restore_path = backup::get_restore_path(&db_path);

    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        return Err(ConfigError::Message("El archivo de origen no existe".to_string()));
    }
    info!("Copiando a área de staging: {}", restore_path.display());
    fs::copy(source, &restore_path).map_err(|e| {
        ConfigError::Io(format!("Error al preparar archivo de restauración: {}", e))
    })?;

    info!("Archivo de restauración listo. Se requiere reinicio.");
    Ok(())
}
