// ==========================================
// src/domain/blacklist_import.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::models::blacklist_import::{CreateBlacklistImportInput, UpdateBlacklistImportInput};
use chrono::NaiveDate;

// ==========================================
// CONSTANTES PARA DETECCIÓN DE NOMBRES COMPUESTOS
// ==========================================

const PREPOSICIONES: &[&str] = &[
    "de", "del", "de la", "de los", "de las",
    "da", "das", "do", "dos",
    "van", "von", "van der",
    "el", "la", "los", "las",
];

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_cedula(cedula: &str) -> Result<(), String> {
    let limpia = cedula.trim();
    
    if limpia.is_empty() {
        return Err("La cédula no puede estar vacía".to_string());
    }
    
    // Validar formato: solo números y guiones
    if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
        return Err("La cédula solo puede contener números y guiones".to_string());
    }
    
    // Validar longitud razonable (Costa Rica: 9 dígitos con guiones = 11 chars max)
    if limpia.len() < 9 || limpia.len() > 15 {
        return Err("La cédula debe tener entre 9 y 15 caracteres".to_string());
    }
    
    Ok(())
}

pub fn validar_nombre(nombre: &str, campo: &str) -> Result<(), String> {
    let limpio = nombre.trim();
    
    if limpio.is_empty() {
        return Err(format!("{} no puede estar vacío", campo));
    }
    
    // Solo letras, espacios y acentos
    if !limpio.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        return Err(format!("{} solo puede contener letras y espacios", campo));
    }
    
    if limpio.len() < 2 {
        return Err(format!("{} debe tener al menos 2 caracteres", campo));
    }
    
    if limpio.len() > 50 {
        return Err(format!("{} no puede exceder 50 caracteres", campo));
    }
    
    Ok(())
}

pub fn validar_empresa(empresa: &str) -> Result<(), String> {
    let limpia = empresa.trim();
    
    if limpia.is_empty() {
        return Err("La empresa no puede estar vacía".to_string());
    }
    
    if limpia.len() < 2 {
        return Err("La empresa debe tener al menos 2 caracteres".to_string());
    }
    
    if limpia.len() > 100 {
        return Err("La empresa no puede exceder 100 caracteres".to_string());
    }
    
    Ok(())
}

pub fn validar_motivo(motivo: &str) -> Result<(), String> {
    let limpio = motivo.trim();
    
    if limpio.is_empty() {
        return Err("El motivo no puede estar vacío".to_string());
    }
    
    if limpio.len() > 200 {
        return Err("El motivo no puede exceder 200 caracteres".to_string());
    }
    
    Ok(())
}

pub fn validar_fecha(fecha_str: &str) -> Result<(), String> {
    let limpia = fecha_str.trim();
    
    if limpia.is_empty() {
        return Err("La fecha no puede estar vacía".to_string());
    }
    
    // Validar formato YYYY-MM-DD
    NaiveDate::parse_from_str(limpia, "%Y-%m-%d")
        .map_err(|_| "La fecha debe estar en formato YYYY-MM-DD (ej: 2025-11-24)".to_string())?;
    
    Ok(())
}

pub fn validar_observaciones(observaciones: &str) -> Result<(), String> {
    let limpia = observaciones.trim();
    
    if limpia.len() > 500 {
        return Err("Las observaciones no pueden exceder 500 caracteres".to_string());
    }
    
    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

/// Valida todos los campos necesarios para crear una entrada
pub fn validar_create_input(input: &CreateBlacklistImportInput) -> Result<(), String> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.primer_nombre, "Primer nombre")?;
    
    if let Some(ref segundo) = input.segundo_nombre {
        if !segundo.trim().is_empty() {
            validar_nombre(segundo, "Segundo nombre")?;
        }
    }
    
    validar_nombre(&input.primer_apellido, "Primer apellido")?;
    
    if let Some(ref segundo) = input.segundo_apellido {
        if !segundo.trim().is_empty() {
            validar_nombre(segundo, "Segundo apellido")?;
        }
    }
    
    validar_empresa(&input.empresa)?;
    
    // Motivo opcional con default "No especificado"
    if let Some(ref motivo) = input.motivo_bloqueo {
        if !motivo.trim().is_empty() {
            validar_motivo(motivo)?;
        }
    }
    
    // Fecha opcional con default fecha actual
    if let Some(ref fecha) = input.fecha_inicio_bloqueo {
        if !fecha.trim().is_empty() {
            validar_fecha(fecha)?;
        }
    }
    
    if let Some(ref obs) = input.observaciones {
        if !obs.trim().is_empty() {
            validar_observaciones(obs)?;
        }
    }
    
    Ok(())
}

/// Valida los campos presentes en un update (solo los que no son None)
pub fn validar_update_input(input: &UpdateBlacklistImportInput) -> Result<(), String> {
    if let Some(ref primer) = input.primer_nombre {
        validar_nombre(primer, "Primer nombre")?;
    }
    
    if let Some(ref segundo) = input.segundo_nombre {
        if !segundo.trim().is_empty() {
            validar_nombre(segundo, "Segundo nombre")?;
        }
    }
    
    if let Some(ref primer) = input.primer_apellido {
        validar_nombre(primer, "Primer apellido")?;
    }
    
    if let Some(ref segundo) = input.segundo_apellido {
        if !segundo.trim().is_empty() {
            validar_nombre(segundo, "Segundo apellido")?;
        }
    }
    
    if let Some(ref empresa) = input.empresa {
        validar_empresa(empresa)?;
    }
    
    if let Some(ref motivo) = input.motivo_bloqueo {
        if !motivo.trim().is_empty() {
            validar_motivo(motivo)?;
        }
    }
    
    if let Some(ref obs) = input.observaciones {
        if !obs.trim().is_empty() {
            validar_observaciones(obs)?;
        }
    }
    
    Ok(())
}

// ==========================================
// HELPERS DE NORMALIZACIÓN DE NOMBRES
// ==========================================

/// Normaliza cédula (trim, sin espacios)
pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().replace(" ", "")
}

/// Capitaliza un nombre (Primera Letra Mayúscula)
pub fn capitalizar_nombre(nombre: &str) -> String {
    nombre
        .split_whitespace()
        .map(|palabra| {
            let mut chars = palabra.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() 
                        + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Normaliza texto genérico (trim + capitalizar)
pub fn normalizar_texto(texto: &str) -> String {
    capitalizar_nombre(texto.trim())
}

/// Detecta si un nombre completo tiene preposiciones (requiere validación manual)
pub fn requiere_validacion_manual(nombre_completo: &str) -> bool {
    let lowercase = nombre_completo.to_lowercase();
    let palabras: Vec<&str> = lowercase.split_whitespace().collect();
    
    // Buscar preposiciones completas como palabras
    PREPOSICIONES.iter().any(|&prep| {
        if prep.contains(' ') {
            // Preposición compuesta: buscar secuencia exacta
            lowercase.contains(prep)
        } else {
            // Preposición simple: buscar palabra completa
            palabras.contains(&prep)
        }
    })
}

/// Intenta separar un nombre completo en partes (heurística simple)
/// Retorna: (primer_nombre, segundo_nombre, primer_apellido, segundo_apellido)
pub fn separar_nombre_automatico(nombre_completo: &str) -> Result<(String, Option<String>, String, Option<String>), String> {
    let partes: Vec<&str> = nombre_completo.split_whitespace().collect();
    
    match partes.len() {
        0 | 1 => Err("El nombre completo debe tener al menos 2 palabras".to_string()),
        
        2 => {
            // "Juan Pérez" -> Nombre: Juan, Apellido: Pérez
            Ok((
                capitalizar_nombre(partes[0]),
                None,
                capitalizar_nombre(partes[1]),
                None
            ))
        }
        
        3 => {
            // "Juan Carlos Pérez" -> Nombres: Juan Carlos, Apellido: Pérez
            Ok((
                capitalizar_nombre(partes[0]),
                Some(capitalizar_nombre(partes[1])),
                capitalizar_nombre(partes[2]),
                None
            ))
        }
        
        4 => {
            // "Juan Carlos Pérez Gómez" -> Nombres: Juan Carlos, Apellidos: Pérez Gómez
            Ok((
                capitalizar_nombre(partes[0]),
                Some(capitalizar_nombre(partes[1])),
                capitalizar_nombre(partes[2]),
                Some(capitalizar_nombre(partes[3]))
            ))
        }
        
        _ => {
            // 5+ palabras -> Requiere validación manual
            Err(format!(
                "El nombre tiene {} palabras. Requiere validación manual.", 
                partes.len()
            ))
        }
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cedula() {
        assert!(validar_cedula("1-2345-6789").is_ok());
        assert!(validar_cedula("123456789").is_ok());
        assert!(validar_cedula("").is_err());
        assert!(validar_cedula("abc-123").is_err());
        assert!(validar_cedula("123").is_err()); // muy corta
    }

    #[test]
    fn test_validar_nombre() {
        assert!(validar_nombre("Juan", "Primer nombre").is_ok());
        assert!(validar_nombre("María José", "Primer nombre").is_ok());
        assert!(validar_nombre("J", "Primer nombre").is_err()); // muy corto
        assert!(validar_nombre("Juan123", "Primer nombre").is_err()); // con números
        assert!(validar_nombre("", "Primer nombre").is_err());
    }

    #[test]
    fn test_capitalizar_nombre() {
        assert_eq!(capitalizar_nombre("juan perez"), "Juan Perez");
        assert_eq!(capitalizar_nombre("MARIA GOMEZ"), "Maria Gomez");
        assert_eq!(capitalizar_nombre("  ana  rodriguez  "), "Ana Rodriguez");
    }

    #[test]
    fn test_normalizar_cedula() {
        assert_eq!(normalizar_cedula("  1-2345-6789  "), "1-2345-6789");
        assert_eq!(normalizar_cedula("1 2345 6789"), "123456789");
    }

    #[test]
    fn test_requiere_validacion_manual() {
        assert!(!requiere_validacion_manual("Juan Pérez"));
        assert!(!requiere_validacion_manual("Juan Carlos Pérez Gómez"));
        assert!(requiere_validacion_manual("Juan de la Cruz"));
        assert!(requiere_validacion_manual("María del Carmen Rodríguez"));
        assert!(requiere_validacion_manual("José van der Berg"));
    }

    #[test]
    fn test_separar_nombre_automatico() {
        // 2 palabras
        let (p1, s1, a1, s2) = separar_nombre_automatico("juan perez").unwrap();
        assert_eq!(p1, "Juan");
        assert_eq!(s1, None);
        assert_eq!(a1, "Perez");
        assert_eq!(s2, None);

        // 3 palabras
        let (p1, s1, a1, s2) = separar_nombre_automatico("juan carlos perez").unwrap();
        assert_eq!(p1, "Juan");
        assert_eq!(s1, Some("Carlos".to_string()));
        assert_eq!(a1, "Perez");
        assert_eq!(s2, None);

        // 4 palabras
        let (p1, s1, a1, s2) = separar_nombre_automatico("juan carlos perez gomez").unwrap();
        assert_eq!(p1, "Juan");
        assert_eq!(s1, Some("Carlos".to_string()));
        assert_eq!(a1, "Perez");
        assert_eq!(s2, Some("Gomez".to_string()));

        // 1 palabra - error
        assert!(separar_nombre_automatico("Juan").is_err());

        // 5+ palabras - error (requiere validación manual)
        assert!(separar_nombre_automatico("Juan Carlos de la Cruz Pérez").is_err());
    }

    #[test]
    fn test_validar_fecha() {
        assert!(validar_fecha("2025-11-24").is_ok());
        assert!(validar_fecha("2025-01-01").is_ok());
        assert!(validar_fecha("24-11-2025").is_err()); // formato incorrecto
        assert!(validar_fecha("2025/11/24").is_err()); // separador incorrecto
        assert!(validar_fecha("").is_err());
    }

    #[test]
    fn test_validar_empresa() {
        assert!(validar_empresa("Constructora XYZ").is_ok());
        assert!(validar_empresa("A").is_err()); // muy corta
        assert!(validar_empresa("").is_err());
        assert!(validar_empresa(&"a".repeat(101)).is_err()); // muy larga
    }
}