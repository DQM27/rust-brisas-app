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
    /// Indica si el gafete está actualmente asignado a un ingreso activo.
    #[serde(default, alias = "en_uso")]
    pub en_uso: bool,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
}

// --------------------------------------------------------------------------
// ENUMS (Tipos Estrictos)
// --------------------------------------------------------------------------

/// Clasificación del gafete según el tipo de visita.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TipoGafete {
    Contratista,
    Proveedor,
    Visita,
    Otro,
}

impl std::fmt::Display for TipoGafete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TipoGafete {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Contratista => "contratista",
            Self::Proveedor => "proveedor",
            Self::Visita => "visita",
            Self::Otro => "otro",
        }
    }

    pub const fn display(&self) -> &str {
        match self {
            Self::Contratista => "Contratista",
            Self::Proveedor => "Proveedor",
            Self::Visita => "Visita",
            Self::Otro => "Otro",
        }
    }
}

impl std::str::FromStr for TipoGafete {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(Self::Contratista),
            "proveedor" => Ok(Self::Proveedor),
            "visita" => Ok(Self::Visita),
            "otro" => Ok(Self::Otro),
            _ => Err(format!("Tipo de gafete desconocido: {s}")),
        }
    }
}

/// Estado operativo y físico del gafete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GafeteEstado {
    Activo,
    Danado,
    Extraviado,
}

impl std::fmt::Display for GafeteEstado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl GafeteEstado {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Activo => "activo",
            Self::Danado => "danado",
            Self::Extraviado => "extraviado",
        }
    }
}

impl std::str::FromStr for GafeteEstado {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(Self::Activo),
            "danado" => Ok(Self::Danado),
            "perdido" | "extraviado" => Ok(Self::Extraviado),
            _ => Err(format!("Estado de gafete desconocido: {s}")),
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
    #[serde(default)]
    pub en_uso: bool,
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
    pub esta_disponible: bool,
    pub status: String,
    // Información de alerta (si hay una pendiente)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerta_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_perdido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quien_perdio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerta_resuelta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportado_por_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resuelto_por_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_resolucion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notas: Option<String>,
    // Info about who has the gafete when "en_uso"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asignado_a: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Gafete> for GafeteResponse {
    fn from(g: Gafete) -> Self {
        let esta_disponible = g.estado == GafeteEstado::Activo && !g.en_uso;
        let status = if g.estado == GafeteEstado::Activo {
            if g.en_uso {
                "en_uso".to_string()
            } else {
                "disponible".to_string()
            }
        } else {
            g.estado.as_str().to_string()
        };

        Self {
            id: g.id.to_string(),
            numero: g.numero,
            tipo: g.tipo.clone(),
            tipo_display: g.tipo.display().to_string(),
            estado_fisico: g.estado.clone(),
            esta_disponible,
            status,
            // Alert fields - set to None by default, enriched by service
            alerta_id: None,
            fecha_perdido: None,
            quien_perdio: None,
            alerta_resuelta: None,
            reportado_por_nombre: None,
            resuelto_por_nombre: None,
            fecha_resolucion: None,
            notas: None,
            asignado_a: None,
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
