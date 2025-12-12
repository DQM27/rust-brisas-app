// src-tauri/src/config/settings.rs

use serde::{Deserialize, Serialize};

/// Configuración completa de la aplicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub terminal: TerminalConfig,
    pub database: DatabaseConfig,
    pub app: AppInfo,
    #[serde(default)]
    pub setup: SetupState,
}

/// Estado de configuración inicial de la app
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupState {
    /// Indica si la app ha sido configurada por primera vez
    #[serde(default)]
    pub is_configured: bool,
    /// Fecha de primera configuración (ISO 8601)
    #[serde(default)]
    pub configured_at: Option<String>,
    /// Versión en la que se configuró
    #[serde(default)]
    pub configured_version: Option<String>,
}

/// Configuración de la terminal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    /// Nombre identificador de la terminal
    pub nombre: String,
    /// ID único generado a partir del hardware
    pub id: String,
    /// Ubicación física de la terminal
    #[serde(default = "default_ubicacion")]
    pub ubicacion: String,
}

fn default_ubicacion() -> String {
    "Sin asignar".to_string()
}

/// Configuración de la base de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Rutas a intentar en orden de prioridad
    pub paths: Vec<String>,
    /// Ruta por defecto si no existe ninguna
    pub default_path: String,
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
                ubicacion: "Sin asignar".to_string(),
            },
            database: DatabaseConfig {
                paths: vec![
                    "./data/brisas.db".to_string(),
                    "C:/ProgramData/Brisas/brisas.db".to_string(),
                ],
                default_path: "".to_string(), // Se calculará en runtime
            },
            app: AppInfo {
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            setup: SetupState::default(),
        }
    }
}
