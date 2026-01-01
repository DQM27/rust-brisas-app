/// Gestión de Roles y Permisos (RBAC).
///
/// Este servicio constituye el motor de seguridad de la aplicación. Define qué
/// acciones puede realizar cada usuario basándose en su rol asignado.
/// Soporta herencia de roles y distingue entre roles de sistema (protegidos)
/// y roles personalizados definidos por el cliente.
use crate::db::surrealdb_role_queries as db;
use crate::domain::role::{self as domain, is_superuser};
use crate::models::role::{
    CreateRoleInput, Module, Permission, Role, RoleListResponse, RoleResponse, RoleUpdateDTO,
    UpdateRoleInput, VisibleModule,
};
use chrono::Utc;
use surrealdb::RecordId;

use crate::domain::errors::RoleError;

/// Limpia y parsea identificadores de roles, manejando la sintaxis de SurrealDB.
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

/// Obtiene todos los roles registrados, categorizándolos para la interfaz de administración.
pub async fn get_all_roles() -> Result<RoleListResponse, RoleError> {
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

    Ok(RoleListResponse {
        roles: responses,
        total,
        system_roles: system_count,
        custom_roles: total - system_count,
    })
}

pub async fn get_role_by_id(id_str: &str) -> Result<RoleResponse, RoleError> {
    let role_id = parse_role_id(id_str);
    let role = db::find_by_id(&role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?
        .ok_or(RoleError::NotFound)?;

    Ok(RoleResponse::from_role(role))
}

/// Crea un nuevo rol personalizado.
///
/// El nombre se normaliza a un 'slug' para ser usado como ID persistente,
/// asegurando que las referencias a roles en el código no se rompan
/// si el nombre visual cambia ligeramente.
pub async fn create_role(input: CreateRoleInput) -> Result<RoleResponse, RoleError> {
    domain::validar_create_input(&input)?;

    let exists =
        db::exists_by_name(&input.name).await.map_err(|e| RoleError::Database(e.to_string()))?;

    if exists {
        return Err(RoleError::NameExists);
    }

    let id_slug = domain::normalizar_nombre(&input.name);
    let dto = crate::models::role::RoleCreateDTO {
        name: domain::normalizar_nombre(&input.name),
        description: input.description,
        is_system: false, // Los roles creados por el usuario nunca son de sistema.
        inherits_from: input.inherits_from.map(|i| parse_role_id(&i)),
        permissions: input.permissions,
    };

    let created_role =
        db::create(&id_slug, dto).await.map_err(|e| RoleError::Database(e.to_string()))?;

    Ok(RoleResponse::from_role(created_role))
}

/// Actualiza los atributos o permisos de un rol existente.
///
/// Se aplica una protección estricta: los roles marcados como 'is_system'
/// (ej. Root, Admin, Guardia) tienen una estructura crítica y no pueden
/// ser modificados por usuarios convencionales, solo por un 'Superuser'.
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

    if role.is_system && !is_superuser(requester_id) {
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

/// Elimina un rol, asegurando que no sea un rol protegido por el sistema.
pub async fn delete_role(id_str: &str) -> Result<(), RoleError> {
    let role_id = parse_role_id(id_str);
    let role = db::find_by_id(&role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?
        .ok_or(RoleError::NotFound)?;

    if role.is_system {
        return Err(RoleError::CannotDeleteSystemRole);
    }

    db::delete(&role_id).await.map_err(|e| RoleError::Database(e.to_string()))?;

    Ok(())
}

/// Calcula el mapa de visibilidad de módulos para un usuario específico.
///
/// Esta función es clave para la interfaz svelte: le dice al frontend qué
/// botones mostrar y qué secciones del menú lateral deben estar disponibles.
/// El 'Superuser' siempre tiene acceso total sin importar el rol asignado.
pub async fn get_user_visible_modules(
    user_id_str: &str,
    role_id_str: &str,
) -> Result<Vec<VisibleModule>, RoleError> {
    if is_superuser(user_id_str) {
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
