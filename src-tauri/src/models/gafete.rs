// ==========================================
// src/models/gafete.rs
// ==========================================

use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gafete {
    pub numero: String,
    pub tipo: TipoGafete,
    pub estado: GafeteEstado,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// ENUM DE TIPOS
// ==========================================

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

// ==========================================
// ENUM DE ESTADO FISICO
// ==========================================

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

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGafeteInput {
    pub numero: String,
    pub tipo: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGafeteRangeInput {
    pub start: u32,
    pub end: u32,
    pub prefix: Option<String>,
    pub padding: Option<usize>,
    pub tipo: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGafeteInput {
    pub tipo: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGafeteStatusInput {
    pub estado: GafeteEstado,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GafeteResponse {
    pub numero: String,
    pub tipo: TipoGafete,
    pub tipo_display: String,
    pub estado_fisico: GafeteEstado,
    pub esta_disponible: bool,
    pub status: String,
    pub alerta_id: Option<String>,
    pub fecha_perdido: Option<String>,
    pub quien_perdio: Option<String>,
    pub alerta_resuelta: Option<bool>,
    pub reportado_por_nombre: Option<String>,
    pub resuelto_por_nombre: Option<String>,
    pub fecha_resolucion: Option<String>,
    pub notas: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}

impl From<Gafete> for GafeteResponse {
    fn from(g: Gafete) -> Self {
        Self {
            numero: g.numero,
            tipo: g.tipo.clone(),
            tipo_display: g.tipo.display().to_string(),
            estado_fisico: g.estado.clone(),
            esta_disponible: false,
            status: String::from("disponible"),
            alerta_id: None,
            fecha_perdido: None,
            quien_perdio: None,
            alerta_resuelta: None,
            reportado_por_nombre: None,
            resuelto_por_nombre: None,
            fecha_resolucion: None,
            notas: None,
            created_at: g.created_at,
            updated_at: g.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GafeteListResponse {
    pub gafetes: Vec<GafeteResponse>,
    pub total: usize,
    pub stats: StatsGafetes,
}

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPorTipo {
    pub contratistas: usize,
    pub proveedores: usize,
    pub visitas: usize,
    pub otros: usize,
}
