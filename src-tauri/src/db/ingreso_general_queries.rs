// src/db/ingreso_general_queries.rs

use crate::models::ingreso::Ingreso;
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES GENERALES (SIN FILTRO DE TIPO)
// ==========================================

/// Busca un ingreso por ID (Cualquier tipo)
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

/// Busca detalles adicionales (Cualquier tipo)
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

/// Obtiene TODOS los ingresos (Contratistas, Visitas, Proveedores)
/// Útil para la bitácora general / historial completo
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
    .map_err(|e| format!("Error al obtener historial general: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        let ingreso = row_to_ingreso(&row)?;
        let details = extract_details(&row);
        results.push((ingreso, details));
    }
    Ok(results)
}

/// Obtiene TODOS los ingresos abiertos (Cualquiera adentro)
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
    .map_err(|e| format!("Error al obtener ingresos abiertos generales: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        let ingreso = row_to_ingreso(&row)?;
        let details = extract_details(&row);
        results.push((ingreso, details));
    }
    Ok(results)
}

/// Busca ingreso por gafete (Genérico, porque un gafete es único en el sistema)
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
    .map_err(|_| format!("No se encontró ingreso activo con gafete {}", gafete_numero))?;

    Ok(row_to_ingreso(&row)?)
}

// ==========================================
// HELPERS
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
