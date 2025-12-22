// ==========================================
// src/db/ingreso_visita_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query_as! para validación y DTO intermedio

use crate::domain::ingreso_visita::{
    CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// DTO & CONVERSION
// ==========================================

#[derive(sqlx::FromRow)]
struct IngresoVisitaRow {
    id: Option<String>,
    visitante_id: Option<String>,
    cita_id: Option<String>,
    anfitrion: Option<String>,
    area_visitada: Option<String>,
    motivo: Option<String>,
    gafete: Option<String>,
    fecha_ingreso: Option<String>,
    fecha_salida: Option<String>,
    estado: Option<String>,
    usuario_ingreso_id: Option<String>,
    usuario_salida_id: Option<String>,
    observaciones: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl From<IngresoVisitaRow> for IngresoVisita {
    fn from(row: IngresoVisitaRow) -> Self {
        IngresoVisita {
            id: row.id.unwrap_or_default(),
            visitante_id: row.visitante_id.unwrap_or_default(),
            cita_id: row.cita_id,
            anfitrion: row.anfitrion.unwrap_or_default(),
            area_visitada: row.area_visitada.unwrap_or_default(),
            motivo: row.motivo.unwrap_or_default(),
            gafete: row.gafete,
            fecha_ingreso: row.fecha_ingreso.unwrap_or_default(),
            fecha_salida: row.fecha_salida,
            estado: row.estado.unwrap_or_default(),
            usuario_ingreso_id: row.usuario_ingreso_id.unwrap_or_default(),
            usuario_salida_id: row.usuario_salida_id,
            observaciones: row.observaciones,
            created_at: row.created_at.unwrap_or_default(),
            updated_at: row.updated_at.unwrap_or_default(),
        }
    }
}

#[derive(sqlx::FromRow)]
struct IngresoVisitaPopulatedRow {
    // Ingreso Fields
    id: Option<String>,
    visitante_id: Option<String>,
    cita_id: Option<String>,
    anfitrion: Option<String>,
    area_visitada: Option<String>,
    motivo: Option<String>,
    gafete: Option<String>,
    fecha_ingreso: Option<String>,
    fecha_salida: Option<String>,
    estado: Option<String>,
    usuario_ingreso_id: Option<String>,
    usuario_salida_id: Option<String>,
    observaciones: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    // Visitante Fields
    visitante_nombre: Option<String>,
    visitante_apellido: Option<String>,
    visitante_cedula: Option<String>,
    visitante_empresa: Option<String>,
}

impl From<IngresoVisitaPopulatedRow> for IngresoVisitaPopulated {
    fn from(row: IngresoVisitaPopulatedRow) -> Self {
        IngresoVisitaPopulated {
            id: row.id.unwrap_or_default(),
            visitante_id: row.visitante_id.unwrap_or_default(),
            cita_id: row.cita_id,
            anfitrion: row.anfitrion.unwrap_or_default(),
            area_visitada: row.area_visitada.unwrap_or_default(),
            motivo: row.motivo.unwrap_or_default(),
            gafete: row.gafete,
            fecha_ingreso: row.fecha_ingreso.unwrap_or_default(),
            fecha_salida: row.fecha_salida,
            estado: row.estado.unwrap_or_default(),
            usuario_ingreso_id: row.usuario_ingreso_id.unwrap_or_default(),
            usuario_salida_id: row.usuario_salida_id,
            observaciones: row.observaciones,
            created_at: row.created_at.unwrap_or_default(),
            updated_at: row.updated_at.unwrap_or_default(),
            // Visitante
            visitante_nombre: row.visitante_nombre.unwrap_or_default(),
            visitante_apellido: row.visitante_apellido.unwrap_or_default(),
            visitante_cedula: row.visitante_cedula.unwrap_or_default(),
            visitante_empresa: row.visitante_empresa,
        }
    }
}

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
    let rows = sqlx::query_as!(
        IngresoVisitaPopulatedRow,
        r#"
        SELECT 
            iv.id, iv.visitante_id, iv.cita_id, iv.anfitrion, iv.area_visitada, iv.motivo, iv.gafete,
            CAST(iv.fecha_ingreso AS TEXT) as fecha_ingreso,
            CAST(iv.fecha_salida AS TEXT) as fecha_salida,
            iv.estado, iv.usuario_ingreso_id, iv.usuario_salida_id, iv.observaciones,
            CAST(iv.created_at AS TEXT) as created_at,
            CAST(iv.updated_at AS TEXT) as updated_at,
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
    let rows = sqlx::query_as!(
        IngresoVisitaPopulatedRow,
        r#"
        SELECT 
            iv.id, iv.visitante_id, iv.cita_id, iv.anfitrion, iv.area_visitada, iv.motivo, iv.gafete,
            CAST(iv.fecha_ingreso AS TEXT) as fecha_ingreso,
            CAST(iv.fecha_salida AS TEXT) as fecha_salida,
            iv.estado, iv.usuario_ingreso_id, iv.usuario_salida_id, iv.observaciones,
            CAST(iv.created_at AS TEXT) as created_at,
            CAST(iv.updated_at AS TEXT) as updated_at,
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
    .await?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}
