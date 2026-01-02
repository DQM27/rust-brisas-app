//! # Queries SurrealDB: Roles
//!
//! Operaciones de base de datos para gestión de roles y permisos.
//!
//! ## Responsabilidades
//! - CRUD de roles
//! - Consulta de permisos por rol
//! - Verificación de existencia por nombre
//!
//! ## Tabla: `role`

use crate::models::role::{Role, RoleCreateDTO, RoleUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::{debug, info, warn};
use surrealdb::RecordId;

/// Obtiene todos los roles ordenados por sistema/nombre.
pub async fn find_all() -> Result<Vec<Role>, SurrealDbError> {
    debug!("📋 Consultando todos los roles");
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM role ORDER BY is_system DESC, name ASC").await?;
    Ok(result.take(0)?)
}

/// Busca un rol por su ID.
pub async fn find_by_id(id: &RecordId) -> Result<Option<Role>, SurrealDbError> {
    debug!("🔍 Buscando rol: {}", id);
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

/// Obtiene los permisos de un rol.
pub async fn get_permissions(role_id: &RecordId) -> Result<Vec<String>, SurrealDbError> {
    debug!("📋 Consultando permisos para rol: {}", role_id);
    let db = get_db().await?;
    let mut result = db.query("SELECT permissions FROM $id").bind(("id", role_id.clone())).await?;

    #[derive(serde::Deserialize)]
    struct RolePermissionsRow {
        permissions: Option<Vec<String>>,
    }

    let row: Option<RolePermissionsRow> = result.take(0)?;
    let perms = row.and_then(|r| r.permissions).unwrap_or_default();
    Ok(perms)
}

/// Crea un nuevo rol con ID personalizado.
pub async fn create(id: &str, dto: RoleCreateDTO) -> Result<Role, SurrealDbError> {
    debug!("➕ Creando rol: {}", id);
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            CREATE type::thing('role', $id) CONTENT $dto
        "#,
        )
        .bind(("id", id.to_string()))
        .bind(("dto", dto))
        .await?;

    let created: Option<Role> = result.take(0)?;
    match created {
        Some(role) => {
            info!("✅ Rol creado: id={}, name={}", role.id, role.name);
            Ok(role)
        }
        None => {
            warn!("⚠️ Error al crear rol: {}", id);
            Err(SurrealDbError::Query("Error creando rol".to_string()))
        }
    }
}

/// Actualiza un rol existente.
pub async fn update(id: &RecordId, dto: RoleUpdateDTO) -> Result<Option<Role>, SurrealDbError> {
    debug!("✏️ Actualizando rol: {}", id);
    let db = get_db().await?;
    let result: Option<Role> = db.update(id.clone()).merge(dto).await?;
    if result.is_some() {
        info!("✅ Rol actualizado: {}", id);
    } else {
        warn!("⚠️ Rol no encontrado para actualizar: {}", id);
    }
    Ok(result)
}

/// Elimina un rol.
pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    warn!("🗑️ Eliminando rol: {}", id);
    let db = get_db().await?;
    db.query("DELETE $id").bind(("id", id.clone())).await?;
    Ok(())
}

/// Verifica si existe un rol con el nombre dado.
pub async fn exists_by_name(name: &str) -> Result<bool, SurrealDbError> {
    debug!("🔍 Verificando existencia de rol: {}", name);
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM role WHERE name = $name GROUP ALL")
        .bind(("name", name.to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    struct Count {
        count: i64,
    }

    let rows: Vec<Count> = result.take(0)?;
    Ok(rows.first().map(|c| c.count > 0).unwrap_or(false))
}
