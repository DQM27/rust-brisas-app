// ==========================================
// src/domain/user.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::UserError;
use crate::models::user::{CreateUserInput, UpdateUserInput};

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_email(email: &str) -> Result<(), UserError> {
    let limpio = email.trim();

    if limpio.is_empty() {
        return Err(UserError::Validation("El email no puede estar vacío".to_string()));
    }

    if !limpio.contains('@') {
        return Err(UserError::Validation("Email inválido".to_string()));
    }

    if limpio.len() > 100 {
        return Err(UserError::Validation("El email no puede exceder 100 caracteres".to_string()));
    }

    Ok(())
}

pub fn validar_nombre(nombre: &str) -> Result<(), UserError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(UserError::Validation("El nombre no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(UserError::Validation("El nombre no puede exceder 50 caracteres".to_string()));
    }

    Ok(())
}

pub fn validar_apellido(apellido: &str) -> Result<(), UserError> {
    let limpio = apellido.trim();

    if limpio.is_empty() {
        return Err(UserError::Validation("El apellido no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(UserError::Validation(
            "El apellido no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_cedula(cedula: &str) -> Result<(), UserError> {
    let limpio = cedula.trim();

    if limpio.is_empty() {
        return Err(UserError::Validation("La cédula no puede estar vacía".to_string()));
    }

    if limpio.len() > 20 {
        return Err(UserError::Validation("La cédula no puede exceder 20 caracteres".to_string()));
    }

    // Validar caracteres validos? Numeros y guiones?
    if !limpio.chars().all(|c| c.is_numeric() || c == '-') {
        return Err(UserError::Validation(
            "La cédula solo puede contener números y guiones".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_password(password: &str) -> Result<(), UserError> {
    // Si viene password, validar longitud
    if password.len() < 6 {
        return Err(UserError::Validation(
            "La contraseña debe tener al menos 6 caracteres".to_string(),
        ));
    }
    Ok(())
}

pub fn validar_opcional(
    valor: Option<&String>,
    max_len: usize,
    nombre_campo: &str,
) -> Result<(), UserError> {
    if let Some(v) = valor {
        if v.trim().len() > max_len {
            return Err(UserError::Validation(format!(
                "{} no puede exceder {} caracteres",
                nombre_campo, max_len
            )));
        }
    }
    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

pub fn validar_create_input(input: &CreateUserInput) -> Result<(), UserError> {
    validar_email(&input.email)?;
    validar_nombre(&input.nombre)?;
    validar_apellido(&input.apellido)?;
    validar_cedula(&input.cedula)?;

    if let Some(ref pwd) = input.password {
        validar_password(pwd)?;
    }

    validar_opcional(input.segundo_nombre.as_ref(), 50, "Segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), 50, "Segundo apellido")?;
    validar_opcional(input.telefono.as_ref(), 20, "Teléfono")?;
    validar_opcional(input.direccion.as_ref(), 200, "Dirección")?;
    validar_opcional(input.numero_gafete.as_ref(), 20, "Gafete")?;

    Ok(())
}

pub fn validar_update_input(input: &UpdateUserInput) -> Result<(), UserError> {
    if let Some(ref email) = input.email {
        validar_email(email)?;
    }

    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }

    if let Some(ref apellido) = input.apellido {
        validar_apellido(apellido)?;
    }

    if let Some(ref cedula) = input.cedula {
        validar_cedula(cedula)?;
    }

    if let Some(ref pwd) = input.password {
        validar_password(pwd)?;
    }

    validar_opcional(input.segundo_nombre.as_ref(), 50, "Segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), 50, "Segundo apellido")?;
    validar_opcional(input.telefono.as_ref(), 20, "Teléfono")?;
    validar_opcional(input.direccion.as_ref(), 200, "Dirección")?;
    validar_opcional(input.numero_gafete.as_ref(), 20, "Gafete")?;

    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

pub fn normalizar_email(email: &str) -> String {
    email.trim().to_lowercase()
}

pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_string()
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_email_valido() {
        assert!(validar_email("test@example.com").is_ok());
    }

    #[test]
    fn test_validar_email_invalido() {
        assert!(validar_email("").is_err());
        assert!(validar_email("no-at-sign").is_err());
    }

    #[test]
    fn test_validar_nombre_valido() {
        assert!(validar_nombre("Juan").is_ok());
    }

    #[test]
    fn test_validar_nombre_vacio() {
        assert!(validar_nombre("   ").is_err());
    }

    #[test]
    fn test_validar_cedula_valida() {
        assert!(validar_cedula("123-456").is_ok());
        assert!(validar_cedula("123456").is_ok());
    }

    #[test]
    fn test_validar_cedula_invalida() {
        assert!(validar_cedula("").is_err());
        assert!(validar_cedula("123 456").is_err()); // space not allowed
        assert!(validar_cedula("ABC-123").is_err()); // letters not allowed
    }

    #[test]
    fn test_validar_password() {
        assert!(validar_password("12345").is_err());
        assert!(validar_password("123456").is_ok());
    }

    #[test]
    fn test_normalizar_email() {
        assert_eq!(normalizar_email("  TEST@Example.Com  "), "test@example.com");
    }
}
