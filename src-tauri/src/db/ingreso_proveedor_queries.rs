// ==========================================
// src/db/ingreso_proveedor_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query_as! para validación y DTO intermedio

use crate::domain::ingreso_proveedor::{
    CreateIngresoProveedorInput, IngresoProveedor, ProveedorSnapshot,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// DTO & CONVERSION
// ==========================================

#[derive(sqlx::FromRow)]
struct IngresoProveedorRow {
    id: Option<String>,
    cedula: Option<String>,
    nombre: Option<String>,
    apellido: Option<String>,
    proveedor_id: Option<String>,
    empresa_id: Option<String>,
    area_visitada: Option<String>,
    motivo: Option<String>,
    gafete: Option<String>,
    tipo_autorizacion: Option<String>,
    modo_ingreso: Option<String>,
    placa_vehiculo: Option<String>,
    fecha_ingreso: Option<String>,
    fecha_salida: Option<String>,
    estado: Option<String>,
    usuario_ingreso_id: Option<String>,
    usuario_salida_id: Option<String>,
    observaciones: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    // Calculated fields
    usuario_ingreso_nombre: Option<String>,
    usuario_salida_nombre: Option<String>,
    empresa_nombre: Option<String>,
}

impl From<IngresoProveedorRow> for IngresoProveedor {
    fn from(row: IngresoProveedorRow) -> Self {
        IngresoProveedor {
            id: row.id.unwrap_or_default(),
            cedula: row.cedula.unwrap_or_default(),
            nombre: row.nombre.unwrap_or_default(),
            apellido: row.apellido.unwrap_or_default(),
            proveedor_id: row.proveedor_id,
            empresa_id: row.empresa_id.unwrap_or_default(),
            area_visitada: row.area_visitada.unwrap_or_default(),
            motivo: row.motivo.unwrap_or_default(),
            gafete: row.gafete,
            tipo_autorizacion: row.tipo_autorizacion,
            modo_ingreso: row.modo_ingreso,
            placa_vehiculo: row.placa_vehiculo,
            fecha_ingreso: row.fecha_ingreso.unwrap_or_default(),
            fecha_salida: row.fecha_salida,
            estado: row.estado.unwrap_or_default(),
            usuario_ingreso_id: row.usuario_ingreso_id.unwrap_or_default(),
            usuario_salida_id: row.usuario_salida_id,
            observaciones: row.observaciones,
            created_at: row.created_at.unwrap_or_default(),
            updated_at: row.updated_at.unwrap_or_default(),
            usuario_ingreso_nombre: row
                .usuario_ingreso_nombre
                .unwrap_or_else(|| "N/A".to_string()),
            usuario_salida_nombre: row
                .usuario_salida_nombre
                .unwrap_or_else(|| "N/A".to_string()),
            empresa_nombre: row
                .empresa_nombre
                .unwrap_or_else(|| "Sin empresa".to_string()),
        }
    }
}

// ==========================================
// QUERIES
// ==========================================

pub async fn create(
    pool: &SqlitePool,
    input: CreateIngresoProveedorInput,
    proveedor_id: &str,
) -> sqlx::Result<IngresoProveedor> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = "ADENTRO";

    sqlx::query!(
        r#"
        INSERT INTO ingresos_proveedores (
            id, cedula, nombre, apellido, empresa_id, area_visitada, motivo, gafete,
            tipo_autorizacion, modo_ingreso, placa_vehiculo,
            fecha_ingreso, estado, usuario_ingreso_id, observaciones, created_at, updated_at, proveedor_id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        input.cedula,
        input.nombre,
        input.apellido,
        input.empresa_id,
        input.area_visitada,
        input.motivo,
        input.gafete,
        input.tipo_autorizacion,
        input.modo_ingreso,
        input.placa_vehiculo,
        now,
        estado,
        input.usuario_ingreso_id,
        input.observaciones,
        now,
        now,
        proveedor_id
    )
    .execute(pool)
    .await?;

    Ok(IngresoProveedor {
        id,
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        proveedor_id: Some(proveedor_id.to_string()),
        empresa_id: input.empresa_id,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        gafete: input.gafete,
        tipo_autorizacion: Some(input.tipo_autorizacion),
        modo_ingreso: Some(input.modo_ingreso),
        placa_vehiculo: input.placa_vehiculo,
        fecha_ingreso: now.clone(),
        fecha_salida: None,
        estado: estado.to_string(),
        usuario_ingreso_id: input.usuario_ingreso_id,
        usuario_salida_id: None,
        observaciones: input.observaciones,
        created_at: now.clone(),
        updated_at: now,
        usuario_ingreso_nombre: String::new(),
        usuario_salida_nombre: String::new(),
        empresa_nombre: String::new(),
    })
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<IngresoProveedor>> {
    let row = sqlx::query_as!(
        IngresoProveedorRow,
        r#"
        SELECT 
            ip.id, ip.cedula, ip.nombre, ip.apellido, ip.proveedor_id, ip.empresa_id, ip.area_visitada, ip.motivo, ip.gafete, 
            ip.tipo_autorizacion, ip.modo_ingreso, ip.placa_vehiculo, 
            CAST(ip.fecha_ingreso AS TEXT) as fecha_ingreso,
            CAST(ip.fecha_salida AS TEXT) as fecha_salida,
            ip.estado, 
            ip.usuario_ingreso_id, ip.usuario_salida_id, ip.observaciones, 
            CAST(ip.created_at AS TEXT) as created_at,
            CAST(ip.updated_at AS TEXT) as updated_at,
            COALESCE(u.nombre || ' ' || u.apellido, 'N/A') as usuario_ingreso_nombre,
            CAST(NULL AS TEXT) as usuario_salida_nombre,
            COALESCE(e.nombre, 'Sin empresa') as empresa_nombre
        FROM ingresos_proveedores ip
        LEFT JOIN users u ON ip.usuario_ingreso_id = u.id
        LEFT JOIN empresas e ON ip.empresa_id = e.id
        WHERE ip.id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.into()))
}

pub async fn find_actives(pool: &SqlitePool) -> sqlx::Result<Vec<IngresoProveedor>> {
    let rows = sqlx::query_as!(
        IngresoProveedorRow,
        r#"
        SELECT 
            ip.id, ip.cedula, ip.nombre, ip.apellido, ip.proveedor_id, ip.empresa_id, ip.area_visitada, ip.motivo, ip.gafete,
            ip.tipo_autorizacion, ip.modo_ingreso, ip.placa_vehiculo, 
            CAST(ip.fecha_ingreso AS TEXT) as fecha_ingreso,
            CAST(ip.fecha_salida AS TEXT) as fecha_salida,
            ip.estado,
            ip.usuario_ingreso_id, ip.usuario_salida_id, ip.observaciones, 
            CAST(ip.created_at AS TEXT) as created_at,
            CAST(ip.updated_at AS TEXT) as updated_at,
            COALESCE(u.nombre || ' ' || u.apellido, 'N/A') as usuario_ingreso_nombre,
            CAST(NULL AS TEXT) as usuario_salida_nombre,
            COALESCE(e.nombre, 'Sin empresa') as empresa_nombre
        FROM ingresos_proveedores ip
        LEFT JOIN users u ON ip.usuario_ingreso_id = u.id
        LEFT JOIN empresas e ON ip.empresa_id = e.id
        WHERE ip.estado = 'ADENTRO' 
        ORDER BY ip.fecha_ingreso DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

pub async fn find_history(pool: &SqlitePool) -> sqlx::Result<Vec<IngresoProveedor>> {
    let rows = sqlx::query_as!(
        IngresoProveedorRow,
        r#"
        SELECT 
            ip.id, ip.cedula, ip.nombre, ip.apellido, ip.proveedor_id, ip.empresa_id, ip.area_visitada, ip.motivo, ip.gafete,
            ip.tipo_autorizacion, ip.modo_ingreso, ip.placa_vehiculo, 
            CAST(ip.fecha_ingreso AS TEXT) as fecha_ingreso,
            CAST(ip.fecha_salida AS TEXT) as fecha_salida,
            ip.estado,
            ip.usuario_ingreso_id, ip.usuario_salida_id, ip.observaciones, 
            CAST(ip.created_at AS TEXT) as created_at,
            CAST(ip.updated_at AS TEXT) as updated_at,
            COALESCE(u_in.nombre || ' ' || u_in.apellido, 'N/A') as usuario_ingreso_nombre,
            COALESCE(u_out.nombre || ' ' || u_out.apellido, 'N/A') as usuario_salida_nombre,
            COALESCE(e.nombre, 'Sin empresa') as empresa_nombre
        FROM ingresos_proveedores ip
        LEFT JOIN users u_in ON ip.usuario_ingreso_id = u_in.id
        LEFT JOIN users u_out ON ip.usuario_salida_id = u_out.id
        LEFT JOIN empresas e ON ip.empresa_id = e.id
        WHERE ip.estado = 'SALIO' 
        ORDER BY ip.fecha_salida DESC
        LIMIT 100
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

pub async fn registrar_salida(
    pool: &SqlitePool,
    id: &str,
    usuario_salida_id: &str,
    observaciones: Option<&str>,
) -> sqlx::Result<()> {
    let now = Utc::now().to_rfc3339();

    sqlx::query!(
        r#"
        UPDATE ingresos_proveedores 
        SET estado = 'SALIO', 
            fecha_salida = ?, 
            usuario_salida_id = ?, 
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?
        "#,
        now,
        usuario_salida_id,
        observaciones,
        now,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn search_distinct_proveedores(
    pool: &SqlitePool,
    query: &str,
) -> sqlx::Result<Vec<ProveedorSnapshot>> {
    let pattern = format!("%{}%", query);
    sqlx::query_as!(
        ProveedorSnapshot,
        r#"
        SELECT DISTINCT ip.cedula, ip.nombre, ip.apellido, ip.empresa_id, 
        COALESCE(e.nombre, '') as empresa_nombre 
        FROM ingresos_proveedores ip
        LEFT JOIN empresas e ON ip.empresa_id = e.id
        WHERE ip.cedula LIKE ? OR ip.nombre LIKE ? OR ip.apellido LIKE ?
        LIMIT 20
        "#,
        pattern,
        pattern,
        pattern
    )
    .fetch_all(pool)
    .await
}

pub async fn find_open_by_proveedor(
    pool: &SqlitePool,
    proveedor_id: &str,
) -> sqlx::Result<Option<IngresoProveedor>> {
    let row = sqlx::query_as!(
        IngresoProveedorRow,
        r#"
        SELECT 
             ip.id, ip.cedula, ip.nombre, ip.apellido, ip.proveedor_id, ip.empresa_id, ip.area_visitada, ip.motivo, ip.gafete,
            ip.tipo_autorizacion, ip.modo_ingreso, ip.placa_vehiculo, 
            CAST(ip.fecha_ingreso AS TEXT) as fecha_ingreso,
            CAST(ip.fecha_salida AS TEXT) as fecha_salida,
            ip.estado,
            ip.usuario_ingreso_id, ip.usuario_salida_id, ip.observaciones, 
            CAST(ip.created_at AS TEXT) as created_at,
            CAST(ip.updated_at AS TEXT) as updated_at,
            CAST(NULL AS TEXT) as usuario_ingreso_nombre,
            CAST(NULL AS TEXT) as usuario_salida_nombre,
            CAST(NULL AS TEXT) as empresa_nombre
        FROM ingresos_proveedores ip
        WHERE proveedor_id = ? AND estado = 'ADENTRO' 
        LIMIT 1
        "#,
        proveedor_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.into()))
}
