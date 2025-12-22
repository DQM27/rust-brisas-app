// ==========================================
// src/db/user_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Sin lógica de negocio, solo interacción con la base de datos

use crate::models::user::User;
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

const SELECT_USERS: &str = "
    SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at,
           cedula, segundo_nombre, segundo_apellido,
           fecha_inicio_labores, numero_gafete, fecha_nacimiento, telefono, direccion,
           contacto_emergencia_nombre, contacto_emergencia_telefono, must_change_password, deleted_at
    FROM users
";

#[derive(sqlx::FromRow)]
struct UserWithPassword {
    #[sqlx(flatten)]
    user: User,
    password_hash: String,
}

/// Busca un usuario por ID (Solo activos)
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<User> {
    sqlx::query_as::<_, User>(&format!(
        "{} WHERE id = ? AND deleted_at IS NULL",
        SELECT_USERS
    ))
    .bind(id)
    .fetch_one(pool)
    .await
}

/// Busca un usuario por email (Solo activos)
pub async fn find_by_email_with_password(
    pool: &SqlitePool,
    email: &str,
) -> sqlx::Result<(User, String)> {
    let row = sqlx::query_as::<_, UserWithPassword>(&format!(
        "{} WHERE email = ? AND deleted_at IS NULL",
        SELECT_USERS.replace("FROM users", ", password_hash FROM users") // Inject password_hash
    ))
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok((row.user, row.password_hash))
}

/// Obtiene todos los usuarios activos ordenados por fecha de creación
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<User>> {
    sqlx::query_as::<_, User>(&format!(
        "{} WHERE deleted_at IS NULL ORDER BY created_at DESC",
        SELECT_USERS
    ))
    .fetch_all(pool)
    .await
}

/// Cuenta cuántos usuarios tienen un email específico (para verificar unicidad)
pub async fn count_by_email(pool: &SqlitePool, email: &str) -> sqlx::Result<i32> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(row.get("count"))
}

/// Cuenta cuántos usuarios tienen un email específico excluyendo un ID
/// (útil para updates)
pub async fn count_by_email_excluding_id(
    pool: &SqlitePool,
    email: &str,
    exclude_id: &str,
) -> sqlx::Result<i32> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE email = ? AND id != ?")
        .bind(email)
        .bind(exclude_id)
        .fetch_one(pool)
        .await?;

    Ok(row.get("count"))
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo usuario en la base de datos
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    email: &str,
    password_hash: &str,
    nombre: &str,
    apellido: &str,
    role: &str,
    created_at: &str,
    updated_at: &str,
    // New params
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
    let must_change_password_int = if must_change_password { 1 } else { 0 };

    sqlx::query(
        "INSERT INTO users (
            id, email, password_hash, nombre, apellido, role, created_at, updated_at,
            cedula, segundo_nombre, segundo_apellido,
            fecha_inicio_labores, numero_gafete, fecha_nacimiento, telefono, direccion,
            contacto_emergencia_nombre, contacto_emergencia_telefono, must_change_password
        ) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(email)
    .bind(password_hash)
    .bind(nombre)
    .bind(apellido)
    .bind(role)
    .bind(created_at)
    .bind(updated_at)
    // New params
    .bind(cedula)
    .bind(segundo_nombre)
    .bind(segundo_apellido)
    .bind(fecha_inicio_labores)
    .bind(numero_gafete)
    .bind(fecha_nacimiento)
    .bind(telefono)
    .bind(direccion)
    .bind(contacto_emergencia_nombre)
    .bind(contacto_emergencia_telefono)
    .bind(must_change_password_int)
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza un usuario existente (campos opcionales)
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    email: Option<&str>,
    password_hash: Option<&str>,
    nombre: Option<&str>,
    apellido: Option<&str>,
    role: Option<&str>,
    is_active: Option<i32>,
    updated_at: &str,
    // New params
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
    let must_change_password_int = must_change_password.map(|b| if b { 1 } else { 0 });

    sqlx::query(
        r#"UPDATE users SET
            email = COALESCE(?, email),
            password_hash = COALESCE(?, password_hash),
            nombre = COALESCE(?, nombre),
            apellido = COALESCE(?, apellido),
            role = COALESCE(?, role),
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
        WHERE id = ?"#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(nombre)
    .bind(apellido)
    .bind(role)
    .bind(is_active)
    .bind(updated_at)
    // New params
    .bind(cedula)
    .bind(segundo_nombre)
    .bind(segundo_apellido)
    .bind(fecha_inicio_labores)
    .bind(numero_gafete)
    .bind(fecha_nacimiento)
    .bind(telefono)
    .bind(direccion)
    .bind(contacto_emergencia_nombre)
    .bind(contacto_emergencia_telefono)
    .bind(must_change_password_int)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina un usuario por ID (Soft Delete)
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE users SET deleted_at = ? WHERE id = ?")
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
