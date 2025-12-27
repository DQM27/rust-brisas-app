// ==========================================
// src/services/user_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db y auth
// Adaptado para SurrealDB

use crate::db::surrealdb_user_queries as db;
use crate::domain::errors::UserError;
use crate::domain::role::{ROLE_GUARDIA_ID, SUPERUSER_ID};
use crate::domain::user as domain;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserListResponse, UserResponse,
};
use crate::services::auth;
use crate::services::search_service::SearchService;

use log::{error, info, warn};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::sync::Arc;
use uuid::Uuid;

// ==========================================
// CREAR USUARIO
// ==========================================

pub async fn create_user(
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
    let role_id = input.role_id.unwrap_or_else(|| ROLE_GUARDIA_ID.to_string());

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

    info!("Creando usuario '{}' con rol '{}'", email_normalizado, role_id);
    let password_hash = auth::hash_password(&password_str)?;

    // 6. Generar ID
    let id = Uuid::new_v4().to_string();

    // 7. Insertar en DB
    db::insert(
        &id,
        &email_normalizado,
        &password_hash,
        &nombre_normalizado,
        &apellido_normalizado,
        &role_id,
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
        input.avatar_path.as_deref(),
    )
    .await
    .map_err(|e| {
        error!("Error de base de datos al crear usuario {}: {}", email_normalizado, e);
        UserError::Database(e.to_string())
    })?;

    info!("Usuario '{}' creado exitosamente con ID {}", email_normalizado, id);

    // 8. Retornar usuario creado
    let mut response = get_user_by_id(&id).await?;
    if must_change_password {
        response.temporary_password = Some(password_str);
    }

    // 9. Indexar en Tantivy
    match db::find_by_id(&id).await {
        Ok(Some(user)) => {
            // Nota: search_service espera domain/models user. User de surreal queries es el mismo models::user::User? Sí.
            if let Err(e) = search_service.add_user(&user).await {
                warn!("Error al indexar usuario {}: {}", id, e);
            }
        }
        Ok(None) => warn!("Usuario creado no encontrado inmediatamente para indexar: {}", id),
        Err(e) => warn!("Error al obtener usuario para indexar {}: {}", id, e),
    }

    Ok(response)
}

// ==========================================
// OBTENER USUARIO POR ID
// ==========================================

pub async fn get_user_by_id(id: &str) -> Result<UserResponse, UserError> {
    let user = db::find_by_id(id)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .ok_or(UserError::NotFound)?;

    let role_name =
        db::get_role_name(&user.role_id).await.unwrap_or_else(|_| "Desconocido".to_string());

    Ok(UserResponse::from_user_with_role(user, role_name))
}

// ==========================================
// OBTENER TODOS LOS USUARIOS
// ==========================================

pub async fn get_all_users() -> Result<UserListResponse, UserError> {
    // Excluir superuser del listado
    let users = db::find_all(SUPERUSER_ID).await.map_err(|e| UserError::Database(e.to_string()))?;

    let mut user_responses = Vec::new();
    for user in users {
        let role_name =
            db::get_role_name(&user.role_id).await.unwrap_or_else(|_| "Desconocido".to_string());
        user_responses.push(UserResponse::from_user_with_role(user, role_name));
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
    domain::validar_update_input(&input)?;

    info!("Actualizando usuario con ID {}", id);

    // 2. Verificar que existe
    let _ = db::find_by_id(&id)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .ok_or(UserError::NotFound)?;

    // 3. Normalizar email si viene
    let email_normalizado = if let Some(ref email) = input.email {
        let normalizado = domain::normalizar_email(email);
        let count = db::count_by_email_excluding_id(&normalizado, &id)
            .await
            .map_err(|e| UserError::Database(e.to_string()))?;
        if count > 0 {
            return Err(UserError::EmailExists);
        }
        Some(normalizado)
    } else {
        None
    };

    // 4. Normalizar nombres
    let nombre_normalizado = input.nombre.as_ref().map(|n| domain::normalizar_nombre(n));
    let apellido_normalizado = input.apellido.as_ref().map(|a| domain::normalizar_nombre(a));

    // 5. Hashear contraseña si viene
    let password_hash =
        if let Some(ref pwd) = input.password { Some(auth::hash_password(pwd)?) } else { None };

    // 8. Actualizar en DB
    db::update(
        &id,
        email_normalizado.as_deref(),
        password_hash.as_deref(),
        nombre_normalizado.as_deref(),
        apellido_normalizado.as_deref(),
        input.role_id.as_deref(),
        input.is_active,
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
        input.avatar_path.as_deref(),
    )
    .await
    .map_err(|e| {
        error!("Error al actualizar usuario {}: {}", id, e);
        UserError::Database(e.to_string())
    })?;

    info!("Usuario {} actualizado exitosamente", id);

    // 9. Retornar actualizado
    let response = get_user_by_id(&id).await?;

    // 10. Actualizar índice
    match db::find_by_id(&id).await {
        Ok(Some(user)) => {
            if let Err(e) = search_service.update_user(&user).await {
                warn!("Error al actualizar índice del usuario {}: {}", id, e);
            }
        }
        Ok(None) => warn!("Usuario actualizado no encontrado para indexar: {}", id),
        Err(e) => warn!("Error al obtener usuario para actualizar índice {}: {}", id, e),
    }

    Ok(response)
}

// ==========================================
// ELIMINAR USUARIO
// ==========================================

pub async fn delete_user(search_service: &Arc<SearchService>, id: String) -> Result<(), UserError> {
    let _ = db::find_by_id(&id)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .ok_or(UserError::NotFound)?;

    info!("Eliminando usuario con ID {}", id);
    db::delete(&id).await.map_err(|e| {
        error!("Error al eliminar usuario {}: {}", id, e);
        UserError::Database(e.to_string())
    })?;

    info!("Usuario {} eliminado exitosamente", id);

    if let Err(e) = search_service.delete_user(&id).await {
        warn!("Error al eliminar usuario del índice {}: {}", id, e);
    }

    Ok(())
}

// ==========================================
// CAMBIAR CONTRASEÑA
// ==========================================

pub async fn change_password(id: String, input: ChangePasswordInput) -> Result<(), UserError> {
    let user = get_user_by_id(&id).await?;
    let found = db::find_by_email_with_password(&user.email)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    let (_, current_hash) = found.ok_or(UserError::NotFound)?;

    // Verificar contraseña actual si se provee
    if let Some(current) = input.current_password {
        let is_valid = auth::verify_password(&current, &current_hash)?;
        if !is_valid {
            error!("Cambio de contraseña fallido para {}: clave actual incorrecta", id);
            return Err(UserError::InvalidCurrentPassword);
        }
    }

    // Validar nueva contraseña
    domain::validar_password(&input.new_password)?;

    // Hashear nueva
    let new_hash = auth::hash_password(&input.new_password)?;

    // Actualizar y quitar flag
    db::update(
        &id,
        None,
        Some(&new_hash),
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
        None,
        None,
        None,
        Some(false),
        None,
    )
    .await
    .map_err(|e| {
        error!("Error al actualizar password para {}: {}", id, e);
        UserError::Database(e.to_string())
    })?;

    info!("Contraseña cambiada para usuario {}", id);

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

    let role_name =
        db::get_role_name(&user.role_id).await.unwrap_or_else(|_| "Desconocido".to_string());

    Ok(UserResponse::from_user_with_role(user, role_name))
}

#[cfg(test)]
mod tests {
    // Tests comentados hasta actualizar mock de SurrealDB si aplica
    // use super::*;
}
