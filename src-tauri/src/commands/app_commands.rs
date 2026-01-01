/// Gestión de Ciclo de Vida y Apariencia de la Aplicación.
///
/// Proporciona comandos básicos para el control de la instancia de la aplicación
/// y la manipulación de las propiedades visuales de la ventana principal.
use tauri::{command, Window};

/// Finaliza inmediatamente el proceso de la aplicación.
#[command]
pub fn exit_app() {
    std::process::exit(0);
}

/// Alterna la visibilidad de los bordes y controles nativos de la ventana.
#[command]
pub fn set_window_decorations(window: Window, decorations: bool) -> Result<(), String> {
    window.set_decorations(decorations).map_err(|e| e.to_string())
}

/// Ajusta las dimensiones físicas de la ventana de la aplicación.
#[command]
pub fn set_window_size(window: Window, width: f64, height: f64) -> Result<(), String> {
    window
        .set_size(tauri::Size::Logical(tauri::LogicalSize { width, height }))
        .map_err(|e| e.to_string())
}
