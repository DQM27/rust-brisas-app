use crate::config::manager::save_config;
use crate::config::{settings::TerminalConfig, AppConfig};
use crate::domain::errors::ConfigError;
use log::info;
use tauri::{command, State};

/// Obtiene la configuración completa actual
#[command]
pub async fn get_app_config(config: State<'_, AppConfig>) -> Result<AppConfig, ConfigError> {
    Ok(config.inner().clone())
}

/// Actualiza la configuración de la terminal (nombre y ubicación)
#[command]
pub async fn update_terminal_config(
    config: State<'_, AppConfig>,
    nombre: String,
    ubicacion: String,
) -> Result<TerminalConfig, ConfigError> {
    info!("Actualizando configuración de terminal: {} - {}", nombre, ubicacion);

    // Clonar config actual para modificarla
    let mut current_config = config.inner().clone();

    current_config.terminal.nombre = nombre;
    current_config.terminal.ubicacion = ubicacion;

    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&current_config, &config_path)
        .map_err(|e| ConfigError::Message(format!("Error al guardar configuración: {}", e)))?;

    info!("Configuración guardada en: {}", config_path.display());

    Ok(current_config.terminal)
}
