// src-tauri/src/config/settings.rs

use serde::{Deserialize, Serialize};

/// Configuración completa de la aplicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub terminal: TerminalConfig,
    pub database: DatabaseConfig,
    pub supabase: SupabaseConfig,  // ← AGREGADO
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

/// Configuración de Supabase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseConfig {
    /// URL del proyecto de Supabase
    pub url: String,
    /// Clave pública (anon key)
    pub anon_key: String,

    pub db_password: String,
}

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
            supabase: SupabaseConfig {
                url: String::new(),
                anon_key: String::new(),
                db_password: String::new(),
            },
            app: AppInfo {
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }
}