// ==========================================
// src/db/vehiculo_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query_as! para validación y DTO intermedio para parseo seguro

use crate::models::vehiculo::Vehiculo;
use sqlx::SqlitePool;
use std::convert::TryFrom;

// ==========================================
// DTO INTERMEDIO PARA MAPEO SEGURO
// ==========================================

#[derive(sqlx::FromRow)]
struct VehiculoRow {
    id: String,
    contratista_id: Option<String>,
    proveedor_id: Option<String>,
    visitante_id: Option<String>,
    tipo_vehiculo: String,
    placa: String,
    marca: Option<String>,
    modelo: Option<String>,
    color: Option<String>,
    is_active: bool,
    created_at: String,
    updated_at: String,
}

impl TryFrom<VehiculoRow> for Vehiculo {
    type Error = sqlx::Error;

    fn try_from(r: VehiculoRow) -> Result<Self, Self::Error> {
        Ok(Vehiculo {
            id: r.id,
            contratista_id: r.contratista_id,
            proveedor_id: r.proveedor_id,
            visitante_id: r.visitante_id,
            tipo_vehiculo: r.tipo_vehiculo.parse().map_err(|e| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e,
                )))
            })?,
            placa: r.placa,
            marca: r.marca,
            modelo: r.modelo,
            color: r.color,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
    }
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un vehículo por ID con datos del contratista
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<Vehiculo>> {
    let row = sqlx::query_as!(
        VehiculoRow,
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.visitante_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(Vehiculo::try_from(r)?)),
        None => Ok(None),
    }
}

/// Busca un vehículo por placa
pub async fn find_by_placa(pool: &SqlitePool, placa: &str) -> sqlx::Result<Option<Vehiculo>> {
    let row = sqlx::query_as!(
        VehiculoRow,
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.visitante_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.placa = ?"#,
        placa
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(Vehiculo::try_from(r)?)),
        None => Ok(None),
    }
}

/// Obtiene todos los vehículos
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query_as!(
        VehiculoRow,
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.visitante_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           ORDER BY v.created_at DESC"#
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(Vehiculo::try_from).collect()
}

/// Obtiene todos los vehículos activos
pub async fn find_activos(pool: &SqlitePool) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query_as!(
        VehiculoRow,
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.visitante_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.is_active = 1
           ORDER BY v.placa"#
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(Vehiculo::try_from).collect()
}

/// Obtiene vehículos de un contratista
pub async fn find_by_contratista(
    pool: &SqlitePool,
    contratista_id: &str,
) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query_as!(
        VehiculoRow,
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.visitante_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.contratista_id = ?
           ORDER BY v.is_active DESC, v.placa"#,
        contratista_id
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(Vehiculo::try_from).collect()
}

/// Obtiene vehículos de un proveedor
pub async fn find_by_proveedor(
    pool: &SqlitePool,
    proveedor_id: &str,
) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query_as!(
        VehiculoRow,
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.visitante_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.proveedor_id = ?
           ORDER BY v.is_active DESC, v.placa"#,
        proveedor_id
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(Vehiculo::try_from).collect()
}

/// Cuenta vehículos por placa
pub async fn count_by_placa(pool: &SqlitePool, placa: &str) -> sqlx::Result<i64> {
    let row = sqlx::query!("SELECT COUNT(*) as count FROM vehiculos WHERE placa = ?", placa)
        .fetch_one(pool)
        .await?;

    Ok(row.count as i64)
}

/// Cuenta vehículos por placa excluyendo ID
pub async fn count_by_placa_excluding_id(
    pool: &SqlitePool,
    placa: &str,
    exclude_id: &str,
) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM vehiculos WHERE placa = ? AND id != ?",
        placa,
        exclude_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Verifica si un contratista existe
pub async fn contratista_exists(pool: &SqlitePool, contratista_id: &str) -> sqlx::Result<bool> {
    let row =
        sqlx::query!("SELECT COUNT(*) as count FROM contratistas WHERE id = ?", contratista_id)
            .fetch_one(pool)
            .await?;

    Ok(row.count > 0)
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo vehículo
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    contratista_id: Option<&str>,
    proveedor_id: Option<&str>,
    tipo_vehiculo: &str,
    placa: &str,
    marca: Option<&str>,
    modelo: Option<&str>,
    color: Option<&str>,
    created_at: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO vehiculos 
           (id, contratista_id, proveedor_id, tipo_vehiculo, placa, marca, modelo, color, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
        id,
        contratista_id,
        proveedor_id,
        tipo_vehiculo,
        placa,
        marca,
        modelo,
        color,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza un vehículo existente
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    tipo_vehiculo: Option<&str>,
    marca: Option<&str>,
    modelo: Option<&str>,
    color: Option<&str>,
    is_active: Option<bool>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE vehiculos SET
            tipo_vehiculo = COALESCE(?, tipo_vehiculo),
            marca = COALESCE(?, marca),
            modelo = COALESCE(?, modelo),
            color = COALESCE(?, color),
            is_active = COALESCE(?, is_active),
            updated_at = ?
        WHERE id = ?"#,
        tipo_vehiculo,
        marca,
        modelo,
        color,
        is_active,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina un vehículo
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM vehiculos WHERE id = ?", id).execute(pool).await?;

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

        let schemas =
            vec!["migrations/2_create_contratista.sql", "migrations/4_create_vehiculo.sql"];

        for path in schemas {
            let sql = std::fs::read_to_string(path).unwrap();
            pool.execute(sql.as_str()).await.unwrap();
        }

        // Seed
        pool.execute("INSERT INTO contratistas (id, cedula, nombre, apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at) 
                      VALUES ('c-1', '12345', 'Juan', 'Perez', 'e-1', '2030-01-01', 'activo', '2025-01-01', '2025-01-01')").await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_crud_vehiculo() {
        let pool = setup_test_env().await;

        // 1. Insert
        insert(
            &pool,
            "v-1",
            Some("c-1"),
            None,
            "automovil",
            "ABC-123",
            Some("Toyota"),
            Some("Corolla"),
            Some("Blanco"),
            "2025-01-01",
            "2025-01-01",
        )
        .await
        .unwrap();

        // 2. Find by ID
        let v = find_by_id(&pool, "v-1").await.unwrap().unwrap();
        assert_eq!(v.placa, "ABC-123");
        assert_eq!(v.marca, Some("Toyota".to_string()));

        // 3. Find by Placa
        let v2 = find_by_placa(&pool, "ABC-123").await.unwrap().unwrap();
        assert_eq!(v2.id, "v-1");

        // 4. Update
        update(&pool, "v-1", None, None, None, Some("Negro"), Some(false), "2025-01-02")
            .await
            .unwrap();
        let v3 = find_by_id(&pool, "v-1").await.unwrap().unwrap();
        assert_eq!(v3.color, Some("Negro".to_string()));
        assert!(!v3.is_active);

        // 5. Delete
        delete(&pool, "v-1").await.unwrap();
        let v4 = find_by_id(&pool, "v-1").await.unwrap();
        assert!(v4.is_none());
    }

    #[tokio::test]
    async fn test_find_by_contratista() {
        let pool = setup_test_env().await;
        insert(
            &pool,
            "v-1",
            Some("c-1"),
            None,
            "motocicleta",
            "XYZ-789",
            None,
            None,
            None,
            "2025-01-01",
            "2025-01-01",
        )
        .await
        .unwrap();

        let list = find_by_contratista(&pool, "c-1").await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].placa, "XYZ-789");
    }

    #[tokio::test]
    async fn test_find_activos() {
        let pool = setup_test_env().await;
        insert(&pool, "v-1", Some("c-1"), None, "automovil", "A1", None, None, None, "now", "now")
            .await
            .unwrap();
        insert(&pool, "v-2", Some("c-1"), None, "automovil", "A2", None, None, None, "now", "now")
            .await
            .unwrap();

        update(&pool, "v-2", None, None, None, None, Some(false), "now").await.unwrap();

        let activos = find_activos(&pool).await.unwrap();
        assert_eq!(activos.len(), 1);
        assert_eq!(activos[0].placa, "A1");
    }
}
