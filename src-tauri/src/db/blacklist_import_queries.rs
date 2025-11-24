// ==========================================
// src/db/blacklist_import_queries.rs
// ==========================================

use sqlx::{SqlitePool, Row};
use crate::models::blacklist_import::BlacklistImportTest;

pub async fn insert_blacklist_import(
    pool: &SqlitePool,
    entry: &BlacklistImportTest,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO blacklist_import_test (
            id, cedula, 
            primer_nombre, segundo_nombre,
            primer_apellido, segundo_apellido,
            empresa, motivo_bloqueo, fecha_inicio_bloqueo,
            observaciones, imported_by
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&entry.id)
    .bind(&entry.cedula)
    .bind(&entry.primer_nombre)
    .bind(&entry.segundo_nombre)
    .bind(&entry.primer_apellido)
    .bind(&entry.segundo_apellido)
    .bind(&entry.empresa)
    .bind(&entry.motivo_bloqueo)
    .bind(&entry.fecha_inicio_bloqueo)
    .bind(&entry.observaciones)
    .bind(&entry.imported_by)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_batch_blacklist_import(
    pool: &SqlitePool,
    entries: &[BlacklistImportTest],
) -> Result<usize, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut inserted = 0;

    for entry in entries {
        sqlx::query(
            r#"
            INSERT INTO blacklist_import_test (
                id, cedula,
                primer_nombre, segundo_nombre,
                primer_apellido, segundo_apellido,
                empresa, motivo_bloqueo, fecha_inicio_bloqueo,
                observaciones, imported_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entry.id)
        .bind(&entry.cedula)
        .bind(&entry.primer_nombre)
        .bind(&entry.segundo_nombre)
        .bind(&entry.primer_apellido)
        .bind(&entry.segundo_apellido)
        .bind(&entry.empresa)
        .bind(&entry.motivo_bloqueo)
        .bind(&entry.fecha_inicio_bloqueo)
        .bind(&entry.observaciones)
        .bind(&entry.imported_by)
        .execute(&mut *tx)
        .await?;

        inserted += 1;
    }

    tx.commit().await?;
    Ok(inserted)
}

pub async fn get_blacklist_import_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<BlacklistImportTest>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT 
            id, cedula,
            primer_nombre, segundo_nombre,
            primer_apellido, segundo_apellido,
            nombre_completo,
            empresa, motivo_bloqueo, fecha_inicio_bloqueo,
            observaciones,
            imported_at, imported_by,
            created_at, updated_at
        FROM blacklist_import_test
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| BlacklistImportTest {
        id: r.get("id"),
        cedula: r.get("cedula"),
        primer_nombre: r.get("primer_nombre"),
        segundo_nombre: r.get("segundo_nombre"),
        primer_apellido: r.get("primer_apellido"),
        segundo_apellido: r.get("segundo_apellido"),
        nombre_completo: r.get("nombre_completo"),
        empresa: r.get("empresa"),
        motivo_bloqueo: r.get("motivo_bloqueo"),
        fecha_inicio_bloqueo: r.get("fecha_inicio_bloqueo"),
        observaciones: r.get("observaciones"),
        imported_at: r.get("imported_at"),
        imported_by: r.get("imported_by"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn get_blacklist_import_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Option<BlacklistImportTest>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT 
            id, cedula,
            primer_nombre, segundo_nombre,
            primer_apellido, segundo_apellido,
            nombre_completo,
            empresa, motivo_bloqueo, fecha_inicio_bloqueo,
            observaciones,
            imported_at, imported_by,
            created_at, updated_at
        FROM blacklist_import_test
        WHERE cedula = ?
        ORDER BY imported_at DESC
        LIMIT 1
        "#
    )
    .bind(cedula)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| BlacklistImportTest {
        id: r.get("id"),
        cedula: r.get("cedula"),
        primer_nombre: r.get("primer_nombre"),
        segundo_nombre: r.get("segundo_nombre"),
        primer_apellido: r.get("primer_apellido"),
        segundo_apellido: r.get("segundo_apellido"),
        nombre_completo: r.get("nombre_completo"),
        empresa: r.get("empresa"),
        motivo_bloqueo: r.get("motivo_bloqueo"),
        fecha_inicio_bloqueo: r.get("fecha_inicio_bloqueo"),
        observaciones: r.get("observaciones"),
        imported_at: r.get("imported_at"),
        imported_by: r.get("imported_by"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn get_all_blacklist_imports(
    pool: &SqlitePool,
) -> Result<Vec<BlacklistImportTest>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT 
            id, cedula,
            primer_nombre, segundo_nombre,
            primer_apellido, segundo_apellido,
            nombre_completo,
            empresa, motivo_bloqueo, fecha_inicio_bloqueo,
            observaciones,
            imported_at, imported_by,
            created_at, updated_at
        FROM blacklist_import_test
        ORDER BY imported_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| BlacklistImportTest {
        id: r.get("id"),
        cedula: r.get("cedula"),
        primer_nombre: r.get("primer_nombre"),
        segundo_nombre: r.get("segundo_nombre"),
        primer_apellido: r.get("primer_apellido"),
        segundo_apellido: r.get("segundo_apellido"),
        nombre_completo: r.get("nombre_completo"),
        empresa: r.get("empresa"),
        motivo_bloqueo: r.get("motivo_bloqueo"),
        fecha_inicio_bloqueo: r.get("fecha_inicio_bloqueo"),
        observaciones: r.get("observaciones"),
        imported_at: r.get("imported_at"),
        imported_by: r.get("imported_by"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect())
}

pub async fn get_blacklist_imports_by_empresa(
    pool: &SqlitePool,
    empresa: &str,
) -> Result<Vec<BlacklistImportTest>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT 
            id, cedula,
            primer_nombre, segundo_nombre,
            primer_apellido, segundo_apellido,
            nombre_completo,
            empresa, motivo_bloqueo, fecha_inicio_bloqueo,
            observaciones,
            imported_at, imported_by,
            created_at, updated_at
        FROM blacklist_import_test
        WHERE empresa = ?
        ORDER BY imported_at DESC
        "#
    )
    .bind(empresa)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| BlacklistImportTest {
        id: r.get("id"),
        cedula: r.get("cedula"),
        primer_nombre: r.get("primer_nombre"),
        segundo_nombre: r.get("segundo_nombre"),
        primer_apellido: r.get("primer_apellido"),
        segundo_apellido: r.get("segundo_apellido"),
        nombre_completo: r.get("nombre_completo"),
        empresa: r.get("empresa"),
        motivo_bloqueo: r.get("motivo_bloqueo"),
        fecha_inicio_bloqueo: r.get("fecha_inicio_bloqueo"),
        observaciones: r.get("observaciones"),
        imported_at: r.get("imported_at"),
        imported_by: r.get("imported_by"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect())
}

pub async fn get_recent_imports(
    pool: &SqlitePool,
    limit: i64,
) -> Result<Vec<BlacklistImportTest>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT 
            id, cedula,
            primer_nombre, segundo_nombre,
            primer_apellido, segundo_apellido,
            nombre_completo,
            empresa, motivo_bloqueo, fecha_inicio_bloqueo,
            observaciones,
            imported_at, imported_by,
            created_at, updated_at
        FROM blacklist_import_test
        ORDER BY imported_at DESC
        LIMIT ?
        "#
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| BlacklistImportTest {
        id: r.get("id"),
        cedula: r.get("cedula"),
        primer_nombre: r.get("primer_nombre"),
        segundo_nombre: r.get("segundo_nombre"),
        primer_apellido: r.get("primer_apellido"),
        segundo_apellido: r.get("segundo_apellido"),
        nombre_completo: r.get("nombre_completo"),
        empresa: r.get("empresa"),
        motivo_bloqueo: r.get("motivo_bloqueo"),
        fecha_inicio_bloqueo: r.get("fecha_inicio_bloqueo"),
        observaciones: r.get("observaciones"),
        imported_at: r.get("imported_at"),
        imported_by: r.get("imported_by"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect())
}

pub async fn update_blacklist_import(
    pool: &SqlitePool,
    id: &str,
    primer_nombre: Option<&str>,
    segundo_nombre: Option<&str>,
    primer_apellido: Option<&str>,
    segundo_apellido: Option<&str>,
    empresa: Option<&str>,
    motivo_bloqueo: Option<&str>,
    observaciones: Option<&str>,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE blacklist_import_test
        SET 
            primer_nombre = COALESCE(?, primer_nombre),
            segundo_nombre = COALESCE(?, segundo_nombre),
            primer_apellido = COALESCE(?, primer_apellido),
            segundo_apellido = COALESCE(?, segundo_apellido),
            empresa = COALESCE(?, empresa),
            motivo_bloqueo = COALESCE(?, motivo_bloqueo),
            observaciones = COALESCE(?, observaciones)
        WHERE id = ?
        "#
    )
    .bind(primer_nombre)
    .bind(segundo_nombre)
    .bind(primer_apellido)
    .bind(segundo_apellido)
    .bind(empresa)
    .bind(motivo_bloqueo)
    .bind(observaciones)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_blacklist_import(
    pool: &SqlitePool,
    id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM blacklist_import_test
        WHERE id = ?
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_all_blacklist_imports(
    pool: &SqlitePool,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM blacklist_import_test
        "#
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn count_all_imports(
    pool: &SqlitePool,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM blacklist_import_test
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(row.get("count"))
}

pub async fn count_imports_by_empresa(
    pool: &SqlitePool,
) -> Result<Vec<(String, i64)>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT empresa, COUNT(*) as count
        FROM blacklist_import_test
        GROUP BY empresa
        ORDER BY count DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter()
        .map(|r| (r.get("empresa"), r.get("count")))
        .collect())
}

pub async fn exists_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM blacklist_import_test
        WHERE cedula = ?
        "#
    )
    .bind(cedula)
    .fetch_one(pool)
    .await?;

    let count: i64 = row.get("count");
    Ok(count > 0)
}