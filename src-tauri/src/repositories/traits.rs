use crate::models::contratista::{
    Contratista, ContratistaCreateDTO, ContratistaFetched, ContratistaUpdateDTO, EstadoContratista,
};
use crate::models::empresa::Empresa;
use crate::models::vehiculo::Vehiculo;
use crate::services::surrealdb_service::SurrealDbError;
use async_trait::async_trait;
use surrealdb::RecordId;

#[async_trait]
pub trait ContratistaRepository: Send + Sync {
    async fn find_by_cedula(&self, cedula: &str) -> Result<Option<Contratista>, SurrealDbError>;
    async fn find_by_cedula_fetched(
        &self,
        cedula: &str,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError>;
    async fn create(&self, dto: ContratistaCreateDTO)
        -> Result<ContratistaFetched, SurrealDbError>;
    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Contratista>, SurrealDbError>;
    async fn find_by_id_fetched(
        &self,
        id: &RecordId,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError>;
    async fn find_all_fetched(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError>;
    async fn update(
        &self,
        id: &RecordId,
        dto: ContratistaUpdateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError>;
    async fn update_status(
        &self,
        id: &RecordId,
        status: EstadoContratista,
    ) -> Result<ContratistaFetched, SurrealDbError>;
    async fn delete(&self, id: &RecordId) -> Result<(), SurrealDbError>;
    async fn restore(&self, id: &RecordId) -> Result<(), SurrealDbError>;
    async fn find_archived(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError>;
}

#[async_trait]
pub trait SecurityRepository: Send + Sync {
    async fn check_if_blocked_by_cedula(
        &self,
        cedula: &str,
    ) -> Result<crate::models::lista_negra::BlockStatus, SurrealDbError>;
}

#[async_trait]
pub trait EmpresaRepository: Send + Sync {
    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Empresa>, SurrealDbError>;
}

#[async_trait]
pub trait VehiculoRepository: Send + Sync {
    async fn find_by_propietario(
        &self,
        propietario_id: &RecordId,
    ) -> Result<Vec<Vehiculo>, SurrealDbError>;
    // Only if needed for update logic, otherwise omitting write methods for now
}

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn insert_praind_historial(
        &self,
        contratista_id: &str,
        fecha_anterior: Option<&str>,
        nueva_fecha: &str,
        usuario_id: &str,
        motivo: Option<&str>,
    ) -> Result<(), SurrealDbError>;

    async fn insert_historial_estado(
        &self,
        contratista_id: &str,
        estado_anterior: &str,
        nuevo_estado: &str,
        usuario_id: Option<&str>,
        motivo: &str,
    ) -> Result<(), SurrealDbError>;
}
