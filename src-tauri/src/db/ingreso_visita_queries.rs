// ==========================================
// src/db/ingreso_visita_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query_as! para validación

use crate::domain::ingreso_visita::{
    CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// QUERIES
// ==========================================

pub async fn create(
    pool: &SqlitePool,
    input: CreateIngresoVisitaInput,
) -> sqlx::Result<IngresoVisita> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = "ADENTRO";

    sqlx::query!(
        r#"
        INSERT INTO ingresos_visitas (
            id, visitante_id, cita_id, anfitrion, area_visitada, motivo, gafete,
            fecha_ingreso, estado, usuario_ingreso_id, observaciones, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        input.visitante_id,
        input.cita_id,
        input.anfitrion,
        input.area_visitada,
        input.motivo,
        input.gafete,
        now,
        estado,
        input.usuario_ingreso_id,
        input.observaciones,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(IngresoVisita {
        id,
        visitante_id: input.visitante_id,
        cita_id: input.cita_id,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        gafete: input.gafete,
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

pub async fn find_actives(pool: &SqlitePool) -> sqlx::Result<Vec<IngresoVisitaPopulated>> {
    sqlx::query_as::<_, IngresoVisitaPopulated>(
        r#"
        SELECT 
            iv.id, iv.visitante_id, iv.cita_id, iv.anfitrion, iv.area_visitada, iv.motivo, iv.gafete,
            iv.fecha_ingreso, iv.fecha_salida, iv.estado, 
            iv.usuario_ingreso_id, iv.usuario_salida_id, iv.observaciones,
            iv.created_at, iv.updated_at,
            v.nombre as visitante_nombre,
            v.apellido as visitante_apellido,
            v.cedula as visitante_cedula,
            v.empresa as visitante_empresa
        FROM ingresos_visitas iv
        INNER JOIN visitantes v ON iv.visitante_id = v.id
        WHERE iv.estado = 'ADENTRO' 
        ORDER BY iv.fecha_ingreso DESC
        "#
    )
    .fetch_all(pool)
    .await
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
        UPDATE ingresos_visitas 
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

pub async fn find_historial(pool: &SqlitePool) -> sqlx::Result<Vec<IngresoVisitaPopulated>> {
    sqlx::query_as::<_, IngresoVisitaPopulated>(
        r#"
        SELECT 
            iv.id, iv.visitante_id, iv.cita_id, iv.anfitrion, iv.area_visitada, iv.motivo, iv.gafete,
            iv.fecha_ingreso, iv.fecha_salida, iv.estado, 
            iv.usuario_ingreso_id, iv.usuario_salida_id, iv.observaciones,
            iv.created_at, iv.updated_at,
            v.nombre as visitante_nombre,
            v.apellido as visitante_apellido,
            v.cedula as visitante_cedula,
            v.empresa as visitante_empresa
        FROM ingresos_visitas iv
        INNER JOIN visitantes v ON iv.visitante_id = v.id
        WHERE iv.estado = 'SALIO' 
        ORDER BY iv.fecha_salida DESC
        LIMIT 500
        "#
    )
    .fetch_all(pool)
    .await
}
