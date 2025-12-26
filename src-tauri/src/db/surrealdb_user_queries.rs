// ==========================================
// SurrealDB User Repository
// ==========================================
// Operaciones CRUD para usuarios usando SurrealDB

use crate::models::user::{CreateUserInput, UpdateUserInput, User};
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// ==========================================
// TIPOS PARA SURREALDB
// ==========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealUser {
    pub id: Thing,
    pub email: String,
    pub password: String,
    pub nombre: String,
    pub apellido: String,
    pub role: Thing,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
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
    pub deleted_at: Option<String>,
}

impl SurrealUser {
    /// Convierte a User de dominio
    pub fn to_domain(self) -> User {
        User {
            id: self.id.id.to_string(),
            email: self.email,
            nombre: self.nombre,
            apellido: self.apellido,
            role_id: self.role.id.to_string(),
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
        }
    }
}

// ==========================================
// QUERIES
// ==========================================

/// Obtiene todos los usuarios
pub async fn get_all_users() -> Result<Vec<User>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let users: Vec<SurrealUser> =
        client.query("SELECT * FROM usuarios WHERE deleted_at IS NONE").await?.take(0)?;

    Ok(users.into_iter().map(|u| u.to_domain()).collect())
}

/// Obtiene un usuario por ID
pub async fn get_user_by_id(id: &str) -> Result<Option<User>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let user: Option<SurrealUser> = client.select(("usuarios", id)).await?;

    Ok(user.map(|u| u.to_domain()))
}

/// Obtiene un usuario por email
pub async fn get_user_by_email(email: String) -> Result<Option<User>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let mut result = client
        .query("SELECT * FROM usuarios WHERE email = $email AND deleted_at IS NONE")
        .bind(("email", email))
        .await?;

    let users: Vec<SurrealUser> = result.take(0)?;
    Ok(users.into_iter().next().map(|u| u.to_domain()))
}

/// Crea un nuevo usuario
pub async fn create_user(
    input: CreateUserInput,
    password_hash: String,
) -> Result<User, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id = uuid::Uuid::new_v4().to_string();
    let role_id = input.role_id.clone().unwrap_or_else(|| "guardia".to_string());
    let now = chrono::Utc::now().to_rfc3339();

    // Clonar todos los valores antes de bind
    let email = input.email.clone();
    let nombre = input.nombre.clone();
    let apellido = input.apellido.clone();
    let cedula = input.cedula.clone();
    let segundo_nombre = input.segundo_nombre.clone();
    let segundo_apellido = input.segundo_apellido.clone();
    let fecha_inicio_labores = input.fecha_inicio_labores.clone();
    let numero_gafete = input.numero_gafete.clone();
    let fecha_nacimiento = input.fecha_nacimiento.clone();
    let telefono = input.telefono.clone();
    let direccion = input.direccion.clone();
    let contacto_emergencia_nombre = input.contacto_emergencia_nombre.clone();
    let contacto_emergencia_telefono = input.contacto_emergencia_telefono.clone();
    let must_change_password = input.must_change_password.unwrap_or(true);

    let user: Option<SurrealUser> = client
        .query(
            r#"
            CREATE usuarios CONTENT {
                id: $id,
                email: $email,
                password: $password,
                nombre: $nombre,
                apellido: $apellido,
                role: type::thing('roles', $role_id),
                is_active: true,
                created_at: $now,
                updated_at: $now,
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
                deleted_at: NONE
            }
            "#,
        )
        .bind(("id", id))
        .bind(("email", email))
        .bind(("password", password_hash))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("role_id", role_id))
        .bind(("now", now.clone()))
        .bind(("cedula", cedula))
        .bind(("segundo_nombre", segundo_nombre))
        .bind(("segundo_apellido", segundo_apellido))
        .bind(("fecha_inicio_labores", fecha_inicio_labores))
        .bind(("numero_gafete", numero_gafete))
        .bind(("fecha_nacimiento", fecha_nacimiento))
        .bind(("telefono", telefono))
        .bind(("direccion", direccion))
        .bind(("contacto_emergencia_nombre", contacto_emergencia_nombre))
        .bind(("contacto_emergencia_telefono", contacto_emergencia_telefono))
        .bind(("must_change_password", must_change_password))
        .await?
        .take(0)?;

    user.map(|u| u.to_domain())
        .ok_or_else(|| SurrealDbError::Query("Error creando usuario".to_string()))
}

/// Actualiza un usuario
pub async fn update_user(id: String, input: UpdateUserInput) -> Result<User, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Construir query dinámicamente basado en campos presentes
    let mut updates = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();

    if input.email.is_some() {
        updates.push("email = $email");
    }
    if input.nombre.is_some() {
        updates.push("nombre = $nombre");
    }
    if input.apellido.is_some() {
        updates.push("apellido = $apellido");
    }
    if input.is_active.is_some() {
        updates.push("is_active = $is_active");
    }
    if input.cedula.is_some() {
        updates.push("cedula = $cedula");
    }
    updates.push("updated_at = $now");

    let query = format!("UPDATE usuarios:{} SET {}", id, updates.join(", "));

    let mut q = client.query(&query);

    if let Some(email) = input.email {
        q = q.bind(("email", email));
    }
    if let Some(nombre) = input.nombre {
        q = q.bind(("nombre", nombre));
    }
    if let Some(apellido) = input.apellido {
        q = q.bind(("apellido", apellido));
    }
    if let Some(is_active) = input.is_active {
        q = q.bind(("is_active", is_active));
    }
    if let Some(cedula) = input.cedula {
        q = q.bind(("cedula", cedula));
    }
    q = q.bind(("now", now));

    let user: Option<SurrealUser> = q.await?.take(0)?;

    user.map(|u| u.to_domain())
        .ok_or_else(|| SurrealDbError::Query("Usuario no encontrado".to_string()))
}

/// Elimina un usuario (soft delete)
pub async fn delete_user(id: String) -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let now = chrono::Utc::now().to_rfc3339();

    client
        .query("UPDATE usuarios:$id SET deleted_at = $now")
        .bind(("id", id))
        .bind(("now", now))
        .await?;

    Ok(())
}

/// Verifica credenciales para login
pub async fn verify_credentials(
    email: String,
    password: String,
) -> Result<Option<User>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Obtener usuario con hash de password
    let mut result = client
        .query(
            r#"
            SELECT * FROM usuarios 
            WHERE email = $email 
            AND deleted_at IS NONE 
            AND is_active = true
            "#,
        )
        .bind(("email", email))
        .await?;

    let users: Vec<SurrealUser> = result.take(0)?;

    if let Some(user) = users.into_iter().next() {
        // Verificar password con argon2
        let parsed_hash = argon2::PasswordHash::new(&user.password)
            .map_err(|e| SurrealDbError::Query(format!("Error parsing hash: {}", e)))?;

        use argon2::PasswordVerifier;
        if argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
            return Ok(Some(user.to_domain()));
        }
    }

    Ok(None)
}
