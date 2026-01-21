/// Comandos Tauri: Gestión de Atajos de Teclado Personalizados
///
/// Expone los comandos para que el frontend pueda gestionar
/// los atajos de teclado personalizados de cada usuario.
use crate::domain::errors::UserError;
use crate::models::user_shortcuts::{
    UserShortcutInput, UserShortcutResponse, UserShortcutsListResponse,
};
use crate::services::session::SessionState;
use crate::services::user_shortcuts_service::UserShortcutService;
use tauri::State;

/// Obtiene todos los atajos personalizados del usuario actual
#[tauri::command]
pub async fn get_user_shortcuts(
    session: State<'_, SessionState>,
) -> Result<UserShortcutsListResponse, UserError> {
    let user = session.require_session()?;

    UserShortcutService::get_user_shortcuts(&user.id)
        .await
        .map_err(|e| UserError::Database(e.to_string()))
}

/// Guarda o actualiza un atajo personalizado
#[tauri::command]
pub async fn save_user_shortcut(
    session: State<'_, SessionState>,
    input: UserShortcutInput,
) -> Result<UserShortcutResponse, UserError> {
    let user = session.require_session()?;

    UserShortcutService::upsert_shortcut(&user.id, input)
        .await
        .map_err(|e| UserError::Database(e.to_string()))
}

/// Elimina un atajo personalizado (vuelve al default)
#[tauri::command]
pub async fn delete_user_shortcut(
    session: State<'_, SessionState>,
    shortcut_id: String,
) -> Result<(), UserError> {
    let user = session.require_session()?;

    UserShortcutService::delete_shortcut(&user.id, &shortcut_id)
        .await
        .map_err(|e| UserError::Database(e.to_string()))
}

/// Resetea todos los atajos personalizados del usuario (vuelve a defaults)
#[tauri::command]
pub async fn reset_user_shortcuts(session: State<'_, SessionState>) -> Result<(), UserError> {
    let user = session.require_session()?;

    UserShortcutService::reset_all_shortcuts(&user.id)
        .await
        .map_err(|e| UserError::Database(e.to_string()))
}
