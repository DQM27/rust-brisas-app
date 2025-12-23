// ==========================================
// src/domain/proveedor.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::ProveedorError;
use crate::models::proveedor::{CreateProveedorInput, UpdateProveedorInput};

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_cedula(cedula: &str) -> Result<(), ProveedorError> {
    let limpia = cedula.trim();

    if limpia.is_empty() {
        return Err(ProveedorError::Validation("La cédula no puede estar vacía".to_string()));
    }

    if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
        return Err(ProveedorError::Validation(
            "La cédula solo puede contener números y guiones".to_string(),
        ));
    }

    if limpia.len() < 7 || limpia.len() > 20 {
        return Err(ProveedorError::Validation(
            "La cédula debe tener entre 7 y 20 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_nombre(nombre: &str) -> Result<(), ProveedorError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(ProveedorError::Validation("El nombre no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(ProveedorError::Validation(
            "El nombre no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_segundo_nombre(segundo_nombre: Option<&String>) -> Result<(), ProveedorError> {
    if let Some(nombre) = segundo_nombre {
        let limpio = nombre.trim();

        if !limpio.is_empty() && limpio.len() > 50 {
            return Err(ProveedorError::Validation(
                "El segundo nombre no puede exceder 50 caracteres".to_string(),
            ));
        }
    }

    Ok(())
}

pub fn validar_apellido(apellido: &str) -> Result<(), ProveedorError> {
    let limpio = apellido.trim();

    if limpio.is_empty() {
        return Err(ProveedorError::Validation("El apellido no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(ProveedorError::Validation(
            "El apellido no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_segundo_apellido(segundo_apellido: Option<&String>) -> Result<(), ProveedorError> {
    if let Some(apellido) = segundo_apellido {
        let limpio = apellido.trim();

        if !limpio.is_empty() && limpio.len() > 50 {
            return Err(ProveedorError::Validation(
                "El segundo apellido no puede exceder 50 caracteres".to_string(),
            ));
        }
    }

    Ok(())
}

pub fn validar_empresa_id(empresa_id: &str) -> Result<(), ProveedorError> {
    let limpia = empresa_id.trim();

    if limpia.is_empty() {
        return Err(ProveedorError::Validation("Debe seleccionar una empresa".to_string()));
    }

    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

pub fn validar_create_input(input: &CreateProveedorInput) -> Result<(), ProveedorError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_segundo_nombre(input.segundo_nombre.as_ref())?;
    validar_apellido(&input.apellido)?;
    validar_segundo_apellido(input.segundo_apellido.as_ref())?;
    validar_empresa_id(&input.empresa_id)?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateProveedorInput) -> Result<(), ProveedorError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cedula_valida() {
        assert!(validar_cedula("1-234-567").is_ok());
    }

    #[test]
    fn test_validar_cedula_invalida_chars() {
        assert!(validar_cedula("123abc").is_err());
    }

    #[test]
    fn test_validar_nombre_valido() {
        assert!(validar_nombre("Juan").is_ok());
    }

    #[test]
    fn test_validar_nombre_vacio() {
        assert!(validar_nombre("   ").is_err());
    }
}
