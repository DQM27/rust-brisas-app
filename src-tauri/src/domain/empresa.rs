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

