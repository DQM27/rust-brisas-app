use crate::domain::errors::SystemError;
use tauri::Manager;

#[tauri::command]
pub async fn show_main_window(app: tauri::AppHandle) -> Result<(), SystemError> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| SystemError::Window(e.to_string()))?;
        window.set_focus().map_err(|e| SystemError::Window(e.to_string()))?;
    }
    Ok(())
}
