// ==========================================
// src/db/contratista_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query! para validación en tiempo de compilación

use crate::models::contratista::{Contratista, EstadoContratista};
use serde::Serialize;
use sqlx::SqlitePool;

// ==========================================
// TIPOS AUXILIARES
// ==========================================

#[derive(Debug, Serialize)]
pub struct ContratistaInfo {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub estado: String,
    pub fecha_vencimiento_praind: String,
}

pub struct ContratistaEnhancedRow {
    pub contratista: Contratista,
    pub empresa_nombre: String,
    pub vehiculo_tipo: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub vehiculo_marca: Option<String>,
    pub vehiculo_modelo: Option<String>,
    pub vehiculo_color: Option<String>,
    pub is_blocked: bool,
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca información básica de un contratista por ID
pub async fn find_basic_info_by_id(
    pool: &SqlitePool,
    id: &str,
) -> sqlx::Result<Option<ContratistaInfo>> {
    let row = sqlx::query!(
        r#"
        SELECT c.id, c.cedula, c.nombre, c.apellido, e.nombre as empresa_nombre, 
               c.estado, c.fecha_vencimiento_praind 
        FROM contratistas c
        LEFT JOIN empresas e ON c.empresa_id = e.id
        WHERE c.id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(ContratistaInfo {
            id: r.id,
            cedula: r.cedula,
            nombre: r.nombre,
            apellido: r.apellido,
            empresa_nombre: r.empresa_nombre.unwrap_or_default(), // Handle NULL join
            estado: r.estado,
            fecha_vencimiento_praind: r.fecha_vencimiento_praind,
        })),
        None => Ok(None),
    }
}

/// Obtiene datos básicos de un contratista (cédula, nombre, apellido)
pub async fn get_basic_data(
    pool: &SqlitePool,
    contratista_id: &str,
) -> sqlx::Result<Option<(String, String, Option<String>, String, Option<String>)>> {
    let row = sqlx::query!(
        "SELECT cedula, nombre, segundo_nombre, apellido, segundo_apellido FROM contratistas WHERE id = ?",
        contratista_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        (
            r.cedula,
            r.nombre,
            r.segundo_nombre,
            r.apellido,
            r.segundo_apellido,
        )
    }))
}

/// Helper para detalles de lista negra
pub async fn get_blacklist_details(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<Option<(String, String)>> {
    let row = sqlx::query!(
        "SELECT motivo_bloqueo, bloqueado_por FROM lista_negra WHERE cedula = ? AND is_active = 1",
        cedula
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| (r.motivo_bloqueo, r.bloqueado_por)))
}

/// Cuenta contratistas en lista negra por cédula
pub async fn count_cedula_in_blacklist(pool: &SqlitePool, cedula: &str) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND is_active = 1",
        cedula
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Cuenta contratistas por cédula
pub async fn count_by_cedula(pool: &SqlitePool, cedula: &str) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM contratistas WHERE cedula = ?",
        cedula
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Cuenta contratistas por cédula excluyendo un ID
pub async fn count_by_cedula_excluding_id(
    pool: &SqlitePool,
    cedula: &str,
    exclude_id: &str,
) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM contratistas WHERE cedula = ? AND id != ?",
        cedula,
        exclude_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Busca contratista por ID con empresa y vehículo
pub async fn find_by_id_with_empresa(
    pool: &SqlitePool,
    id: &str,
) -> sqlx::Result<Option<ContratistaEnhancedRow>> {
    let row = sqlx::query!(
        r#"
        SELECT c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
               c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
               e.nombre as "empresa_nombre?", 
               v.tipo_vehiculo as "tipo_vehiculo?", 
               v.placa as "placa?", 
               v.marca as "marca?", 
               v.modelo as "modelo?", 
               v.color as "color?",
               (ln.id IS NOT NULL) as "is_blocked: bool"
        FROM contratistas c
        LEFT JOIN empresas e ON c.empresa_id = e.id
        LEFT JOIN vehiculos v ON c.id = v.contratista_id
        LEFT JOIN lista_negra ln ON c.cedula = ln.cedula AND ln.is_active = 1
        WHERE c.id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let estado = r.estado.parse().unwrap_or(EstadoContratista::Inactivo);

            Ok(Some(ContratistaEnhancedRow {
                contratista: Contratista {
                    id: r.id,
                    cedula: r.cedula,
                    nombre: r.nombre,
                    segundo_nombre: r.segundo_nombre,
                    apellido: r.apellido,
                    segundo_apellido: r.segundo_apellido,
                    empresa_id: r.empresa_id,
                    fecha_vencimiento_praind: r.fecha_vencimiento_praind,
                    estado,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                empresa_nombre: r.empresa_nombre.unwrap_or_default(),
                vehiculo_tipo: r.tipo_vehiculo,
                vehiculo_placa: r.placa,
                vehiculo_marca: r.marca,
                vehiculo_modelo: r.modelo,
                vehiculo_color: r.color,
                is_blocked: r.is_blocked,
            }))
        }
        None => Ok(None),
    }
}

/// Busca contratista por cédula con empresa y vehículo
pub async fn find_by_cedula_with_empresa(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<Option<ContratistaEnhancedRow>> {
    let row = sqlx::query!(
        r#"
        SELECT c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
               c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
               e.nombre as "empresa_nombre?", 
               v.tipo_vehiculo as "tipo_vehiculo?", 
               v.placa as "placa?", 
               v.marca as "marca?", 
               v.modelo as "modelo?", 
               v.color as "color?",
               (ln.id IS NOT NULL) as "is_blocked: bool"
        FROM contratistas c
        LEFT JOIN empresas e ON c.empresa_id = e.id
        LEFT JOIN vehiculos v ON c.id = v.contratista_id
        LEFT JOIN lista_negra ln ON c.cedula = ln.cedula AND ln.is_active = 1
        WHERE c.cedula = ?
        "#,
        cedula
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let estado = r.estado.parse().unwrap_or(EstadoContratista::Inactivo);

            Ok(Some(ContratistaEnhancedRow {
                contratista: Contratista {
                    id: r.id,
                    cedula: r.cedula,
                    nombre: r.nombre,
                    segundo_nombre: r.segundo_nombre,
                    apellido: r.apellido,
                    segundo_apellido: r.segundo_apellido,
                    empresa_id: r.empresa_id,
                    fecha_vencimiento_praind: r.fecha_vencimiento_praind,
                    estado,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                empresa_nombre: r.empresa_nombre.unwrap_or_default(),
                vehiculo_tipo: r.tipo_vehiculo,
                vehiculo_placa: r.placa,
                vehiculo_marca: r.marca,
                vehiculo_modelo: r.modelo,
                vehiculo_color: r.color,
                is_blocked: r.is_blocked,
            }))
        }
        None => Ok(None),
    }
}

/// Obtiene todos los contratistas con empresa y vehículo
pub async fn find_all_with_empresa(
    pool: &SqlitePool,
) -> sqlx::Result<Vec<(Contratista, String, Option<String>, Option<String>, bool)>> {
    let rows = sqlx::query!(
        r#"
        SELECT c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
               c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
               e.nombre as "empresa_nombre?", 
               v.tipo_vehiculo as "tipo_vehiculo?", 
               v.placa as "placa?", 
               (ln.id IS NOT NULL) as "is_blocked: bool"
        FROM contratistas c
        LEFT JOIN empresas e ON c.empresa_id = e.id
        LEFT JOIN vehiculos v ON c.id = v.contratista_id
        LEFT JOIN lista_negra ln ON c.cedula = ln.cedula AND ln.is_active = 1
        ORDER BY c.updated_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let result: Vec<_> = rows
        .into_iter()
        .map(|r| {
            let estado = r.estado.parse().unwrap_or(EstadoContratista::Inactivo);
            (
                Contratista {
                    id: r.id,
                    cedula: r.cedula,
                    nombre: r.nombre,
                    segundo_nombre: r.segundo_nombre,
                    apellido: r.apellido,
                    segundo_apellido: r.segundo_apellido,
                    empresa_id: r.empresa_id,
                    fecha_vencimiento_praind: r.fecha_vencimiento_praind,
                    estado,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                r.empresa_nombre.unwrap_or_default(),
                r.tipo_vehiculo,
                r.placa,
                r.is_blocked,
            )
        })
        .collect();

    Ok(result)
}

/// Obtiene contratistas activos
pub async fn find_activos_with_empresa(
    pool: &SqlitePool,
) -> sqlx::Result<Vec<(Contratista, String, Option<String>, Option<String>, bool)>> {
    let rows = sqlx::query!(
        r#"
        SELECT c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
               c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
               e.nombre as "empresa_nombre?", 
               v.tipo_vehiculo as "tipo_vehiculo?", 
               v.placa as "placa?", 
               (ln.id IS NOT NULL) as "is_blocked: bool"
        FROM contratistas c
        LEFT JOIN empresas e ON c.empresa_id = e.id
        LEFT JOIN vehiculos v ON c.id = v.contratista_id
        LEFT JOIN lista_negra ln ON c.cedula = ln.cedula AND ln.is_active = 1
        WHERE c.estado = 'activo'
        ORDER BY c.nombre ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    let result: Vec<_> = rows
        .into_iter()
        .map(|r| {
            let estado = r.estado.parse().unwrap_or(EstadoContratista::Inactivo);
            (
                Contratista {
                    id: r.id,
                    cedula: r.cedula,
                    nombre: r.nombre,
                    segundo_nombre: r.segundo_nombre,
                    apellido: r.apellido,
                    segundo_apellido: r.segundo_apellido,
                    empresa_id: r.empresa_id,
                    fecha_vencimiento_praind: r.fecha_vencimiento_praind,
                    estado,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                r.empresa_nombre.unwrap_or_default(),
                r.tipo_vehiculo,
                r.placa,
                r.is_blocked,
            )
        })
        .collect();

    Ok(result)
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo contratista
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    cedula: &str,
    nombre: &str,
    segundo_nombre: Option<&str>,
    apellido: &str,
    segundo_apellido: Option<&str>,
    empresa_id: &str,
    fecha_vencimiento_praind: &str,
    estado: &str,
    created_at: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO contratistas 
        (id, cedula, nombre, segundo_nombre, apellido, segundo_apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        cedula,
        nombre,
        segundo_nombre,
        apellido,
        segundo_apellido,
        empresa_id,
        fecha_vencimiento_praind,
        estado,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza un contratista
#[allow(clippy::too_many_arguments)]
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    nombre: Option<&str>,
    segundo_nombre: Option<&str>,
    apellido: Option<&str>,
    segundo_apellido: Option<&str>,
    empresa_id: Option<&str>,
    fecha_vencimiento_praind: Option<&str>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        UPDATE contratistas SET
            nombre = COALESCE(?, nombre),
            segundo_nombre = COALESCE(?, segundo_nombre),
            apellido = COALESCE(?, apellido),
            segundo_apellido = COALESCE(?, segundo_apellido),
            empresa_id = COALESCE(?, empresa_id),
            fecha_vencimiento_praind = COALESCE(?, fecha_vencimiento_praind),
            updated_at = ?
        WHERE id = ?
        "#,
        nombre,
        segundo_nombre,
        apellido,
        segundo_apellido,
        empresa_id,
        fecha_vencimiento_praind,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza el estado de un contratista
pub async fn update_estado(
    pool: &SqlitePool,
    id: &str,
    estado: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE contratistas SET estado = ?, updated_at = ? WHERE id = ?",
        estado,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina un contratista
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM contratistas WHERE id = ?", id)
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
            "migrations/2_create_contratista.sql",
            "migrations/4_create_vehiculo.sql",
            "migrations/3_create_lista_negra.sql",
        ];

        for path in schemas {
            let sql = std::fs::read_to_string(path).unwrap();
            pool.execute(sql.as_str()).await.unwrap();
        }

        // Seed data
        pool.execute("INSERT INTO empresas (id, nombre, created_at, updated_at) VALUES ('e-1', 'Test Corp', '2025-01-01', '2025-01-01')").await.unwrap();

        pool.execute("INSERT INTO contratistas (id, cedula, nombre, apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
                      VALUES ('c-1', '12345', 'Juan', 'Perez', 'e-1', '2030-01-01', 'activo', '2025-01-01', '2025-01-01')").await.unwrap();

        pool.execute("INSERT INTO vehiculos (id, contratista_id, tipo_vehiculo, placa, marca, modelo, color, created_at, updated_at)
                      VALUES ('v-1', 'c-1', 'automovil', 'ABC-123', 'Toyota', 'Corolla', 'Blanco', '2025-01-01', '2025-01-01')").await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_find_basic_info_by_id() {
        let pool = setup_test_env().await;
        let res = find_basic_info_by_id(&pool, "c-1").await.unwrap().unwrap();
        assert_eq!(res.cedula, "12345");
        assert_eq!(res.empresa_nombre, "Test Corp");
    }

    #[tokio::test]
    async fn test_find_by_id_with_empresa() {
        let pool = setup_test_env().await;
        let res = find_by_id_with_empresa(&pool, "c-1")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res.empresa_nombre, "Test Corp");
        assert_eq!(res.vehiculo_placa, Some("ABC-123".to_string()));
        assert!(!res.is_blocked);
    }

    #[tokio::test]
    async fn test_find_by_id_with_empresa_blocked() {
        let pool = setup_test_env().await;
        pool.execute("INSERT INTO lista_negra (id, cedula, nombre, apellido, motivo_bloqueo, fecha_inicio_bloqueo, bloqueado_por, is_active, created_at, updated_at) 
                      VALUES ('bl-1', '12345', 'Juan', 'Perez', 'Robo', '2025-01-01', 'Admin', 1, '2025-01-01', '2025-01-01')").await.unwrap();

        let res = find_by_id_with_empresa(&pool, "c-1")
            .await
            .unwrap()
            .unwrap();
        assert!(res.is_blocked);
    }

    #[tokio::test]
    async fn test_crud_contratista() {
        let pool = setup_test_env().await;

        // Insert
        insert(
            &pool,
            "c-2",
            "999",
            "Maria",
            None,
            "Lopez",
            None,
            "e-1",
            "2029-01-01",
            "activo",
            "now",
            "now",
        )
        .await
        .unwrap();

        let c = find_basic_info_by_id(&pool, "c-2").await.unwrap().unwrap();
        assert_eq!(c.nombre, "Maria");

        // Update
        update(
            &pool,
            "c-2",
            Some("Maria Updated"),
            None,
            None,
            None,
            None,
            None,
            "new-now",
        )
        .await
        .unwrap();
        let c2 = find_basic_info_by_id(&pool, "c-2").await.unwrap().unwrap();
        assert_eq!(c2.nombre, "Maria Updated");

        // Delete
        delete(&pool, "c-2").await.unwrap();
        let c3 = find_basic_info_by_id(&pool, "c-2").await.unwrap();
        assert!(c3.is_none());
    }
}
