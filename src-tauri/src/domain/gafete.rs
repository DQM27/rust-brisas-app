// ==========================================
// src/domain/gafete.rs
// ==========================================
// Validaciones y reglas de negocio puras - Sin DB

use crate::models::gafete::{CreateGafeteInput, TipoGafete, UpdateGafeteInput};

// ==========================================
// VALIDACIONES DE CAMPOS
// ==========================================

pub fn validar_numero(numero: &str) -> Result<(), String> {
    let limpio = numero.trim();

    if limpio.is_empty() {
        return Err("El número no puede estar vacío".to_string());
    }

    if limpio.len() > 20 {
        return Err("El número no puede exceder 20 caracteres".to_string());
    }

    // S/G es reservado
    if limpio.to_uppercase() == "S/G" {
        return Err("'S/G' es un número reservado del sistema".to_string());
    }

    Ok(())
}

pub fn validar_tipo(tipo_str: &str) -> Result<TipoGafete, String> {
    tipo_str.parse()
}

// ==========================================
// VALIDACIONES DE INPUTS
// ==========================================

pub fn validar_create_input(input: &CreateGafeteInput) -> Result<(), String> {
    validar_numero(&input.numero)?;
    validar_tipo(&input.tipo)?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateGafeteInput) -> Result<(), String> {
    if let Some(ref tipo) = input.tipo {
        validar_tipo(tipo)?;
    }
    Ok(())
}

// ==========================================
// NORMALIZACIONES
// ==========================================

/// Normaliza un número de gafete (trim + uppercase)
pub fn normalizar_numero(numero: &str) -> String {
    numero.trim().to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_numero_valido() {
        assert!(validar_numero("27").is_ok());
        assert!(validar_numero("  A-15  ").is_ok());
    }

    #[test]
    fn test_validar_numero_invalido() {
        assert!(validar_numero("").is_err());
        assert!(validar_numero("   ").is_err());
        assert!(validar_numero("S/G").is_err());
        assert!(validar_numero(&"A".repeat(21)).is_err());
    }

    #[test]
    fn test_normalizar_numero() {
        assert_eq!(normalizar_numero("  a-15  "), "A-15");
        assert_eq!(normalizar_numero("prov-3"), "PROV-3");
    }
}
