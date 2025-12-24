use tauri::{command, Window};

#[command]
pub fn exit_app() {
    std::process::exit(0);
}

#[command]
pub fn set_window_decorations(window: Window, decorations: bool) -> Result<(), String> {
    window.set_decorations(decorations).map_err(|e| e.to_string())
}

#[command]
pub fn set_window_size(window: Window, width: f64, height: f64) -> Result<(), String> {
    window
        .set_size(tauri::Size::Logical(tauri::LogicalSize { width, height }))
        .map_err(|e| e.to_string())
}
