use crate::models::user::{User, UserCreateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde::Deserialize;
use surrealdb::RecordId;

#[derive(Debug, Deserialize)]
struct UserWithPassword {
    #[serde(flatten)]
    user: User,
    password_hash: String,
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
    let mut result = db.query("SELECT * FROM $id").bind(("id", id.clone())).await?;
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
    let mut result = db
        .query("SELECT *, password_hash FROM user WHERE email = $email LIMIT 1")
        .bind(("email", email.to_string()))
        .await?;

    let record: Option<UserWithPassword> = result.take(0)?;
    match record {
        Some(u) => Ok(Some((u.user, u.password_hash))),
        None => Ok(None),
    }
}

pub async fn update(
    id: &RecordId,
    data: serde_json::Value,
) -> Result<Option<User>, SurrealDbError> {
    let db = get_db().await?;
    let mut result =
        db.query("UPDATE $id MERGE $data").bind(("id", id.clone())).bind(("data", data)).await?;

    Ok(result.take(0)?)
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
