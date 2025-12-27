// src/services/cita_service.rs
use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use crate::domain::errors::CitaError;
use crate::models::visitante::CreateVisitanteInput;
use crate::services::visitante_service;

pub async fn agendar_cita(
    cita_input: CreateCitaInput,
    visitante_input: Option<CreateVisitanteInput>,
) -> Result<Cita, CitaError> {
    let mut visitante_id = cita_input.visitante_id.clone();
    if let Some(v_input) = visitante_input {
        let existente = visitante_service::get_visitante_by_cedula(&v_input.cedula)
            .await
            .map_err(|e| CitaError::Database(sqlx::Error::Protocol(e.to_string())))?;
        match existente {
            Some(v) => {
                visitante_id = v.id;
            }
            None => {
                let nuevo = visitante_service::create_visitante(v_input)
                    .await
                    .map_err(|e| CitaError::Database(sqlx::Error::Protocol(e.to_string())))?;
                visitante_id = nuevo.id;
            }
        }
    }
    if visitante_id.is_empty() {
        return Err(CitaError::Validation("Visitante requerido".to_string()));
    }
    let mut input_final = cita_input;
    input_final.visitante_id = visitante_id;
    Err(CitaError::Database(sqlx::Error::Protocol("no implementado".to_string())))
}

pub async fn get_citas_hoy() -> Result<Vec<CitaPopulated>, CitaError> {
    Ok(vec![])
}
pub async fn get_citas_pendientes() -> Result<Vec<CitaPopulated>, CitaError> {
    Ok(vec![])
}
pub async fn update_cita(
    _id: String,
    _fecha: String,
    _anf: String,
    _area: String,
    _mot: Option<String>,
) -> Result<(), CitaError> {
    Err(CitaError::Database(sqlx::Error::Protocol("no implementado".to_string())))
}
pub async fn procesar_ingreso_cita(
    _id: String,
    _gaf: String,
    _u: String,
) -> Result<String, CitaError> {
    Err(CitaError::NotFound)
}
pub async fn cancelar_cita(_id: String) -> Result<(), CitaError> {
    Err(CitaError::Database(sqlx::Error::Protocol("no implementado".to_string())))
}
