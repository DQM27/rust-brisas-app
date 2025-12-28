// ==========================================
// src/services/user_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db y auth
// Adaptado para SurrealDB Native

use crate::db::surrealdb_role_queries as role_db;
use crate::db::surrealdb_user_queries as db;
use crate::domain::errors::UserError;
use crate::domain::role::{ROLE_GUARDIA_ID, SUPERUSER_ID};
use crate::domain::user as domain;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserCreateDTO, UserListResponse,
    UserResponse,
};
use crate::services::auth;
use crate::services::search_service::SearchService;
use crate::services::surrealdb_authorization;

use log::{error, info, warn};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::sync::Arc;
use surrealdb::RecordId;

// ==========================================
// CREAR USUARIO
// ==========================================

pub async fn create_user(
    search_service: &Arc<SearchService>,
    input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar datos
    let email_normalizado = domain::normalizar_email(&input.email);
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);
    let apellido_normalizado = domain::normalizar_nombre(&input.apellido);

    // 3. Verificar email único
    let count = db::count_by_email(&email_normalizado)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    if count > 0 {
        return Err(UserError::EmailExists);
    }

    // 4. Determinar rol (default: Guardia)
    // 4. Determinar rol (default: Guardia)
    // Fix: Clone Option, filter out empty strings, then default.
    let raw_role_id = input
        .role_id
        .clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| ROLE_GUARDIA_ID.to_string());

    // Fix: Remove "role:" prefix if present to avoid double wrapping
    let role_id_str = raw_role_id.strip_prefix("role:").unwrap_or(&raw_role_id);
    let role_record = RecordId::from_table_key("role", role_id_str);

    // 5. Generar o usar contraseña
    let (password_str, must_change_password) = match input.password {
        Some(p) => {
            let force_change = input.must_change_password.unwrap_or(false);
            (p, force_change)
        }
        None => {
            let rng = rand::thread_rng();
            let temp: String = rng.sample_iter(&Alphanumeric).take(12).map(char::from).collect();
            (temp, true)
        }
    };

    info!(
        "Creando usuario '{}' con rol_id resolved: '{}' (original input: {:?})",
        email_normalizado, role_record, input.role_id
    );
    let password_hash = auth::hash_password(&password_str)?;

    // 6. Preparar DTO
    let dto = UserCreateDTO {
        email: email_normalizado.clone(),
        password_hash,
        nombre: nombre_normalizado,
        apellido: apellido_normalizado,
        role: role_record,
        cedula: input.cedula,
        segundo_nombre: input.segundo_nombre,
        segundo_apellido: input.segundo_apellido,
        fecha_inicio_labores: input.fecha_inicio_labores,
        numero_gafete: input.numero_gafete,
        fecha_nacimiento: input.fecha_nacimiento,
        telefono: input.telefono,
        direccion: input.direccion,
        contacto_emergencia_nombre: input.contacto_emergencia_nombre,
        contacto_emergencia_telefono: input.contacto_emergencia_telefono,
        must_change_password,
        avatar_path: input.avatar_path,
    };

    // 7. Insertar en DB
    let user = db::insert(dto).await.map_err(|e| {
        error!("Error de base de datos al crear usuario {}: {}", email_normalizado, e);
        UserError::Database(e.to_string())
    })?;

    info!("Usuario '{}' creado exitosamente con ID {}", email_normalizado, user.id);

    // 8. Retornar respuesta
    // 8. Retornar respuesta
    let role_permissions = surrealdb_authorization::get_role_permissions(&role_id_str)
        .await
        .unwrap_or_default()
        .into_iter()
        .collect();

    let role_obj = role_db::find_by_id(&user.role)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .unwrap_or_else(|| {
            use crate::models::role::Role;
            Role {
                id: user.role.clone(),
                name: "Desconocido".to_string(),
                description: None,
                is_system: false,
                created_at: surrealdb::Datetime::default(),
                updated_at: surrealdb::Datetime::default(),
                permissions: Some(vec![]),
            }
        });

    let mut response = UserResponse::from_user_with_role(user.clone(), role_obj, role_permissions);
    if must_change_password {
        response.temporary_password = Some(password_str);
    }

    // 9. Indexar en Tantivy
    if let Err(e) = search_service.add_user(&user).await {
        warn!("Error al indexar usuario {}: {}", user.id, e);
    }

    Ok(response)
}

// ==========================================
// OBTENER USUARIO POR ID
// ==========================================

pub async fn get_user_by_id(id_str: &str) -> Result<UserResponse, UserError> {
    let id = parse_user_id(id_str);
    let user = db::find_by_id_fetched(&id)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .ok_or(UserError::NotFound)?;

    let permissions = match &user.role {
        Some(role) => surrealdb_authorization::get_role_permissions(&role.id.to_string())
            .await
            .unwrap_or_default()
            .into_iter()
            .collect(),
        None => vec![],
    };

    Ok(UserResponse::from_fetched(user, permissions))
}

// ==========================================
// OBTENER TODOS LOS USUARIOS
// ==========================================

pub async fn get_all_users() -> Result<UserListResponse, UserError> {
    let exclude_record = RecordId::from_table_key("user", SUPERUSER_ID);
    let users = db::find_all_fetched(Some(&exclude_record))
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    let mut user_responses = Vec::new();
    for user in users {
        let permissions = match &user.role {
            Some(role) => surrealdb_authorization::get_role_permissions(&role.id.to_string())
                .await
                .unwrap_or_default()
                .into_iter()
                .collect(),
            None => vec![],
        };

        user_responses.push(UserResponse::from_fetched(user, permissions));
    }

    let total = user_responses.len();
    let activos = user_responses.iter().filter(|u| u.is_active).count();

    Ok(UserListResponse { users: user_responses, total, activos })
}

// ==========================================
// ACTUALIZAR USUARIO
// ==========================================

pub async fn update_user(
    search_service: &Arc<SearchService>,
    id_str: String,
    input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    let id_thing = parse_user_id(&id_str);

    // 1. Validar input
    domain::validar_update_input(&input)?;

    info!("Actualizando usuario con ID {}", id_thing);

    // 2. Verificar que existe
    let _ = db::find_by_id(&id_thing)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .ok_or(UserError::NotFound)?;

    // 3. Preparar DTO de actualización
    let mut dto = crate::models::user::UserUpdateDTO::default();

    if let Some(ref email) = input.email {
        let normalizado = domain::normalizar_email(email);
        let count = db::count_by_email_excluding_id(&normalizado, &id_thing)
            .await
            .map_err(|e| UserError::Database(e.to_string()))?;
        if count > 0 {
            return Err(UserError::EmailExists);
        }
        dto.email = Some(normalizado);
    }

    if let Some(ref nombre) = input.nombre {
        dto.nombre = Some(domain::normalizar_nombre(nombre));
    }
    if let Some(ref apellido) = input.apellido {
        dto.apellido = Some(domain::normalizar_nombre(apellido));
    }
    if let Some(ref role_id) = input.role_id {
        // Fix: Handle empty strings and double prefix
        if !role_id.trim().is_empty() {
            // Avoid double wrapping if frontend sends "role:uuid"
            let clean_id = role_id.strip_prefix("role:").unwrap_or(role_id);
            info!("Actualizando rol de usuario a: '{}' (original input: '{}')", clean_id, role_id);
            dto.role = Some(RecordId::from_table_key("role", clean_id));
        } else {
            info!("Input role_id vacío, ignorando actualización de rol.");
        }
    }
    if let Some(pwd) = input.password {
        if !pwd.trim().is_empty() {
            dto.password_hash = Some(auth::hash_password(&pwd)?);
        }
    }
    if let Some(is_active) = input.is_active {
        dto.is_active = Some(is_active);
    }
    if let Some(cedula) = input.cedula {
        dto.cedula = Some(cedula);
    }
    if let Some(v) = input.segundo_nombre {
        dto.segundo_nombre = Some(v);
    }
    if let Some(v) = input.segundo_apellido {
        dto.segundo_apellido = Some(v);
    }
    if let Some(v) = input.fecha_inicio_labores {
        dto.fecha_inicio_labores = Some(v);
    }
    if let Some(v) = input.numero_gafete {
        dto.numero_gafete = Some(v);
    }
    if let Some(v) = input.fecha_nacimiento {
        if !v.is_empty() {
            dto.fecha_nacimiento = Some(v.clone());
        }
    }
    if let Some(v) = input.telefono {
        dto.telefono = Some(v);
    }
    if let Some(v) = input.direccion {
        dto.direccion = Some(v);
    }
    if let Some(v) = input.contacto_emergencia_nombre {
        dto.contacto_emergencia_nombre = Some(v);
    }
    if let Some(v) = input.contacto_emergencia_telefono {
        dto.contacto_emergencia_telefono = Some(v);
    }
    if let Some(v) = input.must_change_password {
        dto.must_change_password = Some(v);
    }
    if let Some(v) = input.avatar_path {
        dto.avatar_path = Some(v);
    }

    dto.updated_at = Some(surrealdb::Datetime::from(chrono::Utc::now()));

    // 4. Actualizar en DB
    let user = db::update(&id_thing, dto)
        .await
        .map_err(|e| {
            error!("Error al actualizar usuario {}: {}", id_str, e);
            UserError::Database(e.to_string())
        })?
        .ok_or(UserError::NotFound)?;

    info!("Usuario {} actualizado exitosamente", id_str);

    // 5. Indexar
    if let Err(e) = search_service.update_user(&user).await {
        warn!("Error al actualizar índice del usuario {}: {}", id_str, e);
    }

    let role = role_db::find_by_id(&user.role)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .unwrap_or_else(|| {
            use crate::models::role::Role;
            Role {
                id: user.role.clone(),
                name: "Desconocido".to_string(),
                description: None,
                is_system: false,
                created_at: surrealdb::Datetime::default(),
                updated_at: surrealdb::Datetime::default(),
                permissions: Some(vec![]),
            }
        });
    let permissions = surrealdb_authorization::get_role_permissions(&user.role.to_string())
        .await
        .unwrap_or_default()
        .into_iter()
        .collect();

    Ok(UserResponse::from_user_with_role(user, role, permissions))
}

// ==========================================
// ELIMINAR USUARIO
// ==========================================

pub async fn delete_user(
    search_service: &Arc<SearchService>,
    id_str: String,
) -> Result<(), UserError> {
    let id_thing = parse_user_id(&id_str);

    info!("Eliminando usuario con ID {}", id_thing);
    db::delete(&id_thing).await.map_err(|e| {
        error!("Error al eliminar usuario {}: {}", id_str, e);
        UserError::Database(e.to_string())
    })?;

    if let Err(e) = search_service.delete_user(&id_str).await {
        warn!("Error al eliminar usuario del índice {}: {}", id_str, e);
    }

    Ok(())
}

// ==========================================
// CAMBIAR CONTRASEÑA
// ==========================================

pub async fn change_password(id_str: String, input: ChangePasswordInput) -> Result<(), UserError> {
    let user_resp = get_user_by_id(&id_str).await?;
    let found = db::find_by_email_with_password(&user_resp.email)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    let (_, current_hash) = found.ok_or(UserError::NotFound)?;

    if let Some(current) = input.current_password {
        let is_valid = auth::verify_password(&current, &current_hash)?;
        if !is_valid {
            error!("Cambio de contraseña fallido para {}: clave actual incorrecta", id_str);
            return Err(UserError::InvalidCurrentPassword);
        }
    }

    domain::validar_password(&input.new_password)?;
    let new_hash = auth::hash_password(&input.new_password)?;

    // Usar update_password que usa time::now() nativo de SurrealDB
    db::update_password(&parse_user_id(&id_str), &new_hash).await.map_err(|e| {
        error!("Error al actualizar password para {}: {}", id_str, e);
        UserError::Database(e.to_string())
    })?;

    info!("✅ Contraseña actualizada para usuario {}", id_str);
    Ok(())
}

// ==========================================
// LOGIN
// ==========================================

pub async fn login(email: String, password: String) -> Result<UserResponse, UserError> {
    let email_normalizado = domain::normalizar_email(&email);

    let found = db::find_by_email_with_password(&email_normalizado)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    let (user, password_hash) = found.ok_or(UserError::InvalidCredentials)?;

    let is_valid = auth::verify_password(&password, &password_hash)?;
    if !is_valid {
        warn!("Intento de login fallido para {}: credenciales inválidas", email_normalizado);
        return Err(UserError::InvalidCredentials);
    }

    if !user.is_active {
        return Err(UserError::InactiveUser);
    }

    let role_permissions = surrealdb_authorization::get_role_permissions(&user.role.to_string())
        .await
        .unwrap_or_default()
        .into_iter()
        .collect();

    let role_obj = role_db::find_by_id(&user.role)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .unwrap_or_else(|| {
            // Fallback dummy role if not found (shouldn't happen with FKs but be safe)
            use crate::models::role::Role;
            Role {
                id: user.role.clone(),
                name: "Desconocido".to_string(),
                description: None,
                is_system: false,
                created_at: surrealdb::Datetime::default(),
                updated_at: surrealdb::Datetime::default(),
                permissions: Some(vec![]),
            }
        });

    let response = UserResponse::from_user_with_role(user.clone(), role_obj, role_permissions);
    Ok(response)
}

// ==========================================
// HELPERS
// ==========================================

fn parse_user_id(id: &str) -> RecordId {
    // Limpiar brackets Unicode que SurrealDB agrega: ⟨uuid⟩ -> uuid
    let clean_id = id
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
        RecordId::from_table_key("user", clean_id)
    }
}
