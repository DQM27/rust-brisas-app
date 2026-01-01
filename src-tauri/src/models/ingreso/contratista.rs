// ==========================================
// src/models/ingreso/contratista.rs
// ==========================================

use crate::models::contratista::ContratistaFetched;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO DE DOMINIO: INGRESO DE CONTRATISTA
// --------------------------------------------------------------------------

/// Registro de entrada/salida de un contratista.
///
/// Captura una "instantánea" de los datos del contratista y del evento de acceso,
/// permitiendo auditoría histórica incluso si los datos base cambian.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoContratista {
    pub id: RecordId,
    pub contratista: RecordId,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub fecha_hora_ingreso: Datetime,
    pub usuario_ingreso: RecordId,
    pub fecha_hora_salida: Option<Datetime>,
    pub usuario_salida: Option<RecordId>,
    pub observaciones: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

/// Versión "poblada" del ingreso con datos relacionales completos (FETCH).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoContratistaFetched {
    pub id: RecordId,
    pub contratista: ContratistaFetched,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub fecha_hora_ingreso: Datetime,
    pub usuario_ingreso: User,
    pub fecha_hora_salida: Option<Datetime>,
    pub usuario_salida: Option<User>,
    pub observaciones: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoContratistaInput {
    pub contratista_id: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub observaciones: Option<String>,
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct IngresoContratistaCreateDTO {
    pub contratista: RecordId,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub usuario_ingreso: RecordId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observaciones: Option<String>,
}

// --------------------------------------------------------------------------
// ENUMS DE NEGOCIO: PERMANENCIA
// --------------------------------------------------------------------------

/// Representación del estado temporal de un contratista dentro de las instalaciones.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstadoPermanencia {
    /// Situación normal, tiempo < 13h 30min.
    Normal,
    /// Precaución, tiempo >= 13h 30min y < 14h.
    AlertaTemprana,
    /// Incidencia crítica, >= 14h de permanencia.
    TiempoExcedido,
}

impl EstadoPermanencia {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoPermanencia::Normal => "normal",
            EstadoPermanencia::AlertaTemprana => "alerta_temprana",
            EstadoPermanencia::TiempoExcedido => "tiempo_excedido",
        }
    }
}

/// Detalles técnicos de una alerta de tiempo detectada por el sistema.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertaTiempo {
    pub estado: EstadoPermanencia,
    pub minutos_transcurridos: i64,
    pub minutos_restantes: i64,
    pub mensaje: Option<String>,
}

// --------------------------------------------------------------------------
// ENUMS DE NEGOCIO: CIERRE MANUAL
// --------------------------------------------------------------------------

/// Motivo de cierre manual de un ingreso
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotivoCierre {
    /// El guardia olvidó registrar la salida al momento
    OlvidoRegistrarSalida,
    /// Se confirmó que la persona salió sin registrar
    SalioSinRegistrar,
    /// No se encontró a la persona en las instalaciones
    PersonaNoLocalizada,
    /// Un supervisor autorizó el cierre (caso excepcional)
    AutorizacionEspecial,
}

impl MotivoCierre {
    pub fn descripcion(&self) -> &str {
        match self {
            MotivoCierre::OlvidoRegistrarSalida => "Se olvidó registrar la salida",
            MotivoCierre::SalioSinRegistrar => "La persona salió sin registrar",
            MotivoCierre::PersonaNoLocalizada => "No se localizó a la persona en instalaciones",
            MotivoCierre::AutorizacionEspecial => "Cierre autorizado por supervisor",
        }
    }
}

impl std::str::FromStr for MotivoCierre {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "olvido_registrar_salida" => Ok(MotivoCierre::OlvidoRegistrarSalida),
            "salio_sin_registrar" => Ok(MotivoCierre::SalioSinRegistrar),
            "persona_no_localizada" => Ok(MotivoCierre::PersonaNoLocalizada),
            "autorizacion_especial" => Ok(MotivoCierre::AutorizacionEspecial),
            _ => Err(format!("Motivo de cierre desconocido: {}", s)),
        }
    }
}

/// Resultado de evaluación de cierre manual
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoCierreManual {
    pub puede_cerrar: bool,
    pub genera_reporte: bool,
    pub tipo_reporte: Option<String>,
    pub mensaje: Option<String>,
}

// --------------------------------------------------------------------------
// ENUMS DE NEGOCIO: INGRESO EXCEPCIONAL
// --------------------------------------------------------------------------

/// Motivo para un ingreso excepcional (cuando normalmente no podría entrar)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotivoExcepcional {
    /// Orden directa de Seguridad Industrial
    OrdenSeguridadIndustrial,
    /// Emergencia operativa que requiere presencia
    EmergenciaOperativa,
    /// Documentos en trámite con autorización temporal
    DocumentosEnTramite,
    /// Otro motivo especificado en texto libre
    Otro,
}

impl std::str::FromStr for MotivoExcepcional {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "orden_seguridad_industrial" => Ok(MotivoExcepcional::OrdenSeguridadIndustrial),
            "emergencia_operativa" => Ok(MotivoExcepcional::EmergenciaOperativa),
            "documentos_en_tramite" => Ok(MotivoExcepcional::DocumentosEnTramite),
            "otro" => Ok(MotivoExcepcional::Otro),
            _ => Err(format!("Motivo excepcional desconocido: {}", s)),
        }
    }
}

/// Resultado de evaluación de ingreso excepcional
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoIngresoExcepcional {
    pub permitido: bool,
    pub motivo_original_bloqueo: String,
    pub autorizado_por: String,
    pub motivo_excepcional: MotivoExcepcional,
    pub notas: Option<String>,
    pub valido_hasta: String, // Válido solo hasta 23:59 del día
}
