/// Puertos de Entrada: Gestión de Roles y Permisos RBAC (Authorization Bridge).
///
/// Este módulo es el cerebro de la seguridad de la aplicación. Gestiona la
/// jerarquía de roles y los permisos granulares, permitiendo al frontend
/// adaptar su interfaz de manera reactiva según las capacidades del usuario.
use crate::domain::errors::RoleError;
use crate::models::role::{
    CreateRoleInput, Permission, RoleListResponse, RoleResponse, UpdateRoleInput, VisibleModule,
};
use crate::services::role_service;
use crate::services::session::SessionState;
use crate::services::surrealdb_authorization;
use tauri::State;

// ==========================================
// CONSULTAS DE AUTORIZACIÓN
// ==========================================

/// Recupera el catálogo completo de niveles de acceso definidos.
#[tauri::command]
pub async fn get_all_roles(
    session: State<'_, SessionState>,
) -> Result<RoleListResponse, RoleError> {
    require_perm!(session, "roles:read")?;
    role_service::get_all_roles().await
}

#[tauri::command]
pub async fn get_role_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<RoleResponse, RoleError> {
    require_perm!(session, "roles:read")?;
    role_service::get_role_by_id(&id).await
}

/// Lista todas las acciones granulares (Ej: 'users:create', 'ingresos:read') del sistema.
#[tauri::command]
pub async fn get_all_permissions(
    session: State<'_, SessionState>,
) -> Result<Vec<Permission>, RoleError> {
    require_perm!(session, "roles:read")?;
    role_service::get_all_permissions().await
}

/// Orquestador Reactivo: Determina qué módulos de la UI debe mostrar el frontend
/// según los permisos efectivos del usuario actual. Ahorra lógica compleja en Svelte.
#[tauri::command]
pub async fn get_visible_modules(
    session: State<'_, SessionState>,
) -> Result<Vec<VisibleModule>, RoleError> {
    let user = session
        .get_user()
        .ok_or(RoleError::Unauthorized("Sesión requerida para calcular visibilidad".to_string()))?;

    let modules = surrealdb_authorization::get_visible_modules(&user.id, &user.role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?;

    // Obtener permisos efectivos (propios + heredados) para cada módulo
    let permissions = surrealdb_authorization::get_effective_permissions(&user.role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?;

    let visible: Vec<VisibleModule> = modules
        .into_iter()
        .map(|m| VisibleModule {
            module: m.as_str().to_string(),
            display_name: m.display_name().to_string(),
            can_create: permissions.contains(&format!("{}:create", m.as_str())),
            can_read: permissions.contains(&format!("{}:read", m.as_str())),
            can_update: permissions.contains(&format!("{}:update", m.as_str())),
            can_delete: permissions.contains(&format!("{}:delete", m.as_str())),
            can_export: permissions.contains(&format!("{}:export", m.as_str())),
        })
        .collect();

    Ok(visible)
}

// ==========================================
// OPERACIONES DE SEGURIDAD (MUTACIONES)
// ==========================================

/// Define un nuevo nivel de acceso con un conjunto inicial de permisos.
#[tauri::command]
pub async fn create_role(
    session: State<'_, SessionState>,
    input: CreateRoleInput,
) -> Result<RoleResponse, RoleError> {
    require_perm!(session, "roles:create", "Registrando nueva jerarquía de seguridad (Rol)")?;
    role_service::create_role(input).await
}

/// Modifica los permisos granulares de un rol, afectando inmediatamente a los usuarios vinculados.
#[tauri::command]
pub async fn update_role(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateRoleInput,
) -> Result<RoleResponse, RoleError> {
    require_perm!(
        session,
        "roles:update",
        format!("Reestructurando permisos para el rol ID: {}", id)
    )?;
    let user = session
        .get_user()
        .ok_or(RoleError::Unauthorized("Sesión administrativa requerida".to_string()))?;
    role_service::update_role(&id, input, &user.id).await
}

/// Elimina un rol, siempre que no tenga usuarios activos vinculados (protección de integridad).
#[tauri::command]
pub async fn delete_role(session: State<'_, SessionState>, id: String) -> Result<(), RoleError> {
    require_perm!(session, "roles:delete", format!("Dando de baja jerarquía de acceso {}", id))?;
    role_service::delete_role(&id).await
}
