// ==========================================
// src/domain/lista_negra.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::models::lista_negra::{AddToListaNegraInput, UpdateListaNegraInput};
use chrono::NaiveDateTime;

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_cedula(cedula: &str) -> Result<(), String> {
    let limpia = cedula.trim();
    
    if limpia.is_empty() {
        return Err("La cédula no puede estar vacía".to_string());
    }
    
    if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
        return Err("La cédula solo puede contener números y guiones".to_string());
    }
    
    if limpia.len() < 7 || limpia.len() > 20 {
        return Err("La cédula debe tener entre 7 y 20 caracteres".to_string());
    }
    
    Ok(())
}

pub fn validar_nombre(nombre: &str) -> Result<(), String> {
    let limpio = nombre.trim();
    
    if limpio.is_empty() {
        return Err("El nombre no puede estar vacío".to_string());
    }
    
    if limpio.len() > 50 {
        return Err("El nombre no puede exceder 50 caracteres".to_string());
    }
    
    Ok(())
}

pub fn validar_motivo(motivo: &str) -> Result<(), String> {
    let limpio = motivo.trim();
    
    if limpio.is_empty() {
        return Err("Debe especificar un motivo de bloqueo".to_string());
    }
    
    if limpio.len() > 500 {
        return Err("El motivo no puede exceder 500 caracteres".to_string());
    }
    
    Ok(())
}

pub fn validar_bloqueado_por(bloqueado_por: &str) -> Result<(), String> {
    let limpio = bloqueado_por.trim();
    
    if limpio.is_empty() {
        return Err("Debe especificar quién realizó el bloqueo".to_string());
    }
    
    if limpio.len() > 100 {
        return Err("El nombre de quien bloqueó no puede exceder 100 caracteres".to_string());
    }
    
    Ok(())
}

pub fn validar_fecha_fin(fecha_str: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(fecha_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD HH:MM:SS".to_string())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

/// Valida todos los campos necesarios para agregar a lista negra
pub fn validar_add_input(input: &AddToListaNegraInput) -> Result<(), String> {
    // Si tiene contratista_id, no necesita cédula/nombre/apellido
    if input.contratista_id.is_some() {
        validar_motivo(&input.motivo_bloqueo)?;
        validar_bloqueado_por(&input.bloqueado_por)?;
        
        if let Some(ref fecha) = input.fecha_fin_bloqueo {
            validar_fecha_fin(fecha)?;
        }
        
        return Ok(());
    }
    
    // Si NO tiene contratista_id, requiere cédula + nombre + apellido
    let cedula = input.cedula.as_ref()
        .ok_or("Debe proporcionar cédula si no especifica contratista_id")?;
    validar_cedula(cedula)?;
    
    let nombre = input.nombre.as_ref()
        .ok_or("Debe proporcionar nombre si no especifica contratista_id")?;
    validar_nombre(nombre)?;
    
    let apellido = input.apellido.as_ref()
        .ok_or("Debe proporcionar apellido si no especifica contratista_id")?;
    validar_nombre(apellido)?;
    
    validar_motivo(&input.motivo_bloqueo)?;
    validar_bloqueado_por(&input.bloqueado_por)?;
    
    if let Some(ref fecha) = input.fecha_fin_bloqueo {
        validar_fecha_fin(fecha)?;
    }
    
    Ok(())
}

/// Valida los campos presentes en un update (solo los que no son None)
pub fn validar_update_input(input: &UpdateListaNegraInput) -> Result<(), String> {
    if let Some(ref motivo) = input.motivo_bloqueo {
        validar_motivo(motivo)?;
    }
    
    if let Some(ref fecha) = input.fecha_fin_bloqueo {
        validar_fecha_fin(fecha)?;
    }
    
    Ok(())
}

// ==========================================
// HELPERS DE NORMALIZACIÓN
// ==========================================

/// Normaliza texto genérico (trim)
pub fn normalizar_texto(texto: &str) -> String {
    texto.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cedula() {
        assert!(validar_cedula("1234567890").is_ok());
        assert!(validar_cedula("123-456-789").is_ok());
        assert!(validar_cedula("").is_err());
        assert!(validar_cedula("123").is_err());
        assert!(validar_cedula("abc123").is_err());
    }

    #[test]
    fn test_validar_nombre() {
        assert!(validar_nombre("Juan").is_ok());
        assert!(validar_nombre("  María  ").is_ok());
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre(&"a".repeat(51)).is_err());
    }

    #[test]
    fn test_validar_motivo() {
        assert!(validar_motivo("Incumplimiento de normas").is_ok());
        assert!(validar_motivo("").is_err());
        assert!(validar_motivo(&"a".repeat(501)).is_err());
    }

    #[test]
    fn test_validar_fecha_fin() {
        assert!(validar_fecha_fin("2025-12-31 23:59:59").is_ok());
        assert!(validar_fecha_fin("2025-12-31").is_err());
        assert!(validar_fecha_fin("invalid").is_err());
    }
}