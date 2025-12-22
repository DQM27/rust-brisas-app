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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::Executor;
    use uuid::Uuid;

    async fn setup_test_env() -> SqlitePool {
        let db_id = Uuid::new_v4().to_string();
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&format!("sqlite:file:{}?mode=memory&cache=shared", db_id))
            .await
            .unwrap();

        pool.execute(
            r#"
            CREATE TABLE role_permissions (
                role_id TEXT NOT NULL,
                permission_id TEXT NOT NULL,
                PRIMARY KEY (role_id, permission_id)
            );
            
            INSERT INTO role_permissions (role_id, permission_id) VALUES ('admin', 'contratistas:view');
            INSERT INTO role_permissions (role_id, permission_id) VALUES ('admin', 'contratistas:create');
            INSERT INTO role_permissions (role_id, permission_id) VALUES ('user', 'contratistas:view');
            "#,
        )
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_role_has_permission() {
        let pool = setup_test_env().await;

        assert!(
            role_has_permission(&pool, "admin", Module::Contratistas, Action::View)
                .await
                .unwrap()
        );
        assert!(
            role_has_permission(&pool, "admin", Module::Contratistas, Action::Create)
                .await
                .unwrap()
        );
        assert!(
            role_has_permission(&pool, "user", Module::Contratistas, Action::View)
                .await
                .unwrap()
        );
        assert!(
            !role_has_permission(&pool, "user", Module::Contratistas, Action::Create)
                .await
                .unwrap()
        );
        assert!(
            !role_has_permission(&pool, "guest", Module::Contratistas, Action::View)
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_get_role_permissions() {
        let pool = setup_test_env().await;

        let admin_perms = get_role_permissions(&pool, "admin").await.unwrap();
        assert!(admin_perms.contains("contratistas:view"));
        assert!(admin_perms.contains("contratistas:create"));
        assert_eq!(admin_perms.len(), 2);

        let user_perms = get_role_permissions(&pool, "user").await.unwrap();
        assert!(user_perms.contains("contratistas:view"));
        assert_eq!(user_perms.len(), 1);
    }

    #[tokio::test]
    async fn test_check_permission_normal_user() {
        let pool = setup_test_env().await;

        // Autorizado
        let res = check_permission(&pool, "u-1", "admin", Module::Contratistas, Action::View).await;
        assert!(res.is_ok());

        // No autorizado
        let res =
            check_permission(&pool, "u-1", "user", Module::Contratistas, Action::Create).await;
        assert!(matches!(res, Err(AuthError::Unauthorized { .. })));
    }

    #[tokio::test]
    async fn test_check_permission_superuser_bypass() {
        let pool = setup_test_env().await;

        let res = check_permission(
            &pool,
            crate::domain::role::SUPERUSER_ID,
            "none",
            Module::Users,
            Action::Delete,
        )
        .await;
        assert!(res.is_ok(), "Superuser should bypass all permission checks");
    }

    #[tokio::test]
    async fn test_get_visible_modules() {
        let pool = setup_test_env().await;

        let modules = get_visible_modules(&pool, "u-1", "user").await.unwrap();
        assert!(modules.contains(&Module::Contratistas));
        // Depende de si otros módulos están en el setup.
    }
}
