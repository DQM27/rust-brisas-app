// ==========================================
// src/domain/empresa.rs
// ==========================================

use crate::domain::errors::EmpresaError;
use crate::models::empresa::{CreateEmpresaInput, UpdateEmpresaInput};

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_nombre(nombre: &str) -> Result<(), EmpresaError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(EmpresaError::Validation(
            "El nombre de la empresa no puede estar vacío".to_string(),
        ));
    }

    if limpio.len() > 100 {
        return Err(EmpresaError::Validation(
            "El nombre no puede exceder 100 caracteres".to_string(),
        ));
    }

    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

pub fn validar_create_input(input: &CreateEmpresaInput) -> Result<(), EmpresaError> {
    validar_nombre(&input.nombre)?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateEmpresaInput) -> Result<(), EmpresaError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }
    Ok(())
}

// ==========================================
// NORMALIZACIONES
// ==========================================

pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalizar_nombre() {
        assert_eq!(normalizar_nombre("  Empresa ABC  "), "Empresa ABC");
    }
}
