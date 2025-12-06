// src-tauri/src/models/template.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfTemplate {
    pub id: String,
    pub name: String,
    pub is_predefined: bool,
    pub colors: TemplateColors,
    pub fonts: TemplateFonts,
    pub layout: TemplateLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateColors {
    pub header_fill: String, // Hex color e.g., "#e8e8e8"
    pub header_text: String, // Hex color e.g., "#000000"
    pub row_text: String,    // Hex color
    pub border: String,      // Hex color
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFonts {
    pub family: String,  // e.g., "New Computer Modern", "Arial"
    pub size: u8,        // pt
    pub header_size: u8, // pt
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLayout {
    pub page_size: String,   // "us-letter", "a4"
    pub orientation: String, // "portrait", "landscape" (overrides request if enforced, or default)
    pub margin_x: String,    // "1.5cm"
    pub margin_y: String,    // "2cm"
}

impl Default for PdfTemplate {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default".to_string(),
            is_predefined: false,
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
        }
    }
}
