// ==========================================
// src/db/contratista_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Sin lógica de negocio, solo interacción con la base de datos

use crate::models::contratista::Contratista;
use serde::Serialize;
use sqlx::{Row, SqlitePool};

// ==========================================
// TIPOS AUXILIARES
// ==========================================

#[derive(Debug, Serialize)]
pub struct ContratistaInfo {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub estado: String,
    pub fecha_vencimiento_praind: String,
}

pub struct ContratistaEnhancedRow {
    pub contratista: Contratista,
    pub empresa_nombre: String,
    pub vehiculo_tipo: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub vehiculo_marca: Option<String>,
    pub vehiculo_modelo: Option<String>,
    pub vehiculo_color: Option<String>,
    pub is_blocked: bool,
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca información básica de un contratista por ID
pub async fn find_basic_info_by_id(
    pool: &SqlitePool,
    id: &str,
) -> sqlx::Result<Option<ContratistaInfo>> {
    let row = sqlx::query(
        "SELECT c.id, c.cedula, c.nombre, c.apellido, e.nombre as empresa_nombre, c.estado, c.fecha_vencimiento_praind 
         FROM contratistas c
         LEFT JOIN empresas e ON c.empresa_id = e.id
         WHERE c.id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(ContratistaInfo {
            id: r.get("id"),
            cedula: r.get("cedula"),
            nombre: r.get("nombre"),
            apellido: r.get("apellido"),
            empresa_nombre: r.get("empresa_nombre"),
            estado: r.get("estado"),
            fecha_vencimiento_praind: r.get("fecha_vencimiento_praind"),
        })),
        None => Ok(None),
    }
}

/// Obtiene datos básicos de un contratista (cédula, nombre, apellido)
pub async fn get_basic_data(
    pool: &SqlitePool,
    contratista_id: &str,
) -> sqlx::Result<Option<(String, String, Option<String>, String, Option<String>)>> {
    let row = sqlx::query("SELECT cedula, nombre, segundo_nombre, apellido, segundo_apellido FROM contratistas WHERE id = ?")
        .bind(contratista_id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| {
        (
            r.get("cedula"),
            r.get("nombre"),
            r.get("segundo_nombre"),
            r.get("apellido"),
            r.get("segundo_apellido"),
        )
    }))
}

/// Helper para detalles de lista negra
pub async fn get_blacklist_details(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<Option<(String, String)>> {
    let row = sqlx::query(
        "SELECT motivo_bloqueo, bloqueado_por FROM lista_negra WHERE cedula = ? AND is_active = 1",
    )
    .bind(cedula)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| (r.get("motivo_bloqueo"), r.get("bloqueado_por"))))
}

/// Cuenta contratistas en lista negra por cédula
pub async fn count_cedula_in_blacklist(pool: &SqlitePool, cedula: &str) -> sqlx::Result<i64> {
    let row =
        sqlx::query("SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND is_active = 1")
            .bind(cedula)
            .fetch_one(pool)
            .await?;

    Ok(row.get("count"))
}

/// Cuenta contratistas por cédula
pub async fn count_by_cedula(pool: &SqlitePool, cedula: &str) -> sqlx::Result<i64> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM contratistas WHERE cedula = ?")
        .bind(cedula)
        .fetch_one(pool)
        .await?;

    Ok(row.get("count"))
}

/// Cuenta contratistas por cédula excluyendo un ID
pub async fn count_by_cedula_excluding_id(
    pool: &SqlitePool,
    cedula: &str,
    exclude_id: &str,
) -> sqlx::Result<i64> {
    let row =
        sqlx::query("SELECT COUNT(*) as count FROM contratistas WHERE cedula = ? AND id != ?")
            .bind(cedula)
            .bind(exclude_id)
            .fetch_one(pool)
            .await?;

    Ok(row.get("count"))
}

/// Busca contratista por ID con empresa y vehículo
pub async fn find_by_id_with_empresa(
    pool: &SqlitePool,
    id: &str,
) -> sqlx::Result<Option<ContratistaEnhancedRow>> {
    let row = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre, 
           v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
           EXISTS (SELECT 1 FROM lista_negra ln WHERE ln.cedula = c.cedula AND ln.is_active = 1) as is_blocked
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           LEFT JOIN vehiculos v ON c.id = v.contratista_id
           WHERE c.id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(ContratistaEnhancedRow {
            contratista: row_to_contratista(&row),
            empresa_nombre: row.get("empresa_nombre"),
            vehiculo_tipo: row.try_get("tipo_vehiculo").ok(),
            vehiculo_placa: row.try_get("placa").ok(),
            vehiculo_marca: row.try_get("marca").ok(),
            vehiculo_modelo: row.try_get("modelo").ok(),
            vehiculo_color: row.try_get("color").ok(),
            is_blocked: row.get("is_blocked"),
        })),
        None => Ok(None),
    }
}

/// Busca contratista por cédula con empresa y vehículo
pub async fn find_by_cedula_with_empresa(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<Option<ContratistaEnhancedRow>> {
    let row = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre, 
           v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
           EXISTS (SELECT 1 FROM lista_negra ln WHERE ln.cedula = c.cedula AND ln.is_active = 1) as is_blocked
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           LEFT JOIN vehiculos v ON c.id = v.contratista_id
           WHERE c.cedula = ?"#,
    )
    .bind(cedula)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(ContratistaEnhancedRow {
            contratista: row_to_contratista(&row),
            empresa_nombre: row.get("empresa_nombre"),
            vehiculo_tipo: row.try_get("tipo_vehiculo").ok(),
            vehiculo_placa: row.try_get("placa").ok(),
            vehiculo_marca: row.try_get("marca").ok(),
            vehiculo_modelo: row.try_get("modelo").ok(),
            vehiculo_color: row.try_get("color").ok(),
            is_blocked: row.get("is_blocked"),
        })),
        None => Ok(None),
    }
}

/// Obtiene todos los contratistas con empresa y vehículo
pub async fn find_all_with_empresa(
    pool: &SqlitePool,
) -> sqlx::Result<Vec<(Contratista, String, Option<String>, Option<String>, bool)>> {
    let rows = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre, v.tipo_vehiculo, v.placa,
           EXISTS (SELECT 1 FROM lista_negra ln WHERE ln.cedula = c.cedula AND ln.is_active = 1) as is_blocked
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           LEFT JOIN vehiculos v ON c.id = v.contratista_id
           ORDER BY c.updated_at DESC"#,
    )
    .fetch_all(pool)
    .await?;

    let result: Vec<_> = rows
        .iter()
        .map(|row| {
            (
                row_to_contratista(row),
                row.get::<String, _>("empresa_nombre"),
                row.try_get("tipo_vehiculo").ok(),
                row.try_get("placa").ok(),
                row.get("is_blocked"),
            )
        })
        .collect();

    Ok(result)
}

/// Obtiene contratistas activos
pub async fn find_activos_with_empresa(
    pool: &SqlitePool,
) -> sqlx::Result<Vec<(Contratista, String, Option<String>, Option<String>, bool)>> {
    let rows = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre, v.tipo_vehiculo, v.placa,
           EXISTS (SELECT 1 FROM lista_negra ln WHERE ln.cedula = c.cedula AND ln.is_active = 1) as is_blocked
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           LEFT JOIN vehiculos v ON c.id = v.contratista_id
           WHERE c.estado = 'activo'
           ORDER BY c.nombre ASC"#,
    )
    .fetch_all(pool)
    .await?;

    let result: Vec<_> = rows
        .iter()
        .map(|row| {
            (
                row_to_contratista(row),
                row.get::<String, _>("empresa_nombre"),
                row.try_get("tipo_vehiculo").ok(),
                row.try_get("placa").ok(),
                row.get("is_blocked"),
            )
        })
        .collect();

    Ok(result)
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo contratista
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    cedula: &str,
    nombre: &str,
    segundo_nombre: Option<&str>,
    apellido: &str,
    segundo_apellido: Option<&str>,
    empresa_id: &str,
    fecha_vencimiento_praind: &str,
    estado: &str,
    created_at: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"INSERT INTO contratistas 
           (id, cedula, nombre, segundo_nombre, apellido, segundo_apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(id)
    .bind(cedula)
    .bind(nombre)
    .bind(segundo_nombre)
    .bind(apellido)
    .bind(segundo_apellido)
    .bind(empresa_id)
    .bind(fecha_vencimiento_praind)
    .bind(estado)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza un contratista
#[allow(clippy::too_many_arguments)]
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    nombre: Option<&str>,
    segundo_nombre: Option<&str>,
    apellido: Option<&str>,
    segundo_apellido: Option<&str>,
    empresa_id: Option<&str>,
    fecha_vencimiento_praind: Option<&str>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"UPDATE contratistas SET
            nombre = COALESCE(?, nombre),
            segundo_nombre = COALESCE(?, segundo_nombre),
            apellido = COALESCE(?, apellido),
            segundo_apellido = COALESCE(?, segundo_apellido),
            empresa_id = COALESCE(?, empresa_id),
            fecha_vencimiento_praind = COALESCE(?, fecha_vencimiento_praind),
            updated_at = ?
        WHERE id = ?"#,
    )
    .bind(nombre)
    .bind(segundo_nombre)
    .bind(apellido)
    .bind(segundo_apellido)
    .bind(empresa_id)
    .bind(fecha_vencimiento_praind)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza el estado de un contratista
pub async fn update_estado(
    pool: &SqlitePool,
    id: &str,
    estado: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE contratistas SET estado = ?, updated_at = ? WHERE id = ?")
        .bind(estado)
        .bind(updated_at)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Elimina un contratista
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM contratistas WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

fn row_to_contratista(row: &sqlx::sqlite::SqliteRow) -> Contratista {
    let estado_str: String = row.get("estado");
    let estado = crate::models::contratista::EstadoContratista::from_str(&estado_str)
        .unwrap_or(crate::models::contratista::EstadoContratista::Inactivo);

    Contratista {
        id: row.get("id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        segundo_nombre: row.get("segundo_nombre"),
        apellido: row.get("apellido"),
        segundo_apellido: row.get("segundo_apellido"),
        empresa_id: row.get("empresa_id"),
        fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
        estado,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
