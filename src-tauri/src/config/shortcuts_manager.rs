// src-tauri/src/config/shortcuts_manager.rs

use super::shortcuts::ShortcutsConfig;
use std::fs;
use std::path::PathBuf;

/// Obtiene la ruta del archivo shortcuts.toml
fn get_shortcuts_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("shortcuts.toml")
    } else {
        PathBuf::from("./config/shortcuts.toml")
    }
}

/// Carga la configuración de atajos
pub fn load_shortcuts() -> Result<ShortcutsConfig, Box<dyn std::error::Error>> {
    let path = get_shortcuts_path();

    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let config: ShortcutsConfig = toml::from_str(&content)?;
        return Ok(config);
    }

    create_default_shortcuts()
}

/// Crea la configuración por defecto
fn create_default_shortcuts() -> Result<ShortcutsConfig, Box<dyn std::error::Error>> {
    let config = ShortcutsConfig::default();
    save_shortcuts(&config)?;
    Ok(config)
}

/// Guarda la configuración de atajos
pub fn save_shortcuts(config: &ShortcutsConfig) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_shortcuts_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_string = toml::to_string_pretty(config)?;
    fs::write(&path, toml_string)?;

    Ok(())
}
