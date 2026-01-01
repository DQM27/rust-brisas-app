/// Capa de Dominio: Reglas de Negocio para Gafetes.
///
/// Este módulo gestiona la validación de gafetes de identificación
/// utilizados en el sistema de control de acceso. Incluye validaciones
/// para números, tipos y estados de gafetes.
use crate::domain::errors::GafeteError;
use crate::models::gafete::{CreateGafeteInput, TipoGafete, UpdateGafeteInput};

// --------------------------------------------------------------------------
// CONSTANTES DE VALIDACIÓN
// --------------------------------------------------------------------------

/// Longitud máxima del número de gafete.
pub const NUMERO_MAX_LEN: usize = 20;

/// Número reservado del sistema para casos "sin gafete".
pub const NUMERO_RESERVADO: &str = "S/G";

/// Caracteres prohibidos en números de gafete (prevención de inyecciones).
const CHARS_PROHIBIDOS: &[char] = &['<', '>', '{', '}', '|', '\\', '^', '~', '[', ']', '`'];

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud del número de gafete.
pub fn validar_numero(numero: &str) -> Result<(), GafeteError> {
    let limpio = numero.trim();

    if limpio.is_empty() {
        return Err(GafeteError::Validation(
            "El número del gafete no puede estar vacío".to_string(),
        ));
    }

    if limpio.len() > NUMERO_MAX_LEN {
        return Err(GafeteError::Validation(format!(
            "El número no puede exceder {} caracteres",
            NUMERO_MAX_LEN
        )));
    }

    if limpio.to_uppercase() == NUMERO_RESERVADO {
        return Err(GafeteError::Validation(format!(
            "'{}' es un número reservado del sistema",
            NUMERO_RESERVADO
        )));
    }

    // Validar caracteres prohibidos
    if limpio.chars().any(|c| CHARS_PROHIBIDOS.contains(&c)) {
        return Err(GafeteError::Validation(
            "El número contiene caracteres no permitidos".to_string(),
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
    validar_numero(&input.numero)?;
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
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Normaliza el número de gafete a mayúsculas y sin espacios.
pub fn normalizar_numero(numero: &str) -> String {
    numero.trim().to_uppercase()
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
        assert!(validar_numero("123").is_ok());
        assert!(validar_numero("GAF-001").is_ok());
    }

    #[test]
    fn test_validar_numero_vacio() {
        let result = validar_numero("   ");
        assert!(result.is_err());
        match result.unwrap_err() {
            GafeteError::Validation(msg) => assert!(msg.contains("vacío")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validar_numero_largo() {
        let result = validar_numero("ESTE_ES_UN_NUMERO_MUY_LARGO_QUE_EXCEDE_LOS_LIMITES");
        assert!(result.is_err());
        match result.unwrap_err() {
            GafeteError::Validation(msg) => assert!(msg.contains("exceder 20")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validar_numero_reservado() {
        assert!(validar_numero("s/g").is_err());
        assert!(validar_numero("S/G").is_err());
    }

    #[test]
    fn test_validar_numero_chars_prohibidos() {
        assert!(validar_numero("GAF<script>").is_err());
        assert!(validar_numero("123{json}").is_err());
        assert!(validar_numero("G-01|pipe").is_err());
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
        let input = CreateGafeteInput { numero: "101".to_string(), tipo: "visita".to_string() };
        assert!(validar_create_input(&input).is_ok());

        let invalid_input =
            CreateGafeteInput { numero: "".to_string(), tipo: "visita".to_string() };
        assert!(validar_create_input(&invalid_input).is_err());
    }

    #[test]
    fn test_normalizar_numero() {
        assert_eq!(normalizar_numero("  g-01  "), "G-01");
        assert_eq!(normalizar_numero("123"), "123");
    }
}
