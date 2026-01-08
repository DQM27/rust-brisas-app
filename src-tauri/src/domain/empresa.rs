/// Capa de Dominio: Reglas de Negocio para Empresas.
///
/// Este módulo centraliza la lógica de validación pura para las empresas (contratistas
/// o proveedores). Al pertenecer a la capa de dominio, no tiene dependencias de
/// infraestructura, asegurando reglas de negocio consistentes y testeables.
use crate::domain::common::validar_nombre_entidad_estandar;
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{CreateEmpresaInput, UpdateEmpresaInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida los requisitos de identidad de una empresa.
///
/// # Argumentos
/// * `nombre` - El nombre comercial o razón social a validar.
///
/// # Retorno
/// Retorna `Ok(())` si cumple los criterios de longitud y formato, o `EmpresaError::Validation`.
pub fn validar_nombre(nombre: &str) -> Result<(), EmpresaError> {
    validar_nombre_entidad_estandar(nombre, "nombre de la empresa")
        .map_err(|e| EmpresaError::Validation(e.to_string()))
}

/// Valida la dirección de la empresa si se proporciona.
///
/// # Argumentos
/// * `direccion` - La dirección física de la entidad.
///
/// # Retorno
/// Retorna `Ok(())` o error de validación.
pub fn validar_direccion(direccion: &str) -> Result<(), EmpresaError> {
    validar_nombre_entidad_estandar(direccion, "dirección")
        .map_err(|e| EmpresaError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida los datos requeridos para registrar una nueva empresa.
///
/// # Argumentos
/// * `input` - DTO de creación de empresa.
///
/// # Retorno
/// `Ok(())` si el registro es válido.
pub fn validar_create_input(input: &CreateEmpresaInput) -> Result<(), EmpresaError> {
    validar_nombre(&input.nombre)?;
    if let Some(ref direccion) = input.direccion {
        validar_direccion(direccion)?;
    }
    Ok(())
}

/// Valida las modificaciones parciales de una empresa existente.
///
/// # Argumentos
/// * `input` - DTO de actualización (campos opcionales).
///
/// # Retorno
/// `Ok(())` si las modificaciones propuestas son válidas.
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
///
/// # Argumentos
/// * `nombre` - Nombre a normalizar.
///
/// # Retorno
/// Nombre trimmeado.
pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_string()
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::common::{ENTIDAD_NOMBRE_MAX_LEN, MAX_LEN_DIRECCION};

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
        let nombre_largo = "A".repeat(ENTIDAD_NOMBRE_MAX_LEN + 1);
        assert!(validar_nombre(&nombre_largo).is_err());
    }

    #[test]
    fn test_validar_nombre_limite() {
        let nombre_100_chars = "A".repeat(ENTIDAD_NOMBRE_MAX_LEN);
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
        let input = CreateEmpresaInput { nombre: String::new(), direccion: None };
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
            UpdateEmpresaInput { nombre: Some(String::new()), direccion: None, is_active: None };
        assert!(validar_update_input(&input).is_err());
    }
}
