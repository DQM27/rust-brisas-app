// ==========================================
// src/services/authorization.rs
// ==========================================
// Servicio de autorización con superuser bypass

use crate::domain::role::is_superuser;
use crate::models::role::{Action, Module};
use sqlx::SqlitePool;
use std::collections::HashSet;
use thiserror::Error;

// ==========================================
// ERRORES DE AUTORIZACIÓN
// ==========================================

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Sesión requerida")]
    SessionRequired,

    #[error("Acceso denegado: no tienes permiso para {action} en {module}")]
    Unauthorized { module: String, action: String },

    #[error("Rol no encontrado: {0}")]
    RoleNotFound(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// RE-EXPORTAR SUPERUSER CHECK
// ==========================================

pub use crate::domain::role::is_superuser as check_superuser;

// ==========================================
// VERIFICACIÓN DE PERMISOS
// ==========================================

/// Verifica si un rol tiene un permiso específico
pub async fn role_has_permission(
    pool: &SqlitePool,
    role_id: &str,
    module: Module,
    action: Action,
) -> Result<bool, AuthError> {
    let permission_id = format!("{}:{}", module.as_str(), action.as_str());

    let count: i32 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) 
        FROM role_permissions 
        WHERE role_id = ? AND permission_id = ?
        "#,
    )
    .bind(role_id)
    .bind(&permission_id)
    .fetch_one(pool)
    .await?;

    Ok(count > 0)
}

/// Obtiene todos los permisos de un rol
pub async fn get_role_permissions(
    pool: &SqlitePool,
    role_id: &str,
) -> Result<HashSet<String>, AuthError> {
    let permissions: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT permission_id 
        FROM role_permissions 
        WHERE role_id = ?
        "#,
    )
    .bind(role_id)
    .fetch_all(pool)
    .await?;

    Ok(permissions.into_iter().map(|(id,)| id).collect())
}

/// Obtiene los módulos visibles para un usuario
pub async fn get_visible_modules(
    pool: &SqlitePool,
    user_id: &str,
    role_id: &str,
) -> Result<Vec<Module>, AuthError> {
    // Superuser ve todo
    if is_superuser(user_id) {
        return Ok(Module::all());
    }

    let permissions = get_role_permissions(pool, role_id).await?;

    let visible: Vec<Module> = Module::all()
        .into_iter()
        .filter(|module| {
            let view_perm = format!("{}:view", module.as_str());
            permissions.contains(&view_perm)
        })
        .collect();

    Ok(visible)
}

/// Verifica permiso y retorna error si no autorizado
pub async fn check_permission(
    pool: &SqlitePool,
    user_id: &str,
    role_id: &str,
    module: Module,
    action: Action,
) -> Result<(), AuthError> {
    // Superuser bypass
    if is_superuser(user_id) {
        return Ok(());
    }

    let has_permission = role_has_permission(pool, role_id, module, action).await?;

    if has_permission {
        Ok(())
    } else {
        Err(AuthError::Unauthorized {
            module: module.as_str().to_string(),
            action: action.as_str().to_string(),
        })
    }
}

/// Verifica múltiples permisos (todos deben cumplirse)
pub async fn check_permissions(
    pool: &SqlitePool,
    user_id: &str,
    role_id: &str,
    requirements: &[(Module, Action)],
) -> Result<(), AuthError> {
    for (module, action) in requirements {
        check_permission(pool, user_id, role_id, *module, *action).await?;
    }
    Ok(())
}
