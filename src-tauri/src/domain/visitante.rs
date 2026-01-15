//! # Dominio: Reglas de Negocio para Visitantes
//!
//! Contiene las reglas de negocio puras y validaciones para personas
//! particulares que ingresan de forma puntual a las instalaciones.
//!
//! ## Responsabilidades
//! - Validar formatos de identidad (cédulas, nombres).
//! - Normalizar datos para consistencia en búsquedas.
//! - Validar integridad de inputs de registro.

use crate::domain::common::{
    normalizar_nombre_propio, validar_cedula_estandar, validar_nombre_estandar,
};
use crate::domain::errors::VisitanteError;
use crate::models::visitante::CreateVisitanteInput;

// --------------------------------------------------------------------------
// CONSTANTES DE DOMINIO
// --------------------------------------------------------------------------

/// Etiqueta para el campo de Empresa en mensajes de error.
const CAMPO_EMPRESA: &str = "Empresa";

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud de la cédula del visitante.
///
/// # Argumentos
/// * `cedula` - Cédula en formato string.
///
/// # Errores
/// * `VisitanteError::Validation` - Si no cumple el estándar numérico/guiones.
pub fn validar_cedula(cedula: &str) -> Result<(), VisitanteError> {
    validar_cedula_estandar(cedula).map_err(|e| VisitanteError::Validation(e.to_string()))
}

/// Valida el nombre del visitante.
///
/// # Errores
/// * `VisitanteError::Validation` - Si contiene caracteres inválidos o longitud fuera de rango.
pub fn validar_nombre(nombre: &str) -> Result<(), VisitanteError> {
    validar_nombre_estandar(nombre, "nombre").map_err(|e| VisitanteError::Validation(e.to_string()))
}

/// Valida el apellido del visitante.
///
/// # Errores
/// * `VisitanteError::Validation` - Si contiene caracteres inválidos o longitud fuera de rango.
pub fn validar_apellido(apellido: &str) -> Result<(), VisitanteError> {
    validar_nombre_estandar(apellido, "apellido")
        .map_err(|e| VisitanteError::Validation(e.to_string()))
}

/// Valida campos opcionales con un límite de caracteres.
pub fn validar_opcional(
    valor: Option<&String>,
    max_len: usize,
    nombre_campo: &str,
) -> Result<(), VisitanteError> {
    if let Some(v) = valor {
        if v.trim().len() > max_len {
            return Err(VisitanteError::Validation(format!(
                "{nombre_campo} no puede exceder {max_len} caracteres"
            )));
        }
    }
    Ok(())
}

/// Valida un nombre opcional usando el estándar (si existe).
pub fn validar_nombre_opcional(
    valor: Option<&String>,
    nombre_campo: &str,
) -> Result<(), VisitanteError> {
    if let Some(v) = valor {
        // Ignorar si está vacío después de trim (se normalizará a None luego)
        if !v.trim().is_empty() {
            validar_nombre_estandar(v, nombre_campo)
                .map_err(|e| VisitanteError::Validation(e.to_string()))?;
        }
    }
    Ok(())
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida el conjunto completo de datos para un nuevo visitante.
///
/// # Proceso
/// 1. Valida identidad básica (Cédula, Nombre, Apellido).
/// 2. Valida componentes opcionales del nombre si están presentes.
/// 3. Verifica asociación obligatoria a una empresa.
///
/// # Errores
/// * `VisitanteError::Validation` - Si algún campo falla las reglas de integridad.
pub fn validar_create_input(input: &CreateVisitanteInput) -> Result<(), VisitanteError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_apellido(&input.apellido)?;

    validar_nombre_opcional(input.segundo_nombre.as_ref(), "Segundo nombre")?;
    validar_nombre_opcional(input.segundo_apellido.as_ref(), "Segundo apellido")?;

    if input.empresa_id.trim().is_empty() {
        return Err(VisitanteError::Validation(format!(
            "Debe seleccionar una {CAMPO_EMPRESA} válida"
        )));
    }

    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Limpia espacios redundantes en nombres.
pub fn normalizar_nombre(nombre: &str) -> String {
    normalizar_nombre_propio(nombre)
}

/// Normaliza la cédula a mayúsculas para comparaciones consistentes.
pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_uppercase()
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

