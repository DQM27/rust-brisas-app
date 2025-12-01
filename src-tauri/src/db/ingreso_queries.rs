// src/db/ingreso_queries.rs

use crate::models::ingreso::Ingreso;
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un ingreso por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Ingreso, String> {
    let row = sqlx::query(
        "SELECT id, contratista_id, cedula, nombre, apellido, empresa_nombre,
                tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
                gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
                usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
                estado_contratista_al_ingreso, observaciones, created_at, updated_at
         FROM ingresos WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| "Ingreso no encontrado".to_string())?;

    Ok(row_to_ingreso(&row)?)
}

pub struct IngresoDetails {
    pub usuario_ingreso_nombre: Option<String>,
    pub usuario_salida_nombre: Option<String>,
    pub vehiculo_placa: Option<String>,
}

/// Busca detalles adicionales de un ingreso (nombres de usuarios, placa)
pub async fn find_details_by_id(pool: &SqlitePool, id: &str) -> Result<IngresoDetails, String> {
    let row = sqlx::query(
        "SELECT 
            u_ingreso.nombre as usuario_ingreso_nombre,
            u_salida.nombre as usuario_salida_nombre,
            v.placa as vehiculo_placa
         FROM ingresos i
         LEFT JOIN users u_ingreso ON i.usuario_ingreso_id = u_ingreso.id
         LEFT JOIN users u_salida ON i.usuario_salida_id = u_salida.id
         LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
         WHERE i.id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error obteniendo detalles: {}", e))?;

    Ok(extract_details(&row))
}

/// Busca ingreso abierto por cédula
pub async fn find_ingreso_abierto_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Option<Ingreso>, String> {
    let row = sqlx::query(
        "SELECT id, contratista_id, cedula, nombre, apellido, empresa_nombre,
                tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
                gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
                usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
                estado_contratista_al_ingreso, observaciones, created_at, updated_at
         FROM ingresos WHERE cedula = ? AND fecha_hora_salida IS NULL",
    )
    .bind(cedula)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error al buscar ingreso abierto: {}", e))?;

    match row {
        Some(r) => Ok(Some(row_to_ingreso(&r)?)),
        None => Ok(None),
    }
}

/// Busca ingreso abierto por contratista_id
pub async fn find_ingreso_abierto_by_contratista(
    pool: &SqlitePool,
    contratista_id: &str,
) -> Result<Ingreso, String> {
    let row = sqlx::query(
        "SELECT id, contratista_id, cedula, nombre, apellido, empresa_nombre,
                tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
                gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
                usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
                estado_contratista_al_ingreso, observaciones, created_at, updated_at
         FROM ingresos WHERE contratista_id = ? AND fecha_hora_salida IS NULL",
    )
    .bind(contratista_id)
    .fetch_one(pool)
    .await
    .map_err(|_| "No se encontró ingreso abierto para este contratista".to_string())?;

    Ok(row_to_ingreso(&row)?)
}

/// Busca ingreso abierto por número de gafete
pub async fn find_ingreso_by_gafete(
    pool: &SqlitePool,
    gafete_numero: &str,
) -> Result<Ingreso, String> {
    let row = sqlx::query(
        "SELECT id, contratista_id, cedula, nombre, apellido, empresa_nombre,
                tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
                gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
                usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
                estado_contratista_al_ingreso, observaciones, created_at, updated_at
         FROM ingresos WHERE gafete_numero = ? AND fecha_hora_salida IS NULL",
    )
    .bind(gafete_numero)
    .fetch_one(pool)
    .await
    .map_err(|_| {
        format!(
            "No se encontró ingreso abierto con gafete {}",
            gafete_numero
        )
    })?;

    Ok(row_to_ingreso(&row)?)
}

/// Obtiene todos los ingresos (limitado a 500) - SIN DETALLES
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Ingreso>, String> {
    let rows = sqlx::query(
        "SELECT id, contratista_id, cedula, nombre, apellido, empresa_nombre,
                tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
                gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
                usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
                estado_contratista_al_ingreso, observaciones, created_at, updated_at
         FROM ingresos ORDER BY fecha_hora_ingreso DESC LIMIT 500",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener ingresos: {}", e))?;

    rows.iter().map(|row| row_to_ingreso(row)).collect()
}

/// Obtiene todos los ingresos con detalles (JOINs)
pub async fn find_all_with_details(
    pool: &SqlitePool,
) -> Result<Vec<(Ingreso, IngresoDetails)>, String> {
    let rows = sqlx::query(
        "SELECT i.*,
                u_ingreso.nombre as usuario_ingreso_nombre,
                u_salida.nombre as usuario_salida_nombre,
                v.placa as vehiculo_placa
         FROM ingresos i
         LEFT JOIN users u_ingreso ON i.usuario_ingreso_id = u_ingreso.id
         LEFT JOIN users u_salida ON i.usuario_salida_id = u_salida.id
         LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
         ORDER BY i.fecha_hora_ingreso DESC LIMIT 500",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener ingresos con detalles: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        let ingreso = row_to_ingreso(&row)?;
        let details = extract_details(&row);
        results.push((ingreso, details));
    }
    Ok(results)
}

/// Obtiene solo ingresos abiertos (personas adentro)
pub async fn find_ingresos_abiertos(pool: &SqlitePool) -> Result<Vec<Ingreso>, String> {
    let rows = sqlx::query(
        "SELECT id, contratista_id, cedula, nombre, apellido, empresa_nombre,
                tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
                gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
                usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
                estado_contratista_al_ingreso, observaciones, created_at, updated_at
         FROM ingresos WHERE fecha_hora_salida IS NULL ORDER BY fecha_hora_ingreso DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener ingresos abiertos: {}", e))?;

    rows.iter().map(|row| row_to_ingreso(row)).collect()
}

/// Obtiene ingresos abiertos con detalles (JOINs)
pub async fn find_ingresos_abiertos_with_details(
    pool: &SqlitePool,
) -> Result<Vec<(Ingreso, IngresoDetails)>, String> {
    let rows = sqlx::query(
        "SELECT i.*,
                u_ingreso.nombre as usuario_ingreso_nombre,
                u_salida.nombre as usuario_salida_nombre,
                v.placa as vehiculo_placa
         FROM ingresos i
         LEFT JOIN users u_ingreso ON i.usuario_ingreso_id = u_ingreso.id
         LEFT JOIN users u_salida ON i.usuario_salida_id = u_salida.id
         LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
         WHERE i.fecha_hora_salida IS NULL 
         ORDER BY i.fecha_hora_ingreso DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener ingresos abiertos con detalles: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        let ingreso = row_to_ingreso(&row)?;
        let details = extract_details(&row);
        results.push((ingreso, details));
    }
    Ok(results)
}

/// Cuenta ingresos abiertos de una cédula
pub async fn count_ingresos_abiertos_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<i32, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count FROM ingresos WHERE cedula = ? AND fecha_hora_salida IS NULL",
    )
    .bind(cedula)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al contar ingresos abiertos: {}", e))?;

    Ok(row.get("count"))
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo ingreso
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    contratista_id: Option<&str>,
    cedula: &str,
    nombre: &str,
    apellido: &str,
    empresa_nombre: &str,
    tipo_ingreso: &str,
    tipo_autorizacion: &str,
    modo_ingreso: &str,
    vehiculo_id: Option<&str>,
    placa_temporal: Option<&str>,
    gafete_numero: Option<&str>,
    fecha_hora_ingreso: &str,
    usuario_ingreso_id: &str,
    praind_vigente_al_ingreso: Option<bool>,
    estado_contratista_al_ingreso: Option<&str>,
    observaciones: Option<&str>,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    let praind_int = praind_vigente_al_ingreso.map(|b| if b { 1 } else { 0 });

    sqlx::query(
        r#"INSERT INTO ingresos 
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre,
            tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
            gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
            usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
            estado_contratista_al_ingreso, observaciones, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, ?, NULL, ?, ?, ?, ?, ?)"#,
    )
    .bind(id)
    .bind(contratista_id)
    .bind(cedula)
    .bind(nombre)
    .bind(apellido)
    .bind(empresa_nombre)
    .bind(tipo_ingreso)
    .bind(tipo_autorizacion)
    .bind(modo_ingreso)
    .bind(vehiculo_id)
    .bind(placa_temporal)
    .bind(gafete_numero)
    .bind(fecha_hora_ingreso)
    .bind(usuario_ingreso_id)
    .bind(praind_int)
    .bind(estado_contratista_al_ingreso)
    .bind(observaciones)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al crear ingreso: {}", e))?;

    Ok(())
}

/// Registra la salida de un ingreso
pub async fn registrar_salida(
    pool: &SqlitePool,
    id: &str,
    fecha_hora_salida: &str,
    tiempo_permanencia_minutos: i64,
    usuario_salida_id: &str,
    observaciones: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE ingresos SET
            fecha_hora_salida = ?,
            tiempo_permanencia_minutos = ?,
            usuario_salida_id = ?,
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?"#,
    )
    .bind(fecha_hora_salida)
    .bind(tiempo_permanencia_minutos)
    .bind(usuario_salida_id)
    .bind(observaciones)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al registrar salida: {}", e))?;

    Ok(())
}

// ==========================================
// HELPERS INTERNOS
// ==========================================

fn row_to_ingreso(row: &sqlx::sqlite::SqliteRow) -> Result<Ingreso, String> {
    let praind_int: Option<i32> = row.get("praind_vigente_al_ingreso");
    let praind_vigente_al_ingreso = praind_int.map(|v| v != 0);

    Ok(Ingreso {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        empresa_nombre: row.get("empresa_nombre"),
        tipo_ingreso: row.get("tipo_ingreso"),
        tipo_autorizacion: row.get("tipo_autorizacion"),
        modo_ingreso: row.get("modo_ingreso"),
        vehiculo_id: row.get("vehiculo_id"),
        placa_temporal: row.get("placa_temporal"),
        gafete_numero: row.get("gafete_numero"),
        fecha_hora_ingreso: row.get("fecha_hora_ingreso"),
        fecha_hora_salida: row.get("fecha_hora_salida"),
        tiempo_permanencia_minutos: row.get("tiempo_permanencia_minutos"),
        usuario_ingreso_id: row.get("usuario_ingreso_id"),
        usuario_salida_id: row.get("usuario_salida_id"),
        praind_vigente_al_ingreso,
        estado_contratista_al_ingreso: row.get("estado_contratista_al_ingreso"),
        observaciones: row.get("observaciones"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

fn extract_details(row: &sqlx::sqlite::SqliteRow) -> IngresoDetails {
    IngresoDetails {
        usuario_ingreso_nombre: row.try_get("usuario_ingreso_nombre").ok(),
        usuario_salida_nombre: row.try_get("usuario_salida_nombre").ok(),
        vehiculo_placa: row.try_get("vehiculo_placa").ok(),
    }
}
