use crate::domain::ingreso_visita::{CreateIngresoVisitaInput, IngresoVisita};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create(
    pool: &SqlitePool,
    input: CreateIngresoVisitaInput,
) -> Result<IngresoVisita, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = "ADENTRO";

    sqlx::query(
        r#"
        INSERT INTO ingresos_visitas (
            id, visitante_id, cita_id, anfitrion, area_visitada, motivo, gafete,
            fecha_ingreso, estado, usuario_ingreso_id, observaciones, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.visitante_id)
    .bind(&input.cita_id)
    .bind(&input.anfitrion)
    .bind(&input.area_visitada)
    .bind(&input.motivo)
    .bind(&input.gafete)
    .bind(&now)
    .bind(estado)
    .bind(&input.usuario_ingreso_id)
    .bind(&input.observaciones)
    .bind(&now)
    .bind(&now)
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

pub async fn find_actives(pool: &SqlitePool) -> Result<Vec<IngresoVisita>, sqlx::Error> {
    sqlx::query_as::<_, IngresoVisita>(
        "SELECT * FROM ingresos_visitas WHERE estado = 'ADENTRO' ORDER BY fecha_ingreso DESC",
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

    // Si hay observaciones nuevas, las concatenamos o reemplazamos (aquí simple reemplazo o coalesce)
    sqlx::query(
        r#"
        UPDATE ingresos_visitas 
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
