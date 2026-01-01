/// Capa de Dominio: Reglas de Negocio para Empresas.
///
/// Este módulo gestiona la integridad de los datos de las empresas (contratistas
/// o proveedores) registradas en el sistema.
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{CreateEmpresaInput, UpdateEmpresaInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida los requisitos de identidad de una empresa.
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

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida los datos requeridos para registrar una nueva empresa.
pub fn validar_create_input(input: &CreateEmpresaInput) -> Result<(), EmpresaError> {
    validar_nombre(&input.nombre)?;
    Ok(())
}

/// Valida las modificaciones parciales de una empresa existente.
pub fn validar_update_input(input: &UpdateEmpresaInput) -> Result<(), EmpresaError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }
    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Normaliza el nombre de la empresa eliminando espacios redundantes.
pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_string()
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_nombre_valido() {
        assert!(validar_nombre("Empresa ABC").is_ok());
        assert!(validar_nombre("A").is_ok());
    }

    #[test]
    fn test_validar_nombre_vacio() {
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre("   ").is_err());
    }

    #[test]
    fn test_validar_nombre_muy_largo() {
        let nombre_largo = "A".repeat(101);
        assert!(validar_nombre(&nombre_largo).is_err());
    }

    #[test]
    fn test_validar_nombre_limite() {
        let nombre_100_chars = "A".repeat(100);
        assert!(validar_nombre(&nombre_100_chars).is_ok());
    }

    #[test]
    fn test_normalizar_nombre() {
        assert_eq!(normalizar_nombre("  Empresa XYZ  "), "Empresa XYZ");
        assert_eq!(normalizar_nombre("ABC"), "ABC");
        assert_eq!(normalizar_nombre("   "), "");
    }

    #[test]
    fn test_validar_create_input_valido() {
        let input = CreateEmpresaInput {
            nombre: "Empresa Test".to_string(),
            direccion: Some("Calle Principal 123".to_string()),
        };
        assert!(validar_create_input(&input).is_ok());
    }

    #[test]
    fn test_validar_create_input_invalido() {
        let input = CreateEmpresaInput { nombre: "".to_string(), direccion: None };
        assert!(validar_create_input(&input).is_err());
    }

    #[test]
    fn test_validar_update_input_valido() {
        let input = UpdateEmpresaInput {
            nombre: Some("Nuevo Nombre".to_string()),
            direccion: None,
            is_active: None,
        };
        assert!(validar_update_input(&input).is_ok());
    }

    #[test]
    fn test_validar_update_input_sin_cambios() {
        let input = UpdateEmpresaInput { nombre: None, direccion: None, is_active: None };
        assert!(validar_update_input(&input).is_ok());
    }

    #[test]
    fn test_validar_update_input_invalido() {
        let input =
            UpdateEmpresaInput { nombre: Some("".to_string()), direccion: None, is_active: None };
        assert!(validar_update_input(&input).is_err());
    }
}
