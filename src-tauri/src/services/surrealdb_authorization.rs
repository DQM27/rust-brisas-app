//! # Servicio: Autorización Basada en Roles (RBAC)
//!
//! Implementa la lógica de verificación de permisos dinámicos, incluyendo la
//! resolución de la cadena de herencia de roles y la autoridad especial "God Mode".
//!
//! ## Características
//! - Herencia de roles recursiva (hasta 10 niveles).
//! - Caché reactiva de permisos mediante `HashSet`.
//! - Integración con Auditoría (Trazas de acceso denegado/bypass).

use crate::db::surrealdb_role_queries; // Usamos queries ya implementadas
/// Capa de Dominio no necesaria aquí directamente si usamos `has_god_authority`
use crate::models::role::{Action, Module};
use crate::services::surrealdb_service::SurrealDbError;
use log::{debug, info, warn};
use std::collections::HashSet;
use surrealdb::RecordId;

/// Helper para parsear ID de rol (acepta con o sin prefijo)
fn parse_role_id(id_str: &str) -> RecordId {
    let clean_id = id_str
        .trim_start_matches("⟨")
        .trim_end_matches("⟩")
        .trim_start_matches('<')
        .trim_end_matches('>');

    if clean_id.contains(':') {
        let parts: Vec<&str> = clean_id.split(':').collect();
        // Asegurarse de limpiar también la parte del ID si vino con brackets internos
        let key = parts[1]
            .trim_start_matches("⟨")
            .trim_end_matches("⟩")
            .trim_start_matches('<')
            .trim_end_matches('>');
        RecordId::from_table_key(parts[0], key)
    } else {
        RecordId::from_table_key("role", clean_id)
    }
}

/// Define errores de autorización (compatible con legacy `AuthError` si se desea)
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Permiso denegado")]
    PermissionDenied,
    #[error("Sesión requerida")]
    SessionRequired,
    #[error("Error de base de datos: {0}")]
    Database(String),
}

impl From<SurrealDbError> for AuthError {
    fn from(e: SurrealDbError) -> Self {
        Self::Database(e.to_string())
    }
}

/// Obtiene permisos efectivos de un rol (incluye heredados)
pub async fn get_effective_permissions(role_id_str: &str) -> Result<HashSet<String>, AuthError> {
    let mut all_permissions = HashSet::new();
    let mut visited = HashSet::new();
    let mut current_id = Some(role_id_str.to_string());

    debug!("🕸️ Resolviendo herencia de permisos para: {role_id_str}");

    // Recorrer cadena de herencia (máximo 10 niveles para evitar loops)
    while let Some(id_str) = current_id.take() {
        if visited.contains(&id_str) || visited.len() >= 10 {
            if visited.len() >= 10 {
                warn!("⚠️ Límite de herencia de roles alcanzado (10) para: {role_id_str}");
            }
            break; // Prevenir ciclos infinitos
        }
        visited.insert(id_str.clone());

        let role_id = parse_role_id(&id_str);
        let role = surrealdb_role_queries::find_by_id(&role_id)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;

        if let Some(role) = role {
            // Agregar permisos propios
            if let Some(perms) = role.permissions {
                debug!("  ├─ Rol: {} -> +{} permisos", role.name, perms.len());
                all_permissions.extend(perms);
            }
            // Seguir cadena de herencia
            current_id = role.inherits_from.map(|r| r.to_string());
        }
    }

    debug!("✨ Total permisos encontrados: {}", all_permissions.len());
    Ok(all_permissions)
}

/// Obtiene todos los permisos de un rol desde `SurrealDB` (legacy, solo propios)
pub async fn get_role_permissions(role_id_str: &str) -> Result<HashSet<String>, SurrealDbError> {
    let role_id = parse_role_id(role_id_str);
    let perms = surrealdb_role_queries::get_permissions(&role_id).await?;
    Ok(perms.into_iter().collect())
}

/// Obtiene los módulos visibles para un usuario
pub async fn get_visible_modules(
    user_id: &str,
    role_id: &str,
) -> Result<Vec<Module>, SurrealDbError> {
    // La autoridad de God Mode (por estado o por identidad) ve todo
    if crate::domain::role::has_god_authority(Some(user_id)) {
        return Ok(Module::all());
    }

    let permissions = get_effective_permissions(role_id)
        .await
        .map_err(|e| SurrealDbError::Query(e.to_string()))?;

    let visible: Vec<Module> = Module::all()
        .into_iter()
        .filter(|module| {
            let view_perm = format!("{}:view", module.as_str());
            permissions.contains(&view_perm)
        })
        .collect();

    Ok(visible)
}

/// Verifica si un rol tiene un permiso específico
pub async fn role_has_permission(
    role_id: &str,
    module: &str,
    action: &str,
) -> Result<bool, SurrealDbError> {
    let permissions = get_effective_permissions(role_id)
        .await
        .map_err(|e| SurrealDbError::Query(e.to_string()))?;
    let perm_id = format!("{module}:{action}");
    Ok(permissions.contains(&perm_id))
}

/// Verifica si un usuario tiene permiso (incluye lógica de usuario God y God Mode)
pub async fn check_permission(
    user_id: &str,
    role_id: &str,
    module: Module,
    action: Action,
) -> Result<(), AuthError> {
    // La autoridad de God Mode (por estado o por identidad) bypassa todo
    if crate::domain::role::has_god_authority(Some(user_id)) {
        info!(target: "audit", "[GOD_MODE] bypass para {}:{}", module.as_str(), action.as_str());
        return Ok(());
    }

    // 3. Verificar permisos efectivos (propios + heredados)
    let has = role_has_permission(role_id, module.as_str(), action.as_str())
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

    if has {
        Ok(())
    } else {
        warn!(target: "audit", "[PERM_DENIED] user={} perm={}:{}", user_id, module.as_str(), action.as_str());
        Err(AuthError::PermissionDenied)
    }
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_role_id_variants() {
        let cases = vec![
            ("admin", "role", "admin"),
            ("role:admin", "role", "admin"),
            ("⟨role:admin⟩", "role", "admin"),
            ("<role:admin>", "role", "admin"),
            ("⟨admin⟩", "role", "admin"),
        ];

        for (input, expected_table, expected_id) in cases {
            let res = parse_role_id(input);
            assert_eq!(res.table(), expected_table, "Fallo en tabla para {}", input);
            assert_eq!(res.key().to_string(), expected_id, "Fallo en ID para {}", input);
        }
    }
}
