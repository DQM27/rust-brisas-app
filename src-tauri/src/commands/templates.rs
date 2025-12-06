// src-tauri/src/commands/templates.rs

use crate::models::template::PdfTemplate;
use crate::services::template_service;

#[tauri::command]
pub async fn get_templates() -> Result<Vec<PdfTemplate>, String> {
    template_service::get_all_templates()
}

#[tauri::command]
pub async fn save_template(template: PdfTemplate) -> Result<(), String> {
    template_service::save_template(template)
}

#[tauri::command]
pub async fn delete_template(id: String) -> Result<(), String> {
    template_service::delete_template(id)
}
