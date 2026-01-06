//! # Dominio: Reglas de Negocio para Vehículos
//!
//! Este módulo centraliza las políticas de validación, normalización de placas
//! y reglas de integridad para los activos móviles que transitan la planta.
use crate::domain::common::{
    validar_placa_estandar, MAX_LEN_COLOR_VEHICULO, MAX_LEN_MARCA_VEHICULO, MAX_LEN_MODELO_VEHICULO,
};
use crate::domain::errors::VehiculoError;
use crate::models::vehiculo::{CreateVehiculoInput, TipoVehiculo, UpdateVehiculoInput};

// --------------------------------------------------------------------------
// CONSTANTES DE DOMINIO
// --------------------------------------------------------------------------

/// Etiqueta para el campo de Modelo en mensajes de error.
const CAMPO_MODELO: &str = "Modelo";

/// Etiqueta para el campo de Color en mensajes de error.
const CAMPO_COLOR: &str = "Color";

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida que se haya asignado un propietario (Persona o Empresa) al vehículo.
///
/// # Argumentos
/// * `propietario_id` - Identificador del propietario (ej: "contratista:123")
///
/// # Errores
/// * `VehiculoError::Validation` - Si el ID está vacío
pub fn validar_propietario_id(propietario_id: &str) -> Result<(), VehiculoError> {
    let limpio = propietario_id.trim();

    if limpio.is_empty() {
        return Err(VehiculoError::Validation("Debe especificar un propietario".to_string()));
    }

    Ok(())
}

/// Parsea y valida el tipo de vehículo contra el enumerado oficial.
///
/// # Proceso
/// - Intenta convertir el string al enumerado `TipoVehiculo`.
/// - Soporta "motocicleta" y "automovil/automóvil" (insensible a mayúsculas).
///
/// # Errores
/// * `VehiculoError::InvalidType` - Si el tipo no es reconocido
pub fn validar_tipo_vehiculo(tipo_str: &str) -> Result<TipoVehiculo, VehiculoError> {
    tipo_str.parse().map_err(|_| VehiculoError::InvalidType(tipo_str.to_string()))
}

/// Valida el formato de la placa (matrícula) del vehículo.
///
/// Usa el estándar general de `common.rs` para asegurar consistencia.
///
/// # Errores
/// * `VehiculoError::Validation` - Si la placa no cumple con el formato alfanumérico estandarizado.
pub fn validar_placa(placa: &str) -> Result<(), VehiculoError> {
    validar_placa_estandar(placa).map_err(|e| VehiculoError::Validation(e.to_string()))
}

/// Valida la marca del vehículo.
///
/// Asegura que no esté vacía y que no exceda los límites de longitud parametrizados.
///
/// # Errores
/// * `VehiculoError::Validation` - Si está vacía o excede `MAX_LEN_MARCA_VEHICULO`.
pub fn validar_marca(marca: &str) -> Result<(), VehiculoError> {
    let limpia = marca.trim();

    if limpia.is_empty() {
        return Err(VehiculoError::Validation("La marca no puede estar vacía".to_string()));
    }

    if limpia.len() > MAX_LEN_MARCA_VEHICULO {
        return Err(VehiculoError::Validation(format!(
            "La marca no puede exceder {MAX_LEN_MARCA_VEHICULO} caracteres"
        )));
    }

    Ok(())
}

/// Valida la longitud de campos de texto opcionales.
pub fn validar_texto_opcional(
    texto: &str,
    campo: &str,
    max_len: usize,
) -> Result<(), VehiculoError> {
    let limpio = texto.trim();

    if limpio.len() > max_len {
        return Err(VehiculoError::Validation(format!(
            "{campo} no puede exceder {max_len} caracteres"
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
        validar_texto_opcional(modelo, CAMPO_MODELO, MAX_LEN_MODELO_VEHICULO)?;
    }

    if let Some(ref color) = input.color {
        validar_texto_opcional(color, CAMPO_COLOR, MAX_LEN_COLOR_VEHICULO)?;
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
        validar_texto_opcional(modelo, CAMPO_MODELO, MAX_LEN_MODELO_VEHICULO)?;
    }

    if let Some(ref color) = input.color {
        validar_texto_opcional(color, CAMPO_COLOR, MAX_LEN_COLOR_VEHICULO)?;
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

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Tests de validación de propietario
    // -----------------------------------------------------------------------

    #[test]
    fn test_propietario_id_valido() {
        assert!(validar_propietario_id("contratista:123").is_ok());
        assert!(validar_propietario_id("proveedor:abc").is_ok());
    }

    #[test]
    fn test_propietario_id_vacio() {
        assert!(validar_propietario_id("").is_err());
        assert!(validar_propietario_id("   ").is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de validación de tipo de vehículo
    // -----------------------------------------------------------------------

    #[test]
    fn test_tipo_vehiculo_valido() {
        assert!(validar_tipo_vehiculo("motocicleta").is_ok());
        assert!(validar_tipo_vehiculo("automovil").is_ok());
        assert!(validar_tipo_vehiculo("Automóvil").is_ok());
    }

    #[test]
    fn test_tipo_vehiculo_invalido() {
        assert!(validar_tipo_vehiculo("camion").is_err());
        assert!(validar_tipo_vehiculo("").is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de validación de placa
    // -----------------------------------------------------------------------

    #[test]
    fn test_placa_valida() {
        assert!(validar_placa("ABC-123").is_ok());
        assert!(validar_placa("XY123Z").is_ok());
    }

    #[test]
    fn test_placa_vacia() {
        assert!(validar_placa("").is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de validación de marca
    // -----------------------------------------------------------------------

    #[test]
    fn test_marca_valida() {
        assert!(validar_marca("Toyota").is_ok());
        assert!(validar_marca("Mercedes-Benz").is_ok());
    }

    #[test]
    fn test_marca_vacia() {
        assert!(validar_marca("").is_err());
    }

    #[test]
    fn test_marca_muy_larga() {
        let marca_larga = "A".repeat(MAX_LEN_MARCA_VEHICULO + 1);
        assert!(validar_marca(&marca_larga).is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de normalización
    // -----------------------------------------------------------------------

    #[test]
    fn test_normalizar_placa() {
        assert_eq!(normalizar_placa("  abc-123  "), "ABC-123");
        assert_eq!(normalizar_placa("xy 456 z"), "XY 456 Z");
    }

    #[test]
    fn test_normalizar_texto() {
        assert_eq!(normalizar_texto("  Toyota  "), "Toyota");
    }
}
