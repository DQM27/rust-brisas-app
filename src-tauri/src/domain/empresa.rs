/// Capa de Dominio: Reglas de Negocio para Empresas.
///
/// Este módulo gestiona la integridad de los datos de las empresas (contratistas
/// o proveedores) registradas en el sistema.
use crate::domain::common::validar_nombre_entidad_estandar;
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{CreateEmpresaInput, UpdateEmpresaInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida los requisitos de identidad de una empresa.
pub fn validar_nombre(nombre: &str) -> Result<(), EmpresaError> {
    validar_nombre_entidad_estandar(nombre, "nombre de la empresa")
        .map_err(|e| EmpresaError::Validation(e.to_string()))
}

/// Valida la dirección de la empresa si se proporciona.
pub fn validar_direccion(direccion: &str) -> Result<(), EmpresaError> {
    validar_nombre_entidad_estandar(direccion, "dirección")
        .map_err(|e| EmpresaError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida los datos requeridos para registrar una nueva empresa.
pub fn validar_create_input(input: &CreateEmpresaInput) -> Result<(), EmpresaError> {
    validar_nombre(&input.nombre)?;
    if let Some(ref direccion) = input.direccion {
        validar_direccion(direccion)?;
    }
    Ok(())
}

/// Valida las modificaciones parciales de una empresa existente.
pub fn validar_update_input(input: &UpdateEmpresaInput) -> Result<(), EmpresaError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }
    if let Some(ref direccion) = input.direccion {
        validar_direccion(direccion)?;
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
        assert!(validar_nombre("AB").is_ok()); // Mínimo 2 caracteres
    }

    #[test]
    fn test_validar_nombre_vacio() {
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre("   ").is_err());
    }

    #[test]
    fn test_validar_nombre_muy_largo() {
        let nombre_largo = "A".repeat(MAX_LEN_EMPRESA + 1);
        assert!(validar_nombre(&nombre_largo).is_err());
    }

    #[test]
    fn test_validar_nombre_limite() {
        let nombre_100_chars = "A".repeat(MAX_LEN_EMPRESA);
        assert!(validar_nombre(&nombre_100_chars).is_ok());
    }

    #[test]
    fn test_normalizar_nombre() {
        assert_eq!(normalizar_nombre("  Empresa XYZ  "), "Empresa XYZ");
        assert_eq!(normalizar_nombre("ABC"), "ABC");
        assert_eq!(normalizar_nombre("   "), "");
    }

    #[test]
    fn test_validar_nombre_chars_prohibidos() {
        assert!(validar_nombre("Empresa<script>").is_err());
        assert!(validar_nombre("Corp{json}").is_err());
        assert!(validar_nombre("Company|pipe").is_err());
    }

    #[test]
    fn test_validar_direccion_valida() {
        assert!(validar_direccion("Calle Principal 123").is_ok());
        assert!(validar_direccion("Av. Libertador, Piso 5, Oficina 5-A").is_ok());
    }

    #[test]
    fn test_validar_direccion_muy_larga() {
        let direccion_larga = "A".repeat(MAX_LEN_DIRECCION + 1);
        assert!(validar_direccion(&direccion_larga).is_err());
    }

    #[test]
    fn test_validar_direccion_chars_prohibidos() {
        assert!(validar_direccion("Calle <script>").is_err());
        assert!(validar_direccion("Direccion{malware}").is_err());
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
