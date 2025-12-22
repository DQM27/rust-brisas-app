// ==========================================
// src/services/user_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db y auth

use crate::db::user_queries as db;
use crate::domain::errors::UserError;
use crate::domain::role::{ROLE_GUARDIA_ID, SUPERUSER_ID};
use crate::domain::user as domain;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserListResponse, UserResponse,
};
use crate::services::auth;
use crate::services::search_service::SearchService;

use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

// ==========================================
// CREAR USUARIO
// ==========================================

pub async fn create_user(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    mut input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    // 0. Normalizar input
    if let Some(ref p) = input.password {
        if p.trim().is_empty() {
            input.password = None;
        }
    }

    let clean_opt = |opt: &mut Option<String>| {
        if let Some(s) = opt {
            if s.trim().is_empty() {
                *opt = None;
            }
        }
    };

    clean_opt(&mut input.segundo_nombre);
    clean_opt(&mut input.segundo_apellido);
    clean_opt(&mut input.fecha_inicio_labores);
    clean_opt(&mut input.numero_gafete);
    clean_opt(&mut input.fecha_nacimiento);
    clean_opt(&mut input.telefono);
    clean_opt(&mut input.direccion);
    clean_opt(&mut input.contacto_emergencia_nombre);
    clean_opt(&mut input.contacto_emergencia_telefono);

    // 1. Validar input
    domain::validar_create_input(&input).map_err(UserError::Validation)?;

    // 2. Normalizar datos
    let email_normalizado = domain::normalizar_email(&input.email);
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);
    let apellido_normalizado = domain::normalizar_nombre(&input.apellido);

    // 3. Verificar email único
    let count = db::count_by_email(pool, &email_normalizado).await?;
    if count > 0 {
        return Err(UserError::EmailExists);
    }

    // 4. Determinar rol (default: Guardia)
    let role_id = input.role_id.unwrap_or_else(|| ROLE_GUARDIA_ID.to_string());

    // 5. Generar o usar contraseña
    let (password_str, must_change_password) = match input.password {
        Some(p) => {
            let force_change = input.must_change_password.unwrap_or(false);
            (p, force_change)
        }
        None => {
            let rng = rand::thread_rng();
            let temp: String = rng
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();
            (temp, true)
        }
    };

    let password_hash = auth::hash_password(&password_str).map_err(UserError::Auth)?;

    // 6. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 7. Insertar en DB
    db::insert(
        pool,
        &id,
        &email_normalizado,
        &password_hash,
        &nombre_normalizado,
        &apellido_normalizado,
        &role_id,
        &now,
        &now,
        &input.cedula,
        input.segundo_nombre.as_deref(),
        input.segundo_apellido.as_deref(),
        input.fecha_inicio_labores.as_deref(),
        input.numero_gafete.as_deref(),
        input.fecha_nacimiento.as_deref(),
        input.telefono.as_deref(),
        input.direccion.as_deref(),
        input.contacto_emergencia_nombre.as_deref(),
        input.contacto_emergencia_telefono.as_deref(),
        must_change_password,
    )
    .await?;

    // 8. Retornar usuario creado
    let mut response = get_user_by_id(pool, &id).await?;
    if must_change_password {
        response.temporary_password = Some(password_str);
    }

    // 9. Indexar en Tantivy
    match db::find_by_id(pool, &id).await {
        Ok(user) => {
            if let Err(e) = search_service.add_user(&user).await {
                eprintln!("⚠️ Error al indexar usuario {}: {}", id, e);
            }
        }
        Err(e) => eprintln!("⚠️ Error al obtener usuario para indexar {}: {}", id, e),
    }

    Ok(response)
}

// ==========================================
// OBTENER USUARIO POR ID
// ==========================================

pub async fn get_user_by_id(pool: &SqlitePool, id: &str) -> Result<UserResponse, UserError> {
    let user = db::find_by_id(pool, id).await.map_err(UserError::from)?;
    let role_name = db::get_role_name(pool, &user.role_id)
        .await
        .unwrap_or_else(|_| "Desconocido".to_string());

    Ok(UserResponse::from_user_with_role(user, role_name))
}

// ==========================================
// OBTENER TODOS LOS USUARIOS
// ==========================================

pub async fn get_all_users(pool: &SqlitePool) -> Result<UserListResponse, UserError> {
    // Excluir superuser del listado
    let users = db::find_all(pool, SUPERUSER_ID).await?;

    let mut user_responses = Vec::new();
    for user in users {
        let role_name = db::get_role_name(pool, &user.role_id)
            .await
            .unwrap_or_else(|_| "Desconocido".to_string());
        user_responses.push(UserResponse::from_user_with_role(user, role_name));
    }

    let total = user_responses.len();
    let activos = user_responses.iter().filter(|u| u.is_active).count();

    Ok(UserListResponse {
        users: user_responses,
        total,
        activos,
    })
}

// ==========================================
// ACTUALIZAR USUARIO
// ==========================================

pub async fn update_user(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    mut input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    // Limpiar password vacío
    if let Some(ref p) = input.password {
        if p.trim().is_empty() {
            input.password = None;
        }
    }

    // 1. Validar input
    domain::validar_update_input(&input).map_err(UserError::Validation)?;

    // 2. Verificar que existe
    let _ = db::find_by_id(pool, &id).await?;

    // 3. Normalizar email si viene
    let email_normalizado = if let Some(ref email) = input.email {
        let normalizado = domain::normalizar_email(email);
        let count = db::count_by_email_excluding_id(pool, &normalizado, &id).await?;
        if count > 0 {
            return Err(UserError::EmailExists);
        }
        Some(normalizado)
    } else {
        None
    };

    // 4. Normalizar nombres
    let nombre_normalizado = input.nombre.as_ref().map(|n| domain::normalizar_nombre(n));
    let apellido_normalizado = input
        .apellido
        .as_ref()
        .map(|a| domain::normalizar_nombre(a));

    // 5. Hashear contraseña si viene
    let password_hash = if let Some(ref pwd) = input.password {
        Some(auth::hash_password(pwd).map_err(UserError::Auth)?)
    } else {
        None
    };

    // 6. Timestamp
    let now = Utc::now().to_rfc3339();

    // 7. Convertir is_active
    let is_active_int = input.is_active.map(|b| if b { 1 } else { 0 });

    // 8. Actualizar en DB
    db::update(
        pool,
        &id,
        email_normalizado.as_deref(),
        password_hash.as_deref(),
        nombre_normalizado.as_deref(),
        apellido_normalizado.as_deref(),
        input.role_id.as_deref(),
        is_active_int,
        &now,
        input.cedula.as_deref(),
        input.segundo_nombre.as_deref(),
        input.segundo_apellido.as_deref(),
        input.fecha_inicio_labores.as_deref(),
        input.numero_gafete.as_deref(),
        input.fecha_nacimiento.as_deref(),
        input.telefono.as_deref(),
        input.direccion.as_deref(),
        input.contacto_emergencia_nombre.as_deref(),
        input.contacto_emergencia_telefono.as_deref(),
        input.must_change_password,
    )
    .await?;

    // 9. Retornar actualizado
    let response = get_user_by_id(pool, &id).await?;

    // 10. Actualizar índice
    match db::find_by_id(pool, &id).await {
        Ok(user) => {
            if let Err(e) = search_service.update_user(&user).await {
                eprintln!("⚠️ Error al actualizar índice del usuario {}: {}", id, e);
            }
        }
        Err(e) => eprintln!(
            "⚠️ Error al obtener usuario para actualizar índice {}: {}",
            id, e
        ),
    }

    Ok(response)
}

// ==========================================
// ELIMINAR USUARIO
// ==========================================

pub async fn delete_user(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
) -> Result<(), UserError> {
    let _ = db::find_by_id(pool, &id).await?;

    db::delete(pool, &id).await?;

    if let Err(e) = search_service.delete_user(&id).await {
        eprintln!("⚠️ Error al eliminar usuario del índice {}: {}", id, e);
    }

    Ok(())
}

// ==========================================
// CAMBIAR CONTRASEÑA
// ==========================================

pub async fn change_password(
    pool: &SqlitePool,
    id: String,
    input: ChangePasswordInput,
) -> Result<(), UserError> {
    let user = get_user_by_id(pool, &id).await?;
    let (_, current_hash) = db::find_by_email_with_password(pool, &user.email).await?;

    // Verificar contraseña actual si se provee
    if let Some(current) = input.current_password {
        let is_valid = auth::verify_password(&current, &current_hash).map_err(UserError::Auth)?;
        if !is_valid {
            return Err(UserError::InvalidCurrentPassword);
        }
    }

    // Validar nueva contraseña
    domain::validar_password(&input.new_password).map_err(UserError::Validation)?;

    // Hashear nueva
    let new_hash = auth::hash_password(&input.new_password).map_err(UserError::Auth)?;

    // Actualizar y quitar flag
    let now = Utc::now().to_rfc3339();
    db::update(
        pool,
        &id,
        None,
        Some(&new_hash),
        None,
        None,
        None,
        None,
        &now,
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
        Some(false),
    )
    .await?;

    Ok(())
}

// ==========================================
// LOGIN
// ==========================================

pub async fn login(
    pool: &SqlitePool,
    email: String,
    password: String,
) -> Result<UserResponse, UserError> {
    let email_normalizado = domain::normalizar_email(&email);

    let (user, password_hash) = db::find_by_email_with_password(pool, &email_normalizado).await?;

    let is_valid = auth::verify_password(&password, &password_hash).map_err(UserError::Auth)?;
    if !is_valid {
        return Err(UserError::InvalidCredentials);
    }

    if !user.is_active {
        return Err(UserError::InactiveUser);
    }

    let role_name = db::get_role_name(pool, &user.role_id)
        .await
        .unwrap_or_else(|_| "Desconocido".to_string());

    Ok(UserResponse::from_user_with_role(user, role_name))
}
