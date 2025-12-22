// src/db/cita_queries.rs
// Strict Mode: Uso de query! para validación en tiempo de compilación

use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_cita(pool: &SqlitePool, input: CreateCitaInput) -> Result<Cita, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = "PENDIENTE";

    sqlx::query!(
        r#"
        INSERT INTO citas (id, visitante_id, fecha_cita, anfitrion, area_visitada, motivo, estado, registrado_por, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        input.visitante_id,
        input.fecha_cita,
        input.anfitrion,
        input.area_visitada,
        input.motivo,
        estado,
        input.registrado_por,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(Cita {
        id,
        visitante_id: input.visitante_id,
        fecha_cita: input.fecha_cita,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        estado: estado.to_string(),
        registrado_por: input.registrado_por,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_citas_pendientes_del_dia(
    pool: &SqlitePool,
    fecha: &str,
) -> Result<Vec<CitaPopulated>, sqlx::Error> {
    let fecha_inicio = format!("{}T00:00:00", fecha);
    let fecha_fin = format!("{}T23:59:59", fecha);

    let rows = sqlx::query!(
        r#"
        SELECT 
            c.id as "id!",
            c.fecha_cita as "fecha_cita!",
            c.anfitrion as "anfitrion!",
            c.area_visitada as "area_visitada!",
            c.motivo,
            c.estado as "estado!",
            v.id as "visitante_id!",
            v.cedula as "visitante_cedula!",
            v.nombre as "visitante_nombre!",
            v.apellido as "visitante_apellido!",
            (v.nombre || ' ' || v.apellido) as "visitante_nombre_completo!",
            v.empresa as visitante_empresa
        FROM citas c
        JOIN visitantes v ON c.visitante_id = v.id
        WHERE c.estado = 'PENDIENTE'
        AND c.fecha_cita BETWEEN ? AND ?
        ORDER BY c.fecha_cita ASC
        "#,
        fecha_inicio,
        fecha_fin
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| CitaPopulated {
            id: r.id,
            fecha_cita: r.fecha_cita,
            anfitrion: r.anfitrion,
            area_visitada: r.area_visitada,
            motivo: r.motivo,
            estado: r.estado,
            visitante_id: r.visitante_id,
            visitante_cedula: r.visitante_cedula,
            visitante_nombre: r.visitante_nombre,
            visitante_apellido: r.visitante_apellido,
            visitante_nombre_completo: r.visitante_nombre_completo,
            visitante_empresa: r.visitante_empresa,
        })
        .collect())
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Cita>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT 
            id as "id!",
            visitante_id as "visitante_id!",
            fecha_cita as "fecha_cita!",
            anfitrion as "anfitrion!",
            area_visitada as "area_visitada!",
            motivo,
            estado as "estado!",
            registrado_por as "registrado_por!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM citas
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Cita {
        id: r.id,
        visitante_id: r.visitante_id,
        fecha_cita: r.fecha_cita,
        anfitrion: r.anfitrion,
        area_visitada: r.area_visitada,
        motivo: r.motivo,
        estado: r.estado,
        registrado_por: r.registrado_por,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn marcar_cita_completada(pool: &SqlitePool, cita_id: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query!(
        r#"
        UPDATE citas SET estado = 'COMPLETADA', updated_at = ? WHERE id = ?
        "#,
        now,
        cita_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Obtiene todas las citas pendientes (hoy y futuras)
pub async fn get_all_citas_pendientes(
    pool: &SqlitePool,
) -> Result<Vec<CitaPopulated>, sqlx::Error> {
    let today = Utc::now().format("%Y-%m-%dT00:00:00").to_string();

    let rows = sqlx::query!(
        r#"
        SELECT 
            c.id as "id!",
            c.fecha_cita as "fecha_cita!",
            c.anfitrion as "anfitrion!",
            c.area_visitada as "area_visitada!",
            c.motivo,
            c.estado as "estado!",
            v.id as "visitante_id!",
            v.cedula as "visitante_cedula!",
            v.nombre as "visitante_nombre!",
            v.apellido as "visitante_apellido!",
            (v.nombre || ' ' || v.apellido) as "visitante_nombre_completo!",
            v.empresa as visitante_empresa
        FROM citas c
        JOIN visitantes v ON c.visitante_id = v.id
        WHERE c.estado = 'PENDIENTE'
        AND c.fecha_cita >= ?
        ORDER BY c.fecha_cita ASC
        "#,
        today
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| CitaPopulated {
            id: r.id,
            fecha_cita: r.fecha_cita,
            anfitrion: r.anfitrion,
            area_visitada: r.area_visitada,
            motivo: r.motivo,
            estado: r.estado,
            visitante_id: r.visitante_id,
            visitante_cedula: r.visitante_cedula,
            visitante_nombre: r.visitante_nombre,
            visitante_apellido: r.visitante_apellido,
            visitante_nombre_completo: r.visitante_nombre_completo,
            visitante_empresa: r.visitante_empresa,
        })
        .collect())
}

/// Actualiza los detalles de una cita pendiente
pub async fn update_cita(
    pool: &SqlitePool,
    id: &str,
    fecha_cita: &str,
    anfitrion: &str,
    area_visitada: &str,
    motivo: Option<&str>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    sqlx::query!(
        r#"
        UPDATE citas 
        SET fecha_cita = ?, anfitrion = ?, area_visitada = ?, motivo = ?, updated_at = ?
        WHERE id = ? AND estado = 'PENDIENTE'
        "#,
        fecha_cita,
        anfitrion,
        area_visitada,
        motivo,
        now,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
