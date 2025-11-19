// src-tauri/src/config/settings.rs

use serde::{Deserialize, Serialize};

/// Configuración completa de la aplicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub terminal: TerminalConfig,
    pub database: DatabaseConfig,
    // ❌ ELIMINADO: pub supabase: SupabaseConfig
    pub app: AppInfo,
}

/// Configuración de la terminal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    /// Nombre identificador de la terminal
    pub nombre: String,
    /// ID único generado a partir del hardware
    pub id: String,
}

/// Configuración de la base de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Rutas a intentar en orden de prioridad
    pub paths: Vec<String>,
    /// Ruta por defecto si no existe ninguna
    pub default_path: String,
}

// ❌ ELIMINADO: SupabaseConfig struct completo

/// Información de la aplicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub version: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            terminal: TerminalConfig {
                nombre: "Terminal Principal".to_string(),
                id: String::new(), // Se generará automáticamente
            },
            database: DatabaseConfig {
                paths: vec![
                    "./data/brisas.db".to_string(),
                    "C:/ProgramData/Brisas/brisas.db".to_string(),
                ],
                default_path: "".to_string(), // Se calculará en runtime
            },
            // ❌ ELIMINADO: supabase: SupabaseConfig
            app: AppInfo {
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }
}