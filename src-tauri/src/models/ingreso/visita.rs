// ==========================================
// src/models/ingreso/visita.rs
// ==========================================

use crate::models::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO DE DOMINIO: INGRESO DE VISITA
// --------------------------------------------------------------------------

/// Registro de entrada/salida de un visitante ocasional.
///
/// Captura una "instantánea" de la identidad y motivo de la visita.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IngresoVisita {
    pub id: RecordId,
    pub pre_registro: Option<RecordId>, // Nuevo enlace opcional
    pub nombre: String,
    pub apellido: String,
    /// Segundo nombre (snapshot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    /// Segundo apellido (snapshot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
    pub empresa_nombre: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
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

/// Versión "poblada" del ingreso de visita (FETCH).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IngresoVisitaFetched {
    pub id: RecordId,
    pub pre_registro: Option<RecordId>,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
    pub empresa_nombre: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
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
pub struct CreateIngresoVisitaInput {
    pub cedula: String,
    pub pre_registro_id: Option<String>,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub observaciones: Option<String>,
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct IngresoVisitaCreateDTO {
    pub pre_registro: Option<RecordId>,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
    pub empresa_nombre: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub usuario_ingreso: RecordId,
    pub observaciones: Option<String>,
}

// --------------------------------------------------------------------------
// DTOs DE RESPUESTA ESPECÍFICOS
// --------------------------------------------------------------------------

/// Estructura con datos expandidos (populated) para UI de historial/reportes
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoVisitaPopulated {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa_nombre: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub fecha_hora_ingreso: String,
    pub usuario_ingreso_id: String,
    pub usuario_ingreso_nombre: String,
    pub fecha_hora_salida: Option<String>,
    pub usuario_salida_id: Option<String>,
    pub usuario_salida_nombre: Option<String>,
    pub observaciones: Option<String>,
    pub esta_adentro: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Respuesta de validación previa al ingreso de visita
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoVisitaResponse {
    pub puede_ingresar: bool,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub motivo_rechazo: Option<String>,
    pub alertas_gafete: Vec<String>,
    pub tiene_gafetes_pendientes: bool,
}

// --------------------------------------------------------------------------
// MODELO DE DOMINIO: PRE-REGISTRO DE VISITA
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreRegistroEstado {
    Pendiente,
    Completado,
    Cancelado,
    NoShow,
}

impl ToString for PreRegistroEstado {
    fn to_string(&self) -> String {
        match self {
            Self::Pendiente => "PENDIENTE".to_string(),
            Self::Completado => "COMPLETADO".to_string(),
            Self::Cancelado => "CANCELADO".to_string(),
            Self::NoShow => "NO_SHOW".to_string(),
        }
    }
}

impl std::str::FromStr for PreRegistroEstado {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDIENTE" => Ok(Self::Pendiente),
            "COMPLETADO" => Ok(Self::Completado),
            "CANCELADO" => Ok(Self::Cancelado),
            "NO_SHOW" => Ok(Self::NoShow),
            _ => Err(format!("Estado desconocido: {}", s)),
        }
    }
}

/// Cita o invitación previa para una visita.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PreRegistroVisita {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub empresa_nombre: Option<String>,
    pub fecha_esperada: String, // YYYY-MM-DD
    pub hora_esperada: String,  // HH:MM
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa: Option<String>,
    pub empresa_id: Option<String>,
    pub observaciones: Option<String>,
    pub estado: String,              // Enum as String for DB
    pub visitante: Option<RecordId>, // Link to catalog if exists
    pub registrado_por: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PreRegistroVisitaFetched {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub empresa_nombre: Option<String>,
    pub fecha_esperada: String,
    pub hora_esperada: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa: Option<String>,
    pub empresa_id: Option<String>,
    pub estado: String,
    pub observaciones: Option<String>,
    pub registrado_por: User,
    pub created_at: Datetime,
}

// DTO para crear pre-registro
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePreRegistroInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa_nombre: Option<String>,
    pub fecha_esperada: String,
    pub hora_esperada: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa: Option<String>,
    pub empresa_id: Option<String>,
    pub observaciones: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PreRegistroVisitaCreateDTO {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub empresa_nombre: Option<String>,
    pub fecha_esperada: String,
    pub hora_esperada: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa: Option<String>,
    pub empresa_id: Option<RecordId>,
    pub observaciones: Option<String>,
    pub estado: String,
    pub visitante: Option<RecordId>,
    pub registrado_por: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
