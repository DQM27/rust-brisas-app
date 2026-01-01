/// Gestión de Perfiles de Usuario para Exportación.
///
/// Permite que los usuarios guarden, editen y apliquen configuraciones
/// predefinidas de columnas y filtros para sus reportes recurrentes.
use crate::domain::errors::ExportError;
use crate::models::export::ExportProfile;
use crate::services::export_profile_service;

/// Recupera todos los perfiles de exportación personalizados del usuario.
#[tauri::command]
pub async fn get_export_profiles() -> Result<Vec<ExportProfile>, ExportError> {
    Ok(export_profile_service::get_all_profiles()?)
}

/// Guarda o actualiza un perfil de configuración de reporte.
#[tauri::command]
pub async fn save_export_profile(profile: ExportProfile) -> Result<(), ExportError> {
    Ok(export_profile_service::save_profile(profile)?)
}

/// Elimina permanentemente un perfil de exportación.
#[tauri::command]
pub async fn delete_export_profile(id: String) -> Result<(), ExportError> {
    Ok(export_profile_service::delete_profile(id)?)
}

/// Establece un perfil específico como el predeterminado para nuevas exportaciones.
#[tauri::command]
pub async fn set_default_export_profile(id: String) -> Result<(), ExportError> {
    Ok(export_profile_service::set_default_profile(id)?)
}

/// Obtiene el perfil de exportación marcado como favorito o por defecto.
#[tauri::command]
pub async fn get_default_export_profile() -> Result<Option<ExportProfile>, ExportError> {
    Ok(export_profile_service::get_default_profile())
}
