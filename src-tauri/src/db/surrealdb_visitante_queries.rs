//! # Persistencia: Visitantes (`SurrealDB`)
//!
//! Este módulo implementa el acceso directo a la base de datos para la gestión
//! de visitantes, utilizando el driver nativo de `SurrealDB`.

use crate::models::visitante::{
    Visitante, VisitanteCreateDTO, VisitanteFetched, VisitanteUpdateDTO,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::{debug, warn};
use surrealdb::RecordId;

/// Crea un nuevo registro de visitante.
pub async fn create_visitante(dto: VisitanteCreateDTO) -> Result<Visitante, SurrealDbError> {
    let db = get_db().await?;

    debug!("DB: Creando nuevo visitante: {}", dto.cedula);
    let res: Option<Visitante> = db
        .query(
            r"
            CREATE visitante CONTENT $dto
        ",
        )
        .bind(("dto", dto))
        .await?
        .take(0)?;

    res.ok_or_else(|| {
        warn!("DB: Fallo al insertar visitante tras ejecución de query");
        SurrealDbError::TransactionError("Error al crear visitante".to_string())
    })
}

/// Busca un visitante por su ID interno.
pub async fn find_by_id(id: &RecordId) -> Result<Option<Visitante>, SurrealDbError> {
    let db = get_db().await?;
    debug!("DB: Consultando visitante por ID: {id}");
    let result: Option<Visitante> = db.select(id.clone()).await?;
    Ok(result)
}

/// Localiza un visitante activo por su número de cédula.
pub async fn get_visitante_by_cedula(cedula: &str) -> Result<Option<Visitante>, SurrealDbError> {
    let db = get_db().await?;
    debug!("DB: Buscando por cédula: {cedula}");
    let mut result = db
        .query("SELECT * FROM visitante WHERE cedula = $cedula AND deleted_at IS NONE")
        .bind(("cedula", cedula.to_string()))
        .await?;
    Ok(result.take(0)?)
}

/// Obtiene la ficha técnica del visitante hidratando su relación con empresa.
pub async fn find_by_id_fetched(id: &RecordId) -> Result<Option<VisitanteFetched>, SurrealDbError> {
    let db = get_db().await?;
    debug!("DB: Consultando visitante hidratado (FETCH) ID: {id}");
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

/// Motor de búsqueda flexible por cédula, nombre o apellido.
pub async fn search_visitantes(term: &str) -> Result<Vec<VisitanteFetched>, SurrealDbError> {
    let db = get_db().await?;
    let term_upper = term.to_uppercase();
    debug!("DB: Búsqueda de visitantes por término: {term_upper}");
    let mut result = db
        .query(
            r"
            SELECT * FROM visitante 
            WHERE 
                (string::uppercase(cedula) CONTAINS $term OR 
                string::uppercase(nombre) CONTAINS $term OR 
                string::uppercase(apellido) CONTAINS $term) 
                AND deleted_at IS NONE 
            FETCH empresa
        ",
        )
        .bind(("term", term_upper))
        .await?;
    Ok(result.take(0)?)
}

/// Actualiza parcialmente los datos de un visitante.
pub async fn update(id: &RecordId, dto: VisitanteUpdateDTO) -> Result<Visitante, SurrealDbError> {
    let db = get_db().await?;
    debug!("DB: Actualizando visitante: {id}");

    let result: Option<Visitante> = db.update(id.clone()).merge(dto).await?;

    result.ok_or_else(|| {
        warn!("DB: Intento de actualización fallido para visitante ID: {id}");
        SurrealDbError::Query("Visitante no encontrado o error al actualizar".to_string())
    })
}

/// Archiva un visitante (borrado lógico).
pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    debug!("DB: Archivando visitante (deleted_at): {id}");
    let _: Option<Visitante> = db
        .query("UPDATE $id SET deleted_at = time::now()")
        .bind(("id", id.clone()))
        .await?
        .take(0)?;
    Ok(())
}

/// Restaura un visitante previamente archivado.
pub async fn restore(id: &RecordId) -> Result<Visitante, SurrealDbError> {
    let db = get_db().await?;
    debug!("DB: Restaurando visitante archivado: {id}");
    let res: Option<Visitante> =
        db.query("UPDATE $id SET deleted_at = NONE").bind(("id", id.clone())).await?.take(0)?;

    res.ok_or_else(|| {
        warn!("DB: No se pudo restaurar el visitante ID: {id}");
        SurrealDbError::Query("Error restaurando visitante".to_string())
    })
}

pub async fn find_archived() -> Result<Vec<VisitanteFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM visitante WHERE deleted_at IS NOT NONE ORDER BY deleted_at DESC FETCH empresa")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all() -> Result<Vec<VisitanteFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM visitante WHERE deleted_at IS NONE ORDER BY created_at DESC FETCH empresa")
        .await?;
    Ok(result.take(0)?)
}
