use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct IngresoVisita {
    pub id: String,
    pub visitante_id: String,
    pub cita_id: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub fecha_ingreso: String,
    pub fecha_salida: Option<String>,
    pub estado: String, // 'ADENTRO', 'SALIO'
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoVisitaInput {
    pub visitante_id: String,
    pub cita_id: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoVisitaFullInput {
    // Datos Visitante
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa: Option<String>,

    // Datos Ingreso
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,

    // Opcional: Cita ID si viene de cita (aunque si es full, suele ser manual)
    pub cita_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct IngresoVisitaPopulated {
    // Ingreso Fields
    pub id: String,
    pub visitante_id: String,
    pub cita_id: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub fecha_ingreso: String,
    pub fecha_salida: Option<String>,
    pub estado: String,
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,

    // Visitante Fields (Joined)
    pub visitante_nombre: String,
    pub visitante_apellido: String,
    pub visitante_cedula: String,
    pub visitante_empresa: Option<String>,
}
