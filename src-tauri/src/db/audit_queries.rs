// ==========================================
// src/db/audit_queries.rs
// ==========================================
// Queries para tablas de auditoría

use sqlx::SqlitePool;

// ==========================================
// PRAIND HISTORIAL
// ==========================================

pub async fn insert_praind_historial(
    pool: &SqlitePool,
    id: &str,
    contratista_id: &str,
    fecha_anterior: Option<&str>,
    fecha_nueva: &str,
    actualizado_por: &str,
    motivo: Option<&str>,
    created_at: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO praind_historial (id, contratista_id, fecha_anterior, fecha_nueva, actualizado_por, motivo, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(contratista_id)
    .bind(fecha_anterior)
    .bind(fecha_nueva)
    .bind(actualizado_por)
    .bind(motivo)
    .bind(created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_praind_historial_by_contratista(
    pool: &SqlitePool,
    contratista_id: &str,
) -> Result<Vec<PraindHistorialRow>, sqlx::Error> {
    sqlx::query_as::<_, PraindHistorialRow>(
        r#"
        SELECT id, contratista_id, fecha_anterior, fecha_nueva, actualizado_por, motivo, created_at
        FROM praind_historial
        WHERE contratista_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(contratista_id)
    .fetch_all(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PraindHistorialRow {
    pub id: String,
    pub contratista_id: String,
    pub fecha_anterior: Option<String>,
    pub fecha_nueva: String,
    pub actualizado_por: String,
    pub motivo: Option<String>,
    pub created_at: String,
}

// ==========================================
// HISTORIAL ESTADO CONTRATISTA
// ==========================================

pub async fn insert_historial_estado(
    pool: &SqlitePool,
    id: &str,
    contratista_id: &str,
    estado_anterior: &str,
    estado_nuevo: &str,
    cambiado_por: Option<&str>,
    motivo: &str,
    created_at: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO historial_estado_contratista (id, contratista_id, estado_anterior, estado_nuevo, cambiado_por, motivo, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(contratista_id)
    .bind(estado_anterior)
    .bind(estado_nuevo)
    .bind(cambiado_por)
    .bind(motivo)
    .bind(created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_historial_estado_by_contratista(
    pool: &SqlitePool,
    contratista_id: &str,
) -> Result<Vec<HistorialEstadoRow>, sqlx::Error> {
    sqlx::query_as::<_, HistorialEstadoRow>(
        r#"
        SELECT id, contratista_id, estado_anterior, estado_nuevo, cambiado_por, motivo, created_at
        FROM historial_estado_contratista
        WHERE contratista_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(contratista_id)
    .fetch_all(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistorialEstadoRow {
    pub id: String,
    pub contratista_id: String,
    pub estado_anterior: String,
    pub estado_nuevo: String,
    pub cambiado_por: Option<String>,
    pub motivo: String,
    pub created_at: String,
}

// ==========================================
// HISTORIAL BLOQUEOS
// ==========================================

pub async fn insert_historial_bloqueo(
    pool: &SqlitePool,
    id: &str,
    lista_negra_id: &str,
    accion: &str,
    usuario_id: &str,
    motivo: &str,
    created_at: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO historial_bloqueos (id, lista_negra_id, accion, usuario_id, motivo, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(lista_negra_id)
    .bind(accion)
    .bind(usuario_id)
    .bind(motivo)
    .bind(created_at)
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistorialBloqueoRow {
    pub id: String,
    pub lista_negra_id: String,
    pub accion: String,
    pub usuario_id: String,
    pub motivo: String,
    pub created_at: String,
}

// ==========================================
// REPORTES SEGURIDAD
// ==========================================

pub async fn insert_reporte_seguridad(
    pool: &SqlitePool,
    id: &str,
    tipo: &str,
    contratista_id: Option<&str>,
    ingreso_id: Option<&str>,
    descripcion: &str,
    generado_por: Option<&str>,
    created_at: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO reportes_seguridad (id, tipo, contratista_id, ingreso_id, descripcion, generado_por, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(tipo)
    .bind(contratista_id)
    .bind(ingreso_id)
    .bind(descripcion)
    .bind(generado_por)
    .bind(created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn resolver_reporte_seguridad(
    pool: &SqlitePool,
    id: &str,
    resolucion: &str,
    resuelto_por: &str,
    fecha_resolucion: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE reportes_seguridad
        SET resolucion = ?, resuelto_por = ?, fecha_resolucion = ?
        WHERE id = ?
        "#,
    )
    .bind(resolucion)
    .bind(resuelto_por)
    .bind(fecha_resolucion)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_reportes_pendientes(
    pool: &SqlitePool,
) -> Result<Vec<ReporteSeguridadRow>, sqlx::Error> {
    sqlx::query_as::<_, ReporteSeguridadRow>(
        r#"
        SELECT id, tipo, contratista_id, ingreso_id, descripcion, generado_por, resolucion, resuelto_por, fecha_resolucion, created_at
        FROM reportes_seguridad
        WHERE resolucion IS NULL
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_reportes_by_tipo(
    pool: &SqlitePool,
    tipo: &str,
) -> Result<Vec<ReporteSeguridadRow>, sqlx::Error> {
    sqlx::query_as::<_, ReporteSeguridadRow>(
        r#"
        SELECT id, tipo, contratista_id, ingreso_id, descripcion, generado_por, resolucion, resuelto_por, fecha_resolucion, created_at
        FROM reportes_seguridad
        WHERE tipo = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(tipo)
    .fetch_all(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReporteSeguridadRow {
    pub id: String,
    pub tipo: String,
    pub contratista_id: Option<String>,
    pub ingreso_id: Option<String>,
    pub descripcion: String,
    pub generado_por: Option<String>,
    pub resolucion: Option<String>,
    pub resuelto_por: Option<String>,
    pub fecha_resolucion: Option<String>,
    pub created_at: String,
}
