// src/db/ingreso_general_queries.rs
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
// QUERIES GENERALES (SIN FILTRO DE TIPO)
// ==========================================

/// Busca un ingreso por ID (Cualquier tipo)
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
        WHERE id = ?
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

/// Busca detalles adicionales (Cualquier tipo)
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

/// Obtiene TODOS los ingresos (Contratistas, Visitas, Proveedores)
/// Útil para la bitácora general / historial completo
pub async fn find_all_with_details(
    pool: &SqlitePool,
) -> sqlx::Result<Vec<(Ingreso, IngresoDetails)>> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            i.id as "id!",
            i.contratista_id,
            i.cedula as "cedula!",
            i.nombre as "nombre!",
            i.apellido as "apellido!",
            i.empresa_nombre as "empresa_nombre!",
            i.tipo_ingreso as "tipo_ingreso!",
            i.tipo_autorizacion as "tipo_autorizacion!",
            i.modo_ingreso as "modo_ingreso!",
            i.vehiculo_id,
            i.placa_temporal,
            i.gafete_numero,
            i.fecha_hora_ingreso as "fecha_hora_ingreso!",
            i.fecha_hora_salida,
            i.tiempo_permanencia_minutos,
            i.usuario_ingreso_id as "usuario_ingreso_id!",
            i.usuario_salida_id,
            i.praind_vigente_al_ingreso as "praind_vigente_al_ingreso: bool",
            i.estado_contratista_al_ingreso,
            i.observaciones,
            i.created_at as "created_at!",
            i.updated_at as "updated_at!",
            u_ingreso.nombre as usuario_ingreso_nombre,
            u_salida.nombre as usuario_salida_nombre,
            v.placa as vehiculo_placa
        FROM ingresos i
        LEFT JOIN users u_ingreso ON i.usuario_ingreso_id = u_ingreso.id
        LEFT JOIN users u_salida ON i.usuario_salida_id = u_salida.id
        LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
        ORDER BY i.fecha_hora_ingreso DESC 
        LIMIT 500
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| {
            (
                Ingreso {
                    id: r.id,
                    contratista_id: Some(r.contratista_id),
                    cedula: r.cedula,
                    nombre: r.nombre.clone(),
                    apellido: r.apellido.clone(),
                    empresa_nombre: r.empresa_nombre,
                    tipo_ingreso: r.tipo_ingreso,
                    tipo_autorizacion: r.tipo_autorizacion,
                    modo_ingreso: r.modo_ingreso,
                    vehiculo_id: r.vehiculo_id.clone(),
                    placa_temporal: r.placa_temporal,
                    gafete_numero: r.gafete_numero,
                    fecha_hora_ingreso: r.fecha_hora_ingreso,
                    fecha_hora_salida: r.fecha_hora_salida,
                    tiempo_permanencia_minutos: r.tiempo_permanencia_minutos,
                    usuario_ingreso_id: r.usuario_ingreso_id.clone(),
                    usuario_salida_id: r.usuario_salida_id.clone(),
                    praind_vigente_al_ingreso: r.praind_vigente_al_ingreso,
                    estado_contratista_al_ingreso: r.estado_contratista_al_ingreso,
                    observaciones: r.observaciones,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                IngresoDetails {
                    usuario_ingreso_nombre: r.usuario_ingreso_nombre,
                    usuario_salida_nombre: r.usuario_salida_nombre,
                    vehiculo_placa: r.vehiculo_placa,
                },
            )
        })
        .collect())
}

/// Obtiene TODOS los ingresos abiertos (Cualquiera adentro)
pub async fn find_ingresos_abiertos_with_details(
    pool: &SqlitePool,
) -> sqlx::Result<Vec<(Ingreso, IngresoDetails)>> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            i.id as "id!",
            i.contratista_id,
            i.cedula as "cedula!",
            i.nombre as "nombre!",
            i.apellido as "apellido!",
            i.empresa_nombre as "empresa_nombre!",
            i.tipo_ingreso as "tipo_ingreso!",
            i.tipo_autorizacion as "tipo_autorizacion!",
            i.modo_ingreso as "modo_ingreso!",
            i.vehiculo_id,
            i.placa_temporal,
            i.gafete_numero,
            i.fecha_hora_ingreso as "fecha_hora_ingreso!",
            i.fecha_hora_salida,
            i.tiempo_permanencia_minutos,
            i.usuario_ingreso_id as "usuario_ingreso_id!",
            i.usuario_salida_id,
            i.praind_vigente_al_ingreso as "praind_vigente_al_ingreso: bool",
            i.estado_contratista_al_ingreso,
            i.observaciones,
            i.created_at as "created_at!",
            i.updated_at as "updated_at!",
            u_ingreso.nombre as usuario_ingreso_nombre,
            u_salida.nombre as usuario_salida_nombre,
            v.placa as vehiculo_placa
        FROM ingresos i
        LEFT JOIN users u_ingreso ON i.usuario_ingreso_id = u_ingreso.id
        LEFT JOIN users u_salida ON i.usuario_salida_id = u_salida.id
        LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
        WHERE i.fecha_hora_salida IS NULL 
        ORDER BY i.fecha_hora_ingreso DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| {
            (
                Ingreso {
                    id: r.id,
                    contratista_id: Some(r.contratista_id),
                    cedula: r.cedula,
                    nombre: r.nombre.clone(),
                    apellido: r.apellido.clone(),
                    empresa_nombre: r.empresa_nombre,
                    tipo_ingreso: r.tipo_ingreso,
                    tipo_autorizacion: r.tipo_autorizacion,
                    modo_ingreso: r.modo_ingreso,
                    vehiculo_id: r.vehiculo_id.clone(),
                    placa_temporal: r.placa_temporal,
                    gafete_numero: r.gafete_numero,
                    fecha_hora_ingreso: r.fecha_hora_ingreso,
                    fecha_hora_salida: r.fecha_hora_salida,
                    tiempo_permanencia_minutos: r.tiempo_permanencia_minutos,
                    usuario_ingreso_id: r.usuario_ingreso_id.clone(),
                    usuario_salida_id: r.usuario_salida_id.clone(),
                    praind_vigente_al_ingreso: r.praind_vigente_al_ingreso,
                    estado_contratista_al_ingreso: r.estado_contratista_al_ingreso,
                    observaciones: r.observaciones,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                IngresoDetails {
                    usuario_ingreso_nombre: r.usuario_ingreso_nombre,
                    usuario_salida_nombre: r.usuario_salida_nombre,
                    vehiculo_placa: r.vehiculo_placa,
                },
            )
        })
        .collect())
}

/// Busca ingreso por gafete (Genérico, porque un gafete es único en el sistema)
pub async fn find_ingreso_by_gafete(
    pool: &SqlitePool,
    gafete_numero: &str,
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
        WHERE gafete_numero = ? AND fecha_hora_salida IS NULL
        "#,
        gafete_numero
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

/// Busca SALIDAS en un rango de fechas (Cualquier tipo)
pub async fn find_salidas_in_range_with_details(
    pool: &SqlitePool,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> sqlx::Result<Vec<(Ingreso, IngresoDetails)>> {
    // Añadimos hora inicio y fin para cubrir todo el día
    let start = format!("{}T00:00:00", fecha_inicio);
    let end = format!("{}T23:59:59", fecha_fin);

    let rows = sqlx::query!(
        r#"
        SELECT 
            i.id as "id!",
            i.contratista_id,
            i.cedula as "cedula!",
            i.nombre as "nombre!",
            i.apellido as "apellido!",
            i.empresa_nombre as "empresa_nombre!",
            i.tipo_ingreso as "tipo_ingreso!",
            i.tipo_autorizacion as "tipo_autorizacion!",
            i.modo_ingreso as "modo_ingreso!",
            i.vehiculo_id,
            i.placa_temporal,
            i.gafete_numero,
            i.fecha_hora_ingreso as "fecha_hora_ingreso!",
            i.fecha_hora_salida,
            i.tiempo_permanencia_minutos,
            i.usuario_ingreso_id as "usuario_ingreso_id!",
            i.usuario_salida_id,
            i.praind_vigente_al_ingreso as "praind_vigente_al_ingreso: bool",
            i.estado_contratista_al_ingreso,
            i.observaciones,
            i.created_at as "created_at!",
            i.updated_at as "updated_at!",
            u_ingreso.nombre as usuario_ingreso_nombre,
            u_salida.nombre as usuario_salida_nombre,
            v.placa as vehiculo_placa
        FROM ingresos i
        LEFT JOIN users u_ingreso ON i.usuario_ingreso_id = u_ingreso.id
        LEFT JOIN users u_salida ON i.usuario_salida_id = u_salida.id
        LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
        WHERE i.fecha_hora_salida >= ? AND i.fecha_hora_salida <= ?
        ORDER BY i.fecha_hora_salida DESC
        "#,
        start,
        end
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| {
            (
                Ingreso {
                    id: r.id,
                    contratista_id: Some(r.contratista_id),
                    cedula: r.cedula,
                    nombre: r.nombre.clone(),
                    apellido: r.apellido.clone(),
                    empresa_nombre: r.empresa_nombre,
                    tipo_ingreso: r.tipo_ingreso,
                    tipo_autorizacion: r.tipo_autorizacion,
                    modo_ingreso: r.modo_ingreso,
                    vehiculo_id: r.vehiculo_id.clone(),
                    placa_temporal: r.placa_temporal,
                    gafete_numero: r.gafete_numero,
                    fecha_hora_ingreso: r.fecha_hora_ingreso,
                    fecha_hora_salida: r.fecha_hora_salida,
                    tiempo_permanencia_minutos: r.tiempo_permanencia_minutos,
                    usuario_ingreso_id: r.usuario_ingreso_id.clone(),
                    usuario_salida_id: r.usuario_salida_id.clone(),
                    praind_vigente_al_ingreso: r.praind_vigente_al_ingreso,
                    estado_contratista_al_ingreso: r.estado_contratista_al_ingreso,
                    observaciones: r.observaciones,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                IngresoDetails {
                    usuario_ingreso_nombre: r.usuario_ingreso_nombre,
                    usuario_salida_nombre: r.usuario_salida_nombre,
                    vehiculo_placa: r.vehiculo_placa,
                },
            )
        })
        .collect())
}

/// Busca SALIDAS de un día específico
pub async fn find_salidas_by_date_with_details(
    pool: &SqlitePool,
    fecha: &str,
) -> sqlx::Result<Vec<(Ingreso, IngresoDetails)>> {
    find_salidas_in_range_with_details(pool, fecha, fecha).await
}
