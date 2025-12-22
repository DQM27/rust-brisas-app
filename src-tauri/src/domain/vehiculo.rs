// ==========================================
// src/domain/vehiculo.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::VehiculoError;
use crate::models::vehiculo::{CreateVehiculoInput, TipoVehiculo, UpdateVehiculoInput};

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_contratista_id(contratista_id: &str) -> Result<(), VehiculoError> {
    let limpio = contratista_id.trim();

    if limpio.is_empty() {
        return Err(VehiculoError::Validation(
            "Debe especificar un contratista".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_tipo_vehiculo(tipo_str: &str) -> Result<TipoVehiculo, VehiculoError> {
    tipo_str
        .parse()
        .map_err(|_| VehiculoError::InvalidType(tipo_str.to_string()))
}

pub fn validar_placa(placa: &str) -> Result<(), VehiculoError> {
    let limpia = placa.trim().to_uppercase();

    if limpia.is_empty() {
        return Err(VehiculoError::Validation(
            "La placa no puede estar vacía".to_string(),
        ));
    }

    // Validación flexible: solo alfanuméricos, guiones y espacios
    if !limpia
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == ' ')
    {
        return Err(VehiculoError::Validation(
            "La placa solo puede contener letras, números, guiones y espacios".to_string(),
        ));
    }

    if limpia.len() < 3 || limpia.len() > 15 {
        return Err(VehiculoError::Validation(
            "La placa debe tener entre 3 y 15 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_marca(marca: &str) -> Result<(), VehiculoError> {
    let limpia = marca.trim();

    if limpia.is_empty() {
        return Err(VehiculoError::Validation(
            "La marca no puede estar vacía".to_string(),
        ));
    }

    if limpia.len() > 50 {
        return Err(VehiculoError::Validation(
            "La marca no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_texto_opcional(
    texto: &str,
    campo: &str,
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

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

/// Valida todos los campos necesarios para crear un vehículo
pub fn validar_create_input(input: &CreateVehiculoInput) -> Result<(), VehiculoError> {
    validar_contratista_id(&input.contratista_id)?;
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

/// Valida los campos presentes en un update (solo los que no son None)
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

// ==========================================
// HELPERS DE NORMALIZACIÓN
// ==========================================

/// Normaliza una placa (trim + uppercase)
pub fn normalizar_placa(placa: &str) -> String {
    placa.trim().to_uppercase()
}

/// Normaliza texto genérico (trim)
pub fn normalizar_texto(texto: &str) -> String {
    texto.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_tipo_vehiculo() {
        assert!(validar_tipo_vehiculo("motocicleta").is_ok());
        assert!(validar_tipo_vehiculo("automóvil").is_ok());
        assert!(validar_tipo_vehiculo("automovil").is_ok()); // sin tilde
        assert!(validar_tipo_vehiculo("camion").is_err());
    }

    #[test]
    fn test_validar_placa() {
        assert!(validar_placa("ABC-123").is_ok());
        assert!(validar_placa("  abc123  ").is_ok());
        assert!(validar_placa("AB").is_err()); // muy corta
        assert!(validar_placa("").is_err());
        assert!(validar_placa("ABC@123").is_err()); // caracter inválido
    }

    #[test]
    fn test_normalizar_placa() {
        assert_eq!(normalizar_placa("  abc-123  "), "ABC-123");
        assert_eq!(normalizar_placa("xyz789"), "XYZ789");
    }

    #[test]
    fn test_validar_marca() {
        assert!(validar_marca("Toyota").is_ok());
        assert!(validar_marca("").is_err());
        assert!(validar_marca(&"a".repeat(51)).is_err());
    }
}
