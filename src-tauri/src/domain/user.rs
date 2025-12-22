// ==========================================
// src/domain/user.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::models::user::{CreateUserInput, UpdateUserInput};

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_email(email: &str) -> Result<(), String> {
    let limpio = email.trim();

    if limpio.is_empty() {
        return Err("El email no puede estar vacío".to_string());
    }

    if !limpio.contains('@') {
        return Err("Email inválido".to_string());
    }

    if limpio.len() > 100 {
        return Err("El email no puede exceder 100 caracteres".to_string());
    }

    Ok(())
}

pub fn validar_password(password: &str) -> Result<(), String> {
    if password.len() < 6 {
        return Err("La contraseña debe tener al menos 6 caracteres".to_string());
    }

    if password.len() > 100 {
        return Err("La contraseña no puede exceder 100 caracteres".to_string());
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

pub fn validar_apellido(apellido: &str) -> Result<(), String> {
    let limpio = apellido.trim();

    if limpio.is_empty() {
        return Err("El apellido no puede estar vacío".to_string());
    }

    if limpio.len() > 50 {
        return Err("El apellido no puede exceder 50 caracteres".to_string());
    }

    Ok(())
}

pub fn validar_cedula(cedula: &str) -> Result<(), String> {
    let limpio = cedula.trim();
    if limpio.is_empty() {
        return Err("La cédula no puede estar vacía".to_string());
    }
    if limpio.len() > 20 {
        return Err("La cédula no puede exceder 20 caracteres".to_string());
    }
    Ok(())
}

pub fn validar_opcional(
    valor: Option<&String>,
    max_len: usize,
    nombre_campo: &str,
) -> Result<(), String> {
    if let Some(v) = valor {
        let limpio = v.trim();
        if !limpio.is_empty() && limpio.len() > max_len {
            return Err(format!(
                "El {} no puede exceder {} caracteres",
                nombre_campo, max_len
            ));
        }
    }
    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

/// Valida todos los campos necesarios para crear un usuario
pub fn validar_create_input(input: &CreateUserInput) -> Result<(), String> {
    validar_email(&input.email)?;
    if let Some(ref pwd) = input.password {
        validar_password(pwd)?;
    }
    validar_nombre(&input.nombre)?;
    validar_apellido(&input.apellido)?;
    validar_cedula(&input.cedula)?;

    validar_opcional(input.segundo_nombre.as_ref(), 50, "segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), 50, "segundo apellido")?;
    validar_opcional(input.telefono.as_ref(), 20, "teléfono")?;
    validar_opcional(input.direccion.as_ref(), 200, "dirección")?;

    Ok(())
}

/// Valida los campos presentes en un update (solo los que no son None)
pub fn validar_update_input(input: &UpdateUserInput) -> Result<(), String> {
    if let Some(ref email) = input.email {
        validar_email(email)?;
    }

    if let Some(ref password) = input.password {
        validar_password(password)?;
    }

    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }

    if let Some(ref apellido) = input.apellido {
        validar_apellido(apellido)?;
    }

    Ok(())
}

// ==========================================
// HELPERS DE NORMALIZACIÓN
// ==========================================

/// Normaliza un email (trim + lowercase)
pub fn normalizar_email(email: &str) -> String {
    email.trim().to_lowercase()
}

/// Normaliza un nombre (trim + capitalizar primera letra)
pub fn normalizar_nombre(nombre: &str) -> String {
    let limpio = nombre.trim();
    if limpio.is_empty() {
        return String::new();
    }

    let mut chars = limpio.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_email_valido() {
        assert!(validar_email("test@example.com").is_ok());
        assert!(validar_email("  user@domain.com  ").is_ok());
    }

    #[test]
    fn test_validar_email_invalido() {
        assert!(validar_email("").is_err());
        assert!(validar_email("   ").is_err());
        assert!(validar_email("sinArroba").is_err());
        assert!(validar_email(&"a".repeat(101)).is_err());
    }

    #[test]
    fn test_validar_password() {
        assert!(validar_password("123456").is_ok());
        assert!(validar_password("12345").is_err()); // muy corta
        assert!(validar_password(&"a".repeat(101)).is_err()); // muy larga
    }

    #[test]
    fn test_normalizar_email() {
        assert_eq!(normalizar_email("  Test@Example.COM  "), "test@example.com");
    }

    #[test]
    fn test_normalizar_nombre() {
        assert_eq!(normalizar_nombre("  juan  "), "Juan  ");
        assert_eq!(normalizar_nombre(""), "");
    }
}
