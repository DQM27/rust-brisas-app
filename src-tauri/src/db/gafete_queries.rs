// ==========================================
// src/db/gafete_queries.rs
// ==========================================
// Queries SQL puras usando Strict Mode (query_as!)
// Retorna sqlx::Result para manejo tipado en capa superior

use crate::models::gafete::{Gafete, GafeteEstado, TipoGafete};
use sqlx::SqlitePool;

// ==========================================
// DTO & CONVERSION
// ==========================================

#[derive(sqlx::FromRow)]
struct GafeteRow {
    numero: String,
    tipo: String,
    estado: String,
    created_at: String,
    updated_at: String,
}

impl From<GafeteRow> for Gafete {
    fn from(row: GafeteRow) -> Self {
        Gafete {
            numero: row.numero,
            tipo: row.tipo.parse().unwrap_or(TipoGafete::Contratista), // Fallback seguro
            estado: row.estado.parse().unwrap_or(GafeteEstado::Activo),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un gafete por número y tipo
pub async fn find_by_numero_and_tipo(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> sqlx::Result<Gafete> {
    let row = sqlx::query_as!(
        GafeteRow,
        r#"SELECT numero, tipo, estado, created_at, updated_at FROM gafetes WHERE numero = ? AND tipo = ? AND is_deleted = 0"#,
        numero,
        tipo
    )
    .fetch_one(pool)
    .await?;

    Ok(row.into())
}

/// Obtiene todos los gafetes
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Gafete>> {
    let rows = sqlx::query_as!(
        GafeteRow,
        r#"SELECT numero, tipo, estado, created_at, updated_at FROM gafetes WHERE is_deleted = 0 ORDER BY numero"#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(Gafete::from).collect())
}

/// Busca gafetes de un tipo específico
pub async fn find_by_tipo(pool: &SqlitePool, tipo: &str) -> sqlx::Result<Vec<Gafete>> {
    let rows = sqlx::query_as!(
        GafeteRow,
        r#"SELECT numero, tipo, estado, created_at, updated_at FROM gafetes WHERE tipo = ? AND is_deleted = 0 ORDER BY numero"#,
        tipo
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(Gafete::from).collect())
}

/// Cuenta gafetes por número
pub async fn count_by_numero(pool: &SqlitePool, numero: &str) -> sqlx::Result<i64> {
    let result = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM gafetes WHERE numero = ? AND is_deleted = 0"#,
        numero
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count as i64)
}

/// Verifica si ya existe un gafete con ese número + tipo
pub async fn exists_by_numero_and_tipo(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        r#"SELECT 1 as one FROM gafetes WHERE numero = ? AND tipo = ? AND is_deleted = 0 LIMIT 1"#,
        numero,
        tipo
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}

/// Verifica si un gafete está en uso (tiene ingreso activo)
pub async fn is_en_uso(pool: &SqlitePool, numero: &str, tipo: &str) -> sqlx::Result<bool> {
    let count = match tipo {
        "contratista" => {
            sqlx::query_scalar!(
                r#"SELECT COUNT(*) FROM ingresos_contratistas 
                 WHERE gafete_numero = ? 
                 AND tipo_ingreso = 'contratista' 
                 AND fecha_hora_salida IS NULL"#,
                numero
            )
            .fetch_one(pool)
            .await?
        }
        "visita" => {
            sqlx::query_scalar!(
                r#"SELECT COUNT(*) FROM ingresos_visitas 
                 WHERE gafete = ? 
                 AND estado = 'ADENTRO'"#,
                numero
            )
            .fetch_one(pool)
            .await?
        }
        "proveedor" => {
            sqlx::query_scalar!(
                r#"SELECT COUNT(*) FROM ingresos_proveedores 
                 WHERE gafete = ? 
                 AND fecha_salida IS NULL"#,
                numero
            )
            .fetch_one(pool)
            .await?
        }
        _ => 0,
    };

    Ok(count > 0)
}

/// Obtiene números de gafetes disponibles de un tipo especifico
pub async fn find_disponibles_by_tipo(pool: &SqlitePool, tipo: &str) -> sqlx::Result<Vec<String>> {
    // Definimos el struct fuera del match para evitar errores de tipos incompatibles
    struct NumRow {
        numero: String,
    }

    let numeros = match tipo {
        "proveedor" => {
            sqlx::query_as!(
                NumRow,
                r#"SELECT g.numero FROM gafetes g
                 LEFT JOIN ingresos_proveedores ip ON g.numero = ip.gafete AND ip.fecha_salida IS NULL
                 LEFT JOIN alertas_gafetes a ON g.numero = a.gafete_numero AND a.resuelto = 0 AND a.ingreso_proveedor_id IS NOT NULL
                 WHERE g.tipo = 'proveedor' AND g.estado = 'activo'
                 AND ip.id IS NULL AND a.id IS NULL AND g.numero != 'S/G' AND g.is_deleted = 0
                 ORDER BY g.numero"#
            )
            .fetch_all(pool)
            .await?
        }
        "visita" => {
            // Se eliminó el JOIN con alertas_gafetes usando ingreso_visita_id porque la columna no existe
             sqlx::query_as!(
                NumRow,
                r#"SELECT g.numero FROM gafetes g
                 LEFT JOIN ingresos_visitas iv ON g.numero = iv.gafete AND iv.estado = 'ADENTRO'
                 WHERE g.tipo = 'visita' AND g.estado = 'activo'
                 AND iv.id IS NULL AND g.numero != 'S/G' AND g.is_deleted = 0
                 ORDER BY g.numero"#
            )
            .fetch_all(pool)
            .await?
        }
        _ => {
            // Contratista y default
            sqlx::query_as!(
                NumRow,
                r#"SELECT g.numero FROM gafetes g
                 LEFT JOIN ingresos_contratistas i ON g.numero = i.gafete_numero AND i.fecha_hora_salida IS NULL AND i.tipo_ingreso = 'contratista'
                 LEFT JOIN alertas_gafetes a ON g.numero = a.gafete_numero AND a.resuelto = 0 AND a.ingreso_contratista_id IS NOT NULL
                 WHERE g.tipo = ? AND g.estado = 'activo'
                 AND i.id IS NULL AND a.id IS NULL AND g.numero != 'S/G' AND g.is_deleted = 0
                 ORDER BY g.numero"#,
                tipo
            )
            .fetch_all(pool)
            .await?
        }
    };

    Ok(numeros.into_iter().map(|r| r.numero).collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

pub async fn insert(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
    created_at: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO gafetes (numero, tipo, estado, created_at, updated_at) VALUES (?, ?, 'activo', ?, ?)",
        numero,
        tipo,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update(
    pool: &SqlitePool,
    numero: &str,
    tipo_actual: &str,
    tipo_nuevo: Option<&str>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE gafetes SET tipo = COALESCE(?, tipo), updated_at = ? WHERE numero = ? AND tipo = ?",
        tipo_nuevo,
        updated_at,
        numero,
        tipo_actual
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_status(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
    estado: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE gafetes SET estado = ?, updated_at = ? WHERE numero = ? AND tipo = ?",
        estado,
        updated_at,
        numero,
        tipo
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &SqlitePool, numero: &str, tipo: &str) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE gafetes SET is_deleted = 1 WHERE numero = ? AND tipo = ?"#,
        numero,
        tipo
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_last_status_change(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> sqlx::Result<Option<(String, String)>> {
    let result = sqlx::query!(
        r#"SELECT u.nombre, u.apellido, h.fecha_cambio 
           FROM historial_estado_gafetes h 
           LEFT JOIN users u ON h.cambiado_por = u.id 
           WHERE h.gafete_numero = ? AND h.gafete_tipo = ? 
           ORDER BY h.fecha_cambio DESC 
           LIMIT 1"#,
        numero,
        tipo
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| {
        let nombre = r.nombre.unwrap_or_else(|| "Sistema".to_string());
        let nombre_completo = if let Some(apellido) = r.apellido {
            format!("{} {}", nombre, apellido)
        } else {
            nombre
        };
        (nombre_completo, r.fecha_cambio)
    }))
}

// ==========================================
// QUERIES DE ALERTAS (HELPERS)
// ==========================================

/// Obtiene alerta reciente con filtro de tipo (JOIN)
pub async fn get_recent_alert_for_gafete_typed(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> sqlx::Result<
    Option<(
        String,
        String,
        String,
        bool,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )>,
> {
    let result = sqlx::query!(
        r#"SELECT 
            ag.id, 
            ag.fecha_reporte, 
            ag.nombre_completo, 
            ag.resuelto as "resuelto: bool",
            u_rep.nombre as "reporter_nombre",
            u_rep.apellido as "reporter_apellido",
            u_res.nombre as "resolver_nombre",
            u_res.apellido as "resolver_apellido",
            ag.fecha_resolucion,
            ag.notas
         FROM alertas_gafetes ag
         LEFT JOIN users u_rep ON ag.reportado_por = u_rep.id
         LEFT JOIN users u_res ON ag.resuelto_por = u_res.id
         WHERE ag.gafete_numero = ? 
         AND (
            ( ? = 'proveedor' AND ag.ingreso_proveedor_id IS NOT NULL )
            OR
            ( ? = 'contratista' AND ag.ingreso_contratista_id IS NOT NULL )
         )
         ORDER BY ag.fecha_reporte DESC 
         LIMIT 1"#,
        numero,
        tipo,
        tipo
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| {
        let reporter_name = match (r.reporter_nombre, r.reporter_apellido) {
            (Some(n), Some(a)) => Some(format!("{} {}", n, a)),
            _ => None,
        };
        let resolver_name = match (r.resolver_nombre, r.resolver_apellido) {
            (Some(n), Some(a)) => Some(format!("{} {}", n, a)),
            _ => None,
        };

        (
            r.id.unwrap_or_default(),
            r.fecha_reporte,
            r.nombre_completo,
            r.resuelto.unwrap_or(false),
            reporter_name,
            resolver_name,
            r.fecha_resolucion,
            r.notas,
        )
    }))
}

/// Verifica si un gafete tiene una alerta pendiente
pub async fn has_unresolved_alert(pool: &SqlitePool, numero: &str) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM alertas_gafetes WHERE gafete_numero = ? AND resuelto = 0",
        numero
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count > 0)
}

pub async fn has_unresolved_alert_typed(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        r#"SELECT COUNT(*) as count
         FROM alertas_gafetes ag
         WHERE ag.gafete_numero = ? AND ag.resuelto = 0
         AND (
            ( ? = 'proveedor' AND ag.ingreso_proveedor_id IS NOT NULL )
            OR
            ( ? = 'contratista' AND ag.ingreso_contratista_id IS NOT NULL )
         )"#,
        numero,
        tipo,
        tipo
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::Executor;

    async fn setup_test_env() -> SqlitePool {
        let db_id = uuid::Uuid::new_v4().to_string();
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&format!("sqlite:file:{}?mode=memory&cache=shared", db_id))
            .await
            .unwrap();

        pool.execute("PRAGMA foreign_keys = OFF;").await.unwrap();

        let schemas = vec![
            "migrations/1_create_users.sql",
            "migrations/2_create_contratista.sql",
            "migrations/5_create_gafete.sql",
            "migrations/7_create_ingreso.sql",
            "migrations/6_create_alertas_gafetes.sql",
        ];

        for path in schemas {
            let sql = std::fs::read_to_string(path).unwrap();
            pool.execute(sql.as_str()).await.unwrap();
        }

        // Seed user
        pool.execute("INSERT INTO users (id, email, password_hash, nombre, apellido, role_id, created_at, updated_at, cedula, must_change_password, is_active) 
                      VALUES ('u-1', 'admin@test.com', 'hash', 'Admin', 'Test', 'role-admin', '2025-01-01', '2025-01-01', '000', 0, 1)").await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_gafete_crud() {
        let pool = setup_test_env().await;
        let num = "G-999";
        let tipo = "contratista";

        // 1. Insert
        insert(&pool, num, tipo, "now", "now").await.unwrap();

        // 2. Exists
        assert!(exists_by_numero_and_tipo(&pool, num, tipo).await.unwrap());

        // 3. Find
        let g = find_by_numero_and_tipo(&pool, num, tipo).await.unwrap();
        assert_eq!(g.numero, num);

        // 4. Update status
        update_status(&pool, num, tipo, "extraviado", "updated").await.unwrap();
        let g2 = find_by_numero_and_tipo(&pool, num, tipo).await.unwrap();
        assert!(matches!(g2.estado, GafeteEstado::Extraviado));

        // 5. Delete
        delete(&pool, num, tipo).await.unwrap();
        assert!(!exists_by_numero_and_tipo(&pool, num, tipo).await.unwrap());
    }

    #[tokio::test]
    async fn test_find_disponibles() {
        let pool = setup_test_env().await;
        insert(&pool, "G-1", "contratista", "now", "now").await.unwrap();
        insert(&pool, "G-2", "contratista", "now", "now").await.unwrap();

        // G-1 en uso (ingreso abierto)
        pool.execute("INSERT INTO ingresos_contratistas (id, contratista_id, cedula, nombre, apellido, empresa_nombre, tipo_ingreso, tipo_autorizacion, modo_ingreso, gafete_numero, fecha_hora_ingreso, usuario_ingreso_id, created_at, updated_at)
                      VALUES ('i-1', 'c-1', '123', 'J', 'P', 'E', 'contratista', 'praind', 'caminando', 'G-1', 'now', 'u-1', 'now', 'now')").await.unwrap();

        let disp = find_disponibles_by_tipo(&pool, "contratista").await.unwrap();
        assert_eq!(disp.len(), 1);
        assert_eq!(disp[0], "G-2");
    }

    #[tokio::test]
    async fn test_gafete_alerts() {
        let pool = setup_test_env().await;
        insert(&pool, "G-1", "contratista", "now", "now").await.unwrap();

        // Crear alerta sin resolver
        pool.execute("INSERT INTO alertas_gafetes (id, cedula, nombre_completo, gafete_numero, ingreso_contratista_id, fecha_reporte, resuelto, reportado_por, created_at, updated_at)
                      VALUES ('a-1', '123', 'Juan P', 'G-1', 'i-1', '2025-01-01', 0, 'u-1', 'now', 'now')").await.unwrap();

        assert!(has_unresolved_alert(&pool, "G-1").await.unwrap());

        let alert_opt =
            get_recent_alert_for_gafete_typed(&pool, "G-1", "contratista").await.unwrap();
        assert!(alert_opt.is_some());

        let (_, _, name, resuelto, ..) = alert_opt.unwrap();
        assert_eq!(name, "Juan P");
        assert!(!resuelto);
    }
}
