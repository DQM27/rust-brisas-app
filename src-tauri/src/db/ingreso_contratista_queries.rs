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
            "migrations/4_create_vehiculo.sql",
            "migrations/7_create_ingreso.sql",
        ];

        for path in schemas {
            let sql = std::fs::read_to_string(path).unwrap();
            pool.execute(sql.as_str()).await.unwrap();
        }

        // Seed
        pool.execute("INSERT INTO users (id, email, password_hash, nombre, apellido, role_id, created_at, updated_at, cedula, must_change_password, is_active) 
                      VALUES ('u-1', 'admin@test.com', 'hash', 'Admin', 'Test', 'role-admin', '2025-01-01', '2025-01-01', '000', 0, 1)").await.unwrap();

        pool.execute("INSERT INTO contratistas (id, cedula, nombre, apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
                      VALUES ('c-1', '12345', 'Juan', 'Perez', 'e-1', '2030-01-01', 'activo', '2025-01-01', '2025-01-01')").await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_ingreso_contratista_crud() {
        let pool = setup_test_env().await;
        let id = "i-1";

        // 1. Insert
        insert(
            &pool,
            id,
            "c-1",
            "12345",
            "Juan",
            "Perez",
            "Test Corp",
            "contratista",
            "praind",
            "caminando",
            None,
            None,
            Some("G-1"),
            "2025-01-01 10:00",
            "u-1",
            Some(true),
            Some("activo"),
            None,
            "now",
            "now",
        )
        .await
        .unwrap();

        // 2. Find by id
        let ing = find_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(ing.cedula, "12345");
        assert!(ing.praind_vigente_al_ingreso.unwrap_or(false));

        // 3. Find abierto
        let abierto = find_ingreso_abierto_by_contratista(&pool, "c-1")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(abierto.id, id);

        // 4. Registrar salida
        registrar_salida(
            &pool,
            id,
            "2025-01-01 12:00",
            120,
            "u-1",
            Some("Salida OK"),
            "updated-now",
        )
        .await
        .unwrap();

        let ing_fin = find_by_id(&pool, id).await.unwrap().unwrap();
        assert!(ing_fin.fecha_hora_salida.is_some());
        assert_eq!(ing_fin.tiempo_permanencia_minutos, Some(120));

        // 5. Check abierto again (should be none)
        let none = find_ingreso_abierto_by_contratista(&pool, "c-1")
            .await
            .unwrap();
        assert!(none.is_none());
    }

    #[tokio::test]
    async fn test_find_details() {
        let pool = setup_test_env().await;
        insert(
            &pool,
            "i-1",
            "c-1",
            "12345",
            "Juan",
            "Perez",
            "Test Corp",
            "contratista",
            "praind",
            "caminando",
            None,
            None,
            Some("G-1"),
            "2025-01-01 10:00",
            "u-1",
            Some(true),
            Some("activo"),
            None,
            "now",
            "now",
        )
        .await
        .unwrap();

        let details = find_details_by_id(&pool, "i-1").await.unwrap().unwrap();
        assert_eq!(details.usuario_ingreso_nombre, Some("Admin".to_string()));
    }
}
