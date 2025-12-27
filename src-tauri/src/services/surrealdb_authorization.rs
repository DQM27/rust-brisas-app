// ==========================================
// src/services/surrealdb_authorization.rs
// ==========================================
// Servicio de autorización para SurrealDB

use crate::db::surrealdb_role_queries; // Usamos queries ya implementadas
use crate::domain::role::is_superuser;
use crate::models::role::{Action, Module};
use crate::services::surrealdb_service::SurrealDbError;
use std::collections::HashSet;

/// Define errores de autorización (compatible con legacy AuthError si se desea)
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
        AuthError::Database(e.to_string())
    }
}

/// Obtiene todos los permisos de un rol desde SurrealDB
pub async fn get_role_permissions(role_id: &str) -> Result<HashSet<String>, SurrealDbError> {
    // Delegamos a la query que sabe leer el array de permisos
    let perms = surrealdb_role_queries::get_permissions(role_id).await?;
    Ok(perms.into_iter().collect())
}

/// Obtiene los módulos visibles para un usuario
pub async fn get_visible_modules(
    user_id: &str,
    role_id: &str,
) -> Result<Vec<Module>, SurrealDbError> {
    // Superuser ve todo
    if is_superuser(user_id) {
        return Ok(Module::all());
    }

    let permissions = get_role_permissions(role_id).await?;

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
    let permissions = get_role_permissions(role_id).await?;
    let perm_id = format!("{}:{}", module, action);
    Ok(permissions.contains(&perm_id))
}

/// Verifica si un usuario tiene permiso (incluye lógica de superuser)
pub async fn check_permission(
    user_id: &str,
    role_id: &str,
    module: Module,
    action: Action,
) -> Result<(), AuthError> {
    if is_superuser(user_id) {
        return Ok(());
    }

    let has = role_has_permission(role_id, module.as_str(), action.as_str()).await?;
    if has {
        Ok(())
    } else {
        Err(AuthError::PermissionDenied)
    }
}
