/// Gestión de la Configuración Global de la Aplicación.
///
/// Este módulo permite la persistencia y lectura de los ajustes del sistema,
/// incluyendo la identidad de la terminal, preferencias de audio y otros
/// parámetros operativos almacenados en el archivo local TOML.
use crate::config::manager::save_config;
use crate::config::settings::{AppConfig, AppConfigState, TerminalConfig};
use crate::domain::errors::ConfigError;
use log::info;
use tauri::{command, State};

/// Recupera la configuración actual cargada en memoria.
#[command]
pub async fn get_app_config(config: State<'_, AppConfigState>) -> Result<AppConfig, ConfigError> {
    let config_guard = config
        .read()
        .map_err(|e| ConfigError::Message(format!("Error al leer configuración: {}", e)))?;

    Ok(config_guard.clone())
}

/// Actualiza la identidad de la terminal (nombre y ubicación física).
/// Los cambios se persisten inmediatamente en el archivo de configuración brisas.toml.
#[command]
pub async fn update_terminal_config(
    config: State<'_, AppConfigState>,
    nombre: String,
    ubicacion: String,
) -> Result<TerminalConfig, ConfigError> {
    info!("Actualizando configuración de terminal: {} - {}", nombre, ubicacion);

    let mut config_guard = config
        .write()
        .map_err(|e| ConfigError::Message(format!("Error al escribir configuración: {}", e)))?;

    config_guard.terminal.nombre = nombre;
    config_guard.terminal.ubicacion = ubicacion;

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

/// Actualiza la preferencia del sonido de alerta del sistema.
#[command]
pub async fn update_audio_config(
    config: State<'_, AppConfigState>,
    alert_sound: String,
) -> Result<(), ConfigError> {
    info!("Actualizando configuración de audio: {}", alert_sound);

    let mut config_guard = config
        .write()
        .map_err(|e| ConfigError::Message(format!("Error al escribir configuración: {}", e)))?;

    config_guard.audio.alert_sound = alert_sound;

    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&config_guard, &config_path)
        .map_err(|e| ConfigError::Message(format!("Error al guardar configuración: {}", e)))?;

    Ok(())
}
