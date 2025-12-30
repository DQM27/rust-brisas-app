// ==========================================
// src/services/surrealdb_authorization.rs
// ==========================================
// Servicio de autorización para SurrealDB

use crate::db::surrealdb_role_queries; // Usamos queries ya implementadas
use crate::domain::role::is_superuser;
use crate::models::role::{Action, Module};
use crate::services::surrealdb_service::SurrealDbError;
// use log::info;
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

/// Obtiene permisos efectivos de un rol (incluye heredados)
pub async fn get_effective_permissions(role_id_str: &str) -> Result<HashSet<String>, AuthError> {
    let mut all_permissions = HashSet::new();
    let mut visited = HashSet::new();
    let mut current_id = Some(role_id_str.to_string());

    // Recorrer cadena de herencia (máximo 10 niveles para evitar loops)
    while let Some(id_str) = current_id.take() {
        if visited.contains(&id_str) || visited.len() >= 10 {
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
                all_permissions.extend(perms);
            }
            // Seguir cadena de herencia
            current_id = role.inherits_from.map(|r| r.to_string());
        }
    }

    Ok(all_permissions)
}

/// Obtiene todos los permisos de un rol desde SurrealDB (legacy, solo propios)
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
    // Superuser ve todo
    if is_superuser(user_id) {
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
    let perm_id = format!("{}:{}", module, action);
    Ok(permissions.contains(&perm_id))
}

/// Verifica si un usuario tiene permiso (incluye lógica de superuser y God Mode)
pub async fn check_permission(
    user_id: &str,
    role_id: &str,
    module: Module,
    action: Action,
) -> Result<(), AuthError> {
    // 1. God Mode bypassa todo (solo para operaciones internas de sistema)
    if crate::domain::role::is_god_mode() {
        log::info!(target: "audit", "[GOD_MODE] bypass para {}:{}", module.as_str(), action.as_str());
        return Ok(());
    }

    // 2. Superuser siempre tiene permiso
    if is_superuser(user_id) {
        return Ok(());
    }

    // 3. Verificar permisos efectivos (propios + heredados)
    let has = role_has_permission(role_id, module.as_str(), action.as_str())
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

    if has {
        Ok(())
    } else {
        log::warn!(target: "audit", "[PERM_DENIED] user={} perm={}:{}", user_id, module.as_str(), action.as_str());
        Err(AuthError::PermissionDenied)
    }
}
