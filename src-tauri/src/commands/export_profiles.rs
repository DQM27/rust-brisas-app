// src-tauri/src/commands/export_profiles.rs

use crate::models::export::ExportProfile;
use crate::services::export_profile_service;

#[tauri::command]
pub async fn get_export_profiles() -> Result<Vec<ExportProfile>, String> {
    export_profile_service::get_all_profiles().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_export_profile(profile: ExportProfile) -> Result<(), String> {
    export_profile_service::save_profile(profile).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_export_profile(id: String) -> Result<(), String> {
    export_profile_service::delete_profile(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_default_export_profile(id: String) -> Result<(), String> {
    export_profile_service::set_default_profile(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_default_export_profile() -> Result<Option<ExportProfile>, String> {
    Ok(export_profile_service::get_default_profile())
}
