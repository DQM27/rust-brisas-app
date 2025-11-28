// ==========================================
// src/domain/ingreso.rs
// ==========================================
// Validaciones y reglas de negocio puras - Sin DB

use crate::models::ingreso::CreateIngresoContratistaInput;

// ==========================================
// VALIDACIONES DE CAMPOS
// ==========================================

pub fn validar_gafete_numero(numero: &str) -> Result<(), String> {
    let limpio = numero.trim();

    if limpio.is_empty() {
        return Err("El número de gafete no puede estar vacío".to_string());
    }

    if limpio.len() > 20 {
        return Err("El número de gafete no puede exceder 20 caracteres".to_string());
    }

    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS
// ==========================================

pub fn validar_create_contratista_input(
    input: &CreateIngresoContratistaInput,
) -> Result<(), String> {
    // El contratista_id ya existe, solo validar gafete si lo hay
    if let Some(ref gafete_num) = input.gafete_numero {
        validar_gafete_numero(gafete_num)?;
    }

    Ok(())
}

// ==========================================
// NORMALIZACIONES
// ==========================================

pub fn normalizar_numero_gafete(numero: &str) -> String {
    numero.trim().to_uppercase()
}

// ==========================================
// REGLAS DE NEGOCIO
// ==========================================

/// Verifica si un PRAIND está vigente en una fecha específica
pub fn verificar_praind_vigente(fecha_vencimiento: &str) -> Result<bool, String> {
    use chrono::{NaiveDateTime, Utc};

    let fecha_venc = NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", fecha_vencimiento),
        "%Y-%m-%d %H:%M:%S",
    )
    .map_err(|_| "Error al parsear fecha PRAIND".to_string())?;

    let now = Utc::now().naive_utc();
    Ok(fecha_venc >= now)
}

/// Calcula días restantes hasta vencimiento de PRAIND
pub fn dias_hasta_vencimiento_praind(fecha_vencimiento: &str) -> Result<i64, String> {
    use chrono::{NaiveDateTime, Utc};

    let fecha_venc = NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", fecha_vencimiento),
        "%Y-%m-%d %H:%M:%S",
    )
    .map_err(|_| "Error al parsear fecha PRAIND".to_string())?;

    let hoy = Utc::now().naive_utc();
    let dias = (fecha_venc - hoy).num_days();

    Ok(dias)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalizar_gafete_numero() {
        assert_eq!(normalizar_gafete_numero("  a-15  "), "A-15");
        assert_eq!(normalizar_gafete_numero("c-25"), "C-25");
    }

    #[test]
    fn test_validar_gafete_numero() {
        assert!(validar_gafete_numero("A-15").is_ok());
        assert!(validar_gafete_numero("").is_err());
        assert!(validar_gafete_numero("A".repeat(25).as_str()).is_err());
    }
}