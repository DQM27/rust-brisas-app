use crate::domain::contratista as domain;
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    ActualizarPraindInput, CambiarEstadoConHistorialInput, CambiarEstadoInput,
    ContratistaCreateDTO, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    EstadoContratista, UpdateContratistaInput,
};
use crate::models::vehiculo::{VehiculoCreateDTO, VehiculoUpdateDTO};
use crate::repositories::traits::{
    AuditRepository, ContratistaRepository, EmpresaRepository, SecurityRepository,
    VehiculoRepository,
};
use crate::services::search_service::SearchService;
use crate::services::surrealdb_service::SurrealDbError;
use chrono::{DateTime, TimeZone, Utc};
use log::{debug, error, info, warn};
use std::sync::Arc;
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// ESTRUCTURA DEL SERVICIO (DEPENDENCY INJECTION)
// --------------------------------------------------------------------------

pub struct ContratistaService<R, S, E, V, A>
where
    R: ContratistaRepository,
    S: SecurityRepository,
    E: EmpresaRepository,
    V: VehiculoRepository,
    A: AuditRepository,
{
    repo: R,
    sec_repo: S,
    emp_repo: E,
    veh_repo: V,
    audit_repo: A,
    search_service: Option<Arc<SearchService>>,
}

impl<R, S, E, V, A> ContratistaService<R, S, E, V, A>
where
    R: ContratistaRepository,
    S: SecurityRepository,
    E: EmpresaRepository,
    V: VehiculoRepository,
    A: AuditRepository,
{
    pub const fn new(
        repo: R,
        sec_repo: S,
        emp_repo: E,
        veh_repo: V,
        audit_repo: A,
        search_service: Option<Arc<SearchService>>,
    ) -> Self {
        Self { repo, sec_repo, emp_repo, veh_repo, audit_repo, search_service }
    }

    async fn build_response_fetched(
        &self,
        contratista: crate::models::contratista::ContratistaFetched,
    ) -> Result<ContratistaResponse, ContratistaError> {
        let mut response = ContratistaResponse::from_fetched(contratista.clone());

        let vehiculos =
            self.veh_repo.find_by_propietario(&contratista.id).await.map_err(map_db_error)?;

        if let Some(v) = vehiculos.first() {
            response.vehiculo_tipo = Some(v.tipo_vehiculo.to_string());
            response.vehiculo_placa = Some(v.placa.clone());
            response.vehiculo_marca = v.marca.clone();
            response.vehiculo_modelo = v.modelo.clone();
            response.vehiculo_color = v.color.clone();
        }

        let block_status = self
            .sec_repo
            .check_if_blocked_by_cedula(&contratista.cedula)
            .await
            .map_err(map_db_error)?;

        response.esta_bloqueado = block_status.is_blocked;
        if block_status.is_blocked {
            response.puede_ingresar = false;
        }

        Ok(response)
    }

    pub async fn create_contratista(
        &self,
        input: CreateContratistaInput,
    ) -> Result<ContratistaResponse, ContratistaError> {
        debug!("Iniciando registro de contratista: {} {}", input.nombre, input.apellido);
        domain::validar_create_input(&input)?;
        debug!("Validación de dominio exitosa para CI: {}", input.cedula);

        let cedula_normalizada = domain::normalizar_cedula(&input.cedula);

        let block_status = self
            .sec_repo
            .check_if_blocked_by_cedula(&cedula_normalizada)
            .await
            .map_err(map_db_error)?;
        if block_status.is_blocked {
            let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
            warn!("INTENTO DE BLOQUEO: {cedula_normalizada} (Nivel: {nivel})");
            return Err(ContratistaError::Validation(format!(
                "BLOQUEO DE SEGURIDAD: La cédula {cedula_normalizada} figura en la lista negra (Nivel: {nivel})."
            )));
        }

        let existing = self.repo.find_by_cedula(&cedula_normalizada).await.map_err(map_db_error)?;
        if existing.is_some() {
            warn!("Intento de duplicado para CI: {cedula_normalizada}");
            return Err(ContratistaError::CedulaExists);
        }

        let empresa_id = parse_empresa_id(&input.empresa_id);
        let empresa_opt = self.emp_repo.find_by_id(&empresa_id).await.map_err(map_db_error)?;
        if empresa_opt.is_none() {
            error!("Error de integridad: Empresa {} no encontrada", input.empresa_id);
            return Err(ContratistaError::EmpresaNotFound);
        }

        let fecha_vencimiento_naive = domain::validar_fecha(&input.fecha_vencimiento_praind)?;
        let fecha_vencimiento: DateTime<Utc> =
            chrono::Utc.from_utc_datetime(&fecha_vencimiento_naive.and_hms_opt(0, 0, 0).unwrap());

        let dto = ContratistaCreateDTO {
            cedula: cedula_normalizada.clone(),
            nombre: input.nombre.trim().to_string(),
            segundo_nombre: input.segundo_nombre.map(|s| s.trim().to_string()),
            apellido: input.apellido.trim().to_string(),
            segundo_apellido: input.segundo_apellido.map(|s| s.trim().to_string()),
            empresa: empresa_id,
            fecha_vencimiento_praind: surrealdb::Datetime::from(fecha_vencimiento),
            estado: EstadoContratista::Activo,
        };

        let contratista = self.repo.create(dto).await.map_err(|e| {
            error!("Fallo en DB al persistir contratista {cedula_normalizada}: {e}");
            map_db_error(e)
        })?;

        info!(
            "Contratista {} registrado exitosamente (ID: {})",
            cedula_normalizada, contratista.id
        );

        let empresa_nombre = contratista.empresa.nombre.clone();
        if let Some(search) = &self.search_service {
            if let Err(e) = search.add_contratista_fetched(&contratista, &empresa_nombre).await {
                log::warn!("Aviso: Falló la indexación en el motor de búsqueda: {e}");
            }
        }

        self.build_response_fetched(contratista).await
    }

    pub async fn get_contratista_by_id(
        &self,
        id_str: &str,
    ) -> Result<ContratistaResponse, ContratistaError> {
        let id = parse_contratista_id(id_str);
        // debug!("Consultando contratista por ID: {}", id);
        let contratista = self
            .repo
            .find_by_id_fetched(&id)
            .await
            .map_err(map_db_error)?
            .ok_or(ContratistaError::NotFound)?;
        self.build_response_fetched(contratista).await
    }

    pub async fn get_contratista_by_cedula(
        &self,
        cedula: &str,
    ) -> Result<ContratistaResponse, ContratistaError> {
        let cedula_norm = domain::normalizar_cedula(cedula);
        let contratista = self
            .repo
            .find_by_cedula_fetched(&cedula_norm)
            .await
            .map_err(map_db_error)?
            .ok_or(ContratistaError::NotFound)?;
        self.build_response_fetched(contratista).await
    }

    pub async fn get_all_contratistas(&self) -> Result<ContratistaListResponse, ContratistaError> {
        let raw_list = self.repo.find_all_fetched().await.map_err(map_db_error)?;
        let mut contratistas = Vec::new();
        for c in raw_list {
            contratistas.push(self.build_response_fetched(c).await?);
        }

        let total = contratistas.len();
        let activos = contratistas.iter().filter(|c| c.estado == EstadoContratista::Activo).count();
        let con_praind_vencido = contratistas.iter().filter(|c| c.praind_vencido).count();
        let requieren_atencion = contratistas.iter().filter(|c| c.requiere_atencion).count();

        Ok(ContratistaListResponse {
            contratistas,
            total,
            activos,
            con_praind_vencido,
            requieren_atencion,
        })
    }

    pub async fn get_contratistas_activos(
        &self,
    ) -> Result<Vec<ContratistaResponse>, ContratistaError> {
        let raw_list = self.repo.find_all_fetched().await.map_err(map_db_error)?;
        let mut contratistas = Vec::new();
        for c in raw_list {
            let res = self.build_response_fetched(c).await?;
            if res.estado == EstadoContratista::Activo {
                contratistas.push(res);
            }
        }
        Ok(contratistas)
    }

    pub async fn update_contratista(
        &self,
        id_str: String,
        input: UpdateContratistaInput,
    ) -> Result<ContratistaResponse, ContratistaError> {
        use crate::models::contratista::ContratistaUpdateDTO;
        let id = parse_contratista_id(&id_str);
        domain::validar_update_input(&input)?;

        let existing = self
            .repo
            .find_by_id(&id)
            .await
            .map_err(map_db_error)?
            .ok_or(ContratistaError::NotFound)?;
        let mut dto = ContratistaUpdateDTO::default();

        if let Some(v) = input.nombre {
            dto.nombre = Some(v.trim().to_string());
        }
        if let Some(v) = input.segundo_nombre {
            dto.segundo_nombre = Some(v.trim().to_string());
        }
        if let Some(v) = input.apellido {
            dto.apellido = Some(v.trim().to_string());
        }
        if let Some(v) = input.segundo_apellido {
            dto.segundo_apellido = Some(v.trim().to_string());
        }

        if let Some(empresa_id_str) = &input.empresa_id {
            let empresa_id = parse_empresa_id(empresa_id_str);
            if empresa_id != existing.empresa {
                if self.emp_repo.find_by_id(&empresa_id).await.map_err(map_db_error)?.is_none() {
                    return Err(ContratistaError::EmpresaNotFound);
                }
                dto.empresa = Some(empresa_id);
            }
        }

        if let Some(v) = input.fecha_vencimiento_praind {
            let fecha_naive = domain::validar_fecha(&v)?;
            let fecha: DateTime<Utc> =
                chrono::Utc.from_utc_datetime(&fecha_naive.and_hms_opt(0, 0, 0).unwrap());
            dto.fecha_vencimiento_praind = Some(surrealdb::Datetime::from(fecha));
        }

        let updated = self.repo.update(&id, dto).await.map_err(map_db_error)?;

        // Gestión del vehículo
        if input.tiene_vehiculo == Some(true) {
            if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
                if !tipo.is_empty() && !placa.is_empty() {
                    let tipo_norm = crate::domain::vehiculo::validar_tipo_vehiculo(tipo)
                        .map_err(|e| ContratistaError::Validation(e.to_string()))?
                        .as_str()
                        .to_string();
                    let placa_norm = crate::domain::vehiculo::normalizar_placa(placa);
                    let existing_vehiculos =
                        self.veh_repo.find_by_propietario(&id).await.map_err(map_db_error)?;
                    let existing_vehiculo =
                        existing_vehiculos.iter().find(|v| v.placa == placa_norm);

                    use crate::db::surrealdb_vehiculo_queries as veh_db_direct;

                    if let Some(vehiculo) = existing_vehiculo {
                        let update_dto = VehiculoUpdateDTO {
                            tipo_vehiculo: Some(
                                tipo_norm
                                    .parse::<crate::models::vehiculo::TipoVehiculo>()
                                    .map_err(ContratistaError::Validation)?,
                            ),
                            marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                            modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                            color: input.color.as_ref().map(|s| s.trim().to_string()),
                            ..Default::default()
                        };
                        let _ = veh_db_direct::update(&vehiculo.id, update_dto).await;
                    } else {
                        let dto_vehiculo = VehiculoCreateDTO {
                            propietario: updated.id.clone(),
                            tipo_vehiculo: tipo_norm
                                .parse::<crate::models::vehiculo::TipoVehiculo>()
                                .map_err(ContratistaError::Validation)?,
                            placa: placa_norm.clone(),
                            marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                            modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                            color: input.color.as_ref().map(|s| s.trim().to_string()),
                            is_active: true,
                        };
                        let _ = veh_db_direct::insert(dto_vehiculo).await;
                    }
                }
            }
        }

        let empresa_nombre = updated.empresa.nombre.clone();
        if let Some(search) = &self.search_service {
            if let Err(e) = search.update_contratista_fetched(&updated, &empresa_nombre).await {
                log::warn!("Aviso: Falló la sincronización del buscador: {e}");
            }
        }

        self.build_response_fetched(updated).await
    }

    pub async fn cambiar_estado_contratista(
        &self,
        id_str: String,
        input: CambiarEstadoInput,
    ) -> Result<ContratistaResponse, ContratistaError> {
        let id = parse_contratista_id(&id_str);
        self.repo.find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;
        let updated =
            self.repo.update_status(&id, input.estado.clone()).await.map_err(map_db_error)?;
        self.build_response_fetched(updated).await
    }

    pub async fn delete_contratista(&self, id_str: String) -> Result<(), ContratistaError> {
        let id = parse_contratista_id(&id_str);
        self.repo.find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;
        self.repo.delete(&id).await.map_err(map_db_error)?;
        Ok(())
    }

    pub async fn actualizar_praind_con_historial(
        &self,
        input: ActualizarPraindInput,
        usuario_id: String,
    ) -> Result<ContratistaResponse, ContratistaError> {
        use crate::models::contratista::ContratistaUpdateDTO;
        let id = parse_contratista_id(&input.contratista_id);
        let contratista = self
            .repo
            .find_by_id(&id)
            .await
            .map_err(map_db_error)?
            .ok_or(ContratistaError::NotFound)?;

        let dt: chrono::DateTime<chrono::Utc> = contratista
            .fecha_vencimiento_praind
            .to_string()
            .parse()
            .unwrap_or_else(|_| chrono::Utc::now());
        let fecha_anterior = dt.format("%d-%m-%Y").to_string();

        let nueva_fecha_naive = domain::validar_fecha(&input.nueva_fecha_praind)?;
        let nueva_fecha: DateTime<Utc> =
            chrono::Utc.from_utc_datetime(&nueva_fecha_naive.and_hms_opt(0, 0, 0).unwrap());

        let dto = ContratistaUpdateDTO {
            fecha_vencimiento_praind: Some(surrealdb::Datetime::from(nueva_fecha)),
            ..Default::default()
        };

        let updated = self.repo.update(&id, dto).await.map_err(map_db_error)?;
        self.audit_repo
            .insert_praind_historial(
                &input.contratista_id,
                Some(&fecha_anterior),
                &input.nueva_fecha_praind,
                &usuario_id,
                input.motivo.as_deref(),
            )
            .await
            .map_err(map_db_error)?;
        self.build_response_fetched(updated).await
    }

    pub async fn cambiar_estado_con_historial(
        &self,
        input: CambiarEstadoConHistorialInput,
        usuario_id: String,
    ) -> Result<ContratistaResponse, ContratistaError> {
        let id = parse_contratista_id(&input.contratista_id);
        let contratista = self
            .repo
            .find_by_id(&id)
            .await
            .map_err(map_db_error)?
            .ok_or(ContratistaError::NotFound)?;
        let estado_anterior = contratista.estado.as_str().to_string();
        let nuevo_estado = input.nuevo_estado;

        let updated =
            self.repo.update_status(&id, nuevo_estado.clone()).await.map_err(map_db_error)?;
        self.audit_repo
            .insert_historial_estado(
                &input.contratista_id,
                &estado_anterior,
                nuevo_estado.as_str(),
                Some(&usuario_id),
                &input.motivo,
            )
            .await
            .map_err(map_db_error)?;
        self.build_response_fetched(updated).await
    }

    pub async fn restore_contratista(&self, id_str: String) -> Result<(), ContratistaError> {
        let id = parse_contratista_id(&id_str);
        if self.repo.find_by_id(&id).await.map_err(map_db_error)?.is_none() {
            return Err(ContratistaError::NotFound);
        }
        self.repo.restore(&id).await.map_err(map_db_error)?;

        if let Some(contratista) = self.repo.find_by_id_fetched(&id).await.map_err(map_db_error)? {
            let empresa_nombre = contratista.empresa.nombre.clone();
            if let Some(search) = &self.search_service {
                let _ = search.add_contratista_fetched(&contratista, &empresa_nombre).await;
            }
        }
        Ok(())
    }

    pub async fn get_archived_contratistas(
        &self,
    ) -> Result<Vec<ContratistaResponse>, ContratistaError> {
        let raw_list = self.repo.find_archived().await.map_err(map_db_error)?;
        let mut contratistas = Vec::new();
        for c in raw_list {
            contratistas.push(self.build_response_fetched(c).await?);
        }
        Ok(contratistas)
    }
}

// --------------------------------------------------------------------------
// INTERNAL HELPERS
// --------------------------------------------------------------------------

fn map_db_error(e: SurrealDbError) -> ContratistaError {
    ContratistaError::Database(e.to_string())
}

fn parse_contratista_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("contratista", id_str)
    }
}

fn parse_empresa_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("empresa", id_str)
    }
}
