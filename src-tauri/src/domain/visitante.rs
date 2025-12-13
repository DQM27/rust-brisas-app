use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Visitante {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<String>,
    pub has_vehicle: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVisitanteInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<String>,
    pub has_vehicle: bool,
}
