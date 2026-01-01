/// Capa de Dominio: Reglas de Negocio para Vehículos.
///
/// Este módulo gestiona las validaciones de propiedad y características técnicas
/// de los vehículos que ingresan a las instalaciones.
use crate::domain::errors::VehiculoError;
use crate::models::vehiculo::{CreateVehiculoInput, TipoVehiculo, UpdateVehiculoInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida que se haya asignado un propietario (Persona o Empresa) al vehículo.
pub fn validar_propietario_id(propietario_id: &str) -> Result<(), VehiculoError> {
    let limpio = propietario_id.trim();

    if limpio.is_empty() {
        return Err(VehiculoError::Validation("Debe especificar un propietario".to_string()));
    }

    Ok(())
}

/// Parsea y valida el tipo de vehículo contra el enumerado oficial.
pub fn validar_tipo_vehiculo(tipo_str: &str) -> Result<TipoVehiculo, VehiculoError> {
    tipo_str.parse().map_err(|_| VehiculoError::InvalidType(tipo_str.to_string()))
}

/// Valida el formato de la placa (matrícula) del vehículo.
///
/// Permite caracteres alfanuméricos, guiones y espacios. Longitud entre 2 y 15.
pub fn validar_placa(placa: &str) -> Result<(), VehiculoError> {
    let limpia = placa.trim().to_uppercase();

    if limpia.is_empty() {
        return Err(VehiculoError::Validation("La placa no puede estar vacía".to_string()));
    }

    if !limpia.chars().all(|c| c.is_alphanumeric() || c == '-' || c == ' ') {
        return Err(VehiculoError::Validation(
            "La placa solo puede contener letras, números, guiones y espacios".to_string(),
        ));
    }

    if limpia.len() < 2 || limpia.len() > 15 {
        return Err(VehiculoError::Validation(
            "La placa debe tener entre 2 y 15 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida la marca del vehículo.
pub fn validar_marca(marca: &str) -> Result<(), VehiculoError> {
    let limpia = marca.trim();

    if limpia.is_empty() {
        return Err(VehiculoError::Validation("La marca no puede estar vacía".to_string()));
    }

    if limpia.len() > 50 {
        return Err(VehiculoError::Validation(
            "La marca no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida la longitud de campos de texto opcionales.
pub fn validar_texto_opcional(
    texto: &str,
    campo: &get_dbstr,
    max_len: usize,
) -> Result<(), VehiculoError> {
    let limpio = texto.trim();

    if limpio.len() > max_len {
        return Err(VehiculoError::Validation(format!(
            "{} no puede exceder {} caracteres",
            campo, max_len
        )));
    }

    Ok(())
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida el conjunto completo de datos para registrar un vehículo.
pub fn validar_create_input(input: &CreateVehiculoInput) -> Result<(), VehiculoError> {
    validar_propietario_id(&input.propietario_id)?;
    validar_tipo_vehiculo(&input.tipo_vehiculo)?;
    validar_placa(&input.placa)?;

    if let Some(ref marca) = input.marca {
        if !marca.trim().is_empty() {
            validar_marca(marca)?;
        }
    }

    if let Some(ref modelo) = input.modelo {
        validar_texto_opcional(modelo, "Modelo", 50)?;
    }

    if let Some(ref color) = input.color {
        validar_texto_opcional(color, "Color", 30)?;
    }

    Ok(())
}

/// Valida las actualizaciones de datos de un vehículo.
pub fn validar_update_input(input: &UpdateVehiculoInput) -> Result<(), VehiculoError> {
    if let Some(ref tipo) = input.tipo_vehiculo {
        validar_tipo_vehiculo(tipo)?;
    }

    if let Some(ref marca) = input.marca {
        if !marca.trim().is_empty() {
            validar_marca(marca)?;
        }
    }

    if let Some(ref modelo) = input.modelo {
        validar_texto_opcional(modelo, "Modelo", 50)?;
    }

    if let Some(ref color) = input.color {
        validar_texto_opcional(color, "Color", 30)?;
    }

    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Normaliza la placa a mayúsculas y sin espacios laterales.
pub fn normalizar_placa(placa: &str) -> String {
    placa.trim().to_uppercase()
}

/// Limpia espacios redundantes en textos descriptivos.
pub fn normalizar_texto(texto: &str) -> String {
    texto.trim().to_string()
}
