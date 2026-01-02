use super::traits::{
    AuditRepository, ContratistaRepository, EmpresaRepository, SecurityRepository,
    VehiculoRepository,
};
use crate::models::contratista::{
    Contratista, ContratistaCreateDTO, ContratistaFetched, ContratistaUpdateDTO, EstadoContratista,
};
use crate::models::empresa::Empresa;
use crate::models::lista_negra::BlockStatus;
use crate::models::vehiculo::Vehiculo;
use crate::services::surrealdb_service::SurrealDbError;
use async_trait::async_trait;
use surrealdb::RecordId;

// Importaciones de Queries existentes
use crate::db::surrealdb_audit_queries as audit_db;
use crate::db::surrealdb_contratista_queries as db;
use crate::db::surrealdb_empresa_queries as empresa_db;
use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_vehiculo_queries as veh_db;

// ================================================================
// Contratista Repository Implementation
// ================================================================

pub struct SurrealContratistaRepository;

#[async_trait]
impl ContratistaRepository for SurrealContratistaRepository {
    async fn find_by_cedula(&self, cedula: &str) -> Result<Option<Contratista>, SurrealDbError> {
        // En teoria db::find_by_cedula devuelve ContratistaFetched, asi que mapeamos a Contratista
        let fetched = db::find_by_cedula(cedula).await?;
        Ok(fetched.map(std::convert::Into::into))
    }

    async fn find_by_cedula_fetched(
        &self,
        cedula: &str,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        db::find_by_cedula(cedula).await
    }

    async fn create(
        &self,
        dto: ContratistaCreateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        // En un refactor futuro, esto debería usar db::create puro, pero el metodo create actual
        // hace internamente un FETCH. Lo mantenemos compatible.
        db::create(dto).await
    }

    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Contratista>, SurrealDbError> {
        db::find_by_id(id).await
    }

    async fn find_by_id_fetched(
        &self,
        id: &RecordId,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        db::find_by_id_fetched(id).await
    }

    async fn find_all_fetched(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        db::find_all_fetched().await
    }

    async fn update(
        &self,
        id: &RecordId,
        dto: ContratistaUpdateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        db::update(id, dto).await
    }

    async fn update_status(
        &self,
        id: &RecordId,
        status: EstadoContratista,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        db::update_status(id, status).await
    }

    async fn delete(&self, id: &RecordId) -> Result<(), SurrealDbError> {
        db::delete(id).await
    }

    async fn restore(&self, id: &RecordId) -> Result<(), SurrealDbError> {
        db::restore(id).await
    }

    async fn find_archived(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        db::find_archived().await
    }
}

// ================================================================
// Auxiliary Repositories Implementations
// ================================================================

pub struct SurrealSecurityRepository;

#[async_trait]
impl SecurityRepository for SurrealSecurityRepository {
    async fn check_if_blocked_by_cedula(
        &self,
        cedula: &str,
    ) -> Result<BlockStatus, SurrealDbError> {
        ln_db::check_if_blocked_by_cedula(cedula).await
    }
}

pub struct SurrealEmpresaRepository;

#[async_trait]
impl EmpresaRepository for SurrealEmpresaRepository {
    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Empresa>, SurrealDbError> {
        empresa_db::find_by_id(id).await
    }
}

pub struct SurrealVehiculoRepository;

#[async_trait]
impl VehiculoRepository for SurrealVehiculoRepository {
    async fn find_by_propietario(
        &self,
        propietario_id: &RecordId,
    ) -> Result<Vec<Vehiculo>, SurrealDbError> {
        veh_db::find_by_propietario(propietario_id).await
    }
}

pub struct SurrealAuditRepository;

#[async_trait]
impl AuditRepository for SurrealAuditRepository {
    async fn insert_praind_historial(
        &self,
        contratista_id: &str,
        fecha_anterior: Option<&str>,
        nueva_fecha: &str,
        usuario_id: &str,
        motivo: Option<&str>,
    ) -> Result<(), SurrealDbError> {
        audit_db::insert_praind_historial(
            contratista_id,
            fecha_anterior,
            nueva_fecha,
            usuario_id,
            motivo,
        )
        .await
    }

    async fn insert_historial_estado(
        &self,
        contratista_id: &str,
        estado_anterior: &str,
        nuevo_estado: &str,
        usuario_id: Option<&str>,
        motivo: &str,
    ) -> Result<(), SurrealDbError> {
        audit_db::insert_historial_estado(
            contratista_id,
            estado_anterior,
            nuevo_estado,
            usuario_id,
            motivo,
        )
        .await
    }
}
