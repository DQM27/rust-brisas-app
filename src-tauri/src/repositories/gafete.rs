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

    async fn update_status(
        &self,
        numero: i32,
        tipo: &str,
        status: crate::models::gafete::GafeteEstado,
    ) -> Result<(), SurrealDbError> {
        let gafete = gafete_service::get_gafete_by_numero(numero, tipo)
            .await
            .map_err(|e| SurrealDbError::Query(e.to_string()))?
            .ok_or_else(|| SurrealDbError::Query("Gafete no encontrado".to_string()))?;

        gafete_service::update_gafete_status(&gafete.id, status)
            .await
            .map_err(|e| SurrealDbError::Query(e.to_string()))?;
        Ok(())
    }
}
