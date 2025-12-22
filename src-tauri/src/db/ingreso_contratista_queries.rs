// src/db/ingreso_contratista_queries.rs
// Strict Mode: Uso de query! para validación en tiempo de compilación

use crate::models::ingreso::Ingreso;
use sqlx::SqlitePool;

// ==========================================
// TIPOS AUXILIARES
// ==========================================

pub struct IngresoDetails {
    pub usuario_ingreso_nombre: Option<String>,
    pub usuario_salida_nombre: Option<String>,
    pub vehiculo_placa: Option<String>,
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un ingreso por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<Ingreso>> {
    let row = sqlx::query!(
        r#"
        SELECT 
            id as "id!",
            contratista_id,
            cedula as "cedula!",
            nombre as "nombre!",
            apellido as "apellido!",
            empresa_nombre as "empresa_nombre!",
            tipo_ingreso as "tipo_ingreso!",
            tipo_autorizacion as "tipo_autorizacion!",
            modo_ingreso as "modo_ingreso!",
            vehiculo_id,
            placa_temporal,
            gafete_numero,
            fecha_hora_ingreso as "fecha_hora_ingreso!",
            fecha_hora_salida,
            tiempo_permanencia_minutos,
            usuario_ingreso_id as "usuario_ingreso_id!",
            usuario_salida_id,
            praind_vigente_al_ingreso as "praind_vigente_al_ingreso: bool",
            estado_contratista_al_ingreso,
            observaciones,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM ingresos 
        WHERE id = ? AND tipo_ingreso = 'contratista'
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Ingreso {
        id: r.id,
        contratista_id: Some(r.contratista_id),
        cedula: r.cedula,
        nombre: r.nombre,
        apellido: r.apellido,
        empresa_nombre: r.empresa_nombre,
        tipo_ingreso: r.tipo_ingreso,
        tipo_autorizacion: r.tipo_autorizacion,
        modo_ingreso: r.modo_ingreso,
        vehiculo_id: r.vehiculo_id,
        placa_temporal: r.placa_temporal,
        gafete_numero: r.gafete_numero,
        fecha_hora_ingreso: r.fecha_hora_ingreso,
        fecha_hora_salida: r.fecha_hora_salida,
        tiempo_permanencia_minutos: r.tiempo_permanencia_minutos,
        usuario_ingreso_id: r.usuario_ingreso_id,
        usuario_salida_id: r.usuario_salida_id,
        praind_vigente_al_ingreso: r.praind_vigente_al_ingreso,
        estado_contratista_al_ingreso: r.estado_contratista_al_ingreso,
        observaciones: r.observaciones,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

/// Busca detalles adicionales de un ingreso (nombres de usuarios, placa)
pub async fn find_details_by_id(
    pool: &SqlitePool,
    id: &str,
) -> sqlx::Result<Option<IngresoDetails>> {
    let row = sqlx::query!(
        r#"
        SELECT 
            u_ingreso.nombre as usuario_ingreso_nombre,
            u_salida.nombre as usuario_salida_nombre,
            v.placa as vehiculo_placa
        FROM ingresos i
        LEFT JOIN users u_ingreso ON i.usuario_ingreso_id = u_ingreso.id
        LEFT JOIN users u_salida ON i.usuario_salida_id = u_salida.id
        LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
        WHERE i.id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| IngresoDetails {
        usuario_ingreso_nombre: r.usuario_ingreso_nombre,
        usuario_salida_nombre: r.usuario_salida_nombre,
        vehiculo_placa: r.vehiculo_placa,
    }))
}

/// Busca ingreso abierto por contratista_id (Validación clave)
pub async fn find_ingreso_abierto_by_contratista(
    pool: &SqlitePool,
    contratista_id: &str,
) -> sqlx::Result<Option<Ingreso>> {
    let row = sqlx::query!(
        r#"
        SELECT 
            id as "id!",
            contratista_id,
            cedula as "cedula!",
            nombre as "nombre!",
            apellido as "apellido!",
            empresa_nombre as "empresa_nombre!",
            tipo_ingreso as "tipo_ingreso!",
            tipo_autorizacion as "tipo_autorizacion!",
            modo_ingreso as "modo_ingreso!",
            vehiculo_id,
            placa_temporal,
            gafete_numero,
            fecha_hora_ingreso as "fecha_hora_ingreso!",
            fecha_hora_salida,
            tiempo_permanencia_minutos,
            usuario_ingreso_id as "usuario_ingreso_id!",
            usuario_salida_id,
            praind_vigente_al_ingreso as "praind_vigente_al_ingreso: bool",
            estado_contratista_al_ingreso,
            observaciones,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM ingresos 
        WHERE contratista_id = ? 
          AND fecha_hora_salida IS NULL 
          AND tipo_ingreso = 'contratista'
        "#,
        contratista_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Ingreso {
        id: r.id,
        contratista_id: Some(r.contratista_id),
        cedula: r.cedula,
        nombre: r.nombre,
        apellido: r.apellido,
        empresa_nombre: r.empresa_nombre,
        tipo_ingreso: r.tipo_ingreso,
        tipo_autorizacion: r.tipo_autorizacion,
        modo_ingreso: r.modo_ingreso,
        vehiculo_id: r.vehiculo_id,
        placa_temporal: r.placa_temporal,
        gafete_numero: r.gafete_numero,
        fecha_hora_ingreso: r.fecha_hora_ingreso,
        fecha_hora_salida: r.fecha_hora_salida,
        tiempo_permanencia_minutos: r.tiempo_permanencia_minutos,
        usuario_ingreso_id: r.usuario_ingreso_id,
        usuario_salida_id: r.usuario_salida_id,
        praind_vigente_al_ingreso: r.praind_vigente_al_ingreso,
        estado_contratista_al_ingreso: r.estado_contratista_al_ingreso,
        observaciones: r.observaciones,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

/// Obtiene todos los ingresos de contratistas (limitado a 500)
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Ingreso>> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            id as "id!",
            contratista_id,
            cedula as "cedula!",
            nombre as "nombre!",
            apellido as "apellido!",
            empresa_nombre as "empresa_nombre!",
            tipo_ingreso as "tipo_ingreso!",
            tipo_autorizacion as "tipo_autorizacion!",
            modo_ingreso as "modo_ingreso!",
            vehiculo_id,
            placa_temporal,
            gafete_numero,
            fecha_hora_ingreso as "fecha_hora_ingreso!",
            fecha_hora_salida,
            tiempo_permanencia_minutos,
            usuario_ingreso_id as "usuario_ingreso_id!",
            usuario_salida_id,
            praind_vigente_al_ingreso as "praind_vigente_al_ingreso: bool",
            estado_contratista_al_ingreso,
            observaciones,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM ingresos 
        WHERE tipo_ingreso = 'contratista'
        ORDER BY fecha_hora_ingreso DESC 
        LIMIT 500
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Ingreso {
            id: r.id,
            contratista_id: Some(r.contratista_id),
            cedula: r.cedula,
            nombre: r.nombre,
            apellido: r.apellido,
            empresa_nombre: r.empresa_nombre,
            tipo_ingreso: r.tipo_ingreso,
            tipo_autorizacion: r.tipo_autorizacion,
            modo_ingreso: r.modo_ingreso,
            vehiculo_id: r.vehiculo_id,
            placa_temporal: r.placa_temporal,
            gafete_numero: r.gafete_numero,
            fecha_hora_ingreso: r.fecha_hora_ingreso,
            fecha_hora_salida: r.fecha_hora_salida,
            tiempo_permanencia_minutos: r.tiempo_permanencia_minutos,
            usuario_ingreso_id: r.usuario_ingreso_id,
            usuario_salida_id: r.usuario_salida_id,
            praind_vigente_al_ingreso: r.praind_vigente_al_ingreso,
            estado_contratista_al_ingreso: r.estado_contratista_al_ingreso,
            observaciones: r.observaciones,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect())
}

/// Obtiene solo ingresos abiertos (personas adentro)
pub async fn find_ingresos_abiertos(pool: &SqlitePool) -> sqlx::Result<Vec<Ingreso>> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            id as "id!",
            contratista_id,
            cedula as "cedula!",
            nombre as "nombre!",
            apellido as "apellido!",
            empresa_nombre as "empresa_nombre!",
            tipo_ingreso as "tipo_ingreso!",
            tipo_autorizacion as "tipo_autorizacion!",
            modo_ingreso as "modo_ingreso!",
            vehiculo_id,
            placa_temporal,
            gafete_numero,
            fecha_hora_ingreso as "fecha_hora_ingreso!",
            fecha_hora_salida,
            tiempo_permanencia_minutos,
            usuario_ingreso_id as "usuario_ingreso_id!",
            usuario_salida_id,
            praind_vigente_al_ingreso as "praind_vigente_al_ingreso: bool",
            estado_contratista_al_ingreso,
            observaciones,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM ingresos 
        WHERE fecha_hora_salida IS NULL 
          AND tipo_ingreso = 'contratista'
        ORDER BY fecha_hora_ingreso DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Ingreso {
            id: r.id,
            contratista_id: Some(r.contratista_id),
            cedula: r.cedula,
            nombre: r.nombre,
            apellido: r.apellido,
            empresa_nombre: r.empresa_nombre,
            tipo_ingreso: r.tipo_ingreso,
            tipo_autorizacion: r.tipo_autorizacion,
            modo_ingreso: r.modo_ingreso,
            vehiculo_id: r.vehiculo_id,
            placa_temporal: r.placa_temporal,
            gafete_numero: r.gafete_numero,
            fecha_hora_ingreso: r.fecha_hora_ingreso,
            fecha_hora_salida: r.fecha_hora_salida,
            tiempo_permanencia_minutos: r.tiempo_permanencia_minutos,
            usuario_ingreso_id: r.usuario_ingreso_id,
            usuario_salida_id: r.usuario_salida_id,
            praind_vigente_al_ingreso: r.praind_vigente_al_ingreso,
            estado_contratista_al_ingreso: r.estado_contratista_al_ingreso,
            observaciones: r.observaciones,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo ingreso de contratista
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    contratista_id: &str,
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
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO ingresos
        (id, contratista_id, cedula, nombre, apellido, empresa_nombre,
         tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
         gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
         usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso,
         estado_contratista_al_ingreso, observaciones, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, ?, NULL, ?, ?, ?, ?, ?)
        "#,
        id,
        contratista_id,
        cedula,
        nombre,
        apellido,
        empresa_nombre,
        tipo_ingreso,
        tipo_autorizacion,
        modo_ingreso,
        vehiculo_id,
        placa_temporal,
        gafete_numero,
        fecha_hora_ingreso,
        usuario_ingreso_id,
        praind_vigente_al_ingreso,
        estado_contratista_al_ingreso,
        observaciones,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Registra la salida
pub async fn registrar_salida(
    pool: &SqlitePool,
    id: &str,
    fecha_hora_salida: &str,
    tiempo_permanencia_minutos: i64,
    usuario_salida_id: &str,
    observaciones: Option<&str>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        UPDATE ingresos SET
            fecha_hora_salida = ?,
            tiempo_permanencia_minutos = ?,
            usuario_salida_id = ?,
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?
        "#,
        fecha_hora_salida,
        tiempo_permanencia_minutos,
        usuario_salida_id,
        observaciones,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
