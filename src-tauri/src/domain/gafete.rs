// ==========================================
// src/domain/gafete.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::GafeteError;
use crate::models::gafete::{CreateGafeteInput, TipoGafete, UpdateGafeteInput};

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_numero(numero: &str) -> Result<(), GafeteError> {
    let limpio = numero.trim();

    if limpio.is_empty() {
        return Err(GafeteError::Validation(
            "El número del gafete no puede estar vacío".to_string(),
        ));
    }

    if limpio.len() > 20 {
        return Err(GafeteError::Validation(
            "El número no puede exceder 20 caracteres".to_string(),
        ));
    }

    if limpio.to_uppercase() == "S/G" {
        return Err(GafeteError::Validation(
            "'S/G' es un número reservado del sistema".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_tipo(tipo_str: &str) -> Result<TipoGafete, GafeteError> {
    tipo_str.parse().map_err(|_| GafeteError::InvalidType(tipo_str.to_string()))
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

pub fn validar_create_input(input: &CreateGafeteInput) -> Result<(), GafeteError> {
    validar_numero(&input.numero)?;
    validar_tipo(&input.tipo)?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateGafeteInput) -> Result<(), GafeteError> {
    if let Some(ref tipo) = input.tipo {
        validar_tipo(tipo)?;
    }
    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

pub fn normalizar_numero(numero: &str) -> String {
    numero.trim().to_uppercase()
}

// ==========================================
// TESTS
// ==========================================

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
