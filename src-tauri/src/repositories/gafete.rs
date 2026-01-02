use crate::repositories::traits::GafeteRepository;
use crate::services::gafete_service;
use crate::services::surrealdb_service::SurrealDbError;
use async_trait::async_trait;

pub struct SurrealGafeteRepository;

#[async_trait]
impl GafeteRepository for SurrealGafeteRepository {
    async fn is_disponible(&self, numero: i32, tipo: &str) -> Result<bool, SurrealDbError> {
        gafete_service::is_gafete_disponible(numero, tipo)
            .await
            .map_err(|e| SurrealDbError::Query(e.to_string()))
    }

    async fn marcar_en_uso(&self, numero: i32, tipo: &str) -> Result<(), SurrealDbError> {
        gafete_service::marcar_en_uso(numero, tipo)
            .await
            .map_err(|e| SurrealDbError::Query(e.to_string()))?;
        Ok(())
    }

    async fn liberar(&self, numero: i32, tipo: &str) -> Result<(), SurrealDbError> {
        gafete_service::liberar_gafete(numero, tipo)
            .await
            .map_err(|e| SurrealDbError::Query(e.to_string()))?;
        Ok(())
    }
}
