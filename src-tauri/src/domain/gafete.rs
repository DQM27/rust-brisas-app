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

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud del número de gafete.
///
/// Reglas:
/// - No vacío.
/// - Máximo 20 caracteres.
/// - Debe ser "S/G" o un número entero positivo (sin ceros a la izquierda).
/// - No se permiten letras ni caracteres especiales (salvo en "S/G").
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

    // Caso especial: Comodín "Sin Gafete"
    if limpio.to_uppercase() == NUMERO_RESERVADO {
        return Ok(());
    }

    // Caso especial: "0" no es un gafete válido comunmente, pero si el usuario quiere permitirlo:
    // Asumiremos que gafetes son 1..N. Si "0" es válido, cambiar regex a ^(0|[1-9][0-9]*)$.
    // Por ahora: ^[1-9][0-9]*$ (Enteros positivos sin ceros a la izquierda)

    let es_numero_valido = limpio.chars().all(|c| c.is_ascii_digit());
    if !es_numero_valido {
        return Err(GafeteError::Validation("El número solo puede contener dígitos".to_string()));
    }

    // Validar ceros a la izquierda (ej: "04", "001")
    if limpio.starts_with('0') && limpio.len() > 1 {
        return Err(GafeteError::Validation(
            "El número no puede tener ceros a la izquierda".to_string(),
        ));
    }

    // Gafete "0" es alias válido de S/G
    if limpio == "0" {
        return Ok(());
    }

    // Gafete "0" no debería permitirse si es físico, pero si el usuario tiene un gafete marcado con 0...
    // Asumiremos que "0" es válido si es un solo dígito.
    // EDIT: "no quiero que el user defina manualmente para que el ponga 0003... es mas rapido escribir 4"
    // Esto implica que 3 es válido. 0 podría serlo.

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
        assert!(validar_numero("4").is_ok());
        assert!(validar_numero("20").is_ok());
    }

    #[test]
    fn test_validar_numero_con_ceros_izquierda() {
        assert!(validar_numero("001").is_err());
        assert!(validar_numero("04").is_err());
    }

    #[test]
    fn test_validar_numero_cero() {
        assert!(validar_numero("0").is_ok());
    }

    #[test]
    fn test_validar_numero_letras_invalidas() {
        assert!(validar_numero("GAF-001").is_err());
        assert!(validar_numero("A1").is_err());
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
        let result = validar_numero("123456789012345678901");
        assert!(result.is_err());
        match result.unwrap_err() {
            GafeteError::Validation(msg) => assert!(msg.contains("exceder 20")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validar_numero_reservado() {
        assert!(validar_numero("s/g").is_ok());
        assert!(validar_numero("S/G").is_ok());
    }

    #[test]
    fn test_validar_numero_chars_prohibidos() {
        assert!(validar_numero("GAF<script>").is_err());
        assert!(validar_numero("123{json}").is_err());
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
