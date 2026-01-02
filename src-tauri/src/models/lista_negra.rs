// ==========================================
// src/models/lista_negra.rs
// ==========================================

use serde::{Deserialize, Serialize};
use std::fmt;
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// ENUMS (Tipos Estrictos)
// --------------------------------------------------------------------------

/// Nivel de severidad del bloqueo, indica la gravedad del incidente.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[derive(Default)]
pub enum NivelSeveridad {
    Alto,
    Medio,
    #[default]
    Bajo,
}

impl fmt::Display for NivelSeveridad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alto => write!(f, "ALTO"),
            Self::Medio => write!(f, "MEDIO"),
            Self::Bajo => write!(f, "BAJO"),
        }
    }
}

impl std::str::FromStr for NivelSeveridad {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ALTO" => Ok(Self::Alto),
            "MEDIO" => Ok(Self::Medio),
            "BAJO" => Ok(Self::Bajo),
            _ => Err(format!("Nivel de severidad inválido: {s}")),
        }
    }
}


// --------------------------------------------------------------------------
// MODELO DE DOMINIO
// --------------------------------------------------------------------------

/// Representa una persona bloqueada en el sistema.
///
/// Contiene la información de la persona, el motivo del bloqueo y los metadatos de auditoría.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegra {
    pub id: RecordId,
    /// Cédula o identificador de la persona bloqueada.
    pub cedula: String,
    pub nombre: String,
    #[serde(alias = "segundo_nombre")]
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    #[serde(alias = "segundo_apellido")]
    pub segundo_apellido: Option<String>,
    #[serde(alias = "empresa_id")]
    pub empresa_id: Option<RecordId>,
    #[serde(alias = "empresa_nombre")]
    pub empresa_nombre: Option<String>,
    /// Nivel de riesgo asociado al individuo bloqueado.
    #[serde(alias = "nivel_severidad")]
    pub nivel_severidad: String, // TODO: Considerar usar NivelSeveridad enum si la DB lo soporta directamente o via string format
    #[serde(alias = "motivo_bloqueo")]
    pub motivo_bloqueo: String,
    /// Usuario administrativo que realizó el bloqueo.
    #[serde(alias = "bloqueado_por")]
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
    #[serde(alias = "is_active")]
    pub is_active: bool,
    #[serde(alias = "created_at")]
    pub created_at: surrealdb::Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: surrealdb::Datetime,
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA
// --------------------------------------------------------------------------

/// Datos requeridos para agregar a una persona a la lista negra.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToListaNegraInput {
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa_id: Option<String>,
    pub empresa_nombre: Option<String>,
    pub nivel_severidad: String,
    pub motivo_bloqueo: String,
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
}

/// Datos para actualizar un registro de bloqueo existente.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateListaNegraInput {
    pub nivel_severidad: Option<String>,
    pub motivo_bloqueo: Option<String>,
    pub observaciones: Option<String>,
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA
// --------------------------------------------------------------------------

/// Respuesta detallada de un registro de lista negra.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa_id: Option<String>,
    pub empresa_nombre: Option<String>,
    pub nivel_severidad: String,
    pub motivo_bloqueo: String,
    pub bloqueado_por: String,
    pub bloqueado_por_nombre: Option<String>,
    pub observaciones: Option<String>,
    pub is_active: bool,
    pub dias_bloqueado: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ListaNegra> for ListaNegraResponse {
    fn from(ln: ListaNegra) -> Self {
        let nombre_completo = format!(
            "{} {} {} {}",
            ln.nombre,
            ln.segundo_nombre.clone().unwrap_or_default(),
            ln.apellido,
            ln.segundo_apellido.clone().unwrap_or_default()
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

        let created: chrono::DateTime<chrono::Utc> =
            ln.created_at.to_string().parse().unwrap_or_default();
        let updated: chrono::DateTime<chrono::Utc> =
            ln.updated_at.to_string().parse().unwrap_or_default();

        let dias_bloqueado = (chrono::Utc::now() - created).num_days();

        Self {
            id: ln.id.to_string(),
            cedula: ln.cedula,
            nombre: ln.nombre,
            segundo_nombre: ln.segundo_nombre,
            apellido: ln.apellido,
            segundo_apellido: ln.segundo_apellido,
            nombre_completo,
            empresa_id: ln.empresa_id.map(|id| id.to_string()),
            empresa_nombre: ln.empresa_nombre,
            nivel_severidad: ln.nivel_severidad,
            motivo_bloqueo: ln.motivo_bloqueo,
            bloqueado_por: ln.bloqueado_por,
            bloqueado_por_nombre: None,
            observaciones: ln.observaciones,
            is_active: ln.is_active,
            dias_bloqueado,
            created_at: created.to_rfc3339(),
            updated_at: updated.to_rfc3339(),
        }
    }
}

/// Respuesta simplificada de verificación de bloqueo.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockStatus {
    pub is_blocked: bool,
    pub nivel_severidad: Option<String>,
    pub bloqueado_desde: Option<String>,
}

/// Respuesta simplificada de verificación de bloqueo (Legacy alias replacement).
pub type BlockCheckResponse = BlockStatus;

/// Lista paginada de personas bloqueadas.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraListResponse {
    pub bloqueados: Vec<ListaNegraResponse>,
    pub total: usize,
    pub activos: usize,
    pub por_nivel: NivelStats,
}

/// Estadísticas de bloqueos por nivel de severidad.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NivelStats {
    pub alto: usize,
    pub medio: usize,
    pub bajo: usize,
}

/// Resultados de búsqueda para posibles candidatos a lista negra.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonaSearchResult {
    pub tipo_persona: String,
    pub entity_id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa_id: Option<String>,
    pub empresa_nombre: Option<String>,
    pub ya_bloqueado: bool,
}
