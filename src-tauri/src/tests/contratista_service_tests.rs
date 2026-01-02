use crate::models::contratista::{
    Contratista, ContratistaCreateDTO, ContratistaFetched, ContratistaUpdateDTO,
    CreateContratistaInput, EstadoContratista,
};
use crate::models::empresa::Empresa;
use crate::models::lista_negra::BlockStatus;
use crate::models::vehiculo::Vehiculo;
use crate::repositories::traits::{
    AuditRepository, ContratistaRepository, EmpresaRepository, SecurityRepository,
    VehiculoRepository,
};
use crate::services::contratista_service::ContratistaService;
// use crate::services::search_service::SearchService; // Not needed if we pass None
use crate::services::surrealdb_service::SurrealDbError;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use surrealdb::RecordId;

// ================================================================
// MOCKS
// ================================================================

struct MockContratistaRepository {
    pub contratistas: Arc<Mutex<Vec<ContratistaFetched>>>,
}

#[async_trait]
impl ContratistaRepository for MockContratistaRepository {
    async fn find_by_cedula(&self, cedula: &str) -> Result<Option<Contratista>, SurrealDbError> {
        let list = self.contratistas.lock().unwrap();
        let found = list.iter().find(|c| c.cedula == cedula).cloned();
        Ok(found.map(|c| c.into()))
    }

    async fn find_by_cedula_fetched(
        &self,
        cedula: &str,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        let list = self.contratistas.lock().unwrap();
        Ok(list.iter().find(|c| c.cedula == cedula).cloned())
    }

    async fn create(
        &self,
        dto: ContratistaCreateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        let mut list = self.contratistas.lock().unwrap();
        let new_c = ContratistaFetched {
            id: RecordId::from(("contratista", "mock_id")),
            cedula: dto.cedula,
            nombre: dto.nombre,
            segundo_nombre: dto.segundo_nombre,
            apellido: dto.apellido,
            segundo_apellido: dto.segundo_apellido,
            empresa: Empresa {
                id: dto.empresa,
                nombre: "Empresa Mock".to_string(),
                direccion: None,
                is_active: true,
                created_at: None,
                updated_at: None,
            },
            fecha_vencimiento_praind: dto.fecha_vencimiento_praind,
            estado: dto.estado,
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
            deleted_at: None,
        };
        list.push(new_c.clone());
        Ok(new_c)
    }

    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Contratista>, SurrealDbError> {
        // Just verify ID format/existence if needed
        let list = self.contratistas.lock().unwrap();
        if id.key().to_string() == "mock_id" || !list.is_empty() {
            // Return first for simplicity in update test unless specific
            let found = list.first().cloned();
            Ok(found.map(|c| c.into()))
        } else {
            Ok(None)
        }
    }

    async fn find_by_id_fetched(
        &self,
        _id: &RecordId,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        Ok(None)
    }
    async fn find_all_fetched(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        Ok(vec![])
    }
    async fn update(
        &self,
        _id: &RecordId,
        _dto: ContratistaUpdateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        // Return a dummy updated object
        Ok(ContratistaFetched {
            id: RecordId::from(("contratista", "mock_id")),
            cedula: "123".to_string(),
            nombre: "Updated".to_string(),
            apellido: "Updated".to_string(),
            segundo_nombre: None,
            segundo_apellido: None,
            empresa: Empresa {
                id: RecordId::from(("empresa", "mock_id")),
                nombre: "Empresa Update".to_string(),
                direccion: None,
                is_active: true,
                created_at: None,
                updated_at: None,
            },
            fecha_vencimiento_praind: chrono::Utc::now().into(),
            estado: EstadoContratista::Activo,
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
            deleted_at: None,
        })
    }
    async fn update_status(
        &self,
        _id: &RecordId,
        _status: EstadoContratista,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        Err(SurrealDbError::Query("Not Implemented in Mock".into()))
    }
    async fn delete(&self, _id: &RecordId) -> Result<(), SurrealDbError> {
        Ok(())
    }
    async fn restore(&self, _id: &RecordId) -> Result<(), SurrealDbError> {
        Ok(())
    }
    async fn find_archived(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        Ok(vec![])
    }
}

struct MockSecurityRepository {
    pub should_block: bool,
}

#[async_trait]
impl SecurityRepository for MockSecurityRepository {
    async fn check_if_blocked_by_cedula(&self, _: &str) -> Result<BlockStatus, SurrealDbError> {
        Ok(BlockStatus {
            is_blocked: self.should_block,
            nivel_severidad: if self.should_block { Some("ALTO".to_string()) } else { None },
            bloqueado_desde: None,
        })
    }
}

struct MockEmpresaRepository {
    pub exists: bool,
}
#[async_trait]
impl EmpresaRepository for MockEmpresaRepository {
    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Empresa>, SurrealDbError> {
        if self.exists {
            Ok(Some(Empresa {
                id: id.clone(),
                nombre: "Empresa Valid".to_string(),
                direccion: None,
                is_active: true,
                created_at: None,
                updated_at: None,
            }))
        } else {
            Ok(None)
        }
    }
}

struct MockVehiculoRepository;
#[async_trait]
impl VehiculoRepository for MockVehiculoRepository {
    async fn find_by_propietario(&self, _id: &RecordId) -> Result<Vec<Vehiculo>, SurrealDbError> {
        Ok(vec![])
    }
}

struct MockAuditRepository;
#[async_trait]
impl AuditRepository for MockAuditRepository {
    async fn insert_praind_historial(
        &self,
        _: &str,
        _: Option<&str>,
        _: &str,
        _: &str,
        _: Option<&str>,
    ) -> Result<(), SurrealDbError> {
        Ok(())
    }
    async fn insert_historial_estado(
        &self,
        _: &str,
        _: &str,
        _: &str,
        _: Option<&str>,
        _: &str,
    ) -> Result<(), SurrealDbError> {
        Ok(())
    }
}

// ================================================================
// TESTS
// ================================================================

#[tokio::test]
async fn test_create_contratista_success() {
    let repo = MockContratistaRepository { contratistas: Arc::new(Mutex::new(vec![])) };
    let sec_repo = MockSecurityRepository { should_block: false };
    let emp_repo = MockEmpresaRepository { exists: true };
    let veh_repo = MockVehiculoRepository;
    let audit_repo = MockAuditRepository;

    let service = ContratistaService::new(repo, sec_repo, emp_repo, veh_repo, audit_repo, None);

    let input = CreateContratistaInput {
        cedula: "0999999999".to_string(), // Valid RUC format for test normalization? Assuming 09... works or simple
        nombre: "Test".to_string(),
        apellido: "User".to_string(),
        segundo_nombre: None,
        segundo_apellido: None,
        empresa_id: "empresa:123".to_string(),
        fecha_vencimiento_praind: "2030-01-01".to_string(), // Future date
        tiene_vehiculo: Some(false),
        tipo_vehiculo: None,
        placa: None,
        marca: None,
        modelo: None,
        color: None,
    };

    let result = service.create_contratista(input).await;
    assert!(result.is_ok(), "Creation should succeed");

    let response = result.unwrap();
    assert_eq!(response.cedula, "0999999999");
    assert_eq!(response.nombre, "Test");
}

#[tokio::test]
async fn test_create_contratista_blocked_security() {
    let repo = MockContratistaRepository { contratistas: Arc::new(Mutex::new(vec![])) };
    let sec_repo = MockSecurityRepository { should_block: true }; // BLOCKED
    let emp_repo = MockEmpresaRepository { exists: true };
    let veh_repo = MockVehiculoRepository;
    let audit_repo = MockAuditRepository;

    let service = ContratistaService::new(repo, sec_repo, emp_repo, veh_repo, audit_repo, None);

    let input = CreateContratistaInput {
        cedula: "0999999999".to_string(),
        nombre: "Test".to_string(),
        apellido: "Blocked".to_string(),
        segundo_nombre: None,
        segundo_apellido: None,
        empresa_id: "empresa:123".to_string(),
        fecha_vencimiento_praind: "2030-01-01".to_string(),
        tiene_vehiculo: None,
        tipo_vehiculo: None,
        placa: None,
        marca: None,
        modelo: None,
        color: None,
    };

    let result = service.create_contratista(input).await;
    assert!(result.is_err());

    // We expect Validation error due to security block
    match result.unwrap_err() {
        crate::domain::errors::ContratistaError::Validation(msg) => {
            assert!(msg.contains("BLOQUEO DE SEGURIDAD"));
        }
        _ => panic!("Expected Validation error"),
    }
}
