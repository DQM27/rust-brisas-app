use crate::models::ingreso::{
    IngresoContratista, IngresoContratistaCreateDTO, IngresoContratistaFetched,
};
use crate::repositories::traits::IngresoContratistaRepository;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use async_trait::async_trait;
use surrealdb::RecordId;

pub struct SurrealIngresoContratistaRepository;

const TABLE: &str = "ingreso_contratista";

#[async_trait]
impl IngresoContratistaRepository for SurrealIngresoContratistaRepository {
    async fn insert(
        &self,
        dto: IngresoContratistaCreateDTO,
    ) -> Result<IngresoContratistaFetched, SurrealDbError> {
        let db = get_db().await?;

        // CREATE doesn't support FETCH, so we need two queries
        let created: Option<IngresoContratista> =
            db.query(format!("CREATE {TABLE} CONTENT $dto")).bind(("dto", dto)).await?.take(0)?;

        let ingreso = created.ok_or(SurrealDbError::TransactionError(
            "Error al insertar ingreso_contratista".to_string(),
        ))?;

        // Fetch with all relations
        let mut result = db
            .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
            .bind(("id", ingreso.id.clone()))
            .await?;

        let fetched: Option<IngresoContratistaFetched> = result.take(0)?;

        match fetched {
            Some(f) => Ok(f),
            None => Err(SurrealDbError::TransactionError(
                "Ingreso creado pero no se pudo obtener con FETCH".to_string(),
            )),
        }
    }

    async fn find_ingreso_abierto_by_contratista(
        &self,
        contratista_id: &RecordId,
    ) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
        let db = get_db().await?;

        let mut result = db
            .query(format!(
                "SELECT * FROM {TABLE} WHERE contratista = $contratista AND fecha_hora_salida IS NONE LIMIT 1 FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa"
            ))
            .bind(("contratista", contratista_id.clone()))
            .await?;

        Ok(result.take(0)?)
    }

    async fn update_salida(
        &self,
        ingreso_id: &RecordId,
        usuario_salida_id: &RecordId,
        observaciones: Option<String>,
    ) -> Result<IngresoContratistaFetched, SurrealDbError> {
        let db = get_db().await?;

        let mut dto = crate::models::ingreso::IngresoContratistaUpdateDTO::default();
        dto.fecha_hora_salida = Some(surrealdb::Datetime::from(chrono::Utc::now()));
        dto.usuario_salida = Some(usuario_salida_id.clone());
        dto.observaciones = observaciones;

        // UPDATE doesn't support FETCH, so we need two queries
        let _: Option<IngresoContratista> = db
            .query("UPDATE $id MERGE $dto")
            .bind(("id", ingreso_id.clone()))
            .bind(("dto", dto))
            .await?
            .take(0)?;

        // Fetch with all relations
        let mut result = db
            .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
            .bind(("id", ingreso_id.clone()))
            .await?;

        let fetched: Option<IngresoContratistaFetched> = result.take(0)?;

        match fetched {
            Some(f) => Ok(f),
            None => Err(SurrealDbError::TransactionError("Error al registrar salida".to_string())),
        }
    }

    async fn find_by_id(
        &self,
        id: &RecordId,
    ) -> Result<Option<IngresoContratista>, SurrealDbError> {
        let db = get_db().await?;
        let result: Option<IngresoContratista> = db.select(id.clone()).await?;
        Ok(result)
    }

    async fn find_by_id_fetched(
        &self,
        id: &RecordId,
    ) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
        let db = get_db().await?;
        let mut result = db
            .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
            .bind(("id", id.clone()))
            .await?;
        Ok(result.take(0)?)
    }

    async fn find_all_abiertos_fetched(
        &self,
    ) -> Result<Vec<IngresoContratistaFetched>, SurrealDbError> {
        let db = get_db().await?;
        let mut result = db
            .query(format!(
                "SELECT * FROM {TABLE} WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa"
            ))
            .await?;
        Ok(result.take(0)?)
    }

    async fn find_salidas_en_rango_fetched(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Vec<IngresoContratistaFetched>, SurrealDbError> {
        let db = get_db().await?;
        log::debug!("Repo: Querying salidas from {} to {}", start, end);
        let mut result = db
            .query(format!(
                "SELECT * FROM {TABLE} WHERE fecha_hora_salida >= type::datetime($start) AND fecha_hora_salida <= type::datetime($end) ORDER BY fecha_hora_salida DESC FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa"
            ))
            .bind(("start", start.to_string()))
            .bind(("end", end.to_string()))
            .await?;

        let fetched: Vec<IngresoContratistaFetched> = result.take(0)?;
        log::debug!("Repo: Fetched {} raw records from DB", fetched.len());
        Ok(fetched)
    }
}
