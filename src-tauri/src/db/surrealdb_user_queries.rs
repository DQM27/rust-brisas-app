// ==========================================
// src/db/surrealdb_user_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::user::User;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct UserWithPassword {
    #[serde(flatten)]
    user: User,
    password_hash: String,
}

pub async fn insert(
    id: &str,
    email: &str,
    password_hash: &str,
    nombre: &str,
    apellido: &str,
    role_id: &str,
    cedula: &str,
    segundo_nombre: Option<&str>,
    segundo_apellido: Option<&str>,
    fecha_inicio_labores: Option<&str>,
    numero_gafete: Option<&str>,
    fecha_nacimiento: Option<&str>,
    telefono: Option<&str>,
    direccion: Option<&str>,
    contacto_emergencia_nombre: Option<&str>,
    contacto_emergencia_telefono: Option<&str>,
    must_change_password: bool,
    avatar_path: Option<&str>,
) -> Result<User, SurrealDbError> {
    let db = get_db().await?;

    // Convert all to owned types for SurrealDB bind
    let id_owned = id.to_string();
    let email_owned = email.to_string();
    let password_hash_owned = password_hash.to_string();
    let nombre_owned = nombre.to_string();
    let apellido_owned = apellido.to_string();
    let role_id_owned = role_id.to_string();
    let cedula_owned = cedula.to_string();
    let segundo_nombre_owned = segundo_nombre.map(String::from);
    let segundo_apellido_owned = segundo_apellido.map(String::from);
    let fecha_inicio_owned = fecha_inicio_labores.map(String::from);
    let numero_gafete_owned = numero_gafete.map(String::from);
    let fecha_nacimiento_owned = fecha_nacimiento.map(String::from);
    let telefono_owned = telefono.map(String::from);
    let direccion_owned = direccion.map(String::from);
    let contacto_nombre_owned = contacto_emergencia_nombre.map(String::from);
    let contacto_telefono_owned = contacto_emergencia_telefono.map(String::from);
    let avatar_path_owned = avatar_path.map(String::from);

    let mut result = db
        .query(
            r#"
            CREATE type::thing('user', $id) CONTENT {
                id: $id,
                email: $email,
                password_hash: $password_hash,
                nombre: $nombre,
                apellido: $apellido,
                role_id: $role_id,
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
        .bind(("id", id_owned))
        .bind(("email", email_owned))
        .bind(("password_hash", password_hash_owned))
        .bind(("nombre", nombre_owned))
        .bind(("apellido", apellido_owned))
        .bind(("role_id", role_id_owned))
        .bind(("cedula", cedula_owned))
        .bind(("segundo_nombre", segundo_nombre_owned))
        .bind(("segundo_apellido", segundo_apellido_owned))
        .bind(("fecha_inicio_labores", fecha_inicio_owned))
        .bind(("numero_gafete", numero_gafete_owned))
        .bind(("fecha_nacimiento", fecha_nacimiento_owned))
        .bind(("telefono", telefono_owned))
        .bind(("direccion", direccion_owned))
        .bind(("contacto_emergencia_nombre", contacto_nombre_owned))
        .bind(("contacto_emergencia_telefono", contacto_telefono_owned))
        .bind(("must_change_password", must_change_password))
        .bind(("avatar_path", avatar_path_owned))
        .await?;

    let created: Option<User> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("No se pudo crear el usuario".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<User>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("user:").unwrap_or(id).to_string();
    let mut result =
        db.query("SELECT * FROM type::thing('user', $id)").bind(("id", id_only)).await?;
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
    id: &str,
    email: Option<&str>,
    password_hash: Option<&str>,
    nombre: Option<&str>,
    apellido: Option<&str>,
    role_id: Option<&str>,
    is_active: Option<bool>,
    cedula: Option<&str>,
    segundo_nombre: Option<&str>,
    segundo_apellido: Option<&str>,
    fecha_inicio_labores: Option<&str>,
    numero_gafete: Option<&str>,
    fecha_nacimiento: Option<&str>,
    telefono: Option<&str>,
    direccion: Option<&str>,
    contacto_emergencia_nombre: Option<&str>,
    contacto_emergencia_telefono: Option<&str>,
    must_change_password: Option<bool>,
    avatar_path: Option<&str>,
) -> Result<Option<User>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("user:").unwrap_or(id).to_string();

    let mut map = serde_json::Map::new();

    if let Some(v) = email {
        map.insert("email".to_string(), serde_json::json!(v));
    }
    if let Some(v) = password_hash {
        map.insert("password_hash".to_string(), serde_json::json!(v));
    }
    if let Some(v) = nombre {
        map.insert("nombre".to_string(), serde_json::json!(v));
    }
    if let Some(v) = apellido {
        map.insert("apellido".to_string(), serde_json::json!(v));
    }
    if let Some(v) = role_id {
        map.insert("role_id".to_string(), serde_json::json!(v));
    }
    if let Some(v) = is_active {
        map.insert("is_active".to_string(), serde_json::json!(v));
    }
    if let Some(v) = cedula {
        map.insert("cedula".to_string(), serde_json::json!(v));
    }
    if let Some(v) = segundo_nombre {
        map.insert("segundo_nombre".to_string(), serde_json::json!(v));
    }
    if let Some(v) = segundo_apellido {
        map.insert("segundo_apellido".to_string(), serde_json::json!(v));
    }
    if let Some(v) = fecha_inicio_labores {
        map.insert("fecha_inicio_labores".to_string(), serde_json::json!(v));
    }
    if let Some(v) = numero_gafete {
        map.insert("numero_gafete".to_string(), serde_json::json!(v));
    }
    if let Some(v) = fecha_nacimiento {
        map.insert("fecha_nacimiento".to_string(), serde_json::json!(v));
    }
    if let Some(v) = telefono {
        map.insert("telefono".to_string(), serde_json::json!(v));
    }
    if let Some(v) = direccion {
        map.insert("direccion".to_string(), serde_json::json!(v));
    }
    if let Some(v) = contacto_emergencia_nombre {
        map.insert("contacto_emergencia_nombre".to_string(), serde_json::json!(v));
    }
    if let Some(v) = contacto_emergencia_telefono {
        map.insert("contacto_emergencia_telefono".to_string(), serde_json::json!(v));
    }
    if let Some(v) = must_change_password {
        map.insert("must_change_password".to_string(), serde_json::json!(v));
    }
    if let Some(v) = avatar_path {
        map.insert("avatar_path".to_string(), serde_json::json!(v));
    }

    let mut result = db
        .query("UPDATE type::thing('user', $id) MERGE $data")
        .bind(("id", id_only))
        .bind(("data", map))
        .await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("user:").unwrap_or(id).to_string();
    db.query("DELETE type::thing('user', $id)").bind(("id", id_only)).await?;
    Ok(())
}

pub async fn find_all(exclude_id: &str) -> Result<Vec<User>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM user WHERE id != $exclude_id ORDER BY created_at DESC")
        .bind(("exclude_id", exclude_id.to_string()))
        .await?;
    Ok(result.take(0)?)
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
    exclude_id: &str,
) -> Result<i64, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM user WHERE email = $email AND id != $exclude_id GROUP ALL")
        .bind(("email", email.to_string()))
        .bind(("exclude_id", exclude_id.to_string()))
        .await?;

    #[derive(Deserialize)]
    struct CountResult {
        count: i64,
    }

    let rows: Vec<CountResult> = result.take(0)?;
    Ok(rows.first().map(|c| c.count).unwrap_or(0))
}

pub async fn get_role_name(role_id: &str) -> Result<String, SurrealDbError> {
    let db = get_db().await?;
    let role_id_only = role_id.strip_prefix("role:").unwrap_or(role_id).to_string();

    let mut result =
        db.query("SELECT name FROM type::thing('role', $id)").bind(("id", role_id_only)).await?;

    #[derive(Deserialize)]
    struct RoleName {
        name: String,
    }

    let row: Option<RoleName> = result.take(0)?;
    Ok(row.map(|r| r.name).unwrap_or_else(|| "Desconocido".to_string()))
}
