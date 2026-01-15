/// Capa de Dominio: Reglas de Negocio para Usuarios del Sistema.
///
/// Este módulo define las políticas de identidad, seguridad y validaciones
/// para los operadores y administradores de la plataforma.
use crate::domain::common::{
    normalizar_nombre_propio, validar_cedula_estandar, validar_email_estandar,
    validar_nombre_estandar, MAX_LEN_DIRECCION, MAX_LEN_GAFETE, MAX_LEN_NOMBRE, MAX_LEN_TELEFONO,
    MIN_LEN_PASSWORD,
};
use crate::domain::errors::UserError;
use crate::models::user::{CreateUserInput, UpdateUserInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato básico y longitud del correo electrónico.
pub fn validar_email(email: &str) -> Result<(), UserError> {
    validar_email_estandar(email).map_err(|e| UserError::Validation(e.to_string()))
}

/// Valida los requisitos mínimos del nombre del usuario.
pub fn validar_nombre(nombre: &str) -> Result<(), UserError> {
    validar_nombre_estandar(nombre, "nombre").map_err(|e| UserError::Validation(e.to_string()))
}

/// Valida los requisitos mínimos del apellido del usuario.
pub fn validar_apellido(apellido: &str) -> Result<(), UserError> {
    validar_nombre_estandar(apellido, "apellido").map_err(|e| UserError::Validation(e.to_string()))
}

/// Valida el formato numérico de la cédula del usuario administrativo.
pub fn validar_cedula(cedula: &str) -> Result<(), UserError> {
    validar_cedula_estandar(cedula).map_err(|e| UserError::Validation(e.to_string()))
}

/// Valida los requisitos de robustez de la contraseña.
///
/// Mínimo `MIN_LEN_PASSWORD` caracteres para asegurar un nivel base de seguridad.
pub fn validar_password(password: &str) -> Result<(), UserError> {
    if password.len() < MIN_LEN_PASSWORD {
        return Err(UserError::Validation(format!(
            "La contraseña debe tener al menos {MIN_LEN_PASSWORD} caracteres"
        )));
    }
    Ok(())
}

/// Valida campos opcionales del perfil de usuario.
///
/// # Argumentos
/// * `valor` - El valor opcional a validar.
/// * `max_len` - Longitud máxima permitida.
/// * `nombre_campo` - Nombre del campo para mensajes de error.
pub fn validar_opcional(
    valor: Option<&String>,
    max_len: usize,
    nombre_campo: &str,
) -> Result<(), UserError> {
    if let Some(v) = valor {
        if v.trim().len() > max_len {
            return Err(UserError::Validation(format!(
                "{nombre_campo} no puede exceder {max_len} caracteres"
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

    validar_opcional(input.segundo_nombre.as_ref(), MAX_LEN_NOMBRE, "Segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), MAX_LEN_NOMBRE, "Segundo apellido")?;
    validar_opcional(input.telefono.as_ref(), MAX_LEN_TELEFONO, "Teléfono")?;
    validar_opcional(input.direccion.as_ref(), MAX_LEN_DIRECCION, "Dirección")?;
    validar_opcional(input.numero_gafete.as_ref(), MAX_LEN_GAFETE, "Gafete")?;

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

    validar_opcional(input.segundo_nombre.as_ref(), MAX_LEN_NOMBRE, "Segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), MAX_LEN_NOMBRE, "Segundo apellido")?;
    validar_opcional(input.telefono.as_ref(), MAX_LEN_TELEFONO, "Teléfono")?;
    validar_opcional(input.direccion.as_ref(), MAX_LEN_DIRECCION, "Dirección")?;
    validar_opcional(input.numero_gafete.as_ref(), MAX_LEN_GAFETE, "Gafete")?;

    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Normaliza el email a minúsculas para evitar duplicados por capitalización.
pub fn normalizar_email(email: &str) -> String {
    email.trim().to_lowercase()
}

/// Normaliza un nombre o apellido aplicando Title Case.
///
/// Ej: "JUAN pérez" → "Juan Pérez"
pub fn normalizar_nombre(nombre: &str) -> String {
    normalizar_nombre_propio(nombre)
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

