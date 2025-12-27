use crate::models::user::User;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;
use serde::Deserialize;

// Estructura auxiliar para query de password

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
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    // Crear contenido. Nota: password_hash se guarda en la misma tabla normalmente
    let sql = r#"
        CREATE type::thing('users', $id) CONTENT {
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
            created_at: $now,
            updated_at: $now
        }
    "#;

    let mut result = client
        .query(sql)
        .bind(("id", id.to_string()))
        .bind(("email", email.to_string()))
        .bind(("password_hash", password_hash.to_string()))
        .bind(("nombre", nombre.to_string()))
        .bind(("apellido", apellido.to_string()))
        .bind(("role_id", role_id.to_string()))
        .bind(("cedula", cedula.to_string()))
        .bind(("segundo_nombre", segundo_nombre.map(String::from)))
        .bind(("segundo_apellido", segundo_apellido.map(String::from)))
        .bind(("fecha_inicio_labores", fecha_inicio_labores.map(String::from)))
        .bind(("numero_gafete", numero_gafete.map(String::from)))
        .bind(("fecha_nacimiento", fecha_nacimiento.map(String::from)))
        .bind(("telefono", telefono.map(String::from)))
        .bind(("direccion", direccion.map(String::from)))
        .bind(("contacto_emergencia_nombre", contacto_emergencia_nombre.map(String::from)))
        .bind(("contacto_emergencia_telefono", contacto_emergencia_telefono.map(String::from)))
        .bind(("must_change_password", must_change_password))
        .bind(("avatar_path", avatar_path.map(String::from)))
        .bind(("now", now))
        .await?;

    let created: Option<User> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("No se pudo crear el usuario".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<User>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM type::thing('users', $id)";
    let mut result = client.query(sql).bind(("id", id.to_string())).await?;
    Ok(result.take(0)?)
}

pub async fn find_by_email(email: &str) -> Result<Option<User>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM users WHERE email = $email LIMIT 1";
    let mut result = client.query(sql).bind(("email", email.to_string())).await?;
    Ok(result.take(0)?)
}

// Retorna usuario y su hash, necesario para auth
pub async fn find_by_email_with_password(
    email: &str,
) -> Result<Option<(User, String)>, SurrealDbError> {
    let client = get_db().await?;
    // Necesitamos traer el password_hash que usualmente no está en el struct User público si lo excluimos con serde skip,
    // pero en models/user.rs User no tiene password_hash.
    // Así que seleccionamos todo y lo mapeamos a una estructura intermedia que incluya password_hash.
    let sql = "SELECT *, password_hash FROM users WHERE email = $email LIMIT 1";
    let mut result = client.query(sql).bind(("email", email.to_string())).await?;

    // Usamos UserWithPassword para deserializar
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
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    // Construimos query dinámico o usamos MERGE con valores opcionales.
    // SurrealDB MERGE ignora nulos? No, MERGE sobreescribe si le pasas null (None en Rust -> null en JSON).
    // Debemos construir el objeto solo con los campos Some.
    // O usamos una query con `IF $val != NONE THEN ...` es complicado.
    // Mejor estrategia: Crear un Map<String, Value> y pasarlo como CONTENT en MERGE.

    let mut map = serde_json::Map::new();
    map.insert("updated_at".to_string(), serde_json::json!(now));

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

    let sql = "UPDATE type::thing('users', $id) MERGE $data";
    let mut result = client.query(sql).bind(("id", id.to_string())).bind(("data", map)).await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let client = get_db().await?;
    let sql = "DELETE type::thing('users', $id)";
    let mut _result = client.query(sql).bind(("id", id.to_string())).await?;
    Ok(())
}

pub async fn find_all(exclude_id: &str) -> Result<Vec<User>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM users WHERE id != $exclude_id ORDER BY created_at DESC";
    let mut result = client.query(sql).bind(("exclude_id", exclude_id.to_string())).await?;
    Ok(result.take(0)?)
}

pub async fn count_by_email(email: &str) -> Result<i64, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT count() FROM users WHERE email = $email GROUP ALL";
    let mut result = client.query(sql).bind(("email", email.to_string())).await?;
    // Surreal retorna array de objetos { count: N }
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
    let client = get_db().await?;
    let sql = "SELECT count() FROM users WHERE email = $email AND id != $exclude_id GROUP ALL";
    let mut result = client
        .query(sql)
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

// Helper para roles (migración parcial)
pub async fn get_role_name(role_id: &str) -> Result<String, SurrealDbError> {
    // Si role_id es hardcoded "role-admin" o "role-guardia", retornamos nombre directo?
    // O consultamos tabla roles.
    // Asumiremos tabla roles existe en Surreal
    let client = get_db().await?;
    let sql = "SELECT name FROM type::thing('roles', $id)";
    let mut result = client.query(sql).bind(("id", role_id.to_string())).await?;
    #[derive(Deserialize)]
    struct RoleName {
        name: String,
    }
    let row: Option<RoleName> = result.take(0)?;
    Ok(row.map(|r| r.name).unwrap_or_else(|| "Desconocido".to_string()))
}
