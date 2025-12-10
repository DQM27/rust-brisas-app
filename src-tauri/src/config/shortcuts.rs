// src-tauri/src/config/shortcuts.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuración de atajos de teclado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutsConfig {
    /// Versión del esquema de configuración
    pub version: u8,
    /// Mapa de comandos globales (ej. "open.contractors" -> "Ctrl+Shift+C")
    pub global: HashMap<String, String>,
    /// Mapa de contextos -> comandos (ej. "ingreso-list" -> {"new" -> "Ctrl+N"})
    pub contexts: HashMap<String, HashMap<String, String>>,
}

impl Default for ShortcutsConfig {
    fn default() -> Self {
        let mut global = HashMap::new();
        global.insert("module.ingreso".to_string(), "Ctrl+Shift+I".to_string());
        global.insert("module.contractors".to_string(), "Ctrl+Shift+C".to_string());
        global.insert("module.users".to_string(), "Ctrl+Shift+U".to_string());
        global.insert("module.access".to_string(), "Ctrl+Shift+A".to_string());
        global.insert("module.logs".to_string(), "Ctrl+Shift+L".to_string());
        global.insert("user.profile".to_string(), "Ctrl+Shift+P".to_string());
        global.insert("app.settings".to_string(), "Ctrl+,".to_string());

        let mut contexts = HashMap::new();

        // Contexto: Lista de Ingresos
        let mut ingreso_list = HashMap::new();
        ingreso_list.insert("new".to_string(), "Ctrl+N".to_string());
        ingreso_list.insert("search.focus".to_string(), "Ctrl+F".to_string());
        contexts.insert("ingreso-list".to_string(), ingreso_list);

        // Contexto: Formulario de Ingreso
        let mut ingreso_form = HashMap::new();
        ingreso_form.insert("save".to_string(), "Ctrl+S".to_string());
        ingreso_form.insert("cancel".to_string(), "Escape".to_string());
        contexts.insert("ingreso-form".to_string(), ingreso_form);

        // Contexto: Formulario de Usuario
        let mut user_form = HashMap::new();
        user_form.insert("save".to_string(), "Ctrl+S".to_string());
        user_form.insert("cancel".to_string(), "Escape".to_string());
        contexts.insert("user-form".to_string(), user_form);

        // Contexto: Formulario de Contratista
        let mut contractor_form = HashMap::new();
        contractor_form.insert("save".to_string(), "Ctrl+S".to_string());
        contractor_form.insert("cancel".to_string(), "Escape".to_string());
        contexts.insert("contractor-form".to_string(), contractor_form);

        Self {
            version: 1,
            global,
            contexts,
        }
    }
}
