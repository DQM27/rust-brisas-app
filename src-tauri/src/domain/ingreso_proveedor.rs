use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct IngresoProveedor {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_id: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub tipo_autorizacion: Option<String>,
    pub modo_ingreso: Option<String>,
    pub placa_vehiculo: Option<String>,
    pub fecha_ingreso: String,
    pub fecha_salida: Option<String>,
    pub estado: String,
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoProveedorInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_id: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub placa_vehiculo: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}
