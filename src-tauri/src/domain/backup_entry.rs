/// Modelo de entrada de backup para listar backups existentes.
///
/// Representa un archivo de backup en el directorio de backups automáticos.
use serde::{Deserialize, Serialize};

/// Respuesta de listado de backups para el frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupEntryResponse {
    /// Nombre del archivo (e.g., "brisas_backup_2026-01-13_02-00.surql")
    pub nombre: String,
    /// Ruta completa al archivo
    pub ruta: String,
    /// Tamaño en bytes
    pub tamano: u64,
    /// Fecha de creación en formato ISO 8601
    pub fecha_creacion: String,
    /// Días desde la creación
    pub dias_antiguedad: u32,
}
