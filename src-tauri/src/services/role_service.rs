//! # Servicio: Gestión de Roles y Permisos (RBAC)
//!
//! Motor de seguridad de la aplicación que define qué acciones puede realizar
//! cada usuario basándose en su rol asignado.
//!
//! ## Responsabilidades
//! - CRUD de roles personalizados
//! - Protección de roles de sistema (Root, Admin, Guardia)
//! - Cálculo de visibilidad de módulos para UI
//! - Generación de permisos granulares
//!
//! ## Dependencias
//! - `domain::role` - Validaciones y constantes (GOD_ID, etc.)
//! - `db::surrealdb_role_queries` - Persistencia
//! - `surrealdb_authorization` - Permisos efectivos

use crate::db::surrealdb_role_queries as db;
use crate::domain::role::{self as domain};
use crate::models::role::{
    CreateRoleInput, Module, Permission, Role, RoleListResponse, RoleResponse, RoleUpdateDTO,
    UpdateRoleInput, VisibleModule,
};
use chrono::Utc;
use log::{debug, info, warn};
use surrealdb::RecordId;

use crate::domain::errors::RoleError;

// --------------------------------------------------------------------------
// HELPERS INTERNOS
// --------------------------------------------------------------------------

/// Parsea un ID de rol (acepta "role:id" o "id").
fn parse_role_id(id_str: &str) -> RecordId {
    let clean_id = id_str
        .trim_start_matches("⟨")
        .trim_end_matches("⟩")
        .trim_start_matches('<')
        .trim_end_matches('>');

    if clean_id.contains(':') {
        let parts: Vec<&str> = clean_id.split(':').collect();
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

// --------------------------------------------------------------------------
// OPERACIONES DE CONSULTA
// --------------------------------------------------------------------------

/// Obtiene todos los roles registrados, categorizándolos para la interfaz de administración.
pub async fn get_all_roles() -> Result<RoleListResponse, RoleError> {
    debug!("📋 Consultando todos los roles");
    let roles: Vec<Role> = db::find_all().await.map_err(|e| RoleError::Database(e.to_string()))?;

    let mut responses = Vec::new();
    let mut system_count = 0;

    for role in roles {
        if role.is_system {
            system_count += 1;
        }
        responses.push(RoleResponse::from_role(role));
    }

    let total = responses.len();
    debug!(
        "✅ Roles encontrados: {} total, {} sistema, {} custom",
        total,
        system_count,
        total - system_count
    );

    Ok(RoleListResponse {
        roles: responses,
        total,
        system_roles: system_count,
        custom_roles: total - system_count,
    })
}

/// Obtiene un rol por su ID.
pub async fn get_role_by_id(id_str: &str) -> Result<RoleResponse, RoleError> {
    debug!("🔍 Buscando rol: {}", id_str);
    let role_id = parse_role_id(id_str);
    let role = db::find_by_id(&role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?
        .ok_or_else(|| {
            warn!("⚠️ Rol no encontrado: {}", id_str);
            RoleError::NotFound
        })?;

    Ok(RoleResponse::from_role(role))
}

/// Crea un nuevo rol personalizado.
///
/// El nombre se normaliza a un 'slug' para ser usado como ID persistente.
pub async fn create_role(input: CreateRoleInput) -> Result<RoleResponse, RoleError> {
    debug!("➕ Creando rol: {}", input.name);
    domain::validar_create_input(&input)?;

    let exists =
        db::exists_by_name(&input.name).await.map_err(|e| RoleError::Database(e.to_string()))?;

    if exists {
        warn!("⚠️ Rol ya existe: {}", input.name);
        return Err(RoleError::NameExists);
    }

    let id_slug = domain::normalizar_nombre(&input.name);
    let dto = crate::models::role::RoleCreateDTO {
        name: domain::normalizar_nombre(&input.name),
        description: input.description,
        is_system: false,
        inherits_from: input.inherits_from.map(|i| parse_role_id(&i)),
        permissions: input.permissions,
    };

    let created_role =
        db::create(&id_slug, dto).await.map_err(|e| RoleError::Database(e.to_string()))?;

    info!("✅ Rol creado: id={}, name={}", created_role.id, created_role.name);
    Ok(RoleResponse::from_role(created_role))
}

/// Actualiza los atributos o permisos de un rol existente.
///
/// Se aplica una protección estricta: los roles marcados como 'is_system'
/// (ej. Root, Admin, Guardia) tienen una estructura crítica y no pueden
/// ser modificados por usuarios convencionales, solo por un usuario con 'God' authority.
pub async fn update_role(
    id_str: &str,
    input: UpdateRoleInput,
    requester_id: &str,
) -> Result<RoleResponse, RoleError> {
    domain::validar_update_input(&input)?;

    let role_id = parse_role_id(id_str);

    let role = db::find_by_id(&role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?
        .ok_or(RoleError::NotFound)?;

    if role.is_system && !domain::has_god_authority(Some(requester_id)) {
        return Err(RoleError::CannotModifySystemRole);
    }

    let mut dto = RoleUpdateDTO::default();
    if let Some(n) = input.name {
        dto.name = Some(domain::normalizar_nombre(&n));
    }
    if let Some(d) = input.description {
        dto.description = Some(d);
    }
    if let Some(i) = input.inherits_from {
        dto.inherits_from = Some(parse_role_id(&i));
    }
    if let Some(p) = input.permissions {
        dto.permissions = Some(p);
    }
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    let updated = db::update(&role_id, dto)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?
        .ok_or(RoleError::NotFound)?;

    Ok(RoleResponse::from_role(updated))
}

/// Elimina un rol no protegido del sistema.
pub async fn delete_role(id_str: &str) -> Result<(), RoleError> {
    debug!("🗑️ Eliminando rol: {}", id_str);
    let role_id = parse_role_id(id_str);
    let role = db::find_by_id(&role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?
        .ok_or(RoleError::NotFound)?;

    if role.is_system {
        warn!("⚠️ Intento de eliminar rol de sistema: {}", id_str);
        return Err(RoleError::CannotDeleteSystemRole);
    }

    db::delete(&role_id).await.map_err(|e| RoleError::Database(e.to_string()))?;
    warn!("🗑️ Rol eliminado: {}", id_str);

    Ok(())
}

/// Calcula el mapa de visibilidad de módulos para un usuario específico.
///
/// Esta función es clave para la interfaz svelte: le dice al frontend qué
/// botones mostrar y qué secciones del menú lateral deben estar disponibles.
/// El 'God Mode' (por estado o identidad) siempre tiene acceso total.
pub async fn get_user_visible_modules(
    user_id_str: &str,
    role_id_str: &str,
) -> Result<Vec<VisibleModule>, RoleError> {
    if domain::has_god_authority(Some(user_id_str)) {
        return Ok(Module::all()
            .into_iter()
            .map(|m| VisibleModule {
                module: m.as_str().to_string(),
                display_name: m.display_name().to_string(),
                can_create: true,
                can_read: true,
                can_update: true,
                can_delete: true,
                can_export: true,
            })
            .collect());
    }

    use crate::services::surrealdb_authorization;

    // Recuperamos los permisos consolidados (incluyendo herencia si existe).
    let permissions = surrealdb_authorization::get_role_permissions(role_id_str)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?;

    let mut modules = Vec::new();

    for module in Module::all() {
        // La visibilidad de un módulo depende del permiso 'view'.
        // Si no puede ver el módulo, no tiene sentido calcular los demás permisos.
        let view_perm = format!("{}:view", module.as_str());

        if permissions.contains(&view_perm) {
            modules.push(VisibleModule {
                module: module.as_str().to_string(),
                display_name: module.display_name().to_string(),
                can_create: permissions.contains(&format!("{}:create", module.as_str())),
                can_read: permissions.contains(&format!("{}:read", module.as_str())),
                can_update: permissions.contains(&format!("{}:update", module.as_str())),
                can_delete: permissions.contains(&format!("{}:delete", module.as_str())),
                can_export: permissions.contains(&format!("{}:export", module.as_str())),
            });
        }
    }

    Ok(modules)
}

/// Genera la lista completa de permisos granulares disponibles en el sistema.
/// Se utiliza en el panel de creación/edición de roles.
pub async fn get_all_permissions() -> Result<Vec<Permission>, RoleError> {
    let mut perms = Vec::new();

    for module in Module::all() {
        // Para cada módulo (Usuarios, Empresas, etc.), definimos el set estándar de acciones.
        let actions = vec!["view", "create", "read", "update", "delete", "export"];
        for action in actions {
            perms.push(Permission {
                id: format!("{}:{}", module.as_str(), action),
                module: module.as_str().to_string(),
                action: action.to_string(),
                description: Some(format!("{} {}", action, module.display_name())),
            });
        }
    }

    Ok(perms)
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_role_id_simple() {
        let id = parse_role_id("admin");
        assert_eq!(id.table().to_string(), "role");
        assert_eq!(id.key().to_string(), "admin");
    }

    #[test]
    fn test_parse_role_id_con_prefijo() {
        let id = parse_role_id("role:admin");
        assert_eq!(id.table().to_string(), "role");
        assert_eq!(id.key().to_string(), "admin");
    }

    #[test]
    fn test_parse_role_id_con_brackets() {
        let id = parse_role_id("role:⟨custom-role⟩");
        assert_eq!(id.table().to_string(), "role");
        assert_eq!(id.key().to_string(), "custom-role");
    }
}
