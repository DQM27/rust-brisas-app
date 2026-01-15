/// Capa de Dominio: Gestión de Ingresos de Proveedores.
///
/// Este módulo define las reglas de negocio específicas para el control de acceso
/// de proveedores de materiales y servicios a las instalaciones.
use crate::domain::errors::IngresoProveedorError;
use crate::models::ingreso::proveedor::CreateIngresoProveedorInput;

// Re-exportaciones de estructuras
pub use crate::domain::common::{
    evaluar_devolucion_gafete, normalizar_gafete_a_int, DecisionReporteGafete,
};
pub use crate::models::ingreso::proveedor::ValidacionIngresoProveedorResponse;

// Importación de lógica compartida
use crate::domain::common as common_domain;

// --------------------------------------------------------------------------
// VALIDACIONES DE ESTADO
// --------------------------------------------------------------------------

/// Valida que el registro de ingreso no tenga ya una fecha de salida asignada.
pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), IngresoProveedorError> {
    common_domain::validar_ingreso_abierto(fecha_salida)
        .map_err(|e| IngresoProveedorError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// WRAPPERS DE LÓGICA COMPARTIDA (Adaptación de Errores)
// --------------------------------------------------------------------------

/// Valida que la fecha de salida sea posterior a la de ingreso.
pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), IngresoProveedorError> {
    common_domain::validar_tiempo_salida(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoProveedorError::Validation(e.to_string()))
}

/// Calcula el tiempo total de estancia en minutos.
pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, IngresoProveedorError> {
    common_domain::calcular_tiempo_permanencia(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoProveedorError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida que el input de creación sea coherente.
pub fn validar_create_input(
    input: &CreateIngresoProveedorInput,
) -> Result<(), IngresoProveedorError> {
    if input.cedula.trim().is_empty() {
        return Err(IngresoProveedorError::Validation("La cédula es obligatoria".to_string()));
    }
    if input.nombre.trim().is_empty() {
        return Err(IngresoProveedorError::Validation("El nombre es obligatorio".to_string()));
    }
    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

