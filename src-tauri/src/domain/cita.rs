use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Cita {
    pub id: String,
    pub visitante_id: String,
    pub fecha_cita: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub estado: String, // PENDIENTE, COMPLETADA, CANCELADA, EXPIRADA
    pub registrado_por: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct CitaPopulated {
    pub id: String,
    pub fecha_cita: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub estado: String,
    // Datos del visitante "aplanados" para la UI
    pub visitante_id: String,
    pub visitante_cedula: String,
    pub visitante_nombre: String,
    pub visitante_apellido: String,
    pub visitante_nombre_completo: String,
    pub visitante_empresa: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCitaInput {
    pub visitante_id: String,
    pub fecha_cita: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub registrado_por: String,
}
