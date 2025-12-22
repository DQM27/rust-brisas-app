// ==========================================
// src/domain/lista_negra.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{AddToListaNegraInput, UpdateListaNegraInput};
use chrono::NaiveDateTime;

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_cedula(cedula: &str) -> Result<(), ListaNegraError> {
    let limpia = cedula.trim();

    if limpia.is_empty() {
        return Err(ListaNegraError::Validation("La cédula no puede estar vacía".to_string()));
    }

    if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
        return Err(ListaNegraError::Validation(
            "La cédula solo puede contener números y guiones".to_string(),
        ));
    }

    if limpia.len() < 7 || limpia.len() > 20 {
        return Err(ListaNegraError::Validation(
            "La cédula debe tener entre 7 y 20 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_nombre(nombre: &str) -> Result<(), ListaNegraError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(ListaNegraError::Validation("El nombre no puede estar vacío".to_string()));
    }

    if limpio.len() > 100 {
        return Err(ListaNegraError::Validation(
            "El nombre no puede exceder 100 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_motivo(motivo: &str) -> Result<(), ListaNegraError> {
    let limpio = motivo.trim();

    if limpio.is_empty() {
        return Err(ListaNegraError::Validation(
            "Debe especificar un motivo de bloqueo".to_string(),
        ));
    }

    if limpio.len() > 500 {
        return Err(ListaNegraError::Validation(
            "El motivo no puede exceder 500 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_bloqueado_por(bloqueado_por: &str) -> Result<(), ListaNegraError> {
    let limpio = bloqueado_por.trim();

    if limpio.is_empty() {
        return Err(ListaNegraError::Validation(
            "Debe especificar quién realizó el bloqueo".to_string(),
        ));
    }

    if limpio.len() > 100 {
        return Err(ListaNegraError::Validation(
            "El nombre de quien bloqueó no puede exceder 100 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_fecha_fin(fecha_str: &str) -> Result<NaiveDateTime, ListaNegraError> {
    NaiveDateTime::parse_from_str(fecha_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| ListaNegraError::DateParse(fecha_str.to_string()))
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

/// Valida todos los campos necesarios para agregar a lista negra
pub fn validar_add_input(input: &AddToListaNegraInput) -> Result<(), ListaNegraError> {
    // Si tiene contratista_id, no necesita cédula/nombre/apellido obligatoriamente, pero si vienen se validan??
    // Asumimos lógica original: si contratista_id -> solo valida motivo/bloqueado_por
    if input.contratista_id.is_some() {
        validar_motivo(&input.motivo_bloqueo)?;
        validar_bloqueado_por(&input.bloqueado_por)?;

        if let Some(ref fecha) = input.fecha_fin_bloqueo {
            validar_fecha_fin(fecha)?;
        }

        return Ok(());
    }

    // Si NO tiene contratista_id, requiere cédula + nombre + apellido
    let cedula = input.cedula.as_ref().ok_or(ListaNegraError::Validation(
        "Debe proporcionar cédula si no especifica contratista_id".to_string(),
    ))?;
    validar_cedula(cedula)?;

    let nombre = input.nombre.as_ref().ok_or(ListaNegraError::Validation(
        "Debe proporcionar nombre si no especifica contratista_id".to_string(),
    ))?;
    validar_nombre(nombre)?;

    // Apellido también obligatorio? Asumimos que sí por lógica anterior inferida
    let apellido = input.apellido.as_ref().ok_or(ListaNegraError::Validation(
        "Debe proporcionar apellido si no especifica contratista_id".to_string(),
    ))?;
    validar_nombre(apellido)?;

    validar_motivo(&input.motivo_bloqueo)?;
    validar_bloqueado_por(&input.bloqueado_por)?;

    if let Some(ref fecha) = input.fecha_fin_bloqueo {
        validar_fecha_fin(fecha)?;
    }

    Ok(())
}

/// Valida los campos presentes en un update (solo los que no son None)
pub fn validar_update_input(input: &UpdateListaNegraInput) -> Result<(), ListaNegraError> {
    if let Some(ref motivo) = input.motivo_bloqueo {
        validar_motivo(motivo)?;
    }

    if let Some(ref fecha) = input.fecha_fin_bloqueo {
        validar_fecha_fin(fecha)?;
    }

    Ok(())
}

// ==========================================
// HELPERS DE NORMALIZACIÓN
// ==========================================

/// Normaliza texto genérico (trim)
pub fn normalizar_texto(texto: &str) -> String {
    texto.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cedula() {
        assert!(validar_cedula("1234567890").is_ok());
        assert!(validar_cedula("123-456-789").is_ok());
        assert!(validar_cedula("").is_err());
        assert!(validar_cedula("123").is_err());
        assert!(validar_cedula("abc123").is_err());
    }

    #[test]
    fn test_validar_nombre() {
        assert!(validar_nombre("Juan").is_ok());
        assert!(validar_nombre("  María  ").is_ok());
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre(&"a".repeat(101)).is_err());
    }

    #[test]
    fn test_validar_motivo() {
        assert!(validar_motivo("Comportamiento indebido").is_ok());
        assert!(validar_motivo("").is_err());
        assert!(validar_motivo(&"a".repeat(501)).is_err());
    }
}
