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
    pub gafete_numero: Option<i32>,
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
    pub gafete_numero: Option<i32>,
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
    pub gafete_numero: Option<i32>,
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
    pub gafete_numero: Option<i32>,
    pub usuario_ingreso: RecordId,
    pub observaciones: Option<String>,
}

/// Estructura con datos expandidos (populated) para UI
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoVisitaPopulated {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
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
    pub motivo_rechazo: Option<String>,
    pub alertas_gafete: Vec<String>,
    pub tiene_gafetes_pendientes: bool,
}
