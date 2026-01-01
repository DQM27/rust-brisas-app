/// Comandos de Orquestación de Ventanas.
///
/// Maneja la visibilidad y el foco de las ventanas de la aplicación,
/// facilitando transiciones fluidas entre el Splash Screen, el Login
/// y el Dashboard principal.
use crate::domain::errors::SystemError;
use tauri::Manager;

/// Hace visible la ventana principal del sistema y le otorga el foco de entrada.
#[tauri::command]
pub async fn show_main_window(app: tauri::AppHandle) -> Result<(), SystemError> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| SystemError::Window(e.to_string()))?;
        window.set_focus().map_err(|e| SystemError::Window(e.to_string()))?;
    }
    Ok(())
}
