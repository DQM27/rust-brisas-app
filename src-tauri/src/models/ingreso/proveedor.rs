use crate::models::proveedor::ProveedorFetched;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngresoProveedor {
    pub id: RecordId,
    pub proveedor: RecordId,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<String>,
    pub fecha_hora_ingreso: surrealdb::Datetime,
    pub usuario_ingreso: RecordId,
    pub fecha_hora_salida: Option<surrealdb::Datetime>,
    pub usuario_salida: Option<RecordId>,
    pub observaciones: Option<String>,
    pub created_at: surrealdb::Datetime,
    pub updated_at: surrealdb::Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngresoProveedorFetched {
    pub id: RecordId,
    pub proveedor: ProveedorFetched,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<String>,
    pub fecha_hora_ingreso: surrealdb::Datetime,
    pub usuario_ingreso: User,
    pub fecha_hora_salida: Option<surrealdb::Datetime>,
    pub usuario_salida: Option<User>,
    pub observaciones: Option<String>,
    pub created_at: surrealdb::Datetime,
    pub updated_at: surrealdb::Datetime,
}

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
    pub gafete_numero: Option<String>,
    pub observaciones: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IngresoProveedorCreateDTO {
    pub proveedor: RecordId,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<String>,
    pub usuario_ingreso: RecordId,
    pub observaciones: Option<String>,
}

/// Respuesta de validación previa al ingreso de proveedor
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoProveedorResponse {
    pub puede_ingresar: bool,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub motivo_rechazo: Option<String>,
    pub alertas_gafete: Vec<String>,
    pub tiene_gafetes_pendientes: bool,
}
