/// Capa de Dominio: Gestión de Ingresos de Visitantes Particulares.
///
/// Este módulo define las reglas de negocio para el control de acceso y salida
/// de personas particulares (visitas personales, anfitriones, etc.) a las instalaciones.
use crate::domain::errors::IngresoVisitaError;
use crate::models::ingreso::visita::CreateIngresoVisitaInput;

// Re-exportaciones para mantener consistencia con la capa de dominio
pub use crate::domain::common::{
    evaluar_devolucion_gafete, normalizar_gafete_a_int, DecisionReporteGafete,
};
pub use crate::models::ingreso::visita::{IngresoVisitaPopulated, ValidacionIngresoVisitaResponse};

// Importación de lógica compartida
use crate::domain::common as common_domain;

// --------------------------------------------------------------------------
// VALIDACIONES DE ESTADO
// --------------------------------------------------------------------------

/// Valida que el registro de ingreso no tenga ya una fecha de salida asignada.
pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), IngresoVisitaError> {
    common_domain::validar_ingreso_abierto(fecha_salida)
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// WRAPPERS DE LÓGICA COMPARTIDA (Adaptación de Errores)
// --------------------------------------------------------------------------

/// Valida que la fecha de salida sea posterior a la de ingreso.
pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), IngresoVisitaError> {
    common_domain::validar_tiempo_salida(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))
}

/// Calcula el tiempo total de estancia en minutos.
pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, IngresoVisitaError> {
    common_domain::calcular_tiempo_permanencia(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida que el input de creación sea coherente.
pub fn validar_create_input(input: &CreateIngresoVisitaInput) -> Result<(), IngresoVisitaError> {
    if input.cedula.trim().is_empty() {
        return Err(IngresoVisitaError::Validation("La cédula es obligatoria".to_string()));
    }
    if input.nombre.trim().is_empty() {
        return Err(IngresoVisitaError::Validation("El nombre es obligatorio".to_string()));
    }
    if input.apellido.trim().is_empty() {
        return Err(IngresoVisitaError::Validation("El apellido es obligatorio".to_string()));
    }
    if input.anfitrion.trim().is_empty() {
        return Err(IngresoVisitaError::Validation("El anfitrión es obligatorio".to_string()));
    }
    if input.area_visitada.trim().is_empty() {
        return Err(IngresoVisitaError::Validation("El área visitada es obligatoria".to_string()));
    }
    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

