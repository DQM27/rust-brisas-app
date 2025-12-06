// src-tauri/src/services/template_service.rs

use crate::config::manager::{get_database_path, load_config};
use crate::models::template::PdfTemplate;
use crate::models::template::{TemplateColors, TemplateFonts, TemplateLayout};
use std::fs;
use std::path::PathBuf;

const TEMPLATE_FILE_NAME: &str = "templates.json";

/// Obtiene la ruta del archivo de templates (al lado de la DB)
fn get_templates_path() -> Result<PathBuf, String> {
    let config = load_config().map_err(|e| e.to_string())?;
    let db_path = get_database_path(&config);

    // Usar el directorio de la DB
    let parent = db_path
        .parent()
        .ok_or("No se pudo obtener el directorio de la DB")?;
    Ok(parent.join(TEMPLATE_FILE_NAME))
}

/// Obtiene templates predefinidos (Hardcoded)
fn get_predefined_templates() -> Vec<PdfTemplate> {
    vec![
        PdfTemplate {
            id: "default-classic".to_string(),
            name: "Clásico (Gris)".to_string(),
            is_predefined: true,
            colors: TemplateColors {
                header_fill: "#e8e8e8".to_string(),
                header_text: "#000000".to_string(),
                row_text: "#000000".to_string(),
                border: "#000000".to_string(),
            },
            fonts: TemplateFonts {
                family: "New Computer Modern".to_string(),
                size: 10,
                header_size: 11,
            },
            layout: TemplateLayout {
                page_size: "us-letter".to_string(),
                orientation: "landscape".to_string(),
                margin_x: "1.5cm".to_string(),
                margin_y: "2cm".to_string(),
            },
        },
        PdfTemplate {
            id: "blue-modern".to_string(),
            name: "Moderno (Azul)".to_string(),
            is_predefined: true,
            colors: TemplateColors {
                header_fill: "#dbeafe".to_string(), // blue-100
                header_text: "#1e3a8a".to_string(), // blue-900
                row_text: "#1e293b".to_string(),    // slate-800
                border: "#93c5fd".to_string(),      // blue-300
            },
            fonts: TemplateFonts {
                family: "Arial".to_string(),
                size: 11,
                header_size: 12,
            },
            layout: TemplateLayout {
                page_size: "a4".to_string(),
                orientation: "landscape".to_string(),
                margin_x: "1cm".to_string(),
                margin_y: "1.5cm".to_string(),
            },
        },
    ]
}

/// Carga todos los templates (Predefinidos + Custom)
pub fn get_all_templates() -> Result<Vec<PdfTemplate>, String> {
    let mut templates = get_predefined_templates();

    // Cargar customs
    if let Ok(path) = get_templates_path() {
        if path.exists() {
            let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let custom_templates: Vec<PdfTemplate> =
                serde_json::from_str(&content).unwrap_or_default();
            templates.extend(custom_templates);
        }
    }

    Ok(templates)
}

/// Guarda un nuevo template (o actualiza uno existente)
pub fn save_template(template: PdfTemplate) -> Result<(), String> {
    if template.is_predefined {
        return Err("No se pueden modificar templates predefinidos".to_string());
    }

    let path = get_templates_path()?;
    let mut custom_templates: Vec<PdfTemplate> = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Upsert
    if let Some(idx) = custom_templates.iter().position(|t| t.id == template.id) {
        custom_templates[idx] = template;
    } else {
        custom_templates.push(template);
    }

    let json = serde_json::to_string_pretty(&custom_templates).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;

    Ok(())
}

/// Elimina un template por ID
pub fn delete_template(id: String) -> Result<(), String> {
    let predefined = get_predefined_templates();
    if predefined.iter().any(|t| t.id == id) {
        return Err("No se puede eliminar un template predefinido".to_string());
    }

    let path = get_templates_path()?;
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut custom_templates: Vec<PdfTemplate> = serde_json::from_str(&content).unwrap_or_default();

    let initial_len = custom_templates.len();
    custom_templates.retain(|t| t.id != id);

    if custom_templates.len() != initial_len {
        let json = serde_json::to_string_pretty(&custom_templates).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn get_template_by_id(id: &str) -> Option<PdfTemplate> {
    get_all_templates().ok()?.into_iter().find(|t| t.id == id)
}
