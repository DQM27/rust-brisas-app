// ==========================================
// src/services/surrealdb_authorization.rs
// ==========================================
// Servicio de autorización para SurrealDB

use crate::domain::role::is_superuser;
use crate::models::role::Module;
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use std::collections::HashSet;

/// Obtiene todos los permisos de un rol desde SurrealDB
pub async fn get_role_permissions(role_id: &str) -> Result<HashSet<String>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Normalizar role_id (quitar prefijo 'roles:' si existe)
    let normalized_role_id = role_id.strip_prefix("roles:").unwrap_or(role_id);
    println!(
        "🔍 [AUTH] Buscando permisos para role_id: '{}' (normalizado: '{}')",
        role_id, normalized_role_id
    );

    #[derive(serde::Deserialize)]
    struct PermResult {
        permission_id: String,
    }

    let mut result = client
        .query(
            r#"
            SELECT permission_id FROM role_permissions 
            WHERE role_id = $role_id
            "#,
        )
        .bind(("role_id", normalized_role_id.to_string()))
        .await?;

    let perms: Vec<PermResult> = result.take(0)?;
    println!("🔍 [AUTH] Permisos encontrados: {}", perms.len());
    Ok(perms.into_iter().map(|p| p.permission_id).collect())
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
