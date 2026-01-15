use async_trait::async_trait;
use brisas_app_lib::domain::errors::ContratistaError;
use brisas_app_lib::models::contratista::{
    Contratista, ContratistaCreateDTO, ContratistaFetched, ContratistaUpdateDTO,
    CreateContratistaInput, EstadoContratista,
};
use brisas_app_lib::models::empresa::Empresa;
use brisas_app_lib::models::lista_negra::BlockStatus;
use brisas_app_lib::models::vehiculo::Vehiculo;
use brisas_app_lib::repositories::traits::{
    AuditRepository, ContratistaRepository, EmpresaRepository, SecurityRepository,
    VehiculoRepository,
};
use brisas_app_lib::services::contratista_service::ContratistaService;
use brisas_app_lib::services::surrealdb_service::SurrealDbError;
use surrealdb::{engine::local::Db, RecordId, Surreal};

mod common;
use common::setup_test_db;

// ============================================================================
// TEST REPOSITORIES (In-Memory implementations using Surreal<Db>)
// ============================================================================

struct TestContratistaRepo {
    db: Surreal<Db>,
}

#[async_trait]
impl ContratistaRepository for TestContratistaRepo {
    async fn find_by_cedula(&self, cedula: &str) -> Result<Option<Contratista>, SurrealDbError> {
        let sql = "SELECT * FROM contratista WHERE cedula = $cedula AND deleted_at IS NONE LIMIT 1";
        let mut result = self
            .db
            .query(sql)
            .bind(("cedula", cedula.to_string()))
            .await
            .map_err(|e: surrealdb::Error| SurrealDbError::Query(e.to_string()))?;
        let item: Option<Contratista> = result.take(0).unwrap_or(None);
        Ok(item)
    }

    async fn find_by_cedula_fetched(
        &self,
        cedula: &str,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        let sql = "SELECT * FROM contratista WHERE cedula = $cedula AND deleted_at IS NONE FETCH empresa LIMIT 1";
        let mut result = self
            .db
            .query(sql)
            .bind(("cedula", cedula.to_string()))
            .await
            .map_err(|e: surrealdb::Error| SurrealDbError::Query(e.to_string()))?;
        let item: Option<ContratistaFetched> = result.take(0).unwrap_or(None);
        Ok(item)
    }

    async fn create(
        &self,
        dto: ContratistaCreateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        // Create
        let sql_create = "CREATE contratista CONTENT $dto";
        let mut res = self
            .db
            .query(sql_create)
            .bind(("dto", dto))
            .await
            .map_err(|e: surrealdb::Error| SurrealDbError::Query(e.to_string()))?;
        let created: Option<Contratista> = res.take(0).unwrap_or(None);
        let c = created.ok_or(SurrealDbError::Query("Failed to create".into()))?;

        // Fetch
        let sql_fetch = "SELECT * FROM $id FETCH empresa";
        let mut res_fetch = self
            .db
            .query(sql_fetch)
            .bind(("id", c.id.clone()))
            .await
            .map_err(|e: surrealdb::Error| SurrealDbError::Query(e.to_string()))?;
        let fetched: Option<ContratistaFetched> = res_fetch.take(0).unwrap_or(None);
        Ok(fetched.unwrap())
    }

    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Contratista>, SurrealDbError> {
        let result: Option<Contratista> = self
            .db
            .select(id.clone())
            .await
            .map_err(|e: surrealdb::Error| SurrealDbError::Query(e.to_string()))?;
        Ok(result)
    }

    // Implementing bare minimum for tests
    async fn find_by_id_fetched(
        &self,
        id: &RecordId,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        let sql = "SELECT * FROM $id FETCH empresa";
        let mut result = self
            .db
            .query(sql)
            .bind(("id", id.clone()))
            .await
            .map_err(|e: surrealdb::Error| SurrealDbError::Query(e.to_string()))?;
        let fetched: Option<ContratistaFetched> = result.take(0).unwrap_or(None);
        Ok(fetched)
    }

    async fn find_all_fetched(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        Ok(vec![])
    }
    async fn update(
        &self,
        _id: &RecordId,
        _dto: ContratistaUpdateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        unimplemented!()
    }
    async fn update_status(
        &self,
        _id: &RecordId,
        _status: EstadoContratista,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        unimplemented!()
    }
    async fn delete(&self, _id: &RecordId) -> Result<(), SurrealDbError> {
        unimplemented!()
    }
    async fn restore(&self, _id: &RecordId) -> Result<(), SurrealDbError> {
        unimplemented!()
    }
    async fn find_archived(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        Ok(vec![])
    }
}

struct TestEmpresaRepo {
    db: Surreal<Db>,
}
#[async_trait]
impl EmpresaRepository for TestEmpresaRepo {
    async fn find_by_id(&self, id: &RecordId) -> Result<Option<Empresa>, SurrealDbError> {
        let res: Option<Empresa> = self
            .db
            .select(id.clone())
            .await
            .map_err(|e: surrealdb::Error| SurrealDbError::Query(e.to_string()))?;
        Ok(res)
    }
}

struct TestSecurityRepo;
#[async_trait]
impl SecurityRepository for TestSecurityRepo {
    async fn check_if_blocked_by_cedula(
        &self,
        _cedula: &str,
    ) -> Result<BlockStatus, SurrealDbError> {
        // Correct fields based on src/models/lista_negra.rs
        Ok(BlockStatus { is_blocked: false, nivel_severidad: None, bloqueado_desde: None })
    }
}

struct TestVehiculoRepo;
#[async_trait]
impl VehiculoRepository for TestVehiculoRepo {
    async fn find_by_propietario(
        &self,
        _propietario_id: &RecordId,
    ) -> Result<Vec<Vehiculo>, SurrealDbError> {
        Ok(vec![])
    }
}

struct TestAuditRepo;
#[async_trait]
impl AuditRepository for TestAuditRepo {
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

// ============================================================================
// TESTS
// ============================================================================

fn create_input_valido(empresa_id: &str) -> CreateContratistaInput {
    CreateContratistaInput {
        cedula: "12345678".to_string(),
        nombre: "Test".to_string(),
        segundo_nombre: None,
        apellido: "User".to_string(),
        segundo_apellido: None,
        empresa_id: empresa_id.to_string(),
        fecha_vencimiento_praind: "2030-01-01".to_string(), // Future date
        tiene_vehiculo: None,
        tipo_vehiculo: None,
        placa: None,
        marca: None,
        modelo: None,
        color: None,
    }
}

async fn create_empresa(db: &Surreal<Db>) -> RecordId {
    let sql = "CREATE empresa CONTENT { nombre: 'Empresa Test', rut: '123' }";
    let mut res = db.query(sql).await.unwrap();
    let empresa: Option<Empresa> = res.take(0).unwrap();
    empresa.unwrap().id
}

#[tokio::test]
async fn test_rn_cont_001_creacion_exitosa_estado_activo() {
    // Given
    let db: Surreal<Db> = setup_test_db().await;
    let empresa_id = create_empresa(&db).await;

    let repo = TestContratistaRepo { db: db.clone() };
    let emp_repo = TestEmpresaRepo { db: db.clone() };
    let sec_repo = TestSecurityRepo;
    let veh_repo = TestVehiculoRepo;
    let audit_repo = TestAuditRepo;

    let service = ContratistaService::new(repo, sec_repo, emp_repo, veh_repo, audit_repo, None);

    // When
    let input = create_input_valido(&empresa_id.to_string());
    let result = service.create_contratista(input).await;

    // Then
    assert!(result.is_ok(), "Service create failed: {:?}", result.err());
    let contratista = result.unwrap();
    assert_eq!(contratista.estado, EstadoContratista::Activo);
    assert_eq!(contratista.cedula, "12345678");
}

#[tokio::test]
async fn test_rn_cont_002_cedula_debe_ser_unica() {
    // Given
    let db: Surreal<Db> = setup_test_db().await;
    let empresa_id = create_empresa(&db).await;

    let repo = TestContratistaRepo { db: db.clone() };
    let emp_repo = TestEmpresaRepo { db: db.clone() };

    let service = ContratistaService::new(
        repo,
        TestSecurityRepo,
        emp_repo,
        TestVehiculoRepo,
        TestAuditRepo,
        None,
    );

    // First create
    let input1 = create_input_valido(&empresa_id.to_string());
    service.create_contratista(input1).await.expect("First create should succeed");

    // When
    // Instantiate new service with cloned dependencies
    let repo2 = TestContratistaRepo { db: db.clone() };
    let emp_repo2 = TestEmpresaRepo { db: db.clone() };
    // Other repos are ZST (Zero Sized Types), can be instantiated freely.
    let service2 = ContratistaService::new(
        repo2,
        TestSecurityRepo,
        emp_repo2,
        TestVehiculoRepo,
        TestAuditRepo,
        None,
    );

    let input2 = create_input_valido(&empresa_id.to_string()); // Same cedula
    let result = service2.create_contratista(input2).await;

    // Then
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ContratistaError::CedulaExists));
}

#[tokio::test]
async fn test_create_contratista_empresa_no_existe_falla() {
    // Given
    let db: Surreal<Db> = setup_test_db().await;
    // Don't create empresa

    let repo = TestContratistaRepo { db: db.clone() };
    let emp_repo = TestEmpresaRepo { db: db.clone() };

    let service = ContratistaService::new(
        repo,
        TestSecurityRepo,
        emp_repo,
        TestVehiculoRepo,
        TestAuditRepo,
        None,
    );

    // When
    let input = create_input_valido("empresa:non_existent");
    let result = service.create_contratista(input).await;

    // Then
    assert!(matches!(result.unwrap_err(), ContratistaError::EmpresaNotFound));
}
