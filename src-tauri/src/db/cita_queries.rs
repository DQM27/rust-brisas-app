use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_cita(pool: &SqlitePool, input: CreateCitaInput) -> Result<Cita, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = "PENDIENTE";

    sqlx::query(
        r#"
        INSERT INTO citas (id, visitante_id, fecha_cita, anfitrion, area_visitada, motivo, estado, registrado_por, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&input.visitante_id)
    .bind(&input.fecha_cita)
    .bind(&input.anfitrion)
    .bind(&input.area_visitada)
    .bind(&input.motivo)
    .bind(estado)
    .bind(&input.registrado_por)
    .bind(&now)
    .bind(&now)
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

    sqlx::query_as::<_, CitaPopulated>(
        r#"
        SELECT 
            c.id, c.fecha_cita, c.anfitrion, c.area_visitada, c.motivo, c.estado,
            v.id as visitante_id, v.cedula as visitante_cedula, v.nombre as visitante_nombre, v.apellido as visitante_apellido,
            (v.nombre || ' ' || v.apellido) as visitante_nombre_completo,
            v.empresa as visitante_empresa
        FROM citas c
        JOIN visitantes v ON c.visitante_id = v.id
        WHERE c.estado = 'PENDIENTE'
        AND c.fecha_cita BETWEEN ? AND ?
        ORDER BY c.fecha_cita ASC
        "#
    )
    .bind(fecha_inicio)
    .bind(fecha_fin)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Cita>, sqlx::Error> {
    sqlx::query_as::<_, Cita>(
        r#"
        SELECT id, visitante_id, fecha_cita, anfitrion, area_visitada, motivo, estado, registrado_por, created_at, updated_at
        FROM citas
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn marcar_cita_completada(pool: &SqlitePool, cita_id: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        r#"
        UPDATE citas SET estado = 'COMPLETADA', updated_at = ? WHERE id = ?
        "#,
    )
    .bind(now)
    .bind(cita_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Obtiene todas las citas pendientes (hoy y futuras)
pub async fn get_all_citas_pendientes(
    pool: &SqlitePool,
) -> Result<Vec<CitaPopulated>, sqlx::Error> {
    let today = Utc::now().format("%Y-%m-%dT00:00:00").to_string();

    sqlx::query_as::<_, CitaPopulated>(
        r#"
        SELECT 
            c.id, c.fecha_cita, c.anfitrion, c.area_visitada, c.motivo, c.estado,
            v.id as visitante_id, v.cedula as visitante_cedula, v.nombre as visitante_nombre, v.apellido as visitante_apellido,
            (v.nombre || ' ' || v.apellido) as visitante_nombre_completo,
            v.empresa as visitante_empresa
        FROM citas c
        JOIN visitantes v ON c.visitante_id = v.id
        WHERE c.estado = 'PENDIENTE'
        AND c.fecha_cita >= ?
        ORDER BY c.fecha_cita ASC
        "#
    )
    .bind(today)
    .fetch_all(pool)
    .await
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

    sqlx::query(
        r#"
        UPDATE citas 
        SET fecha_cita = ?, anfitrion = ?, area_visitada = ?, motivo = ?, updated_at = ?
        WHERE id = ? AND estado = 'PENDIENTE'
        "#,
    )
    .bind(fecha_cita)
    .bind(anfitrion)
    .bind(area_visitada)
    .bind(motivo)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
