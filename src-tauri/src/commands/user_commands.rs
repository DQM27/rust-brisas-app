// src/commands/user_commands.rs

use crate::domain::errors::UserError;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserListResponse, UserResponse,
};
use tauri::State;

// Session imports (needed for both backends)
#[cfg(not(feature = "surrealdb-backend"))]
use crate::services::session::{SessionState, SessionUser};

// ==========================================
// SQLITE IMPORTS (default)
// ==========================================
#[cfg(not(feature = "surrealdb-backend"))]
use crate::db::DbPool;
#[cfg(not(feature = "surrealdb-backend"))]
use crate::services::search_service::SearchState;
#[cfg(not(feature = "surrealdb-backend"))]
use crate::services::user_service;

// ==========================================
// SURREALDB IMPORTS (experimental)
// ==========================================
#[cfg(feature = "surrealdb-backend")]
use crate::services::surrealdb_user_service as user_service;

// ==========================================
// COMMANDS - Conditional implementation
// ==========================================

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn create_user(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    user_service::create_user(&pool, &search_service, input).await
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn create_user(input: CreateUserInput) -> Result<UserResponse, UserError> {
    user_service::create_user(input).await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn update_user(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    user_service::update_user(&pool, &search_service, id.clone(), input).await
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn update_user(id: String, input: UpdateUserInput) -> Result<UserResponse, UserError> {
    user_service::update_user(id, input).await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn delete_user(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
) -> Result<(), UserError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    user_service::delete_user(&pool, &search_service, id.clone()).await
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn delete_user(id: String) -> Result<(), UserError> {
    user_service::delete_user(id).await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn get_user_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;
    user_service::get_user_by_id(&pool, &id).await
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn get_user_by_id(id: String) -> Result<UserResponse, UserError> {
    user_service::get_user_by_id(&id).await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn get_all_users(pool_state: State<'_, DbPool>) -> Result<UserListResponse, UserError> {
    let pool = pool_state.0.read().await;
    user_service::get_all_users(&pool).await
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn get_all_users() -> Result<UserListResponse, UserError> {
    user_service::get_all_users().await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn login(
    pool_state: State<'_, DbPool>,
    session: State<'_, SessionState>, // Add session state
    email: String,
    password: String,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;

    // DEBUG: Verificar a qué DB estamos conectados
    if let Ok(path) =
        sqlx::query_scalar::<_, String>("SELECT file FROM pragma_database_list WHERE name='main'")
            .fetch_one(&*pool)
            .await
    {
        log::info!("🔐 Login request using SQLite DB: {}", path);
    }

    let user_response = user_service::login(&pool, email, password).await?;

    // Update SessionState
    let session_user = SessionUser {
        id: user_response.id.clone(),
        email: user_response.email.clone(),
        nombre: user_response.nombre.clone(),
        apellido: user_response.apellido.clone(),
        role_id: user_response.role_id.clone(),
        role_name: user_response.role_name.clone(),
    };

    session.set_user(session_user);
    log::info!("✅ Sesión establecida para: {}", user_response.email);

    Ok(user_response)
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn login(email: String, password: String) -> Result<UserResponse, UserError> {
    log::info!("🔐 Login request using SurrealDB");
    user_service::login(email, password).await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn change_password(
    pool_state: State<'_, DbPool>,
    id: String,
    input: ChangePasswordInput,
) -> Result<(), UserError> {
    let pool = pool_state.0.read().await;
    user_service::change_password(&pool, id, input).await
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn change_password(id: String, input: ChangePasswordInput) -> Result<(), UserError> {
    user_service::change_password(id, input).await
}

/// Ejecuta el seed de demostración y logea con un usuario demo
/// email debe ser uno de: marie.curie@demo.com, albert.einstein@demo.com, richard.feynman@demo.com
#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn demo_login(
    pool_state: State<'_, DbPool>,
    session: State<'_, SessionState>, // Add session state
    email: String,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;

    // 1. Ejecutar seed_demo (es idempotente, no duplica datos)
    crate::config::seed_demo::run_demo_seed(&pool)
        .await
        .map_err(|e| UserError::Database(sqlx::Error::Protocol(e.to_string())))?;

    // 2. Logear con el usuario demo (password siempre es demo123)
    let user_response = user_service::login(&pool, email, "demo123".to_string()).await?;

    // Update SessionState
    let session_user = SessionUser {
        id: user_response.id.clone(),
        email: user_response.email.clone(),
        nombre: user_response.nombre.clone(),
        apellido: user_response.apellido.clone(),
        role_id: user_response.role_id.clone(),
        role_name: user_response.role_name.clone(),
    };

    session.set_user(session_user);
    log::info!("✅ Sesión DEMO establecida para: {}", user_response.email);

    Ok(user_response)
}

// ==========================================
// AVATAR COMMANDS (ENCRYPTED VAULT)
// ==========================================

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn upload_user_avatar(
    pool_state: State<'_, DbPool>,
    user_id: String,
    file_path: String,
) -> Result<String, UserError> {
    use crate::commands::security_commands::encrypt_data;
    use std::fs;
    use std::path::Path;
    use uuid::Uuid;

    let pool = pool_state.0.read().await;

    // 1. Validar archivo
    let source_path = Path::new(&file_path);
    if !source_path.exists() {
        return Err(UserError::Validation("El archivo no existe".to_string()));
    }

    // 2. Leer contenido
    let file_content = fs::read(source_path).map_err(|e| UserError::IO(e.to_string()))?;

    // 3. Encriptar (ChaCha20Poly1305)
    let encrypted_content =
        encrypt_data(&file_content).map_err(|e| UserError::Database(sqlx::Error::Protocol(e)))?;

    // 4. Preparar destino (Vault)
    let data_dir = dirs::data_local_dir()
        .ok_or(UserError::Validation("No ses pudo obtener directorio de datos".to_string()))?
        .join("Brisas")
        .join("images")
        .join("avatars");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| UserError::IO(e.to_string()))?;
    }

    // 5. Generar UUID para el archivo (Nombre ofuscado)
    let file_uuid = Uuid::new_v4().to_string();
    let dest_path = data_dir.join(format!("{}.enc", file_uuid));

    // 6. Guardar blob encriptado
    fs::write(&dest_path, encrypted_content).map_err(|e| UserError::IO(e.to_string()))?;

    // 7. Actualizar DB
    crate::db::user_queries::update(
        &pool,
        &user_id,
        None,
        None,
        None,
        None,
        None,
        None,  // Campos no modificados
        "now", // updated_at
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&file_uuid), // avatar_path (solo el UUID)
    )
    .await
    .map_err(UserError::Database)?;

    Ok(file_uuid)
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn get_user_avatar(
    pool_state: State<'_, DbPool>,
    user_id: String,
) -> Result<String, UserError> {
    use crate::commands::security_commands::decrypt_data;
    use base64::{engine::general_purpose, Engine as _};
    use std::fs;

    let pool = pool_state.0.read().await;

    // 1. Obtener UUID de la DB
    let user =
        crate::db::user_queries::find_by_id(&pool, &user_id).await.map_err(UserError::Database)?;

    let avatar_uuid =
        user.avatar_path.ok_or(UserError::Validation("Usuario sin avatar".to_string()))?;

    // 2. Buscar en Bóveda
    let file_path = dirs::data_local_dir()
        .ok_or(UserError::Validation("Error sistema de archivos".to_string()))?
        .join("Brisas")
        .join("images")
        .join("avatars")
        .join(format!("{}.enc", avatar_uuid));

    if !file_path.exists() {
        return Err(UserError::Validation("Archivo de avatar no encontrado en bóveda".to_string()));
    }

    // 3. Leer y Desencriptar
    let encrypted_content = fs::read(file_path).map_err(|e| UserError::IO(e.to_string()))?;
    let decrypted_content = decrypt_data(&encrypted_content)
        .map_err(|e| UserError::Database(sqlx::Error::Protocol(e)))?;

    // 4. Retornar como Base64 para el frontend
    Ok(general_purpose::STANDARD.encode(decrypted_content))
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn demo_login(email: String) -> Result<UserResponse, UserError> {
    // TODO: Implementar seed_demo para SurrealDB
    log::warn!("⚠️ demo_login: seed_demo no implementado para SurrealDB");
    user_service::login(email, "demo123".to_string()).await
}

// ==========================================
// AVATAR STUBS FOR SURREALDB (TODO: implement)
// ==========================================

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn upload_user_avatar(_user_id: String, _file_path: String) -> Result<String, UserError> {
    Err(UserError::Validation("Avatar upload no implementado para SurrealDB aún".to_string()))
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn get_user_avatar(_user_id: String) -> Result<String, UserError> {
    Err(UserError::Validation("Avatar retrieval no implementado para SurrealDB aún".to_string()))
}
