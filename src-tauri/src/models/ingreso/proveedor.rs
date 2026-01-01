// ==========================================
// src/models/ingreso/proveedor.rs
// ==========================================

use crate::models::proveedor::ProveedorFetched;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO DE DOMINIO: INGRESO DE PROVEEDOR
// --------------------------------------------------------------------------

/// Registro de entrada/salida de un proveedor.
///
/// Captura una "instantánea" de la identidad del proveedor al momento del acceso.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoProveedor {
    pub id: RecordId,
    pub proveedor: RecordId,
    pub nombre: String,
    pub apellido: String,
    /// Segundo nombre (snapshot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    /// Segundo apellido (snapshot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
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

/// Versión "poblada" del ingreso con datos completos (FETCH).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoProveedorFetched {
    pub id: RecordId,
    pub proveedor: ProveedorFetched,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
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
pub struct CreateIngresoProveedorInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub proveedor_id: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<i32>,
    pub observaciones: Option<String>,
    // Nota: El frontend puede enviar segundo_nombre/apellido si los tiene,
    // pero idealmente se toman del registro maestro.
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct IngresoProveedorCreateDTO {
    pub proveedor: RecordId,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    pub cedula: String,
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

/// Respuesta de validación previa al ingreso de proveedor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoProveedorResponse {
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
