// src-tauri/src/services/export_profile_service.rs

use crate::config::manager::{get_database_path, load_config};
use crate::models::export::ExportProfile;
use std::fs;
use std::path::PathBuf;

const PROFILE_FILE_NAME: &str = "export_profiles.json";

/// Obtiene la ruta del archivo de perfiles (al lado de la DB)
fn get_profiles_path() -> Result<PathBuf, String> {
    let config = load_config().map_err(|e| e.to_string())?;
    let db_path = get_database_path(&config);

    // Usar el directorio de la DB
    let parent = db_path
        .parent()
        .ok_or("No se pudo obtener el directorio de la DB")?;
    Ok(parent.join(PROFILE_FILE_NAME))
}

/// Obtiene perfiles predefinidos (por defecto)
fn get_default_profiles() -> Vec<ExportProfile> {
    vec![
        ExportProfile {
            id: "default-pdf-landscape".to_string(),
            name: "PDF Horizontal Rápido".to_string(),
            format: "pdf".to_string(),
            is_default: true,
            options: serde_json::json!({
                "title": "Reporte",
                "orientation": "landscape",
                "show_preview": false
            }),
        },
        ExportProfile {
            id: "default-pdf-portrait".to_string(),
            name: "PDF Vertical".to_string(),
            format: "pdf".to_string(),
            is_default: false,
            options: serde_json::json!({
                "title": "Reporte",
                "orientation": "portrait",
                "show_preview": false
            }),
        },
        ExportProfile {
            id: "default-excel".to_string(),
            name: "Excel Estándar".to_string(),
            format: "excel".to_string(),
            is_default: false,
            options: serde_json::json!({}),
        },
        ExportProfile {
            id: "default-csv".to_string(),
            name: "CSV para Excel".to_string(),
            format: "csv".to_string(),
            is_default: false,
            options: serde_json::json!({
                "delimiter": "comma",
                "include_bom": true
            }),
        },
    ]
}

/// Carga todos los perfiles (Default + Custom)
pub fn get_all_profiles() -> Result<Vec<ExportProfile>, String> {
    let path = get_profiles_path()?;

    if path.exists() {
        // Si existe el archivo, cargar de ahí
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let profiles: Vec<ExportProfile> = serde_json::from_str(&content)
            .map_err(|e| format!("Error parseando perfiles: {}", e))?;
        Ok(profiles)
    } else {
        // Primera vez, retornar defaults y guardarlos
        let defaults = get_default_profiles();
        save_all_profiles(&defaults)?;
        Ok(defaults)
    }
}

/// Guarda todos los perfiles (reemplaza el archivo completo)
fn save_all_profiles(profiles: &[ExportProfile]) -> Result<(), String> {
    let path = get_profiles_path()?;
    let json = serde_json::to_string_pretty(profiles).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Guarda un perfil (o actualiza uno existente)
pub fn save_profile(profile: ExportProfile) -> Result<(), String> {
    let mut profiles = get_all_profiles()?;

    // Upsert
    if let Some(idx) = profiles.iter().position(|p| p.id == profile.id) {
        profiles[idx] = profile;
    } else {
        profiles.push(profile);
    }

    save_all_profiles(&profiles)?;
    Ok(())
}

/// Elimina un perfil por ID
pub fn delete_profile(id: String) -> Result<(), String> {
    let mut profiles = get_all_profiles()?;

    // No permitir eliminar si es el único default
    let default_count = profiles.iter().filter(|p| p.is_default).count();
    if default_count == 1 {
        if let Some(profile) = profiles.iter().find(|p| p.id == id) {
            if profile.is_default {
                return Err("No se puede eliminar el único perfil predeterminado. Marca otro como predeterminado primero.".to_string());
            }
        }
    }

    let initial_len = profiles.len();
    profiles.retain(|p| p.id != id);

    if profiles.len() != initial_len {
        save_all_profiles(&profiles)?;
    }

    Ok(())
}

/// Establece un perfil como predeterminado
pub fn set_default_profile(id: String) -> Result<(), String> {
    let mut profiles = get_all_profiles()?;

    // Verificar que el perfil existe
    if !profiles.iter().any(|p| p.id == id) {
        return Err("Perfil no encontrado".to_string());
    }

    // Quitar default de todos y establecer solo el seleccionado
    for profile in profiles.iter_mut() {
        profile.is_default = profile.id == id;
    }

    save_all_profiles(&profiles)?;
    Ok(())
}

/// Obtiene el perfil predeterminado
pub fn get_default_profile() -> Option<ExportProfile> {
    get_all_profiles()
        .ok()?
        .into_iter()
        .find(|p| p.is_default)
}

/// Obtiene un perfil por ID
pub fn get_profile_by_id(id: &str) -> Option<ExportProfile> {
    get_all_profiles()
        .ok()?
        .into_iter()
        .find(|p| p.id == id)
}
