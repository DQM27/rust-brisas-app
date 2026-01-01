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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::gafete::TipoGafete;

    #[test]
    fn test_validar_numero_valido() {
        assert!(validar_numero(123).is_ok());
        assert!(validar_numero(4).is_ok());
        assert!(validar_numero(20).is_ok());
    }

    #[test]
    fn test_validar_numero_cero() {
        assert!(validar_numero(0).is_ok()); // 0 es S/G
    }

    #[test]
    fn test_validar_numero_negativo() {
        assert!(validar_numero(-1).is_err());
        assert!(validar_numero(-100).is_err());
    }

    #[test]
    fn test_validar_tipo_valido() {
        assert!(validar_tipo("contratista").is_ok());
        assert!(matches!(validar_tipo("contratista").unwrap(), TipoGafete::Contratista));
    }

    #[test]
    fn test_validar_tipo_invalido() {
        let result = validar_tipo("fantasma");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GafeteError::InvalidType(_)));
    }

    #[test]
    fn test_validar_create_input() {
        let input = CreateGafeteInput { numero: 101, tipo: "visita".to_string() };
        assert!(validar_create_input(&input).is_ok());

        let invalid_input = CreateGafeteInput { numero: -5, tipo: "visita".to_string() };
        assert!(validar_create_input(&invalid_input).is_err());
    }
}
