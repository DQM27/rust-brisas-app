/// Capa de Dominio: Reglas de Negocio para Usuarios del Sistema.
///
/// Este módulo define las políticas de identidad, seguridad y validaciones
/// para los operadores y administradores de la plataforma.
use crate::domain::errors::UserError;
use crate::models::user::{CreateUserInput, UpdateUserInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato básico y longitud del correo electrónico.
pub fn validar_email(email: &str) -> Result<(), UserError> {
    let limpio = email.trim();

    if limpio.is_empty() {
        return Err(UserError::Validation("El email no puede estar vacío".to_string()));
    }

    if !limpio.contains('@') {
        return Err(UserError::Validation("Formato de correo electrónico inválido".to_string()));
    }

    if limpio.len() > 100 {
        return Err(UserError::Validation("El email no puede exceder 100 caracteres".to_string()));
    }

    Ok(())
}

/// Valida los requisitos mínimos del nombre del usuario.
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

/// Valida los requisitos mínimos del apellido del usuario.
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

/// Valida el formato numérico de la cédula del usuario administrativo.
pub fn validar_cedula(cedula: &str) -> Result<(), UserError> {
    let limpio = cedula.trim();

    if limpio.is_empty() {
        return Err(UserError::Validation("La cédula no puede estar vacía".to_string()));
    }

    if !limpio.chars().all(|c| c.is_numeric() || c == '-') {
        return Err(UserError::Validation(
            "La cédula solo puede contener números y guiones".to_string(),
        ));
    }

    if limpio.len() > 20 {
        return Err(UserError::Validation("La cédula no puede exceder 20 caracteres".to_string()));
    }

    Ok(())
}

/// Valida los requisitos de robustez de la contraseña.
///
/// Mínimo 6 caracteres para asegurar un nivel base de seguridad.
pub fn validar_password(password: &str) -> Result<(), UserError> {
    if password.len() < 6 {
        return Err(UserError::Validation(
            "La contraseña debe tener al menos 6 caracteres".to_string(),
        ));
    }
    Ok(())
}

/// Valida campos opcionales del perfil de usuario.
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

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida los datos requeridos para registrar un nuevo usuario operador.
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

/// Valida las modificaciones en el perfil de un usuario existente.
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

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Normaliza el email a minúsculas para evitar duplicados por capitalización.
pub fn normalizar_email(email: &str) -> String {
    email.trim().to_lowercase()
}

/// Limpia espacios laterales en el nombre.
pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_string()
}
