use crate::models::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngresoVisita {
    pub id: RecordId,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub anfitrion: String,
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
pub struct IngresoVisitaFetched {
    pub id: RecordId,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub anfitrion: String,
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
pub struct CreateIngresoVisitaInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<String>,
    pub observaciones: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IngresoVisitaCreateDTO {
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub gafete_numero: Option<String>,
    pub usuario_ingreso: RecordId,
    pub observaciones: Option<String>,
}
