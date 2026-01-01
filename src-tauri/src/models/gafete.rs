use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO DE DOMINIO
// --------------------------------------------------------------------------

/// Representa un gafete físico o tarjeta de acceso.
///
/// Un gafete es la credencial física que se entrega a un visitante, contratista o proveedor
/// para permitir su acceso a las instalaciones.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gafete {
    pub id: RecordId,
    /// Número visible en el gafete físico.
    pub numero: i32,
    /// Tipo de usuario al que está destinado este gafete.
    pub tipo: TipoGafete,
    /// Estado físico del gafete.
    pub estado: GafeteEstado,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
}

// --------------------------------------------------------------------------
// ENUMS (Tipos Estrictos)
// --------------------------------------------------------------------------

/// Clasificación del gafete según el tipo de visita.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoGafete {
    Contratista,
    Proveedor,
    Visita,
    Otro,
}

impl TipoGafete {
    pub fn as_str(&self) -> &str {
        match self {
            TipoGafete::Contratista => "contratista",
            TipoGafete::Proveedor => "proveedor",
            TipoGafete::Visita => "visita",
            TipoGafete::Otro => "otro",
        }
    }

    pub fn display(&self) -> &str {
        match self {
            TipoGafete::Contratista => "Contratista",
            TipoGafete::Proveedor => "Proveedor",
            TipoGafete::Visita => "Visita",
            TipoGafete::Otro => "Otro",
        }
    }
}

impl std::str::FromStr for TipoGafete {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(TipoGafete::Contratista),
            "proveedor" => Ok(TipoGafete::Proveedor),
            "visita" => Ok(TipoGafete::Visita),
            "otro" => Ok(TipoGafete::Otro),
            _ => Err(format!("Tipo de gafete desconocido: {}", s)),
        }
    }
}

/// Estado operativo y físico del gafete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GafeteEstado {
    Activo,
    Danado,
    Extraviado,
}

impl GafeteEstado {
    pub fn as_str(&self) -> &str {
        match self {
            GafeteEstado::Activo => "activo",
            GafeteEstado::Danado => "danado",
            GafeteEstado::Extraviado => "extraviado",
        }
    }
}

impl std::str::FromStr for GafeteEstado {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(GafeteEstado::Activo),
            "danado" => Ok(GafeteEstado::Danado),
            "extraviado" => Ok(GafeteEstado::Extraviado),
            _ => Err(format!("Estado de gafete desconocido: {}", s)),
        }
    }
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA
// --------------------------------------------------------------------------

/// Datos necesarios para crear un único gafete.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGafeteInput {
    pub numero: i32,
    pub tipo: String,
}

/// Datos para la creación masiva de gafetes por rango.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGafeteRangeInput {
    pub start: i32,
    pub end: i32,
    pub tipo: String,
}

/// Datos para actualizar información general de un gafete.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGafeteInput {
    pub tipo: Option<String>,
}

/// Datos para cambiar el estado de un gafete (ej. reportar daño).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGafeteStatusInput {
    pub estado: GafeteEstado,
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

/// DTO intermedio para insertar en base de datos.
#[derive(Debug, Serialize)]
pub struct GafeteCreateDTO {
    pub numero: i32,
    pub tipo: TipoGafete,
    pub estado: GafeteEstado,
}

// --------------------------------------------------------------------------
// DTOs DE RESPUESTA
// --------------------------------------------------------------------------

/// Respuesta estándar con la información pública del gafete.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GafeteResponse {
    pub id: String,
    pub numero: i32,
    pub tipo: TipoGafete,
    pub tipo_display: String,
    pub estado_fisico: GafeteEstado,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Gafete> for GafeteResponse {
    fn from(g: Gafete) -> Self {
        Self {
            id: g.id.to_string(),
            numero: g.numero,
            tipo: g.tipo.clone(),
            tipo_display: g.tipo.display().to_string(),
            estado_fisico: g.estado.clone(),
            status: String::from("disponible"), // La lógica de estado real (uso) suele enriquecerse en el servicio
            created_at: g.created_at.to_string(),
            updated_at: g.updated_at.to_string(),
        }
    }
}

/// Respuesta paginada para listas de gafetes.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GafeteListResponse {
    pub gafetes: Vec<GafeteResponse>,
    pub total: usize,
    pub stats: StatsGafetes,
}

/// Estadísticas globales de inventario de gafetes.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsGafetes {
    pub total: usize,
    pub disponibles: usize,
    pub en_uso: usize,
    pub danados: usize,
    pub extraviados: usize,
    pub por_tipo: StatsPorTipo,
}

/// Desglose de estadísticas por tipo.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPorTipo {
    pub contratistas: usize,
    pub proveedores: usize,
    pub visitas: usize,
    pub otros: usize,
}
