//! # Queries `SurrealDB`: Contratistas
//!
//! Operaciones de base de datos para gestión de contratistas externos.
//!
//! ## Responsabilidades
//! - CRUD completo de registros de contratistas
//! - Consultas con FETCH para relaciones (empresa)
//! - Soft delete con campo `deleted_at`
//!
//! ## Tabla: `contratista`

use crate::models::contratista::{
    Contratista, ContratistaCreateDTO, ContratistaFetched, ContratistaUpdateDTO,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

/// Crea un nuevo contratista en la base de datos.
pub async fn create(dto: ContratistaCreateDTO) -> Result<ContratistaFetched, SurrealDbError> {
    let db = get_db().await?;

    // CREATE doesn't support FETCH, so we need two queries:
    // 1. Create the record and get the raw result
    let created: Option<Contratista> =
        db.query("CREATE contratista CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    let contratista =
        created.ok_or(SurrealDbError::Query("No se pudo crear el contratista".to_string()))?;

    // 2. Fetch the created record with empresa populated
    let mut result =
        db.query("SELECT * FROM $id FETCH empresa").bind(("id", contratista.id.clone())).await?;

    let fetched: Option<ContratistaFetched> = result.take(0)?;
    match fetched {
        Some(f) => Ok(f),
        None => Err(SurrealDbError::Query(
            "Contratista creado pero no se pudo obtener con FETCH".to_string(),
        )),
    }
}

/// Busca un contratista por su ID (sin relaciones).
///
/// ## Query
/// ```sql
/// SELECT * FROM $id
/// ```
///
/// ## Parámetros
/// * `id` - `RecordId` del contratista
///
/// ## Retorno
/// * `Ok(Some(Contratista))` - Contratista encontrado
/// * `Ok(None)` - No existe contratista con ese ID
pub async fn find_by_id(id: &RecordId) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Contratista> = db.select(id.clone()).await?;
    Ok(result)
}

/// Busca un contratista por ID con empresa expandida.
///
/// ## Query
/// ```sql
/// SELECT * FROM $id FETCH empresa
/// ```
///
/// ## Uso de FETCH
/// Popula `empresa` para evitar query adicional al mostrar detalles.
///
/// ## Parámetros
/// * `id` - `RecordId` del contratista
///
/// ## Retorno
/// * `Ok(Some(ContratistaFetched))` - Con empresa populated
/// * `Ok(None)` - No existe
pub async fn find_by_id_fetched(
    id: &RecordId,
) -> Result<Option<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

/// Busca un contratista por cédula (solo activos).
///
/// ## Precondición
/// La cédula debe estar normalizada con `domain::contratista::normalizar_cedula()`.
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista
/// WHERE cedula = $cedula AND deleted_at IS NONE
/// FETCH empresa
/// ```
///
/// ## Soft Delete
/// Excluye registros eliminados (`deleted_at IS NONE`).
///
/// ## Parámetros
/// * `cedula` - Cédula normalizada (ej: "12345678")
///
/// ## Retorno
/// * `Ok(Some(ContratistaFetched))` - Contratista encontrado
/// * `Ok(None)` - No existe contratista activo con esa cédula
pub async fn find_by_cedula(cedula: &str) -> Result<Option<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            "SELECT * FROM contratista WHERE cedula = $cedula AND deleted_at IS NONE FETCH empresa",
        )
        .bind(("cedula", cedula.to_string()))
        .await?;
    let contratista: Option<ContratistaFetched> = result.take(0)?;
    Ok(contratista)
}

/// Obtiene todos los contratistas activos (sin relaciones).
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista
/// WHERE deleted_at IS NONE
/// LIMIT 1000
/// ```
///
/// ## Límite de Seguridad
/// Retorna máximo 1000 registros para protección de memoria.
///
/// ## Soft Delete
/// Excluye registros eliminados.
///
/// ## Retorno
/// * `Ok(Vec<Contratista>)` - Lista de contratistas (máx 1000)
pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Vec<Contratista> =
        db.query("SELECT * FROM contratista WHERE deleted_at IS NONE LIMIT 1000").await?.take(0)?;
    Ok(result)
}

/// Obtiene todos los contratistas activos con empresa expandida.
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista
/// WHERE deleted_at IS NONE
/// FETCH empresa
/// LIMIT 1000
/// ```
///
/// ## Uso de FETCH
/// Popula `empresa` para cada contratista en un solo query,
/// evitando el problema N+1.
///
/// ## Límite de Seguridad
/// Retorna máximo 1000 registros.
///
/// ## Retorno
/// * `Ok(Vec<ContratistaFetched>)` - Lista con empresas populated
pub async fn find_all_fetched() -> Result<Vec<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NONE FETCH empresa LIMIT 1000")
        .await?;
    Ok(result.take(0)?)
}

/// Obtiene contratistas por empresa.
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista
/// WHERE empresa = $empresa AND deleted_at IS NONE
/// LIMIT 500
/// ```
///
/// ## Índice Requerido
/// Este query se beneficia de un índice en `empresa`:
/// ```sql
/// DEFINE INDEX idx_contratista_empresa ON contratista FIELDS empresa;
/// ```
///
/// ## Parámetros
/// * `empresa_id` - `RecordId` de la empresa
///
/// ## Retorno
/// * `Ok(Vec<Contratista>)` - Contratistas de esa empresa (máx 500)
pub async fn find_by_empresa(empresa_id: &RecordId) -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            "SELECT * FROM contratista WHERE empresa = $empresa AND deleted_at IS NONE LIMIT 500",
        )
        .bind(("empresa", empresa_id.clone()))
        .await?;
    let contratistas: Vec<Contratista> = result.take(0)?;
    Ok(contratistas)
}

/// Actualiza datos de un contratista existente.
///
/// ## Operación en 2 Pasos
/// 1. `UPDATE $id MERGE $dto` - Actualiza campos
/// 2. `SELECT * FROM $id FETCH empresa` - Retorna con relación
///
/// `SurrealDB` no soporta `UPDATE ... FETCH` en un solo query.
///
/// ## Parámetros
/// * `id` - `RecordId` del contratista
/// * `dto` - Campos a actualizar (solo los presentes se modifican)
///
/// ## Retorno
/// * `Ok(ContratistaFetched)` - Contratista actualizado con empresa
///
/// ## Errores
/// * `SurrealDbError::Query` - Si el contratista no existe o falla la actualización
pub async fn update(
    id: &RecordId,
    dto: ContratistaUpdateDTO,
) -> Result<ContratistaFetched, SurrealDbError> {
    let db = get_db().await?;

    // 1. Update using native SDK (consistent with User module)
    let _: Option<Contratista> = db.update(id.clone()).merge(dto).await?;

    // 2. Fetch with empresa populated
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;

    let fetched: Option<ContratistaFetched> = result.take(0)?;
    fetched.ok_or_else(|| {
        SurrealDbError::Query(format!("Contratista no encontrado después de UPDATE: {id}"))
    })
}

/// Actualiza solo el estado de un contratista.
///
/// ## Query
/// ```sql
/// UPDATE $id SET estado = $estado
/// ```
///
/// ## Parámetros
/// * `id` - `RecordId` del contratista
/// * `estado` - Nuevo estado (Activo, Inactivo, Bloqueado)
///
/// ## Retorno
/// * `Ok(ContratistaFetched)` - Con estado actualizado
///
/// ## Errores
/// * `SurrealDbError::Query` - Si el contratista no existe
pub async fn update_status(
    id: &RecordId,
    estado: crate::models::contratista::EstadoContratista,
) -> Result<ContratistaFetched, SurrealDbError> {
    let db = get_db().await?;

    // 1. Update status
    let _: Option<Contratista> = db
        .query("UPDATE $id SET estado = $estado")
        .bind(("id", id.clone()))
        .bind(("estado", estado))
        .await?
        .take(0)?;

    // 2. Fetch with empresa populated
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;

    let fetched: Option<ContratistaFetched> = result.take(0)?;
    fetched.ok_or_else(|| {
        SurrealDbError::Query(format!("No se pudo actualizar estado del contratista: {id}"))
    })
}

/// Marca un contratista como eliminado (soft delete).
///
/// ## Query
/// ```sql
/// UPDATE $id SET deleted_at = time::now()
/// ```
///
/// ## Soft Delete
/// No elimina físicamente el registro. Para recuperarlo, usar `restore()`.
/// Los queries normales filtran `deleted_at IS NONE`.
///
/// ## Parámetros
/// * `id` - `RecordId` del contratista a eliminar
///
/// ## Retorno
/// * `Ok(())` - Eliminado exitosamente
pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Contratista> = db
        .query("UPDATE $id SET deleted_at = time::now()")
        .bind(("id", id.clone()))
        .await?
        .take(0)?;
    Ok(())
}

/// Restaura un contratista previamente eliminado.
///
/// ## Query
/// ```sql
/// UPDATE $id SET deleted_at = NONE
/// ```
///
/// ## Restauración
/// Limpia el campo `deleted_at` para que el registro sea visible
/// nuevamente en queries normales.
///
/// ## Parámetros
/// * `id` - `RecordId` del contratista a restaurar
///
/// ## Retorno
/// * `Ok(())` - Restaurado exitosamente
pub async fn restore(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Contratista> =
        db.query("UPDATE $id SET deleted_at = NONE").bind(("id", id.clone())).await?.take(0)?;
    Ok(())
}

/// Obtiene contratistas eliminados (archivados).
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista
/// WHERE deleted_at IS NOT NONE
/// ORDER BY deleted_at DESC
/// FETCH empresa
/// ```
///
/// ## Ordenamiento
/// Ordena por fecha de eliminación descendente (más recientes primero).
///
/// ## Retorno
/// * `Ok(Vec<ContratistaFetched>)` - Contratistas eliminados
pub async fn find_archived() -> Result<Vec<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NOT NONE ORDER BY deleted_at DESC FETCH empresa LIMIT 500")
        .await?;
    Ok(result.take(0)?)
}

/// Obtiene el nombre de una empresa por su ID.
///
/// ## Query
/// ```sql
/// SELECT nombre FROM $id
/// ```
///
/// ## Uso
/// Útil para mostrar el nombre de empresa cuando solo se tiene el `RecordId`.
///
/// ## Parámetros
/// * `empresa_id` - `RecordId` de la empresa
///
/// ## Retorno
/// * `Ok(String)` - Nombre de la empresa, o "Empresa desconocida" si no existe
pub async fn get_empresa_nombre(empresa_id: &RecordId) -> Result<String, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db.query("SELECT nombre FROM $id").bind(("id", empresa_id.clone())).await?;

    #[derive(serde::Deserialize)]
    struct NombreResult {
        nombre: String,
    }

    let res: Option<NombreResult> = result.take(0)?;
    Ok(res.map_or_else(|| "Empresa desconocida".to_string(), |r| r.nombre))
}
