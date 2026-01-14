/// Módulo: Ingreso.
///
/// Este módulo centraliza todos los submodelos y tipos relacionados con el control de acceso
/// (entradas y salidas) de personal y visitantes.
///
/// Submódulos:
/// - `contratista`: Ingresos de personal contratista (PRAIND).
/// - `proveedor`: Ingresos de proveedores (entregas, servicios).
/// - `visita`: Ingresos de visitantes ocasionales.
/// - `alerta`: Gestión de incidentes de gafetes y seguridad.
/// - `response`: Estructuras de respuesta unificadas (DTOs de salida).
// src/models/ingreso/mod.rs
pub mod alerta;
pub mod contratista;
pub mod proveedor;
pub mod response;
pub mod visita;

pub use alerta::*;
pub use contratista::*;
pub use proveedor::*;
pub use response::*;
pub use visita::*;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// ==========================================
// ENUMS DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(try_from = "String")]
pub enum TipoIngreso {
    Contratista,
    Visita,
    Proveedor,
}

impl TipoIngreso {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Contratista => "contratista",
            Self::Visita => "visita",
            Self::Proveedor => "proveedor",
        }
    }

    pub const fn display(&self) -> &str {
        match self {
            Self::Contratista => "Contratista",
            Self::Visita => "Visita",
            Self::Proveedor => "Proveedor",
        }
    }
}

impl std::str::FromStr for TipoIngreso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(Self::Contratista),
            "visita" => Ok(Self::Visita),
            "proveedor" => Ok(Self::Proveedor),
            _ => Err(format!("Tipo de ingreso desconocido: {s}")),
        }
    }
}

impl TryFrom<String> for TipoIngreso {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(try_from = "String")]
pub enum TipoAutorizacion {
    Praind,
    Correo,
}

impl TipoAutorizacion {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Praind => "praind",
            Self::Correo => "correo",
        }
    }
}

impl std::str::FromStr for TipoAutorizacion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "praind" => Ok(Self::Praind),
            "correo" => Ok(Self::Correo),
            _ => Err(format!("Tipo de autorización desconocido: {s}")),
        }
    }
}

impl TryFrom<String> for TipoAutorizacion {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(try_from = "String")]
pub enum ModoIngreso {
    Caminando,
    Vehiculo,
}

impl ModoIngreso {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Caminando => "caminando",
            Self::Vehiculo => "vehiculo",
        }
    }

    pub const fn display(&self) -> &str {
        match self {
            Self::Caminando => "Caminando",
            Self::Vehiculo => "Vehículo",
        }
    }
}

impl std::str::FromStr for ModoIngreso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "caminando" => Ok(Self::Caminando),
            "vehiculo" => Ok(Self::Vehiculo),
            _ => Err(format!("Modo de ingreso desconocido: {s}")),
        }
    }
}

impl TryFrom<String> for ModoIngreso {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

// ==========================================
// DTOs COMPARTIDOS
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tipo_ingreso")]
pub enum UniversalIngresoFetched {
    #[serde(rename = "ingreso_contratista")]
    Contratista(IngresoContratistaFetched),
    #[serde(rename = "ingreso_proveedor")]
    Proveedor(IngresoProveedorFetched),
    #[serde(rename = "ingreso_visita")]
    Visita(IngresoVisitaFetched),
}

impl UniversalIngresoFetched {
    pub fn to_response(self) -> Result<IngresoResponse, String> {
        match self {
            Self::Contratista(i) => IngresoResponse::from_contratista_fetched(i),
            Self::Proveedor(i) => Ok(IngresoResponse::from_proveedor_fetched(i)),
            Self::Visita(i) => Ok(IngresoResponse::from_visita_fetched(i)),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "tipo", rename_all = "lowercase")]
pub enum CreateIngresoInput {
    Contratista(CreateIngresoContratistaInput),
    Visita(CreateIngresoVisitaInput),
    Proveedor(CreateIngresoProveedorInput),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarSalidaInput {
    pub ingreso_id: String,
    pub devolvio_gafete: bool,
    pub usuario_salida_id: String,
    pub observaciones_salida: Option<String>,
}

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct IngresoUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_hora_salida: Option<surrealdb::Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usuario_salida: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observaciones: Option<String>,
}

pub type IngresoContratistaUpdateDTO = IngresoUpdateDTO;
pub type IngresoProveedorUpdateDTO = IngresoUpdateDTO;
pub type IngresoVisitaUpdateDTO = IngresoUpdateDTO;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoListResponse {
    pub ingresos: Vec<IngresoResponse>,
    pub total: usize,
    pub adentro: usize,
    pub salieron: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub severidad_lista_negra: Option<String>,
    pub alertas: Vec<String>,
    pub contratista: Option<serde_json::Value>,
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoResponse>,
}

// --------------------------------------------------------------------------
// DTOs DE SERVICIO (Respuestas Enriquecidas)
// --------------------------------------------------------------------------

/// Resultado de validación previa a la salida.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoValidacionSalida {
    pub puede_salir: bool,
    pub errores: Vec<String>,
    pub advertencias: Vec<String>,
}

/// Respuesta de ingreso con estado de permanencia.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoConEstadoResponse {
    pub ingreso: IngresoResponse,
    pub minutos_transcurridos: i64,
    pub estado: String,
}

/// Alerta por tiempo de permanencia excedido.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertaTiempoExcedido {
    pub ingreso_id: String,
    pub cedula: String,
    pub nombre_completo: String,
    pub empresa_nombre: String,
    pub fecha_hora_ingreso: String,
    pub minutos_transcurridos: i64,
    pub minutos_excedidos: i64,
    pub estado: String,
}

/// Input para cierre manual de un ingreso.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CerrarIngresoManualInput {
    pub ingreso_id: String,
    pub motivo_cierre: String,
    pub fecha_salida_estimada: Option<String>,
    pub notas: Option<String>,
}

/// Respuesta de cierre manual con indicador de reporte.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoCierreManualResponse {
    pub ingreso: IngresoResponse,
    pub genera_reporte: bool,
    pub tipo_reporte: Option<String>,
    pub mensaje: Option<String>,
}

/// Input para ingreso excepcional (bypass de bloqueos).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoExcepcionalInput {
    pub contratista_id: String,
    pub autorizado_por: String,
    pub motivo_excepcional: String,
    pub notas: Option<String>,
    pub vehiculo_id: Option<String>,
    pub gafete_numero: Option<String>,
    pub modo_ingreso: String,
    pub observaciones: Option<String>,
}

/// Respuesta de ingreso excepcional con trazabilidad.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoExcepcionalResponse {
    pub ingreso: IngresoResponse,
    pub motivo_original_bloqueo: String,
    pub autorizado_por: String,
    pub valido_hasta: String,
}
