// src-tauri/src/config/settings.rs

use serde::{Deserialize, Serialize};

/// Configuración completa de la aplicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub terminal: TerminalConfig,
    pub database: DatabaseConfig,
    pub app: AppInfo,
    #[serde(default)]
    pub audio: AudioConfig,
    #[serde(default)]
    pub setup: SetupState,
    #[serde(default)]
    pub backup: BackupConfig,
}

/// Configuración de backup automático
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Backup automático habilitado
    #[serde(default)]
    pub enabled: bool,
    /// Hora del backup en formato "HH:MM" (24h)
    #[serde(default = "default_backup_hora")]
    pub hora: String,
    /// Días de retención antes de eliminar backups antiguos
    #[serde(default = "default_dias_retencion")]
    pub dias_retencion: u32,
    /// Directorio donde se almacenan los backups automáticos
    #[serde(default)]
    pub directorio: Option<String>,
    /// Timestamp ISO del último backup exitoso
    #[serde(default)]
    pub ultimo_backup: Option<String>,
}

fn default_backup_hora() -> String {
    "02:00".to_string()
}

const fn default_dias_retencion() -> u32 {
    30
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            hora: default_backup_hora(),
            dias_retencion: default_dias_retencion(),
            directorio: None,
            ultimo_backup: None,
        }
    }
}

/// Configuración de audio y alertas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Sonido de alerta nativo de Windows (Hand, Exclamation, Beep, Question, Asterisk)
    #[serde(default = "default_alert_sound")]
    pub alert_sound: String,
    /// Ruta al archivo de sonido personalizado
    #[serde(default)]
    pub custom_sound_path: Option<String>,
    /// Indica si se debe usar el sonido personalizado en lugar del nativo
    #[serde(default)]
    pub use_custom: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self { alert_sound: default_alert_sound(), custom_sound_path: None, use_custom: false }
    }
}

fn default_alert_sound() -> String {
    "Hand".to_string()
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
    /// Indica si los datos base (roles, usuarios, módulos) ya fueron sembrados
    #[serde(default)]
    pub is_seeded: bool,
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
                    "./data/surrealdb".to_string(),
                    "C:/ProgramData/Brisas/surrealdb".to_string(),
                ],
                default_path: String::new(), // Se calculará en runtime
            },
            app: AppInfo { version: env!("CARGO_PKG_VERSION").to_string() },
            audio: AudioConfig::default(),
            setup: SetupState::default(),
            backup: BackupConfig::default(),
        }
    }
}

/// Type alias para `AppConfig` mutable compartido
/// Permite actualizar la configuración en runtime desde cualquier comando
pub type AppConfigState = std::sync::Arc<std::sync::RwLock<AppConfig>>;
