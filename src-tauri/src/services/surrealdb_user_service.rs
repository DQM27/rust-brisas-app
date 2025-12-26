// ==========================================
// src/services/surrealdb_user_service.rs
// ==========================================
// User service implementation using SurrealDB
// Activated with: cargo run --features surrealdb-backend

use crate::db::surrealdb_user_queries as db;
use crate::domain::errors::UserError;
use crate::domain::role::ROLE_GUARDIA_ID;
use crate::domain::user as domain;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserListResponse, UserResponse,
};
use crate::services::auth;
use crate::services::surrealdb_service::SurrealDbError;

use chrono::Utc;
use log::{error, info, warn};
use rand::distributions::Alphanumeric;
use rand::Rng;

// ==========================================
// ERROR CONVERSION
// ==========================================

impl From<SurrealDbError> for UserError {
    fn from(e: SurrealDbError) -> Self {
        UserError::Database(sqlx::Error::Protocol(e.to_string()))
    }
}

// ==========================================
// CREAR USUARIO
// ==========================================

pub async fn create_user(mut input: CreateUserInput) -> Result<UserResponse, UserError> {
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
    input.email = email_normalizado.clone();
    input.nombre = domain::normalizar_nombre(&input.nombre);
    input.apellido = domain::normalizar_nombre(&input.apellido);

    // 3. Verificar email único
    if let Some(_) = db::get_user_by_email(email_normalizado.clone()).await? {
        return Err(UserError::EmailExists);
    }

    // 4. Determinar rol (default: Guardia)
    if input.role_id.is_none() {
        input.role_id = Some(ROLE_GUARDIA_ID.to_string());
    }

    // 5. Generar o usar contraseña
    let (password_str, must_change_password) = match input.password.clone() {
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

    input.must_change_password = Some(must_change_password);

    info!("Creando usuario '{}' con rol '{:?}'", input.email, input.role_id);
    let password_hash = auth::hash_password(&password_str)?;

    // 6. Crear en SurrealDB
    let user = db::create_user(input, password_hash).await?;

    info!("Usuario '{}' creado exitosamente con ID {}", user.email, user.id);

    // 7. Construir respuesta
    let mut response = UserResponse::from_user_with_role(user, "Guardia".to_string());
    if must_change_password {
        response.temporary_password = Some(password_str);
    }

    Ok(response)
}

// ==========================================
// OBTENER USUARIO POR ID
// ==========================================

pub async fn get_user_by_id(id: &str) -> Result<UserResponse, UserError> {
    let user = db::get_user_by_id(id).await?.ok_or(UserError::NotFound)?;

    // TODO: Obtener nombre del rol desde SurrealDB
    Ok(UserResponse::from_user_with_role(user, "Guardia".to_string()))
}

// ==========================================
// OBTENER TODOS LOS USUARIOS
// ==========================================

pub async fn get_all_users() -> Result<UserListResponse, UserError> {
    let users = db::get_all_users().await?;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|u| UserResponse::from_user_with_role(u, "Guardia".to_string()))
        .collect();

    let total = user_responses.len();
    let activos = user_responses.iter().filter(|u| u.is_active).count();

    Ok(UserListResponse { users: user_responses, total, activos })
}

// ==========================================
// ACTUALIZAR USUARIO
// ==========================================

pub async fn update_user(
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
    let _ = db::get_user_by_id(&id).await?.ok_or(UserError::NotFound)?;

    // 3. Normalizar email si viene
    if let Some(ref email) = input.email {
        let normalizado = domain::normalizar_email(email);
        if let Some(existing) = db::get_user_by_email(normalizado.clone()).await? {
            if existing.id != id {
                return Err(UserError::EmailExists);
            }
        }
        input.email = Some(normalizado);
    }

    // 4. Normalizar nombres
    if let Some(ref n) = input.nombre {
        input.nombre = Some(domain::normalizar_nombre(n));
    }
    if let Some(ref a) = input.apellido {
        input.apellido = Some(domain::normalizar_nombre(a));
    }

    // 5. Actualizar en SurrealDB
    let user = db::update_user(id.clone(), input).await?;

    info!("Usuario {} actualizado exitosamente", id);

    Ok(UserResponse::from_user_with_role(user, "Guardia".to_string()))
}

// ==========================================
// ELIMINAR USUARIO
// ==========================================

pub async fn delete_user(id: String) -> Result<(), UserError> {
    let _ = db::get_user_by_id(&id).await?.ok_or(UserError::NotFound)?;

    info!("Eliminando usuario con ID {}", id);
    db::delete_user(id.clone()).await?;

    info!("Usuario {} eliminado exitosamente", id);
    Ok(())
}

// ==========================================
// CAMBIAR CONTRASEÑA
// ==========================================

pub async fn change_password(id: String, input: ChangePasswordInput) -> Result<(), UserError> {
    let user = get_user_by_id(&id).await?;

    // Verificar contraseña actual si se provee
    if let Some(current) = input.current_password {
        // TODO: Obtener hash actual de SurrealDB y verificar
        warn!("Verificación de password actual no implementada para SurrealDB");
    }

    // Validar nueva contraseña
    domain::validar_password(&input.new_password)?;

    // Hashear nueva
    let new_hash = auth::hash_password(&input.new_password)?;

    // Actualizar password
    let update_input = UpdateUserInput {
        email: None,
        password: Some(new_hash),
        nombre: None,
        apellido: None,
        role_id: None,
        is_active: None,
        cedula: None,
        segundo_nombre: None,
        segundo_apellido: None,
        fecha_inicio_labores: None,
        numero_gafete: None,
        fecha_nacimiento: None,
        telefono: None,
        direccion: None,
        contacto_emergencia_nombre: None,
        contacto_emergencia_telefono: None,
        must_change_password: Some(false),
    };

    db::update_user(id.clone(), update_input).await?;

    info!("Contraseña cambiada para usuario {}", id);
    Ok(())
}

// ==========================================
// LOGIN
// ==========================================

pub async fn login(email: String, password: String) -> Result<UserResponse, UserError> {
    let email_normalizado = domain::normalizar_email(&email);

    let user =
        db::verify_credentials(email_normalizado.clone(), password).await?.ok_or_else(|| {
            warn!("Intento de login fallido para {}: credenciales inválidas", email_normalizado);
            UserError::InvalidCredentials
        })?;

    if !user.is_active {
        return Err(UserError::InactiveUser);
    }

    Ok(UserResponse::from_user_with_role(user, "Guardia".to_string()))
}
