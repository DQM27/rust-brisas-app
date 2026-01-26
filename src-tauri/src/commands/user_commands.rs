/// Puertos de Entrada: Gestión de Identidad y Seguridad de Usuarios (Identity Bridge).
///
/// Este módulo centraliza las operaciones críticas de administración de usuarios,
/// procesos de autenticación (Login) y la gestión de recursos biométricos (Avatares)
/// mediante un flujo seguro que integra el estado de sesión (RBAC).
use crate::domain::errors::UserError;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, SessionUser, UpdateUserInput, UserListResponse,
    UserResponse,
};
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use crate::services::user_service;
use std::sync::Arc;
use tauri::State;

// ==========================================
// ADMISTRACIÓN DE USUARIOS
// ==========================================

/// Registra un nuevo operador en el sistema.
/// Sincroniza automáticamente con el motor de búsqueda.
#[tauri::command]
pub async fn create_user(
    session: State<'_, SessionState>,
    search: State<'_, Arc<SearchService>>,
    input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    require_perm!(session, "users:create", "Creando nuevo usuario operativo")?;
    user_service::create_user(&search, input).await
}

/// Actualiza el perfil del usuario.
/// Requiere permisos elevados 'users:update'.
#[tauri::command]
pub async fn update_user(
    session: State<'_, SessionState>,
    search: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    require_perm!(session, "users:update", format!("Actualizando perfil de usuario ID: {}", id))?;
    user_service::update_user(&search, id, input).await
}

#[tauri::command]
pub async fn delete_user(
    session: State<'_, SessionState>,
    search: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), UserError> {
    require_perm!(session, "users:delete", format!("Eliminando usuario ID: {}", id))?;
    user_service::delete_user(&search, id).await
}

#[tauri::command]
pub async fn restore_user(
    session: State<'_, SessionState>,
    search: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<UserResponse, UserError> {
    require_perm!(session, "users:delete", format!("Restaurando usuario ID: {}", id))?;
    user_service::restore_user(&search, id).await
}

#[tauri::command]
pub async fn get_archived_users(
    session: State<'_, SessionState>,
) -> Result<UserListResponse, UserError> {
    require_perm!(session, "users:read", "Consultando archivo de usuarios")?;
    user_service::get_archived_users().await
}

#[tauri::command]
pub async fn get_user_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<UserResponse, UserError> {
    require_perm!(session, "users:read")?;
    user_service::get_user_by_id(&id).await
}

/// Recupera la nómina completa de usuarios registrados.
#[tauri::command]
pub async fn get_all_users(
    session: State<'_, SessionState>,
) -> Result<UserListResponse, UserError> {
    require_perm!(session, "users:read")?;
    user_service::get_all_users().await
}

// ==========================================
// PROTOCOLOS DE AUTENTICACIÓN
// ==========================================

/// Gatekeeper: Valida las credenciales contra `SurrealDB` y establece el estado de sesión global.
/// Es el punto de entrada principal para el acceso a la aplicación.
#[tauri::command]
pub async fn login(
    session: State<'_, SessionState>,
    email: String,
    password: String,
) -> Result<UserResponse, UserError> {
    log::info!("🔐 Intento de acceso detectado para: {email}");
    let user_response = user_service::login(email, password).await?;

    // Inyecta el usuario en el gestor de sesiones de Rust (Thread-safe)
    let session_user = SessionUser {
        id: user_response.id.clone(),
        email: user_response.email.clone(),
        nombre: user_response.nombre.clone(),
        apellido: user_response.apellido.clone(),
        role_id: user_response.role_id.clone(),
        role_name: user_response.role_name.clone(),
    };

    session.set_user(session_user);
    log::info!("✅ Sesión autorizada y establecida para: {}", user_response.email);

    Ok(user_response)
}

#[tauri::command]
pub async fn change_password(id: String, input: ChangePasswordInput) -> Result<(), UserError> {
    user_service::change_password(id, input).await
}

// ==========================================
// GESTIÓN DE AVATARES (Almacenamiento Cifrado)
// ==========================================

/// Carga y procesa la foto del usuario.
/// Delega al `AvatarService` el cifrado persistente.
#[tauri::command]
pub async fn upload_user_avatar(user_id: String, file_path: String) -> Result<String, UserError> {
    log::info!("📸 Procesando nueva imagen de perfil para usuario: {user_id}");
    crate::services::avatar_service::upload_avatar(&user_id, &file_path).await
}

/// Recupera la imagen del usuario en formato base64 tras su descifrado reactivo.
#[tauri::command]
pub async fn get_user_avatar(user_id: String) -> Result<String, UserError> {
    crate::services::avatar_service::get_avatar(&user_id).await
}
