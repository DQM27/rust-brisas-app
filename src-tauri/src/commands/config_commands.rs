use crate::config::manager::save_config;
use crate::config::{settings::TerminalConfig, AppConfig};
use tauri::{command, State};

/// Obtiene la configuración completa actual
#[command]
pub async fn get_app_config(config: State<'_, AppConfig>) -> Result<AppConfig, String> {
    Ok(config.inner().clone())
}

/// Actualiza la configuración de la terminal (nombre y ubicación)
#[command]
pub async fn update_terminal_config(
    config: State<'_, AppConfig>,
    nombre: String,
    ubicacion: String,
) -> Result<TerminalConfig, String> {
    println!("⚙️ Actualizando configuración de terminal: {} - {}", nombre, ubicacion);

    // Clonar config actual para modificarla
    // Nota: El State de Tauri es inmutable por defecto durante la ejecución para seguridad de hilos.
    // Sin embargo, necesitamos persistir el cambio en DISCO y devolver el nuevo valor.
    // En una app real production-grade, usaríamos un RwLock<AppConfig> para actualizar el estado en memoria.
    // Aquí, para simplificar y dado que el reinicio es rápido, guardamos en disco.
    // La próxima vez que arranque leerá esto.

    // 1. Cargar desde disco para tener la versión más reciente (por si hubo cambios externos)
    // O mejor, usar la del estado y modificarla.
    let mut current_config = config.inner().clone();

    current_config.terminal.nombre = nombre;
    current_config.terminal.ubicacion = ubicacion;

    // 2. Guardar en disco
    // Necesitamos reconstruir la ruta del archivo. `manager.rs` tiene la lógica pero `save_config` pide path.
    // Reusaremos la lógica de `get_default_config_path` o similar si fuera pública,
    // pero como `load_config` busca en varios lados, lo ideal es sobrescribir donde se encontró.
    // POR SIMPLICIDAD: Sobrescribiremos en la ruta por defecto de datos del usuario que es la prioritaria (2).

    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&current_config, &config_path)
        .map_err(|e| format!("Error al guardar configuración: {}", e))?;

    println!("✅ Configuración guardada en: {}", config_path.display());

    Ok(current_config.terminal)
}
