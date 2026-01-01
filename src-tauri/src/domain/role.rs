/// Capa de Dominio: Gestión de Roles y Permisos de Sistema.
///
/// Este módulo define la jerarquía de accesos, constantes de roles críticos
/// y las reglas para la gestión de usuarios privilegiados ("God Mode").
use crate::domain::errors::RoleError;
use crate::models::role::{CreateRoleInput, UpdateRoleInput};
use std::sync::atomic::{AtomicBool, Ordering};

// --------------------------------------------------------------------------
// CONSTANTES DE IDENTIDAD DE ROL
// --------------------------------------------------------------------------

/// Identificador único del Superusuario raíz.
pub const SUPERUSER_ID: &str = "e0d6da3e-07a8-48c6-9304-436154b7c845";

/// Identificador único para el rol de Administrador.
pub const ROLE_ADMIN_ID: &str = "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11";

/// Identificador único para el rol de Guardia de Seguridad.
pub const ROLE_GUARDIA_ID: &str = "27221d6e-9818-430c-99c3-5694a971216b";

/// Correo electrónico reservado para el Superusuario del sistema.
pub const SUPERUSER_EMAIL: &str = "admin@brisas.local";

// --------------------------------------------------------------------------
// MECANISMO DE "GOD MODE" (SISTEMA INTERNO)
// --------------------------------------------------------------------------

/// Indicador global de privilegios elevados para operaciones críticas del sistema.
static GOD_MODE: AtomicBool = AtomicBool::new(false);

/// Activa temporalmente los privilegios de sistema.
///
/// > [!WARNING]
/// > Usar únicamente en contextos de inicialización o mantenimiento controlado.
pub fn enable_god_mode() {
    GOD_MODE.store(true, Ordering::SeqCst);
    log::warn!(target: "audit", "[GOD_MODE] ⚡ ACTIVADO - Operaciones de sistema en curso");
}

/// Restaura los niveles de seguridad normales del sistema.
pub fn disable_god_mode() {
    GOD_MODE.store(false, Ordering::SeqCst);
    log::warn!(target: "audit", "[GOD_MODE] 🔒 DESACTIVADO");
}

/// Consulta si el sistema se encuentra actualmente en estado de privilegios elevados.
pub fn is_god_mode() -> bool {
    GOD_MODE.load(Ordering::SeqCst)
}

/// Autoridad Unificada de Sistema (God Mode).
///
/// Esta es la única fuente de verdad para determinar si una operación
/// tiene privilegios de sistema.
///
/// Se otorga autoridad si:
/// 1. El modo Dios global está activo (`is_god_mode`).
/// 2. El ID de usuario corresponde al Superusuario raíz.
pub fn has_god_authority(user_id: Option<&str>) -> bool {
    if is_god_mode() {
        return true;
    }

    if let Some(id) = user_id {
        return id == SUPERUSER_ID;
    }

    false
}

/// Estructura RAII para garantizar la desactivación automática del God Mode al salir de un scope.
pub struct GodModeGuard;

impl GodModeGuard {
    /// Activa el God Mode y retorna un guard que lo desactivará al destruirse.
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

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida los requisitos de nomenclatura para un nuevo rol.
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

/// Valida la descripción administrativa asociada a un rol.
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

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida los datos necesarios para la creación de un nuevo rol personalizado.
pub fn validar_create_input(input: &CreateRoleInput) -> Result<(), RoleError> {
    validar_nombre(&input.name)?;
    validar_descripcion(input.description.as_ref())?;
    Ok(())
}

/// Valida las actualizaciones sobre un rol existente.
pub fn validar_update_input(input: &UpdateRoleInput) -> Result<(), RoleError> {
    if let Some(ref name) = input.name {
        validar_nombre(name)?;
    }

    validar_descripcion(input.description.as_ref())?;
    Ok(())
}

// --------------------------------------------------------------------------
// SEGURIDAD DE ROLES DE SISTEMA
// --------------------------------------------------------------------------

/// Protege los roles críticos del sistema contra modificaciones o eliminaciones accidentales.
pub fn check_system_role_modification(role_id: &str) -> Result<(), RoleError> {
    if role_id == SUPERUSER_ID || role_id == ROLE_ADMIN_ID || role_id == ROLE_GUARDIA_ID {
        return Err(RoleError::SystemRole);
    }
    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Normaliza el nombre del rol a mayúsculas para asegurar unicidad en el sistema.
pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_uppercase()
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proteccion_roles_sistema() {
        assert!(check_system_role_modification(SUPERUSER_ID).is_err());
        assert!(check_system_role_modification("rol-personalizado").is_ok());
    }

    #[test]
    fn test_normalizacion_rol() {
        assert_eq!(normalizar_nombre("  admin  "), "ADMIN");
    }
}
