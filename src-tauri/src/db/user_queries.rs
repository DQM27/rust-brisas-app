// ==========================================
// src/db/user_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query! y query_as! para validación en tiempo de compilación

use crate::models::user::User;
use sqlx::SqlitePool;

// ==========================================
// TIPOS AUXILIARES
// ==========================================

pub struct UserWithPassword {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role_id: String,
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
    pub password_hash: String,
}

impl UserWithPassword {
    pub fn split(self) -> (User, String) {
        (
            User {
                id: self.id,
                email: self.email,
                nombre: self.nombre,
                apellido: self.apellido,
                role_id: self.role_id,
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
            },
            self.password_hash,
        )
    }
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un usuario por ID (Solo activos)
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            email,
            nombre,
            apellido,
            role_id,
            is_active as "is_active: bool",
            created_at,
            updated_at,
            cedula,
            segundo_nombre,
            segundo_apellido,
            fecha_inicio_labores,
            numero_gafete,
            fecha_nacimiento,
            telefono,
            direccion,
            contacto_emergencia_nombre,
            contacto_emergencia_telefono,
            must_change_password as "must_change_password: bool",
            deleted_at
        FROM users
        WHERE id = ? AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

/// Busca un usuario por email con password (Solo activos)
pub async fn find_by_email_with_password(
    pool: &SqlitePool,
    email: &str,
) -> sqlx::Result<(User, String)> {
    let result = sqlx::query_as!(
        UserWithPassword,
        r#"
        SELECT
            id,
            email,
            nombre,
            apellido,
            role_id,
            is_active as "is_active: bool",
            created_at,
            updated_at,
            cedula,
            segundo_nombre,
            segundo_apellido,
            fecha_inicio_labores,
            numero_gafete,
            fecha_nacimiento,
            telefono,
            direccion,
            contacto_emergencia_nombre,
            contacto_emergencia_telefono,
            must_change_password as "must_change_password: bool",
            deleted_at,
            password_hash
        FROM users
        WHERE email = ? AND deleted_at IS NULL
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(result.split())
}

/// Obtiene todos los usuarios activos (excluyendo superuser)
pub async fn find_all(pool: &SqlitePool, exclude_superuser_id: &str) -> sqlx::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            email,
            nombre,
            apellido,
            role_id,
            is_active as "is_active: bool",
            created_at,
            updated_at,
            cedula,
            segundo_nombre,
            segundo_apellido,
            fecha_inicio_labores,
            numero_gafete,
            fecha_nacimiento,
            telefono,
            direccion,
            contacto_emergencia_nombre,
            contacto_emergencia_telefono,
            must_change_password as "must_change_password: bool",
            deleted_at
        FROM users
        WHERE deleted_at IS NULL AND id != ?
        ORDER BY created_at DESC
        "#,
        exclude_superuser_id
    )
    .fetch_all(pool)
    .await
}

/// Cuenta cuántos usuarios tienen un email específico
pub async fn count_by_email(pool: &SqlitePool, email: &str) -> sqlx::Result<i32> {
    let result = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE email = ?", email)
        .fetch_one(pool)
        .await?;

    Ok(result.count)
}

/// Cuenta cuántos usuarios tienen un email excluyendo un ID
pub async fn count_by_email_excluding_id(
    pool: &SqlitePool,
    email: &str,
    exclude_id: &str,
) -> sqlx::Result<i32> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM users WHERE email = ? AND id != ?",
        email,
        exclude_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count)
}

/// Obtiene el nombre de un rol por ID
pub async fn get_role_name(pool: &SqlitePool, role_id: &str) -> sqlx::Result<String> {
    let result = sqlx::query!("SELECT name FROM roles WHERE id = ?", role_id)
        .fetch_one(pool)
        .await?;

    Ok(result.name)
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo usuario
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    email: &str,
    password_hash: &str,
    nombre: &str,
    apellido: &str,
    role_id: &str,
    created_at: &str,
    updated_at: &str,
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
) -> sqlx::Result<()> {
    // SQLx maneja la conversión de bool a numeric en SQLite automáticamente
    // cuando se usa query! macro si el tipo de la columna es compatible
    sqlx::query!(
        r#"
        INSERT INTO users (
            id, email, password_hash, nombre, apellido, role_id, created_at, updated_at,
            cedula, segundo_nombre, segundo_apellido,
            fecha_inicio_labores, numero_gafete, fecha_nacimiento, telefono, direccion,
            contacto_emergencia_nombre, contacto_emergencia_telefono, must_change_password, is_active
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1)
        "#,
        id, email, password_hash, nombre, apellido, role_id, created_at, updated_at,
        cedula, segundo_nombre, segundo_apellido,
        fecha_inicio_labores, numero_gafete, fecha_nacimiento, telefono, direccion,
        contacto_emergencia_nombre, contacto_emergencia_telefono, must_change_password
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza un usuario existente
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    email: Option<&str>,
    password_hash: Option<&str>,
    nombre: Option<&str>,
    apellido: Option<&str>,
    role_id: Option<&str>,
    is_active: Option<bool>,
    updated_at: &str,
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
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        UPDATE users SET
            email = COALESCE(?, email),
            password_hash = COALESCE(?, password_hash),
            nombre = COALESCE(?, nombre),
            apellido = COALESCE(?, apellido),
            role_id = COALESCE(?, role_id),
            is_active = COALESCE(?, is_active),
            updated_at = ?,
            cedula = COALESCE(?, cedula),
            segundo_nombre = COALESCE(?, segundo_nombre),
            segundo_apellido = COALESCE(?, segundo_apellido),
            fecha_inicio_labores = COALESCE(?, fecha_inicio_labores),
            numero_gafete = COALESCE(?, numero_gafete),
            fecha_nacimiento = COALESCE(?, fecha_nacimiento),
            telefono = COALESCE(?, telefono),
            direccion = COALESCE(?, direccion),
            contacto_emergencia_nombre = COALESCE(?, contacto_emergencia_nombre),
            contacto_emergencia_telefono = COALESCE(?, contacto_emergencia_telefono),
            must_change_password = COALESCE(?, must_change_password)
        WHERE id = ?
        "#,
        email,
        password_hash,
        nombre,
        apellido,
        role_id,
        is_active,
        updated_at,
        cedula,
        segundo_nombre,
        segundo_apellido,
        fecha_inicio_labores,
        numero_gafete,
        fecha_nacimiento,
        telefono,
        direccion,
        contacto_emergencia_nombre,
        contacto_emergencia_telefono,
        must_change_password,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Soft delete de usuario
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query!("UPDATE users SET deleted_at = ? WHERE id = ?", now, id)
        .execute(pool)
        .await?;

    Ok(())
}
