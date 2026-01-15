/// Capa de Dominio: Reglas de Negocio para la Gestión de Citas.
///
/// Este módulo centraliza la lógica de validación pura para las citas programadas.
/// Al pertenecer a la capa de dominio, no tiene dependencias de base de datos,
/// enfocándose exclusivamente en asegurar que los datos de entrada cumplan con los
/// requerimientos operativos del sistema antes de ser procesados por los servicios.
use crate::domain::common::{
    validar_cedula_estandar, validar_fecha_rfc3339, validar_nombre_entidad_estandar,
    validar_nombre_estandar,
};
use crate::domain::errors::CitaError;
use crate::models::cita::CreateCitaInput;

// --------------------------------------------------------------------------
// CONSTANTES DE VALIDACIÓN
// --------------------------------------------------------------------------

/// Longitud máxima para el nombre del anfitrión.
pub const ANFITRION_MAX_LEN: usize = 100;

/// Longitud máxima para el área visitada.
pub const AREA_MAX_LEN: usize = 100;

/// Longitud máxima para el motivo de la visita.
pub const MOTIVO_MAX_LEN: usize = 255;

// --------------------------------------------------------------------------
// VALIDACIONES DE NEGOCIO
// --------------------------------------------------------------------------

/// Verifica la integridad de los datos para la programación de una nueva cita.
///
/// # Argumentos
/// * `input` - Estructura con los datos de entrada para la creación de la cita.
///
/// # Retorno
/// Retorna `Ok(())` si todas las validaciones pasan, o `CitaError::Validation`.
pub fn validar_create_input(input: &CreateCitaInput) -> Result<(), CitaError> {
    // 1. Validación de Identidad del Visitante
    if input.visitante_id.is_none() {
        if let Some(ref cedula) = input.visitante_cedula {
            validar_cedula_estandar(cedula).map_err(|e| CitaError::Validation(e.to_string()))?;
        } else {
            return Err(CitaError::Validation(
                "Debe proporcionar un visitante registrado o la cédula para uno nuevo".to_string(),
            ));
        }

        if let Some(ref nombre) = input.visitante_nombre {
            validar_nombre_estandar(nombre, "nombre del visitante")
                .map_err(|e| CitaError::Validation(e.to_string()))?;
        } else {
            return Err(CitaError::Validation(
                "El nombre del visitante es obligatorio para registros nuevos".to_string(),
            ));
        }

        if let Some(ref apellido) = input.visitante_apellido {
            validar_nombre_estandar(apellido, "apellido del visitante")
                .map_err(|e| CitaError::Validation(e.to_string()))?;
        }
    }

    // 2. Validación de Campos Operativos (Límites y Vacío)
    validar_nombre_estandar(&input.anfitrion, "anfitrión")
        .map_err(|e| CitaError::Validation(e.to_string()))?;

    validar_nombre_entidad_estandar(&input.area_visitada, "área visitada")
        .map_err(|e| CitaError::Validation(e.to_string()))?;

    validar_nombre_entidad_estandar(&input.motivo, "motivo")
        .map_err(|e| CitaError::Validation(e.to_string()))?;

    // 3. Validación de Fecha (Estándar RFC 3339 / ISO 8601)
    validar_fecha_rfc3339(&input.fecha_cita).map_err(|e| CitaError::Validation(e.to_string()))?;

    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

