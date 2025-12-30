// ==========================================
// src/domain/role.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::domain::errors::RoleError;
use crate::models::role::{CreateRoleInput, UpdateRoleInput};

// ==========================================
// CONSTANTES
// ==========================================

pub const SUPERUSER_ID: &str = "e0d6da3e-07a8-48c6-9304-436154b7c845";
pub const ROLE_ADMIN_ID: &str = "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11";
pub const ROLE_GUARDIA_ID: &str = "27221d6e-9818-430c-99c3-5694a971216b";
pub const ROLE_GUARDIA_SENIOR_ID: &str = "9f27c73a-6c1b-4d7a-8dcf-7a54a01c801e";
pub const ROLE_SUPERVISOR_ID: &str = "75438848-185d-400e-953a-7a54a01c801e";
pub const SUPERUSER_EMAIL: &str = "admin@brisas.local";

pub fn is_superuser(user_id: &str) -> bool {
    // Logic to check if user is superuser. Usually checks if ID is SUPERUSER_ID? or if user has ROLE?
    // In role_service.rs usage: is_superuser(requester_id) implies checking if they ARE the superuser user.
    // If it's by ID, then just check ID. But usually strictly "superuser" role.
    // Assuming implementation checks against SUPERUSER_ID or user email?
    // Let's assume user_id check.
    // Wait, create_superuser uses fixed ID.
    user_id == SUPERUSER_ID
}

// ==========================================
// GOD MODE (INTERNAL ONLY)
// ==========================================

use std::sync::atomic::{AtomicBool, Ordering};

/// Flag global de God Mode (solo activable por proceso interno)
static GOD_MODE: AtomicBool = AtomicBool::new(false);

/// Activa God Mode temporalmente (solo para operaciones de sistema)
pub fn enable_god_mode() {
    GOD_MODE.store(true, Ordering::SeqCst);
    log::warn!(target: "audit", "[GOD_MODE] ⚡ ENABLED - Operaciones de sistema activas");
}

pub fn disable_god_mode() {
    GOD_MODE.store(false, Ordering::SeqCst);
    log::warn!(target: "audit", "[GOD_MODE] 🔒 DISABLED");
}

pub fn is_god_mode() -> bool {
    GOD_MODE.load(Ordering::SeqCst)
}

/// Guard para God Mode que se desactiva automáticamente al salir del scope (RAII)
pub struct GodModeGuard;

impl GodModeGuard {
    pub fn activate() -> Self {
        enable_god_mode();
        Self
    }
}

impl Drop for GodModeGuard {
    fn drop(&mut self) {
        disable_god_mode();
    }
}

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

pub fn validar_nombre(nombre: &str) -> Result<(), RoleError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(RoleError::Validation("El nombre del rol no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(RoleError::Validation(
            "El nombre del rol no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

pub fn validar_descripcion(descripcion: Option<&String>) -> Result<(), RoleError> {
    if let Some(desc) = descripcion {
        let limpio = desc.trim();
        if !limpio.is_empty() && limpio.len() > 200 {
            return Err(RoleError::Validation(
                "La descripción no puede exceder 200 caracteres".to_string(),
            ));
        }
    }
    Ok(())
}

// ==========================================
// VALIDACIONES DE INPUTS COMPLETOS
// ==========================================

pub fn validar_create_input(input: &CreateRoleInput) -> Result<(), RoleError> {
    validar_nombre(&input.name)?;
    validar_descripcion(input.description.as_ref())?;
    Ok(())
}

pub fn validar_update_input(input: &UpdateRoleInput) -> Result<(), RoleError> {
    if let Some(ref name) = input.name {
        validar_nombre(name)?;
    }

    validar_descripcion(input.description.as_ref())?;
    Ok(())
}

// ==========================================
// VALIDACIONES DE SISTEMA
// ==========================================

pub fn check_system_role_modification(role_id: &str) -> Result<(), RoleError> {
    if role_id == SUPERUSER_ID || role_id == ROLE_ADMIN_ID || role_id == ROLE_GUARDIA_ID {
        return Err(RoleError::SystemRole);
    }
    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_uppercase()
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_superuser() {
        assert!(is_superuser(SUPERUSER_ID));
        assert!(!is_superuser("anyone-else"));
    }

    #[test]
    fn test_validar_nombre_valido() {
        assert!(validar_nombre("Guardia").is_ok());
    }

    #[test]
    fn test_validar_nombre_vacio() {
        let result = validar_nombre("   ");
        assert!(result.is_err());
        match result.unwrap_err() {
            RoleError::Validation(msg) => assert!(msg.contains("vacío")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validar_descripcion_larga() {
        let long_desc = "a".repeat(201);
        let result = validar_descripcion(Some(&long_desc));
        assert!(result.is_err());
        match result.unwrap_err() {
            RoleError::Validation(msg) => assert!(msg.contains("exceder 200")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_check_system_role_modification() {
        assert!(check_system_role_modification(SUPERUSER_ID).is_err());
        assert!(check_system_role_modification(ROLE_ADMIN_ID).is_err());
        assert!(check_system_role_modification(ROLE_GUARDIA_ID).is_err());
        assert!(check_system_role_modification("custom-role").is_ok());
    }

    #[test]
    fn test_normalizar_nombre() {
        assert_eq!(normalizar_nombre("  admin  "), "ADMIN");
    }
}
