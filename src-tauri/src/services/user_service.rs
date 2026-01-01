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

/// Crea un nuevo usuario en el sistema, gestionando todo el ciclo de vida inicial.
///
/// Este proceso incluye:
/// 1. Validación estricta del formato de los datos.
/// 2. Normalización de campos clave (como el email) para evitar duplicados por minúsculas/mayúsculas.
/// 3. Verificación de unicidad en la base de datos.
/// 4. Gestión de seguridad: hashing de la contraseña o generación de una temporal segura.
/// 5. Asignación de roles y permisos.
/// 6. Indexado en el motor de búsqueda Tantivy para que el usuario sea localizable de inmediato.
pub async fn create_user(
    search_service: &Arc<SearchService>,
    input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    // Validamos los campos de entrada según las reglas de negocio (ej. longitud de nombre).
    domain::validar_create_input(&input)?;

    // Normalizamos el email a minúsculas y limpiamos espacios. Esto garantiza que 'Juan@Ejemplo.com'
    // sea tratado exactamente igual que 'juan@ejemplo.com', evitando colisiones.
    let email_normalizado = domain::normalizar_email(&input.email);
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);
    let apellido_normalizado = domain::normalizar_nombre(&input.apellido);

    // Comprobamos si el email ya está en uso antes de intentar la inserción.
    let count = db::count_by_email(&email_normalizado)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    if count > 0 {
        return Err(UserError::EmailExists);
    }

    // El sistema asigna por defecto el rol de 'Guardia' si no se provee uno.
    // Esto asegura que cada usuario tenga un marco mínimo de permisos.
    let raw_role_id = input
        .role_id
        .clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| ROLE_GUARDIA_ID.to_string());

    let role_record = parse_role_id(&raw_role_id);

    // Si no se proporciona una contraseña, se genera una aleatoria de alta entropía (12 caracteres).
    // En este caso, se marca el flag 'must_change_password' para obligar al usuario a actualizarla en su primer inicio.
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

    info!("Registrando usuario '{}' con rol: '{}'", email_normalizado, role_record);
    // Nunca almacenamos la contraseña en texto plano. Usamos Argon2 para generar un hash seguro y resistente.
    let password_hash = auth::hash_password(&password_str)?;

    // Construcción del DTO (Data Transfer Object) para la capa de persistencia.
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

    let user = db::insert(dto).await.map_err(|e| {
        error!("Fallo en la inserción del usuario {}: {}", email_normalizado, e);
        UserError::Database(e.to_string())
    })?;

    info!("Usuario '{}' creado y persistido correctamente.", email_normalizado);

    // Calculamos los permisos efectivos basados en el rol asignado.
    // Esto es necesario para devolver al frontend una vista completa de lo que el usuario puede hacer.
    let role_permissions = surrealdb_authorization::get_role_permissions(&user.role.to_string())
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
                inherits_from: None,
                created_at: surrealdb::Datetime::default(),
                updated_at: surrealdb::Datetime::default(),
                permissions: Some(vec![]),
            }
        });

    let mut response = UserResponse::from_user_with_role(user.clone(), role_obj, role_permissions);
    if must_change_password {
        response.temporary_password = Some(password_str);
    }

    // Actualizamos el índice de búsqueda de forma asíncrona.
    // Si falla, el usuario aún existe pero no será localizable mediante la búsqueda global hasta un reindexado.
    if let Err(e) = search_service.add_user(&user).await {
        warn!("Fallo no crítico al indexar al usuario {}: {}", user.id, e);
    }

    Ok(response)
}

/// Recupera la información detallada de un usuario junto con sus permisos de acceso.
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

/// Lista todos los usuarios registrados, excluyendo al superusuario del sistema.
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

/// Actualiza parcialmente los datos de un usuario existente.
pub async fn update_user(
    search_service: &Arc<SearchService>,
    id_str: String,
    input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    let id_thing = parse_user_id(&id_str);

    domain::validar_update_input(&input)?;

    // Verificación de existencia antes de proceder.
    let _ = db::find_by_id(&id_thing)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?
        .ok_or(UserError::NotFound)?;

    let mut dto = crate::models::user::UserUpdateDTO::default();

    // Actualización selectiva de campos basada en la entrada.
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
        if !role_id.trim().is_empty() {
            dto.role = Some(parse_role_id(role_id));
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

    let user = db::update(&id_thing, dto)
        .await
        .map_err(|e| {
            error!("Error al actualizar el usuario {}: {}", id_str, e);
            UserError::Database(e.to_string())
        })?
        .ok_or(UserError::NotFound)?;

    // Reflejar la actualización en el motor de búsqueda.
    if let Err(e) = search_service.update_user(&user).await {
        warn!("Error al actualizar el índice de búsqueda para el usuario {}: {}", id_str, e);
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
                inherits_from: None,
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

/// Elimina un usuario del sistema y de los índices de búsqueda.
pub async fn delete_user(
    search_service: &Arc<SearchService>,
    id_str: String,
) -> Result<(), UserError> {
    let id_thing = parse_user_id(&id_str);

    db::delete(&id_thing).await.map_err(|e| {
        error!("Error al eliminar el usuario {}: {}", id_str, e);
        UserError::Database(e.to_string())
    })?;

    if let Err(e) = search_service.delete_user(&id_str).await {
        warn!("No se pudo eliminar al usuario del índice de búsqueda: {}", e);
    }

    Ok(())
}

/// Cambia la contraseña de un usuario, verificando la contraseña actual si es proporcionada.
pub async fn change_password(id_str: String, input: ChangePasswordInput) -> Result<(), UserError> {
    let user_resp = get_user_by_id(&id_str).await?;
    let found = db::find_by_email_with_password(&user_resp.email)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    let (_, current_hash) = found.ok_or(UserError::NotFound)?;

    if let Some(current) = input.current_password {
        let is_valid = auth::verify_password(&current, &current_hash)?;
        if !is_valid {
            error!("Fallo en cambio de contraseña para {}: contraseña actual incorrecta.", id_str);
            return Err(UserError::InvalidCurrentPassword);
        }
    }

    domain::validar_password(&input.new_password)?;
    let new_hash = auth::hash_password(&input.new_password)?;

    db::update_password(&parse_user_id(&id_str), &new_hash).await.map_err(|e| {
        error!("Error al actualizar contraseña para el usuario {}: {}", id_str, e);
        UserError::Database(e.to_string())
    })?;

    info!("✅ Contraseña actualizada exitosamente.");
    Ok(())
}

/// Realiza la autenticación del usuario mediante correo y contraseña.
pub async fn login(email: String, password: String) -> Result<UserResponse, UserError> {
    let email_normalizado = domain::normalizar_email(&email);

    let found = db::find_by_email_with_password(&email_normalizado)
        .await
        .map_err(|e| UserError::Database(e.to_string()))?;

    let (user, password_hash) = found.ok_or(UserError::InvalidCredentials)?;

    let is_valid = auth::verify_password(&password, &password_hash)?;
    if !is_valid {
        warn!("Intento fallido de inicio de sesión para: {}", email_normalizado);
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
            use crate::models::role::Role;
            Role {
                id: user.role.clone(),
                name: "Desconocido".to_string(),
                description: None,
                is_system: false,
                inherits_from: None,
                created_at: surrealdb::Datetime::default(),
                updated_at: surrealdb::Datetime::default(),
                permissions: Some(vec![]),
            }
        });

    Ok(UserResponse::from_user_with_role(user.clone(), role_obj, role_permissions))
}

// Helpers internos para la gestión de IDs y registros.

fn parse_user_id(id: &str) -> RecordId {
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

fn parse_role_id(id: &str) -> RecordId {
    let clean_id = id
        .trim()
        .trim_start_matches("⟨")
        .trim_end_matches("⟩")
        .trim_start_matches('<')
        .trim_end_matches('>');

    if clean_id.to_lowercase().starts_with("role:") {
        let parts: Vec<&str> = clean_id.splitn(2, ':').collect();
        let key = parts[1]
            .trim_start_matches("⟨")
            .trim_end_matches("⟩")
            .trim_start_matches('<')
            .trim_end_matches('>');
        RecordId::from_table_key("role", key)
    } else {
        RecordId::from_table_key("role", clean_id)
    }
}
