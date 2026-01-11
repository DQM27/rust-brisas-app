//! # Queries `SurrealDB`: Usuarios
//!
//! Operaciones de base de datos para gestión de usuarios e autenticación.
//!
//! ## Responsabilidades
//! - CRUD de usuarios
//! - Autenticación (`find_by_email_with_password`)
//! - Gestión de avatares
//! - Conteo y validación de unicidad
//!
//! ## Tabla: `user`

use crate::models::user::{Operacion, User, UserCreateDTO, UserFetched, UserUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::{debug, error, info, warn};
use serde::Deserialize;
use surrealdb::{Datetime, RecordId};

/// Struct para deserializar usuario con `password_hash`
/// NO usamos #[serde(flatten)] porque causa errores en `SurrealDB` 2.x
#[derive(Debug, Deserialize)]
struct UserWithPassword {
    pub id: RecordId,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role: RecordId,
    pub operacion: Option<Operacion>,
    pub is_active: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub cedula: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub fecha_inicio_labores: Option<String>,
    pub numero_gafete: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
    pub vencimiento_portacion: Option<String>,
    pub must_change_password: bool,
    pub deleted_at: Option<Datetime>,
    pub avatar_path: Option<String>,
    pub password_hash: String,
}

impl UserWithPassword {
    fn into_user_and_password(self) -> (User, String) {
        let user = User {
            id: self.id,
            email: self.email,
            nombre: self.nombre,
            apellido: self.apellido,
            role: self.role,
            operacion: self.operacion,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            cedula: self.cedula,
            segundo_nombre: self.segundo_nombre,
            segundo_apellido: self.segundo_apellido,
            fecha_inicio_labores: self.fecha_inicio_labores,
            numero_gafete: self.numero_gafete,
            fecha_nacimiento: self.fecha_nacimiento,
            telefono: self.telefono,
            direccion: self.direccion,
            contacto_emergencia_nombre: self.contacto_emergencia_nombre,
            contacto_emergencia_telefono: self.contacto_emergencia_telefono,
            vencimiento_portacion: self.vencimiento_portacion,
            must_change_password: self.must_change_password,
            deleted_at: self.deleted_at,
            avatar_path: self.avatar_path,
        };
        (user, self.password_hash)
    }
}

/// Crea un nuevo usuario en el sistema.
pub async fn insert(dto: UserCreateDTO) -> Result<User, SurrealDbError> {
    debug!("➕ Creando nuevo usuario");
    let db = get_db().await?;

    let mut result = db
        .query(
            r"
            CREATE user CONTENT {
                email: $email,
                password_hash: $password_hash,
                nombre: $nombre,
                apellido: $apellido,
                role: $role,
                operacion: $operacion,
                cedula: $cedula,
                segundo_nombre: $segundo_nombre,
                segundo_apellido: $segundo_apellido,
                fecha_inicio_labores: $fecha_inicio_labores,
                numero_gafete: $numero_gafete,
                fecha_nacimiento: $fecha_nacimiento,
                telefono: $telefono,
                direccion: $direccion,
                contacto_emergencia_nombre: $contacto_emergencia_nombre,
                contacto_emergencia_telefono: $contacto_emergencia_telefono,
                vencimiento_portacion: $vencimiento_portacion,
                must_change_password: $must_change_password,
                avatar_path: $avatar_path,
                is_active: true,
                created_at: time::now(),
                updated_at: time::now()
            }
        ",
        )
        .bind(dto)
        .await?;

    let created: Option<User> = result.take(0)?;
    if let Some(user) = created {
        info!("✅ Usuario creado: id={}, email={}", user.id, user.email);
        Ok(user)
    } else {
        warn!("⚠️ Error al crear usuario: CREATE no retornó registro");
        Err(SurrealDbError::Query("No se pudo crear el usuario".to_string()))
    }
}

/// Busca un usuario por su ID.
pub async fn find_by_id(id: &RecordId) -> Result<Option<User>, SurrealDbError> {
    debug!("🔍 Buscando usuario por ID: {id}");
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id").bind(("id", id.clone())).await?;
    let user: Option<User> = result.take(0)?;
    debug!("🔍 Resultado: encontrado={}", user.is_some());
    Ok(user)
}

/// Busca un usuario por ID con su rol poblado.
pub async fn find_by_id_fetched(id: &RecordId) -> Result<Option<UserFetched>, SurrealDbError> {
    debug!("🔍 Buscando usuario (fetched) por ID: {id}");
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id FETCH role").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

/// Busca un usuario por email.
pub async fn find_by_email(email: &str) -> Result<Option<User>, SurrealDbError> {
    debug!("🔍 Buscando usuario por email: {email}");
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM user WHERE email = $email LIMIT 1")
        .bind(("email", email.to_string()))
        .await?;
    Ok(result.take(0)?)
}

/// Busca un usuario por email incluyendo el `password_hash` para autenticación.
pub async fn find_by_email_with_password(
    email: &str,
) -> Result<Option<(User, String)>, SurrealDbError> {
    let db = get_db().await?;
    debug!("🔐 Autenticando usuario: {email}");

    let mut result = db
        .query("SELECT *, password_hash FROM user WHERE email = $email LIMIT 1")
        .bind(("email", email.to_string()))
        .await?;

    let record: Option<UserWithPassword> = match result.take(0) {
        Ok(r) => r,
        Err(e) => {
            error!("❌ Error deserializando UserWithPassword: {e}");
            return Err(e.into());
        }
    };

    match &record {
        Some(_) => info!("✅ Usuario encontrado para autenticación: {email}"),
        None => warn!("⚠️ Usuario no encontrado para login: {email}"),
    }

    Ok(record.map(UserWithPassword::into_user_and_password))
}

/// Actualiza un usuario existente.
pub async fn update(id: &RecordId, dto: UserUpdateDTO) -> Result<Option<User>, SurrealDbError> {
    debug!("✏️ Actualizando usuario: {id}");
    let db = get_db().await?;
    let updated: Option<User> = db.update(id.clone()).merge(dto).await?;

    if updated.is_some() {
        info!("✅ Usuario actualizado: {id}");
    } else {
        warn!("⚠️ Usuario no encontrado para actualizar: {id}");
    }

    Ok(updated)
}

/// Actualiza el `password_hash` de un usuario.
pub async fn update_password(id: &RecordId, password_hash: &str) -> Result<(), SurrealDbError> {
    debug!("🔐 Actualizando contraseña para usuario: {id}");
    let db = get_db().await?;
    db.query(
        r"
        UPDATE $id SET 
            password_hash = $password_hash,
            must_change_password = false,
            updated_at = time::now()
        ",
    )
    .bind(("id", id.clone()))
    .bind(("password_hash", password_hash.to_string()))
    .await?
    .check()?;

    info!("✅ Contraseña actualizada para usuario: {id}");
    Ok(())
}

/// Elimina un usuario del sistema.
pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    warn!("🗑️ Eliminando usuario: {id}");
    let db = get_db().await?;
    db.query("DELETE $id").bind(("id", id.clone())).await?;
    warn!("🗑️ Usuario eliminado: {id}");
    Ok(())
}

pub async fn find_all(exclude_id: Option<&RecordId>) -> Result<Vec<User>, SurrealDbError> {
    let db = get_db().await?;
    let mut query = "SELECT * FROM user".to_string();
    if exclude_id.is_some() {
        query.push_str(" WHERE id != $exclude_id");
    }
    query.push_str(" ORDER BY created_at DESC");

    let mut result = db.query(&query);
    if let Some(id) = exclude_id {
        result = result.bind(("exclude_id", id.clone()));
    }

    let mut response = result.await?;
    Ok(response.take(0)?)
}

pub async fn find_all_fetched(
    exclude_id: Option<&RecordId>,
) -> Result<Vec<UserFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut query = "SELECT * FROM user".to_string();
    if exclude_id.is_some() {
        query.push_str(" WHERE id != $exclude_id");
    }
    query.push_str(" ORDER BY created_at DESC FETCH role");

    let mut result = db.query(&query);
    if let Some(id) = exclude_id {
        result = result.bind(("exclude_id", id.clone()));
    }

    let mut response = result.await?;
    Ok(response.take(0)?)
}

pub async fn count_by_email(email: &str) -> Result<i64, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM user WHERE email = $email GROUP ALL")
        .bind(("email", email.to_string()))
        .await?;

    #[derive(Deserialize)]
    struct CountResult {
        count: i64,
    }

    let rows: Vec<CountResult> = result.take(0)?;
    Ok(rows.first().map_or(0, |c| c.count))
}

pub async fn count_by_email_excluding_id(
    email: &str,
    exclude_id: &RecordId,
) -> Result<i64, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM user WHERE email = $email AND id != $exclude_id GROUP ALL")
        .bind(("email", email.to_string()))
        .bind(("exclude_id", exclude_id.clone()))
        .await?;

    #[derive(Deserialize)]
    struct CountResult {
        count: i64,
    }

    let rows: Vec<CountResult> = result.take(0)?;
    Ok(rows.first().map_or(0, |c| c.count))
}

pub async fn get_role_name(role: &RecordId) -> Result<String, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT name FROM $role").bind(("role", role.clone())).await?;

    #[derive(Deserialize)]
    struct RoleName {
        name: String,
    }

    let row: Option<RoleName> = result.take(0)?;
    Ok(row.map_or_else(|| "Desconocido".to_string(), |r| r.name))
}

pub async fn update_avatar_path(user_id: &str, avatar_path: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let clean_id = user_id.trim_start_matches("user:").replace(['⟨', '⟩', '<', '>'], "");
    let user_record = RecordId::from_table_key("user", &clean_id);

    db.query("UPDATE $id SET avatar_path = $avatar_path, updated_at = time::now()")
        .bind(("id", user_record))
        .bind(("avatar_path", avatar_path.to_string()))
        .await?
        .check()?;
    Ok(())
}

pub async fn get_avatar_path(user_id: &str) -> Result<String, SurrealDbError> {
    let db = get_db().await?;
    let clean_id = user_id.trim_start_matches("user:").replace(['⟨', '⟩', '<', '>'], "");
    let user_record = RecordId::from_table_key("user", &clean_id);

    #[derive(Deserialize)]
    struct AvatarPath {
        avatar_path: Option<String>,
    }

    let mut result = db.query("SELECT avatar_path FROM $id").bind(("id", user_record)).await?;

    let row: Option<AvatarPath> = result.take(0)?;
    Ok(row.and_then(|r| r.avatar_path).unwrap_or_default())
}
