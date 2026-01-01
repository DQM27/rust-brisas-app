/// Capa de Dominio: Reglas de Negocio para Contratistas.
///
/// Este módulo define las validaciones y lógicas puras aplicables a los
/// contratistas externos. Estas reglas aseguran la integridad de los datos
/// de filiación y laborales antes de su almacenamiento en la base de datos.
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CreateContratistaInput, EstadoContratista, UpdateContratistaInput,
};
use chrono::NaiveDate;

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud de la cédula de identidad.
pub fn validar_cedula(cedula: &str) -> Result<(), ContratistaError> {
    let limpia = cedula.trim();

    if limpia.is_empty() {
        return Err(ContratistaError::Validation("La cédula no puede estar vacía".to_string()));
    }

    if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
        return Err(ContratistaError::Validation(
            "La cédula solo puede contener números y guiones".to_string(),
        ));
    }

    if limpia.len() < 7 || limpia.len() > 20 {
        return Err(ContratistaError::Validation(
            "La cédula debe tener entre 7 y 20 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida los requisitos mínimos del nombre.
pub fn validar_nombre(nombre: &str) -> Result<(), ContratistaError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(ContratistaError::Validation("El nombre no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(ContratistaError::Validation(
            "El nombre no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida los requisitos mínimos del apellido.
pub fn validar_apellido(apellido: &str) -> Result<(), ContratistaError> {
    let limpio = apellido.trim();

    if limpio.is_empty() {
        return Err(ContratistaError::Validation("El apellido no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(ContratistaError::Validation(
            "El apellido no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida que el ID de la empresa vinculada sea válido.
pub fn validar_empresa_id(empresa_id: &str) -> Result<(), ContratistaError> {
    let limpia = empresa_id.trim();

    if limpia.is_empty() {
        return Err(ContratistaError::Validation(
            "Debe seleccionar una empresa válida".to_string(),
        ));
    }

    Ok(())
}

/// Parsea y valida una fecha en formato estándar (YYYY-MM-DD).
pub fn validar_fecha(fecha_str: &str) -> Result<NaiveDate, ContratistaError> {
    NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d").map_err(|_| {
        ContratistaError::Validation("Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    })
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida el conjunto completo de datos para la creación de un contratista.
pub fn validar_create_input(input: &CreateContratistaInput) -> Result<(), ContratistaError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_apellido(&input.apellido)?;
    validar_empresa_id(&input.empresa_id)?;
    validar_fecha(&input.fecha_vencimiento_praind)?;
    Ok(())
}

/// Valida los cambios parciales solicitados en una actualización.
pub fn validar_update_input(input: &UpdateContratistaInput) -> Result<(), ContratistaError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }

    if let Some(ref apellido) = input.apellido {
        validar_apellido(apellido)?;
    }

    if let Some(ref empresa_id) = input.empresa_id {
        validar_empresa_id(empresa_id)?;
    }

    if let Some(ref fecha) = input.fecha_vencimiento_praind {
        validar_fecha(fecha)?;
    }

    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Limpia y normaliza el texto para su persistencia.
pub fn normalizar_texto(texto: &str) -> String {
    texto.trim().to_string()
}

/// Normaliza una cédula eliminando espacios y convirtiéndola a mayúsculas.
pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_uppercase()
}

/// Valida que el estado del contratista sea uno de los valores permitidos.
pub fn validar_estado(estado: &str) -> Result<(), ContratistaError> {
    estado
        .parse::<EstadoContratista>()
        .map_err(|_| ContratistaError::Validation(format!("Estado inválido: {}", estado)))?;
    Ok(())
}
