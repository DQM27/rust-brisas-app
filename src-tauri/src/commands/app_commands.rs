use crate::AppState;
use tauri::{AppHandle, Manager}; // We will define this in lib.rs

#[tauri::command]
pub async fn app_ready(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
    // If backend is ready, close splash and show main
    // If not, we do nothing (frontend should listen for event)
    // Actually, to be safe, we can enforce consistency here.

    if state
        .backend_ready
        .load(std::sync::atomic::Ordering::SeqCst)
    {
        if let Some(splash) = app.get_webview_window("splash") {
            let _ = splash.close();
        }
        if let Some(main) = app.get_webview_window("main") {
            let _ = main.show();
            let _ = main.set_focus();
        }
    }

    Ok(())
}
