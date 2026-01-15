/// Capa de Dominio: Reglas de Negocio para Gafetes.
///
/// Este módulo gestiona la validación de gafetes. A partir de ahora, el número
/// es un entero (i32), donde 0 representa "Sin Gafete" (S/G).
use crate::domain::errors::GafeteError;
use crate::models::gafete::{CreateGafeteInput, TipoGafete, UpdateGafeteInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el número de gafete.
///
/// Reglas:
/// - Debe ser mayor o igual a 0.
/// - 0 es el valor reservado para "Sin Gafete" (S/G).
pub fn validar_numero(numero: i32) -> Result<(), GafeteError> {
    if numero < 0 {
        return Err(GafeteError::Validation(
            "El número de gafete no puede ser negativo".to_string(),
        ));
    }
    Ok(())
}

/// Valida que el tipo de gafete sea uno de los valores permitidos.
pub fn validar_tipo(tipo_str: &str) -> Result<TipoGafete, GafeteError> {
    tipo_str.parse().map_err(|_| GafeteError::InvalidType(tipo_str.to_string()))
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida los datos requeridos para crear un nuevo gafete.
pub fn validar_create_input(input: &CreateGafeteInput) -> Result<(), GafeteError> {
    validar_numero(input.numero)?;
    validar_tipo(&input.tipo)?;
    Ok(())
}

/// Valida las modificaciones parciales de un gafete existente.
pub fn validar_update_input(input: &UpdateGafeteInput) -> Result<(), GafeteError> {
    if let Some(ref tipo) = input.tipo {
        validar_tipo(tipo)?;
    }
    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

