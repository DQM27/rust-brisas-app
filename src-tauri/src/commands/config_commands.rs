use crate::config::manager::save_config;
use crate::config::settings::{AppConfig, AppConfigState, TerminalConfig};
use crate::domain::errors::ConfigError;
use log::info;
use tauri::{command, State};

/// Obtiene la configuración completa actual
#[command]
pub async fn get_app_config(config: State<'_, AppConfigState>) -> Result<AppConfig, ConfigError> {
    let config_guard = config
        .read()
        .map_err(|e| ConfigError::Message(format!("Error al leer configuración: {}", e)))?;
    Ok(config_guard.clone())
}

/// Actualiza la configuración de la terminal (nombre y ubicación)
#[command]
pub async fn update_terminal_config(
    config: State<'_, AppConfigState>,
    nombre: String,
    ubicacion: String,
) -> Result<TerminalConfig, ConfigError> {
    info!("Actualizando configuración de terminal: {} - {}", nombre, ubicacion);

    // Obtener lock de escritura y modificar
    let mut config_guard = config
        .write()
        .map_err(|e| ConfigError::Message(format!("Error al escribir configuración: {}", e)))?;

    config_guard.terminal.nombre = nombre;
    config_guard.terminal.ubicacion = ubicacion;

    // Guardar en archivo
    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&config_guard, &config_path)
        .map_err(|e| ConfigError::Message(format!("Error al guardar configuración: {}", e)))?;

    info!("Configuración guardada en: {}", config_path.display());

    Ok(config_guard.terminal.clone())
}

/// Habilita o deshabilita el modo demo en la pantalla de login
#[command]
pub async fn toggle_demo_mode(
    config: State<'_, AppConfigState>,
    enabled: bool,
) -> Result<bool, ConfigError> {
    info!("Cambiando modo demo a: {}", enabled);

    // Obtener lock de escritura y modificar
    let mut config_guard = config
        .write()
        .map_err(|e| ConfigError::Message(format!("Error al escribir configuración: {}", e)))?;

    config_guard.setup.show_demo_mode = enabled;

    // Guardar en archivo
    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&config_guard, &config_path)
        .map_err(|e| ConfigError::Message(format!("Error al guardar configuración: {}", e)))?;

    info!("Modo demo guardado en: {}", config_path.display());

    Ok(enabled)
}
