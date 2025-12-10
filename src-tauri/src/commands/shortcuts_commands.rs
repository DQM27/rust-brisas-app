// src-tauri/src/commands/shortcuts_commands.rs

use crate::config::shortcuts::ShortcutsConfig;
use crate::config::shortcuts_manager;

#[tauri::command]
pub fn get_shortcuts() -> Result<ShortcutsConfig, String> {
    shortcuts_manager::load_shortcuts().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_shortcuts(config: ShortcutsConfig) -> Result<(), String> {
    shortcuts_manager::save_shortcuts(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_shortcuts() -> Result<ShortcutsConfig, String> {
    // Forzar recarga creará default si no existe,
    // pero si queremos reset real:
    let config = ShortcutsConfig::default();
    shortcuts_manager::save_shortcuts(&config).map_err(|e| e.to_string())?;
    Ok(config)
}
