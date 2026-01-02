use crate::models::contratista::{Contratista, ContratistaFetched, EstadoContratista};
use crate::models::empresa::Empresa;
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoContratista, IngresoContratistaCreateDTO,
    IngresoContratistaFetched, RegistrarSalidaInput,
};
use crate::models::lista_negra::BlockStatus;
use crate::models::user::User;
use crate::repositories::traits::{
    ContratistaRepository, GafeteRepository, IngresoContratistaRepository, SecurityRepository,
};
use crate::services::ingreso_contratista_service::IngresoContratistaService;
use crate::services::surrealdb_service::SurrealDbError;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use surrealdb::RecordId;

// ================================================================
// MOCKS
// ================================================================

struct MockIngresoRepo {
    pub ingresos: Arc<Mutex<Vec<IngresoContratistaFetched>>>,
}

fn create_mock_user(id: RecordId) -> User {
    User {
        id,
        email: "mock@example.com".to_string(),
        nombre: "Mock".to_string(),
        apellido: "User".to_string(),
        role: RecordId::from(("role", "mock")),
        is_active: true,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        cedula: "0000000000".to_string(),
        segundo_nombre: None,
        segundo_apellido: None,
        fecha_inicio_labores: None,
        numero_gafete: None,
        fecha_nacimiento: None,
        telefono: None,
        direccion: None,
        contacto_emergencia_nombre: None,
        contacto_emergencia_telefono: None,
        must_change_password: false,
        deleted_at: None,
        avatar_path: None,
    }
}

fn create_mock_contratista_fetched(id: RecordId) -> ContratistaFetched {
    ContratistaFetched {
        id,
        cedula: "123".to_string(),
        nombre: "Test".to_string(),
        apellido: "User".to_string(),
        segundo_nombre: None,
        segundo_apellido: None,
        empresa: Empresa {
            id: RecordId::from(("empresa", "e1")),
            nombre: "TestCorp".to_string(),
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
    }
}

#[async_trait]
impl IngresoContratistaRepository for MockIngresoRepo {
    async fn insert(
        &self,
        dto: IngresoContratistaCreateDTO,
    ) -> Result<IngresoContratistaFetched, SurrealDbError> {
        let mut list = self.ingresos.lock().unwrap();
        let new_ing = IngresoContratistaFetched {
            id: RecordId::from(("ingreso_contratista", "mock_ing")),
            contratista: create_mock_contratista_fetched(dto.contratista),
            nombre: dto.nombre,
            apellido: dto.apellido,
            segundo_nombre: dto.segundo_nombre,
            segundo_apellido: dto.segundo_apellido,
            cedula: dto.cedula,
            tipo_autorizacion: dto.tipo_autorizacion,
            modo_ingreso: dto.modo_ingreso,
            placa_vehiculo: dto.placa_vehiculo,
            gafete_numero: dto.gafete_numero,
            fecha_hora_ingreso: chrono::Utc::now().into(),
            fecha_hora_salida: None,
            usuario_ingreso: create_mock_user(dto.usuario_ingreso),
            usuario_salida: None,
            observaciones: dto.observaciones,
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };
        list.push(new_ing.clone());
        Ok(new_ing)
    }

    async fn find_ingreso_abierto_by_contratista(
        &self,
        contratista_id: &RecordId,
    ) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
        let list = self.ingresos.lock().unwrap();
        Ok(list
            .iter()
            .find(|i| i.contratista.id == *contratista_id && i.fecha_hora_salida.is_none())
            .cloned())
    }

    async fn update_salida(
        &self,
        ingreso_id: &RecordId,
        usuario_salida_id: &RecordId,
        observaciones: Option<String>,
    ) -> Result<IngresoContratistaFetched, SurrealDbError> {
        let mut list = self.ingresos.lock().unwrap();
        if let Some(ing) = list.iter_mut().find(|i| i.id == *ingreso_id) {
            ing.fecha_hora_salida = Some(chrono::Utc::now().into());
            ing.usuario_salida = Some(create_mock_user(usuario_salida_id.clone()));
            ing.observaciones = observaciones;
            ing.updated_at = chrono::Utc::now().into();
            return Ok(ing.clone());
        }
        Err(SurrealDbError::Query("Ingreso not found".into()))
    }

    async fn find_by_id(
        &self,
        _id: &RecordId,
    ) -> Result<Option<IngresoContratista>, SurrealDbError> {
        Ok(None)
    }
    async fn find_by_id_fetched(
        &self,
        _id: &RecordId,
    ) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
        Ok(None)
    }
}

struct MockGafeteRepo {
    pub disponible: bool,
    pub en_uso_called: Arc<Mutex<bool>>,
    pub liberado_called: Arc<Mutex<bool>>,
}

#[async_trait]
impl GafeteRepository for MockGafeteRepo {
    async fn is_disponible(&self, _: i32, _: &str) -> Result<bool, SurrealDbError> {
        Ok(self.disponible)
    }
    async fn marcar_en_uso(&self, _: i32, _: &str) -> Result<(), SurrealDbError> {
        let mut called = self.en_uso_called.lock().unwrap();
        *called = true;
        Ok(())
    }
    async fn liberar(&self, _: i32, _: &str) -> Result<(), SurrealDbError> {
        let mut called = self.liberado_called.lock().unwrap();
        *called = true;
        Ok(())
    }
}

struct MockContratistaRepo {
    pub contratista: Option<ContratistaFetched>,
}

#[async_trait]
impl ContratistaRepository for MockContratistaRepo {
    async fn find_by_cedula(&self, _: &str) -> Result<Option<Contratista>, SurrealDbError> {
        Ok(self.contratista.clone().map(|f| f.into()))
    }
    async fn find_by_cedula_fetched(
        &self,
        _: &str,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        Ok(self.contratista.clone())
    }
    async fn create(
        &self,
        _: crate::models::contratista::ContratistaCreateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        Err(SurrealDbError::Query("Not implemented".into()))
    }
    async fn find_by_id(&self, _: &RecordId) -> Result<Option<Contratista>, SurrealDbError> {
        Ok(self.contratista.clone().map(|f| f.into()))
    }
    async fn find_by_id_fetched(
        &self,
        _: &RecordId,
    ) -> Result<Option<ContratistaFetched>, SurrealDbError> {
        Ok(self.contratista.clone())
    }
    async fn find_all_fetched(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        Ok(vec![])
    }
    async fn update(
        &self,
        _: &RecordId,
        _: crate::models::contratista::ContratistaUpdateDTO,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        Err(SurrealDbError::Query("Not implemented".into()))
    }
    async fn update_status(
        &self,
        _: &RecordId,
        _: EstadoContratista,
    ) -> Result<ContratistaFetched, SurrealDbError> {
        Err(SurrealDbError::Query("Not implemented".into()))
    }
    async fn delete(&self, _: &RecordId) -> Result<(), SurrealDbError> {
        Ok(())
    }
    async fn restore(&self, _: &RecordId) -> Result<(), SurrealDbError> {
        Ok(())
    }
    async fn find_archived(&self) -> Result<Vec<ContratistaFetched>, SurrealDbError> {
        Ok(vec![])
    }
}

struct MockSecurityRepo {
    pub blocked: bool,
}

#[async_trait]
impl SecurityRepository for MockSecurityRepo {
    async fn check_if_blocked_by_cedula(&self, _: &str) -> Result<BlockStatus, SurrealDbError> {
        Ok(BlockStatus {
            is_blocked: self.blocked,
            nivel_severidad: if self.blocked { Some("ALTO".into()) } else { None },
            bloqueado_desde: None,
        })
    }
}

// ================================================================
// TESTS
// ================================================================

#[tokio::test]
async fn test_validar_ingreso_success() {
    let contratista = create_mock_contratista_fetched(RecordId::from(("contratista", "c1")));

    let service = IngresoContratistaService::new(
        MockIngresoRepo { ingresos: Arc::new(Mutex::new(vec![])) },
        MockGafeteRepo {
            disponible: true,
            en_uso_called: Arc::new(Mutex::new(false)),
            liberado_called: Arc::new(Mutex::new(false)),
        },
        MockContratistaRepo { contratista: Some(contratista) },
        MockSecurityRepo { blocked: false },
    );

    let result = service.validar_ingreso_contratista("c1".to_string()).await;
    assert!(result.is_ok());
    assert!(result.unwrap().puede_ingresar);
}

#[tokio::test]
async fn test_crear_ingreso_success() {
    let contratista = create_mock_contratista_fetched(RecordId::from(("contratista", "c1")));

    let en_uso_called = Arc::new(Mutex::new(false));
    let service = IngresoContratistaService::new(
        MockIngresoRepo { ingresos: Arc::new(Mutex::new(vec![])) },
        MockGafeteRepo {
            disponible: true,
            en_uso_called: en_uso_called.clone(),
            liberado_called: Arc::new(Mutex::new(false)),
        },
        MockContratistaRepo { contratista: Some(contratista) },
        MockSecurityRepo { blocked: false },
    );

    let input = CreateIngresoContratistaInput {
        contratista_id: "c1".to_string(),
        tipo_autorizacion: "NORMAL".to_string(),
        modo_ingreso: "PEATONAL".to_string(),
        placa_vehiculo: None,
        gafete_numero: Some(101),
        observaciones: None,
    };

    let result = service.crear_ingreso_contratista(input, "u1".to_string()).await;
    assert!(result.is_ok());
    assert!(*en_uso_called.lock().unwrap(), "Gafete should be marked in use");
}

#[tokio::test]
async fn test_registrar_salida_success() {
    let c_id = RecordId::from(("contratista", "c1"));
    let ing_id = RecordId::from(("ingreso_contratista", "mock_ing"));

    let initial_ingreso = IngresoContratistaFetched {
        id: ing_id.clone(),
        contratista: create_mock_contratista_fetched(c_id.clone()),
        nombre: "Test".into(),
        apellido: "User".into(),
        segundo_nombre: None,
        segundo_apellido: None,
        cedula: "123".into(),
        tipo_autorizacion: "NORMAL".into(),
        modo_ingreso: "PEATONAL".into(),
        placa_vehiculo: None,
        gafete_numero: Some(101),
        fecha_hora_ingreso: chrono::Utc::now().into(),
        fecha_hora_salida: None,
        usuario_ingreso: create_mock_user(RecordId::from(("user", "u1"))),
        usuario_salida: None,
        observaciones: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let liberado_called = Arc::new(Mutex::new(false));
    let service = IngresoContratistaService::new(
        MockIngresoRepo { ingresos: Arc::new(Mutex::new(vec![initial_ingreso])) },
        MockGafeteRepo {
            disponible: true,
            en_uso_called: Arc::new(Mutex::new(false)),
            liberado_called: liberado_called.clone(),
        },
        MockContratistaRepo { contratista: None },
        MockSecurityRepo { blocked: false },
    );

    let input = RegistrarSalidaInput {
        ingreso_id: "mock_ing".to_string(),
        devolvio_gafete: true,
        usuario_salida_id: "u2".to_string(),
        observaciones_salida: Some("Todo ok".to_string()),
    };

    let result = service.registrar_salida(input, "u2".to_string()).await;
    assert!(result.is_ok());
    assert!(*liberado_called.lock().unwrap(), "Gafete should be liberated on exit");
    assert!(result.unwrap().fecha_hora_salida.is_some());
}
