// ==========================================
// src/domain/empresa.rs
// ==========================================

use crate::models::empresa::{CreateEmpresaInput, UpdateEmpresaInput};

// ==========================================
// VALIDACIONES DE CAMPOS
// ==========================================

pub fn validar_nombre(nombre: &str) -> Result<(), String> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err("El nombre de la empresa no puede estar vacío".to_string());
    }

    if limpio.len() < 2 {
        return Err("El nombre debe tener al menos 2 caracteres".to_string());
    }

    if limpio.len() > 100 {
        return Err("El nombre no puede exceder 100 caracteres".to_string());
    }

    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS
// ==========================================

pub fn validar_create_input(input: &CreateEmpresaInput) -> Result<(), String> {
    validar_nombre(&input.nombre)?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateEmpresaInput) -> Result<(), String> {
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
    fn test_validar_nombre() {
        assert!(validar_nombre("Empresa ABC").is_ok());
        assert!(validar_nombre("AB").is_ok());
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre("A").is_err());
    }

    #[test]
    fn test_normalizar_nombre() {
        assert_eq!(normalizar_nombre("  Empresa ABC  "), "Empresa ABC");
    }
}