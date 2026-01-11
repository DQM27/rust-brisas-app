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
    pub nombre: String,
    pub apellido: String,
    /// Segundo nombre (snapshot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    /// Segundo apellido (snapshot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
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
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
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
    pub nombre: String,
    pub apellido: String,
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
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
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
