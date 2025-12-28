use crate::models::contratista::ContratistaFetched;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngresoContratista {
    pub id: RecordId,
    pub contratista: RecordId,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub tipo_autorizacion: String,
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
pub struct IngresoContratistaFetched {
    pub id: RecordId,
    pub contratista: ContratistaFetched,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub tipo_autorizacion: String,
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
pub struct CreateIngresoContratistaInput {
    pub contratista_id: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<String>,
    pub observaciones: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IngresoContratistaCreateDTO {
    pub contratista: RecordId,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<String>,
    pub usuario_ingreso: RecordId,
    pub observaciones: Option<String>,
}
