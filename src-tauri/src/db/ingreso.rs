// ==========================================
// src/db/ingreso.rs
// ==========================================

use sqlx::{SqlitePool, Transaction, Sqlite, Row};
use crate::models::ingreso::*;
use crate::domain::errors::IngresoError;

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Verifica si una cédula tiene un ingreso abierto
pub async fn tiene_ingreso_abierto(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<bool, IngresoError> {
    let count = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM ingresos WHERE cedula = ? AND fecha_hora_salida IS NULL"
    )
    .bind(cedula)
    .fetch_one(pool)
    .await?;
    
    Ok(count > 0)
}

/// Busca un ingreso abierto por cédula
pub async fn find_abierto_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<IngresoResponse, IngresoError> {
    let query = format!("{} WHERE i.cedula = ? AND i.fecha_hora_salida IS NULL", BASE_QUERY);
    
    let row = sqlx::query(&query)
        .bind(cedula)
        .fetch_one(pool)
        .await
        .map_err(|_| IngresoError::NoEncontrado)?;
    
    row_to_response(row)
}

/// Busca un ingreso por ID
pub async fn find_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<IngresoResponse, IngresoError> {
    fetch_one(pool, "i.id = ?", id).await
}

/// Busca un ingreso abierto por número de gafete
pub async fn find_by_gafete(
    pool: &SqlitePool,
    gafete_numero: &str,
) -> Result<IngresoResponse, IngresoError> {
    let numero_normalizado = gafete_numero.trim().to_uppercase();
    
    let query = format!(
        "{} WHERE i.gafete_numero = ? AND i.fecha_hora_salida IS NULL",
        BASE_QUERY
    );
    
    let row = sqlx::query(&query)
        .bind(&numero_normalizado)
        .fetch_one(pool)
        .await
        .map_err(|_| IngresoError::NoEncontrado)?;
    
    row_to_response(row)
}

/// Obtiene todos los ingresos
pub async fn find_all(
    pool: &SqlitePool,
) -> Result<Vec<IngresoResponse>, IngresoError> {
    let query = format!(
        "{} ORDER BY i.fecha_hora_ingreso DESC LIMIT 500",
        BASE_QUERY
    );
    
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await?;
    
    Ok(rows.into_iter()
        .filter_map(|row| row_to_response(row).ok())
        .collect())
}

/// Obtiene solo ingresos abiertos (personas adentro)
pub async fn find_abiertos(
    pool: &SqlitePool,
) -> Result<Vec<IngresoResponse>, IngresoError> {
    let query = format!(
        "{} WHERE i.fecha_hora_salida IS NULL ORDER BY i.fecha_hora_ingreso DESC",
        BASE_QUERY
    );
    
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await?;
    
    Ok(rows.into_iter()
        .filter_map(|row| row_to_response(row).ok())
        .collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo ingreso
pub async fn insertar(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    data: &CreateIngresoData,
) -> Result<(), IngresoError> {
    sqlx::query(
        r#"INSERT INTO ingresos 
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre,
            tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
            gafete_id, gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
            usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso, estado_contratista_al_ingreso,
            observaciones, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, ?, NULL, ?, ?, ?, ?, ?)"#
    )
    .bind(id)
    .bind(&data.contratista_id)
    .bind(&data.cedula)
    .bind(&data.nombre)
    .bind(&data.apellido)
    .bind(&data.empresa_nombre)
    .bind(data.tipo_ingreso.as_str())
    .bind(data.tipo_autorizacion.as_str())
    .bind(data.modo_ingreso.as_str())
    .bind(&data.vehiculo_id)
    .bind(&data.placa_temporal)
    .bind(&data.gafete_id)
    .bind(&data.gafete_numero)
    .bind(&data.fecha_hora_ingreso)
    .bind(&data.usuario_ingreso_id)
    .bind(&data.praind_vigente_al_ingreso)
    .bind(&data.estado_contratista_al_ingreso)
    .bind(&data.observaciones)
    .bind(&data.timestamp)
    .bind(&data.timestamp)
    .execute(&mut **tx)
    .await?;
    
    Ok(())
}

/// Registra la salida de un ingreso
pub async fn registrar_salida(
    pool: &SqlitePool,
    id: &str,
    data: &RegistrarSalidaData,
) -> Result<(), IngresoError> {
    sqlx::query(
        r#"UPDATE ingresos SET
            fecha_hora_salida = ?,
            tiempo_permanencia_minutos = ?,
            usuario_salida_id = ?,
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(&data.fecha_hora_salida)
    .bind(data.tiempo_permanencia_minutos)
    .bind(&data.usuario_salida_id)
    .bind(&data.observaciones_salida)
    .bind(&data.timestamp)
    .bind(id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Cierra un ingreso automáticamente (por ingreso duplicado)
pub async fn cerrar_automatico(
    pool: &SqlitePool,
    cedula: &str,
    data: &CerrarAutomaticoData,
) -> Result<(), IngresoError> {
    sqlx::query(
        r#"UPDATE ingresos SET
            fecha_hora_salida = ?,
            tiempo_permanencia_minutos = ?,
            usuario_salida_id = ?,
            observaciones = 'Cerrado automáticamente por nuevo ingreso',
            updated_at = ?
        WHERE cedula = ? AND fecha_hora_salida IS NULL"#
    )
    .bind(&data.fecha_hora_salida)
    .bind(data.tiempo_permanencia_minutos)
    .bind(&data.usuario_salida_id)
    .bind(&data.timestamp)
    .bind(cedula)
    .execute(pool)
    .await?;
    
    Ok(())
}

// ==========================================
// HELPERS PRIVADOS
// ==========================================

const BASE_QUERY: &str = r#"
    SELECT 
        i.id, i.contratista_id, i.cedula, i.nombre, i.apellido, i.empresa_nombre,
        i.tipo_ingreso, i.tipo_autorizacion, i.modo_ingreso, i.vehiculo_id, i.placa_temporal,
        i.gafete_id, i.gafete_numero, i.fecha_hora_ingreso, i.fecha_hora_salida,
        i.tiempo_permanencia_minutos, i.usuario_ingreso_id, i.usuario_salida_id,
        i.praind_vigente_al_ingreso, i.estado_contratista_al_ingreso, i.observaciones,
        i.created_at, i.updated_at,
        u_ing.username as usuario_ingreso_nombre,
        u_sal.username as usuario_salida_nombre,
        v.placa as vehiculo_placa
    FROM ingresos i
    LEFT JOIN users u_ing ON i.usuario_ingreso_id = u_ing.id
    LEFT JOIN users u_sal ON i.usuario_salida_id = u_sal.id
    LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
"#;

async fn fetch_one(
    pool: &SqlitePool,
    where_clause: &str,
    param: &str,
) -> Result<IngresoResponse, IngresoError> {
    let query = format!("{} WHERE {}", BASE_QUERY, where_clause);
    
    let row = sqlx::query(&query)
        .bind(param)
        .fetch_one(pool)
        .await
        .map_err(|_| IngresoError::NoEncontrado)?;
    
    row_to_response(row)
}

fn row_to_response(
    row: sqlx::sqlite::SqliteRow
) -> Result<IngresoResponse, IngresoError> {
    let ingreso = Ingreso {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        empresa_nombre: row.get("empresa_nombre"),
        tipo_ingreso: TipoIngreso::from_str(row.get("tipo_ingreso"))
            .map_err(IngresoError::ParseError)?,
        tipo_autorizacion: TipoAutorizacion::from_str(row.get("tipo_autorizacion"))
            .map_err(IngresoError::ParseError)?,
        modo_ingreso: ModoIngreso::from_str(row.get("modo_ingreso"))
            .map_err(IngresoError::ParseError)?,
        vehiculo_id: row.get("vehiculo_id"),
        placa_temporal: row.get("placa_temporal"),
        gafete_id: row.get("gafete_id"),
        gafete_numero: row.get("gafete_numero"),
        fecha_hora_ingreso: row.get("fecha_hora_ingreso"),
        fecha_hora_salida: row.get("fecha_hora_salida"),
        tiempo_permanencia_minutos: row.get("tiempo_permanencia_minutos"),
        usuario_ingreso_id: row.get("usuario_ingreso_id"),
        usuario_salida_id: row.get("usuario_salida_id"),
        praind_vigente_al_ingreso: row.get("praind_vigente_al_ingreso"),
        estado_contratista_al_ingreso: row.get("estado_contratista_al_ingreso"),
        observaciones: row.get("observaciones"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let usuario_ingreso_nombre: Option<String> = row.get("usuario_ingreso_nombre");
    let usuario_salida_nombre: Option<String> = row.get("usuario_salida_nombre");
    let vehiculo_placa: Option<String> = row.get("vehiculo_placa");
    
    Ok(IngresoResponse::new(
        ingreso,
        usuario_ingreso_nombre,
        usuario_salida_nombre,
        vehiculo_placa,
    ))
}

// ==========================================
// DTOs INTERNOS
// ==========================================

pub struct CreateIngresoData {
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: TipoIngreso,
    pub tipo_autorizacion: TipoAutorizacion,
    pub modo_ingreso: ModoIngreso,
    pub vehiculo_id: Option<String>,
    pub placa_temporal: Option<String>,
    pub gafete_id: String,
    pub gafete_numero: String,
    pub fecha_hora_ingreso: String,
    pub usuario_ingreso_id: String,
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
    pub timestamp: String,
}

pub struct RegistrarSalidaData {
    pub fecha_hora_salida: String,
    pub tiempo_permanencia_minutos: i64,
    pub usuario_salida_id: String,
    pub observaciones_salida: Option<String>,
    pub timestamp: String,
}

pub struct CerrarAutomaticoData {
    pub fecha_hora_salida: String,
    pub tiempo_permanencia_minutos: i64,
    pub usuario_salida_id: String,
    pub timestamp: String,
}