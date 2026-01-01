use crate::models::user::{User, UserCreateDTO, UserFetched, UserUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::{error, info};
use serde::Deserialize;
use surrealdb::{Datetime, RecordId};

/// Struct para deserializar usuario con password_hash
/// NO usamos #[serde(flatten)] porque causa errores en SurrealDB 2.x
#[derive(Debug, Deserialize)]
struct UserWithPassword {
    pub id: RecordId,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role: RecordId,
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
            must_change_password: self.must_change_password,
            deleted_at: self.deleted_at,
            avatar_path: self.avatar_path,
        };
        (user, self.password_hash)
    }
}

pub async fn insert(dto: UserCreateDTO) -> Result<User, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            CREATE user CONTENT {
                email: $email,
                password_hash: $password_hash,
                nombre: $nombre,
                apellido: $apellido,
                role: $role,
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
                must_change_password: $must_change_password,
                avatar_path: $avatar_path,
                is_active: true,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(dto)
        .await?;

    let created: Option<User> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("No se pudo crear el usuario".to_string()))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<User>, SurrealDbError> {
    let db = get_db().await?;
    info!("🔍 find_by_id buscando: {:?}", id);
    let mut result = db.query("SELECT * FROM $id").bind(("id", id.clone())).await?;
    let user: Option<User> = result.take(0)?;
    info!("🔍 find_by_id resultado: {:?}", user.is_some());
    Ok(user)
}

pub async fn find_by_id_fetched(id: &RecordId) -> Result<Option<UserFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id FETCH role").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

pub async fn find_by_email(email: &str) -> Result<Option<User>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM user WHERE email = $email LIMIT 1")
        .bind(("email", email.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_email_with_password(
    email: &str,
) -> Result<Option<(User, String)>, SurrealDbError> {
    let db = get_db().await?;
    info!("🔍 Buscando usuario con password: {}", email);

    let mut result = db
        .query("SELECT *, password_hash FROM user WHERE email = $email LIMIT 1")
        .bind(("email", email.to_string()))
        .await?;

    let record: Option<UserWithPassword> = match result.take(0) {
        Ok(r) => r,
        Err(e) => {
            error!("❌ Error deserializando UserWithPassword: {}", e);
            return Err(e.into());
        }
    };

    if record.is_some() {
        info!("✅ Usuario encontrado con password_hash");
    } else {
        info!("⚠️ Usuario no encontrado para email: {}", email);
    }

    Ok(record.map(|u| u.into_user_and_password()))
}

pub async fn update(id: &RecordId, dto: UserUpdateDTO) -> Result<Option<User>, SurrealDbError> {
    let db = get_db().await?;

    // 1. MERGE principal usando el cliente
    // Nota: El merge usando cliente a veces tiene problemas con campos opcionales en ciertas versiones,
    // pero mantenemos la implementación estándar
    let updated: Option<User> = db.update(id.clone()).merge(dto).await?;

    Ok(updated)
}

/// Update password hash using native SurrealDB time::now()
pub async fn update_password(id: &RecordId, password_hash: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    db.query(
        r#"
        UPDATE $id SET 
            password_hash = $password_hash,
            must_change_password = false,
            updated_at = time::now()
        "#,
    )
    .bind(("id", id.clone()))
    .bind(("password_hash", password_hash.to_string()))
    .await?
    .check()?;

    Ok(())
}

pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    db.query("DELETE $id").bind(("id", id.clone())).await?;
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
    Ok(rows.first().map(|c| c.count).unwrap_or(0))
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
    Ok(rows.first().map(|c| c.count).unwrap_or(0))
}

pub async fn get_role_name(role: &RecordId) -> Result<String, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT name FROM $role").bind(("role", role.clone())).await?;

    #[derive(Deserialize)]
    struct RoleName {
        name: String,
    }

    let row: Option<RoleName> = result.take(0)?;
    Ok(row.map(|r| r.name).unwrap_or_else(|| "Desconocido".to_string()))
}
