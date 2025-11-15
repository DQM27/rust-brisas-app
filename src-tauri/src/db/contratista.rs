// db/contratista.rs
use sqlx::{SqlitePool, Transaction, Sqlite, Row};
use crate::models::contratista::*;
use crate::domain::errors::ContratistaError;

// ==========================================
// QUERIES DE LECTURA
// ==========================================
pub async fn existe_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<bool, ContratistaError> {
    let count = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM contratistas WHERE cedula = ?"
    )
    .bind(cedula)
    .fetch_one(pool)
    .await?;
    
    Ok(count > 0)
}

pub async fn find_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    fetch_one(pool, "c.id = ?", id).await
}

pub async fn find_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    fetch_one(pool, "c.cedula = ?", cedula).await
}

pub async fn find_all(
    pool: &SqlitePool,
) -> Result<Vec<ContratistaResponse>, ContratistaError> {
    let rows = sqlx::query(BASE_QUERY)
        .fetch_all(pool)
        .await?;
    
    Ok(rows.into_iter()
        .filter_map(|row| row_to_response(row).ok())
        .collect())
}

pub async fn find_activos(
    pool: &SqlitePool,
) -> Result<Vec<ContratistaResponse>, ContratistaError> {
    let query = format!("{} WHERE c.estado = ? ORDER BY c.apellido, c.nombre", BASE_QUERY);
    
    let rows = sqlx::query(&query)
        .bind(EstadoContratista::Activo.as_str())
        .fetch_all(pool)
        .await?;
    
    Ok(rows.into_iter()
        .filter_map(|row| row_to_response(row).ok())
        .collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================
pub async fn insertar(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    data: &CreateData,
) -> Result<(), ContratistaError> {
    sqlx::query(
        r#"INSERT INTO contratistas 
           (id, cedula, nombre, apellido, empresa_id, 
            fecha_vencimiento_praind, estado, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(id)
    .bind(&data.cedula)
    .bind(&data.nombre)
    .bind(&data.apellido)
    .bind(&data.empresa_id)
    .bind(&data.fecha_vencimiento_praind)
    .bind(EstadoContratista::Activo.as_str())
    .bind(&data.timestamp)
    .bind(&data.timestamp)
    .execute(&mut **tx)
    .await?;
    
    Ok(())
}

pub async fn actualizar(
    pool: &SqlitePool,
    id: &str,
    data: &UpdateData,
) -> Result<(), ContratistaError> {
    sqlx::query(
        r#"UPDATE contratistas SET
            nombre = COALESCE(?, nombre),
            apellido = COALESCE(?, apellido),
            empresa_id = COALESCE(?, empresa_id),
            fecha_vencimiento_praind = COALESCE(?, fecha_vencimiento_praind),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(&data.nombre)
    .bind(&data.apellido)
    .bind(&data.empresa_id)
    .bind(&data.fecha_vencimiento_praind)
    .bind(&data.timestamp)
    .bind(id)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn cambiar_estado(
    pool: &SqlitePool,
    id: &str,
    estado: EstadoContratista,
    timestamp: &str,
) -> Result<(), ContratistaError> {
    sqlx::query(
        "UPDATE contratistas SET estado = ?, updated_at = ? WHERE id = ?"
    )
    .bind(estado.as_str())
    .bind(timestamp)
    .bind(id)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn eliminar(
    pool: &SqlitePool,
    id: &str,
) -> Result<(), ContratistaError> {
    sqlx::query("DELETE FROM contratistas WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(())
}

// ==========================================
// HELPERS PRIVADOS
// ==========================================
const BASE_QUERY: &str = r#"
    SELECT 
        c.id, c.cedula, c.nombre, c.apellido, c.empresa_id,
        c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
        e.nombre as empresa_nombre
    FROM contratistas c
    INNER JOIN empresas e ON c.empresa_id = e.id
"#;

async fn fetch_one(
    pool: &SqlitePool,
    where_clause: &str,
    param: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    let query = format!("{} WHERE {}", BASE_QUERY, where_clause);
    
    let row = sqlx::query(&query)
        .bind(param)
        .fetch_one(pool)
        .await
        .map_err(|_| ContratistaError::NoEncontrado)?;
    
    row_to_response(row)
}

fn row_to_response(
    row: sqlx::sqlite::SqliteRow
) -> Result<ContratistaResponse, ContratistaError> {
    let contratista = Contratista {
        id: row.get("id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        empresa_id: row.get("empresa_id"),
        fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
        estado: EstadoContratista::from_str(row.get("estado"))
            .map_err(|e| ContratistaError::ParseError(e))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let empresa_nombre: String = row.get("empresa_nombre");
    
    Ok(ContratistaResponse::new(contratista, empresa_nombre))
}

// ==========================================
// DTOs INTERNOS
// ==========================================
pub struct CreateData {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_id: String,
    pub fecha_vencimiento_praind: String,
    pub timestamp: String,
}

pub struct UpdateData {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub empresa_id: Option<String>,
    pub fecha_vencimiento_praind: Option<String>,
    pub timestamp: String,
}