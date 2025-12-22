// ==========================================
// src/domain/contratista.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CreateContratistaInput, EstadoContratista, UpdateContratistaInput,
};
use chrono::NaiveDate;

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_cedula(cedula: &str) -> Result<(), ContratistaError> {
    let limpia = cedula.trim();

    if limpia.is_empty() {
        return Err(ContratistaError::Validation(
            "La cédula no puede estar vacía".to_string(),
        ));
    }

    if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
        return Err(ContratistaError::Validation(
            "La cédula solo puede contener números y guiones".to_string(),
        ));
    }

    if limpia.len() < 7 || limpia.len() > 20 {
        return Err(ContratistaError::Validation(
            "La cédula debe tener entre 7 y 20 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_nombre(nombre: &str) -> Result<(), ContratistaError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(ContratistaError::Validation(
            "El nombre no puede estar vacío".to_string(),
        ));
    }

    if limpio.len() > 50 {
        return Err(ContratistaError::Validation(
            "El nombre no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_segundo_nombre(segundo_nombre: Option<&String>) -> Result<(), ContratistaError> {
    if let Some(nombre) = segundo_nombre {
        let limpio = nombre.trim();

        if !limpio.is_empty() && limpio.len() > 50 {
            return Err(ContratistaError::Validation(
                "El segundo nombre no puede exceder 50 caracteres".to_string(),
            ));
        }
    }

    Ok(())
}

pub fn validar_apellido(apellido: &str) -> Result<(), ContratistaError> {
    let limpio = apellido.trim();

    if limpio.is_empty() {
        return Err(ContratistaError::Validation(
            "El apellido no puede estar vacío".to_string(),
        ));
    }

    if limpio.len() > 50 {
        return Err(ContratistaError::Validation(
            "El apellido no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_segundo_apellido(segundo_apellido: Option<&String>) -> Result<(), ContratistaError> {
    if let Some(apellido) = segundo_apellido {
        let limpio = apellido.trim();

        if !limpio.is_empty() && limpio.len() > 50 {
            return Err(ContratistaError::Validation(
                "El segundo apellido no puede exceder 50 caracteres".to_string(),
            ));
        }
    }

    Ok(())
}

pub fn validar_empresa_id(empresa_id: &str) -> Result<(), ContratistaError> {
    let limpia = empresa_id.trim();

    if limpia.is_empty() {
        return Err(ContratistaError::Validation(
            "Debe seleccionar una empresa".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_fecha(fecha_str: &str) -> Result<NaiveDate, ContratistaError> {
    NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d").map_err(|_| {
        ContratistaError::Validation("Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    })
}

pub fn validar_estado(estado_str: &str) -> Result<EstadoContratista, ContratistaError> {
    estado_str
        .parse()
        .map_err(|_| ContratistaError::Validation(format!("Estado inválido: {}", estado_str)))
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

pub fn validar_create_input(input: &CreateContratistaInput) -> Result<(), ContratistaError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_segundo_nombre(input.segundo_nombre.as_ref())?;
    validar_apellido(&input.apellido)?;
    validar_segundo_apellido(input.segundo_apellido.as_ref())?;
    validar_empresa_id(&input.empresa_id)?;
    validar_fecha(&input.fecha_vencimiento_praind)?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateContratistaInput) -> Result<(), ContratistaError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }

    if let Some(ref segundo_nombre) = input.segundo_nombre {
        validar_segundo_nombre(Some(segundo_nombre))?;
    }

    if let Some(ref apellido) = input.apellido {
        validar_apellido(apellido)?;
    }

    if let Some(ref segundo_apellido) = input.segundo_apellido {
        validar_segundo_apellido(Some(segundo_apellido))?;
    }

    if let Some(ref empresa_id) = input.empresa_id {
        validar_empresa_id(empresa_id)?;
    }

    if let Some(ref fecha) = input.fecha_vencimiento_praind {
        validar_fecha(fecha)?;
    }

    Ok(())
}

// ==========================================
// HELPERS DE NORMALIZACIÓN
// ==========================================

pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_string()
}

pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_string()
}

pub fn normalizar_segundo_nombre(segundo_nombre: Option<&String>) -> Option<String> {
    segundo_nombre
        .map(|n| {
            let trimmed = n.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .flatten()
}

pub fn normalizar_apellido(apellido: &str) -> String {
    apellido.trim().to_string()
}

pub fn normalizar_segundo_apellido(segundo_apellido: Option<&String>) -> Option<String> {
    segundo_apellido
        .map(|a| {
            let trimmed = a.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .flatten()
}
