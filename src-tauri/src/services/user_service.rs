// ==========================================
// src/services/user_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db y auth
// Contiene la lógica de negocio completa

use crate::domain::user as domain;

use crate::db::user_queries as db;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, RoleStats, UpdateUserInput, UserListResponse,
    UserResponse, UserRole,
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
) -> Result<UserResponse, String> {
    // 0. Normalizar input: Si campos opcionales vienen vacíos, convertirlos a None
    if let Some(ref p) = input.password {
        if p.trim().is_empty() {
            input.password = None;
        }
    }

    // Función helper interna para limpiar opciones
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

    // 3. Verificar que el email no exista
    let count = db::count_by_email(pool, &email_normalizado).await?;
    if count > 0 {
        return Err("Ya existe un usuario con este email".to_string());
    }

    // 4. Determinar rol (default: Guardia)
    let role = if let Some(ref r) = input.role {
        UserRole::from_str(r)?
    } else {
        UserRole::Guardia
    };

    // 5. Generar o usar contraseña
    let (password_str, must_change_password) = match input.password {
        Some(p) => (p, false), // Admin asignó contraseña (opcional)
        None => {
            // Generar temporal
            let rng = rand::thread_rng();
            let temp: String = rng
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();
            (temp, true)
        }
    };

    let password_hash = auth::hash_password(&password_str)?;

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
        role.as_str(),
        &now,
        &now,
        &input.cedula,
        input.segundo_nombre.as_deref(),
        input.segundo_apellido.as_deref(),
        // New fields
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

    // 8. Retornar usuario creado con la contraseña temporal si aplica
    let mut response = get_user_by_id(pool, &id).await?;
    if must_change_password {
        response.temporary_password = Some(password_str.to_string());
    }

    // 9. Indexar en Tantivy (automático)
    // Obtenemos el User crudo para indexar
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

pub async fn get_user_by_id(pool: &SqlitePool, id: &str) -> Result<UserResponse, String> {
    let user = db::find_by_id(pool, id).await?;
    Ok(UserResponse::from(user))
}

// ==========================================
// OBTENER TODOS LOS USUARIOS
// ==========================================

pub async fn get_all_users(pool: &SqlitePool) -> Result<UserListResponse, String> {
    let users = db::find_all(pool).await?;

    // Convertir a UserResponse
    let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    // Calcular estadísticas
    let total = user_responses.len();
    let activos = user_responses.iter().filter(|u| u.is_active).count();
    let admins = user_responses
        .iter()
        .filter(|u| u.role == UserRole::Admin)
        .count();
    let supervisores = user_responses
        .iter()
        .filter(|u| u.role == UserRole::Supervisor)
        .count();
    let guardias = user_responses
        .iter()
        .filter(|u| u.role == UserRole::Guardia)
        .count();

    Ok(UserListResponse {
        users: user_responses,
        total,
        activos,
        por_rol: RoleStats {
            admins,
            supervisores,
            guardias,
        },
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
) -> Result<UserResponse, String> {
    // Función helper interna para limpiar opciones
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
    clean_opt(&mut input.email);
    clean_opt(&mut input.password);

    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que el usuario existe
    let _ = db::find_by_id(pool, &id).await?;

    // 3. Normalizar y verificar email si viene
    let email_normalizado = if let Some(ref email) = input.email {
        let normalizado = domain::normalizar_email(email);

        // Verificar email único
        let count = db::count_by_email_excluding_id(pool, &normalizado, &id).await?;
        if count > 0 {
            return Err("Ya existe otro usuario con este email".to_string());
        }

        Some(normalizado)
    } else {
        None
    };

    // 4. Normalizar nombres si vienen
    let nombre_normalizado = input.nombre.as_ref().map(|n| domain::normalizar_nombre(n));

    let apellido_normalizado = input
        .apellido
        .as_ref()
        .map(|a| domain::normalizar_nombre(a));

    // 5. Convertir rol si viene
    let role_str = if let Some(ref r) = input.role {
        Some(UserRole::from_str(r)?.as_str().to_string())
    } else {
        None
    };

    // 6. Hashear contraseña si viene
    let password_hash = if let Some(ref pwd) = input.password {
        Some(auth::hash_password(pwd)?)
    } else {
        None
    };

    // 7. Timestamp de actualización
    let now = Utc::now().to_rfc3339();

    // 8. Convertir is_active a i32 si viene
    let is_active_int = input.is_active.map(|b| if b { 1 } else { 0 });

    // 9. Actualizar en DB
    db::update(
        pool,
        &id,
        email_normalizado.as_deref(),
        password_hash.as_deref(),
        nombre_normalizado.as_deref(),
        apellido_normalizado.as_deref(),
        role_str.as_deref(),
        is_active_int,
        &now,
        // New params
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

    // 10. Retornar usuario actualizado
    let response = get_user_by_id(pool, &id).await?;

    // 11. Actualizar índice de Tantivy (automático)
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
) -> Result<(), String> {
    // Verificar que existe antes de eliminar
    let _ = db::find_by_id(pool, &id).await?;

    // Eliminar
    // Eliminar
    db::delete(pool, &id).await?;

    // Eliminar del índice
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
) -> Result<(), String> {
    // 1. Obtener usuario (incluye password hash para verificar si es necesario)
    let (_, current_hash) =
        db::find_by_email_with_password(pool, &get_user_by_id(pool, &id).await?.email).await?;

    // 2. Verificar contraseña actual si se provee (obligatorio para usuario normal)
    if let Some(current) = input.current_password {
        let is_valid = auth::verify_password(&current, &current_hash)?;
        if !is_valid {
            return Err("La contraseña actual es incorrecta".to_string());
        }
    } else {
        // Si no se provee current, asumir que es admin reset (validar permisos en capa superior si es necesario)
        // Ojo: En este diseño básico confiamos que si llega sin current es porque el comando lo permitió
    }

    // 3. Validar nueva contraseña
    domain::validar_password(&input.new_password)?;

    // 4. Hashear nueva
    let new_hash = auth::hash_password(&input.new_password)?;

    // 5. Actualizar en DB y QUITAR flag de must_change_password
    let now = Utc::now().to_rfc3339();

    // Reusamos db::update pasando solo lo necesario
    db::update(
        pool,
        &id,
        None, // email
        Some(&new_hash),
        None,
        None,
        None,
        None, // nombre, apellido, role, is_active
        &now, // updated_at
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,        // otros campos opcionales
        Some(false), // must_change_password = FALSE
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
) -> Result<UserResponse, String> {
    // 1. Normalizar email
    let email_normalizado = domain::normalizar_email(&email);

    // 2. Buscar usuario con password_hash
    let (user, password_hash) = db::find_by_email_with_password(pool, &email_normalizado).await?;

    // 3. Verificar contraseña
    let is_valid = auth::verify_password(&password, &password_hash)?;
    if !is_valid {
        return Err("Credenciales inválidas".to_string());
    }

    // 4. Verificar que esté activo
    if !user.is_active {
        return Err("Usuario inactivo".to_string());
    }

    // 5. Retornar usuario
    Ok(UserResponse::from(user))
}
