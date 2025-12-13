use crate::domain::ingreso_proveedor::{CreateIngresoProveedorInput, IngresoProveedor};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create(
    pool: &SqlitePool,
    input: CreateIngresoProveedorInput,
    proveedor_id: &str,
) -> Result<IngresoProveedor, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = "ADENTRO";

    sqlx::query(
        r#"
        INSERT INTO ingresos_proveedores (
            id, cedula, nombre, apellido, empresa_id, area_visitada, motivo, gafete,
            tipo_autorizacion, modo_ingreso, placa_vehiculo,
            fecha_ingreso, estado, usuario_ingreso_id, observaciones, created_at, updated_at, proveedor_id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.cedula)
    .bind(&input.nombre)
    .bind(&input.apellido)
    .bind(&input.empresa_id)
    .bind(&input.area_visitada)
    .bind(&input.motivo)
    .bind(&input.gafete)
    .bind(&input.tipo_autorizacion)
    .bind(&input.modo_ingreso)
    .bind(&input.placa_vehiculo)
    .bind(&now)
    .bind(estado)
    .bind(&input.usuario_ingreso_id)
    .bind(&input.observaciones)
    .bind(&now)
    .bind(&now)
    .bind(proveedor_id)
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
    })
}

pub async fn find_actives(pool: &SqlitePool) -> Result<Vec<IngresoProveedor>, sqlx::Error> {
    sqlx::query_as::<_, IngresoProveedor>(
        "SELECT * FROM ingresos_proveedores WHERE estado = 'ADENTRO' ORDER BY fecha_ingreso DESC",
    )
    .fetch_all(pool)
    .await
}

pub async fn registrar_salida(
    pool: &SqlitePool,
    id: &str,
    usuario_salida_id: &str,
    observaciones: Option<&str>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        UPDATE ingresos_proveedores 
        SET estado = 'SALIO', 
            fecha_salida = ?, 
            usuario_salida_id = ?, 
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&now)
    .bind(usuario_salida_id)
    .bind(observaciones)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

use crate::domain::ingreso_proveedor::ProveedorSnapshot;

pub async fn search_distinct_proveedores(
    pool: &SqlitePool,
    query: &str,
) -> Result<Vec<ProveedorSnapshot>, sqlx::Error> {
    let pattern = format!("%{}%", query);
    sqlx::query_as::<_, ProveedorSnapshot>(
        r#"
        SELECT DISTINCT ip.cedula, ip.nombre, ip.apellido, ip.empresa_id, e.nombre as empresa_nombre
        FROM ingresos_proveedores ip
        LEFT JOIN empresas e ON ip.empresa_id = e.id
        WHERE ip.cedula LIKE ? OR ip.nombre LIKE ? OR ip.apellido LIKE ?
        LIMIT 20
        "#,
    )
    .bind(&pattern)
    .bind(&pattern)
    .bind(&pattern)
    .fetch_all(pool)
    .await
}

pub async fn find_open_by_proveedor(
    pool: &SqlitePool,
    proveedor_id: &str,
) -> Result<Option<IngresoProveedor>, sqlx::Error> {
    sqlx::query_as::<_, IngresoProveedor>(
        "SELECT * FROM ingresos_proveedores WHERE proveedor_id = ? AND estado = 'ADENTRO' LIMIT 1",
    )
    .bind(proveedor_id)
    .fetch_optional(pool)
    .await
}
