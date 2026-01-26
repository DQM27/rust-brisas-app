use crate::services::module_service::{ModuleService, ModuleStatus};
use crate::services::session::SessionState;
use crate::services::surrealdb_service::SurrealDbError;
use tauri::{command, State};

#[command]
pub async fn get_modules_status() -> Result<Vec<ModuleStatus>, SurrealDbError> {
    ModuleService::get_all_modules().await
}

#[command]
pub async fn update_module_status(
    session: State<'_, SessionState>,
    key: String,
    status: String,
) -> Result<(), SurrealDbError> {
    // Obtenemos el ID del usuario actual de la sesión
    let user_id = session
        .get_user()
        .map(|u| u.id)
        .ok_or_else(|| SurrealDbError::Auth("No hay sesión activa".to_string()))?;

    ModuleService::update_status(&user_id, &key, &status).await
}
