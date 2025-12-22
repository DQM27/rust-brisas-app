// ==========================================
// src/domain/gafete.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::GafeteError;
use crate::models::gafete::{CreateGafeteInput, TipoGafete, UpdateGafeteInput};

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_numero(numero: &str) -> Result<(), GafeteError> {
    let limpio = numero.trim();

    if limpio.is_empty() {
        return Err(GafeteError::Validation(
            "El número del gafete no puede estar vacío".to_string(),
        ));
    }

    if limpio.len() > 20 {
        return Err(GafeteError::Validation(
            "El número no puede exceder 20 caracteres".to_string(),
        ));
    }

    if limpio.to_uppercase() == "S/G" {
        return Err(GafeteError::Validation(
            "'S/G' es un número reservado del sistema".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_tipo(tipo_str: &str) -> Result<TipoGafete, GafeteError> {
    tipo_str
        .parse()
        .map_err(|_| GafeteError::InvalidType(tipo_str.to_string()))
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

pub fn validar_create_input(input: &CreateGafeteInput) -> Result<(), GafeteError> {
    validar_numero(&input.numero)?;
    validar_tipo(&input.tipo)?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateGafeteInput) -> Result<(), GafeteError> {
    if let Some(ref tipo) = input.tipo {
        validar_tipo(tipo)?;
    }
    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

pub fn normalizar_numero(numero: &str) -> String {
    numero.trim().to_uppercase()
}
