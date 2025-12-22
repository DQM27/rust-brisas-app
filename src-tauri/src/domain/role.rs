// ==========================================
// src/domain/role.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::models::role::{CreateRoleInput, UpdateRoleInput};

// ==========================================
// CONSTANTES
// ==========================================

/// ID del superuser (bypass total de permisos)
pub const SUPERUSER_ID: &str = "00000000-0000-0000-0000-000000000000";
pub const SUPERUSER_EMAIL: &str = "root@system.local";

/// IDs de roles del sistema
pub const ROLE_ADMIN_ID: &str = "role-admin";
pub const ROLE_SUPERVISOR_ID: &str = "role-supervisor";
pub const ROLE_GUARDIA_ID: &str = "role-guardia";

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_nombre_rol(nombre: &str) -> Result<(), String> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err("El nombre del rol no puede estar vacío".to_string());
    }

    if limpio.len() > 50 {
        return Err("El nombre del rol no puede exceder 50 caracteres".to_string());
    }

    // No permitir caracteres especiales problemáticos
    if limpio.contains(':') || limpio.contains('/') || limpio.contains('\\') {
        return Err("El nombre del rol no puede contener : / \\".to_string());
    }

    Ok(())
}

pub fn validar_descripcion(descripcion: Option<&String>) -> Result<(), String> {
    if let Some(desc) = descripcion {
        if desc.len() > 200 {
            return Err("La descripción no puede exceder 200 caracteres".to_string());
        }
    }
    Ok(())
}

pub fn validar_permission_id(permission_id: &str) -> Result<(), String> {
    // Formato esperado: "modulo:accion"
    if !permission_id.contains(':') {
        return Err(format!(
            "Permiso inválido: '{}'. Formato esperado: modulo:accion",
            permission_id
        ));
    }

    let parts: Vec<&str> = permission_id.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Permiso inválido: '{}'. Debe tener exactamente un ':'",
            permission_id
        ));
    }

    if parts[0].is_empty() || parts[1].is_empty() {
        return Err(format!(
            "Permiso inválido: '{}'. Módulo y acción no pueden estar vacíos",
            permission_id
        ));
    }

    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

/// Valida todos los campos necesarios para crear un rol
pub fn validar_create_input(input: &CreateRoleInput) -> Result<(), String> {
    validar_nombre_rol(&input.name)?;
    validar_descripcion(input.description.as_ref())?;

    // Validar cada permiso
    for perm_id in &input.permissions {
        validar_permission_id(perm_id)?;
    }

    Ok(())
}

/// Valida los campos presentes en un update
pub fn validar_update_input(input: &UpdateRoleInput) -> Result<(), String> {
    if let Some(ref nombre) = input.name {
        validar_nombre_rol(nombre)?;
    }

    if let Some(ref desc) = input.description {
        validar_descripcion(Some(desc))?;
    }

    if let Some(ref perms) = input.permissions {
        for perm_id in perms {
            validar_permission_id(perm_id)?;
        }
    }

    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

/// Verifica si un ID es el superuser
pub fn is_superuser(user_id: &str) -> bool {
    user_id == SUPERUSER_ID
}

/// Verifica si un rol es del sistema (no editable por admin normal)
pub fn is_system_role(role_id: &str) -> bool {
    matches!(
        role_id,
        ROLE_ADMIN_ID | ROLE_SUPERVISOR_ID | ROLE_GUARDIA_ID
    )
}

/// Normaliza el nombre de un rol
pub fn normalizar_nombre_rol(nombre: &str) -> String {
    nombre.trim().to_string()
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_nombre_rol_valido() {
        assert!(validar_nombre_rol("Supervisor Nocturno").is_ok());
        assert!(validar_nombre_rol("admin").is_ok());
    }

    #[test]
    fn test_validar_nombre_rol_invalido() {
        assert!(validar_nombre_rol("").is_err());
        assert!(validar_nombre_rol("   ").is_err());
        assert!(validar_nombre_rol("rol:invalido").is_err());
        assert!(validar_nombre_rol(&"a".repeat(51)).is_err());
    }

    #[test]
    fn test_validar_permission_id() {
        assert!(validar_permission_id("users:create").is_ok());
        assert!(validar_permission_id("lista_negra:view").is_ok());
        assert!(validar_permission_id("invalid").is_err());
        assert!(validar_permission_id(":create").is_err());
        assert!(validar_permission_id("users:").is_err());
    }

    #[test]
    fn test_is_superuser() {
        assert!(is_superuser(SUPERUSER_ID));
        assert!(!is_superuser("otro-id"));
    }

    #[test]
    fn test_is_system_role() {
        assert!(is_system_role(ROLE_ADMIN_ID));
        assert!(is_system_role(ROLE_SUPERVISOR_ID));
        assert!(is_system_role(ROLE_GUARDIA_ID));
        assert!(!is_system_role("custom-role-id"));
    }
}
