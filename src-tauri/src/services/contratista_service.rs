use crate::domain::contratista as domain;
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    ActualizarPraindInput, CambiarEstadoConHistorialInput, CambiarEstadoInput,
    ContratistaCreateDTO, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    EstadoContratista, UpdateContratistaInput,
};
use crate::models::vehiculo::VehiculoCreateDTO;
/// Servicio: Gestión Integral de Contratistas
///
/// Orquestador del ciclo de vida completo de un contratista: desde su validación
/// inicial contra listas negras hasta su indexación en el motor de búsqueda.
///
/// Responsabilidades:
/// - Registro con verificación de seguridad (Lista Negra).
/// - Actualización de perfiles y certificaciones (PRAIND).
/// - Gestión de vehículos vinculados.
/// - Sincronización con motor de búsqueda.
// crate::db::* imports removed in favor of repositories
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
// HELPERS INTERNOS
// --------------------------------------------------------------------------

/// Mapeo de errores de infraestructura a dominio.
fn map_db_error(e: SurrealDbError) -> ContratistaError {
    ContratistaError::Database(e.to_string())
}

/// Normalización de IDs de contratista para SurrealDB.
fn parse_contratista_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("contratista", id_str)
    }
}

/// Normalización de IDs de empresa para SurrealDB.
fn parse_empresa_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("empresa", id_str)
    }
}

// --------------------------------------------------------------------------
// OPERACIONES DE CREACIÓN
// --------------------------------------------------------------------------

/// Registra un nuevo contratista en el sistema.
///
/// El proceso sigue este flujo crítico:
/// 1. Validación de formato de datos (Dominio).
/// 2. Chequeo de seguridad (Lista Negra): Si la persona está vetada, se aborta.
/// 3. Unicidad de Identidad: No se permiten duplicados de cédula.
/// 4. Vinculación Empresarial: Debe pertenecer a una empresa válida.
/// 5. Creación de Identidad Digital y (opcionalmente) de su Patrimonio Vehicular.
/// 6. Sincronización: Se notifica al motor de búsqueda.
///
/// # Argumentos
/// * `search_service` - Referencia al servicio de indexación.
/// * `input` - Datos del contratista a registrar.
///
/// # Retorno
/// El perfil del contratista recién creado.
///
/// # Errores
/// - `ContratistaError::Validation`: Datos inválidos o bloqueo de seguridad.
/// - `ContratistaError::CedulaExists`: Cédula ya registrada.
/// - `ContratistaError::EmpresaNotFound`: Empresa vinculada no existe.
/// - `ContratistaError::Database`: Error de persistencia.
pub async fn create_contratista(
    search_service: &Arc<SearchService>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    // Instanciar repositorios (Inyección manual por simplicidad en esta fase)
    use crate::repositories::contratista::{
        SurrealContratistaRepository, SurrealEmpresaRepository, SurrealSecurityRepository,
    };
    let repo = SurrealContratistaRepository;
    let security_repo = SurrealSecurityRepository;
    let empresa_repo = SurrealEmpresaRepository;

    debug!("Iniciando registro de contratista: {} {}", input.nombre, input.apellido);

    domain::validar_create_input(&input)?;
    debug!("Validación de dominio exitosa para CI: {}", input.cedula);

    let cedula_normalizada = domain::normalizar_cedula(&input.cedula);

    // 1. Seguridad: Verificar lista negra
    let block_status = security_repo
        .check_if_blocked_by_cedula(&cedula_normalizada)
        .await
        .map_err(map_db_error)?;

    if block_status.is_blocked {
        let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
        warn!(
            "INTENTO DE BLOQUEO: Contratista {} rechazado por lista negra (Nivel: {})",
            cedula_normalizada, nivel
        );
        return Err(ContratistaError::Validation(format!(
            "BLOQUEO DE SEGURIDAD: La cédula {} figura en la lista negra (Nivel: {}).",
            cedula_normalizada, nivel
        )));
    }

    // 2. Unicidad: Verificar si ya existe
    let existing = repo.find_by_cedula(&cedula_normalizada).await.map_err(map_db_error)?;
    if existing.is_some() {
        warn!("Intento de duplicado para CI: {}", cedula_normalizada);
        return Err(ContratistaError::CedulaExists);
    }

    // 3. Integridad: Verificar empresa
    let empresa_id = parse_empresa_id(&input.empresa_id);
    let empresa_opt = empresa_repo.find_by_id(&empresa_id).await.map_err(map_db_error)?;
    if empresa_opt.is_none() {
        error!("Error de integridad: Empresa {} no encontrada", input.empresa_id);
        return Err(ContratistaError::EmpresaNotFound);
    }

    // 4. Preparar DTO
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

    debug!("Persistiendo contratista en DB: {:?}", dto);

    // 5. Persistencia
    let contratista = repo.create(dto).await.map_err(|e| {
        error!("Fallo en DB al persistir contratista {}: {}", cedula_normalizada, e);
        map_db_error(e)
    })?;

    info!("Contratista {} registrado exitosamente (ID: {})", cedula_normalizada, contratista.id);

    // Nota: La creación de vehículos se ha desacoplado y movido al frontend.

    // 6. Indexación
    let empresa_nombre = contratista.empresa.nombre.clone();
    if let Err(e) = search_service.add_contratista_fetched(&contratista, &empresa_nombre).await {
        log::warn!("Aviso: Falló la indexación en el motor de búsqueda: {}", e);
    }

    build_response_fetched(contratista).await
}

// --------------------------------------------------------------------------
// CONSULTAS DE CONTRATISTAS
// --------------------------------------------------------------------------

// ==========================================
// CONSULTAS DE CONTRATISTAS
// ==========================================

/// Obtiene el perfil completo de un contratista por su ID.
///
/// # Argumentos
/// * `id_str` - ID del contratista (formato "contratista:id" o "id").
///
/// # Retorno
/// Perfil enriquecido con datos de vehículo y estado de bloqueo.
///
/// # Errores
/// - `ContratistaError::NotFound`: El contratista no existe.
/// - `ContratistaError::Database`: Error de acceso a datos.
pub async fn get_contratista_by_id(id_str: &str) -> Result<ContratistaResponse, ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    let id = parse_contratista_id(id_str);
    debug!("Consultando contratista por ID: {}", id);

    let contratista = repo
        .find_by_id_fetched(&id)
        .await
        .map_err(map_db_error)?
        .ok_or(ContratistaError::NotFound)?;
    build_response_fetched(contratista).await
}

/// Localiza un contratista por su número de cédula.
///
/// # Argumentos
/// * `cedula` - Cédula de identidad.
///
/// # Retorno
/// Perfil del contratista si existe.
///
/// # Errores
/// - `ContratistaError::NotFound`: No existe contratista con esa cédula.
pub async fn get_contratista_by_cedula(
    cedula: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    let cedula_norm = domain::normalizar_cedula(cedula);
    debug!("Buscando contratista por Cédula: {}", cedula_norm);

    let contratista = repo
        .find_by_cedula_fetched(&cedula_norm)
        .await
        .map_err(map_db_error)?
        .ok_or(ContratistaError::NotFound)?;
    build_response_fetched(contratista).await
}

/// Recupera el censo completo de contratistas con estadísticas.
///
/// # Retorno
/// Lista de contratistas con conteos de activos, PRAIND vencido, etc.
///
/// # Errores
/// - `ContratistaError::Database`: Error de consulta.
pub async fn get_all_contratistas() -> Result<ContratistaListResponse, ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    debug!("Iniciando censo completo de contratistas");
    let raw_list = repo.find_all_fetched().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        contratistas.push(build_response_fetched(c).await?);
    }

    let total = contratistas.len();
    let activos = contratistas.iter().filter(|c| c.estado == EstadoContratista::Activo).count();
    let con_praind_vencido = contratistas.iter().filter(|c| c.praind_vencido).count();
    let requieren_atencion = contratistas.iter().filter(|c| c.requiere_atencion).count();

    info!(
        "Censo finalizado. Total: {}, Activos: {}, PRAIND Vencido: {}",
        total, activos, con_praind_vencido
    );

    Ok(ContratistaListResponse {
        contratistas,
        total,
        activos,
        con_praind_vencido,
        requieren_atencion,
    })
}

/// Filtra exclusivamente contratistas con estado Activo.
///
/// # Retorno
/// Vector de contratistas habilitados para laborar.
pub async fn get_contratistas_activos() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    let raw_list = repo.find_all_fetched().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        let res = build_response_fetched(c).await?;
        if res.estado == EstadoContratista::Activo {
            contratistas.push(res);
        }
    }

    Ok(contratistas)
}

// --------------------------------------------------------------------------
// OPERACIONES DE ACTUALIZACIÓN
// --------------------------------------------------------------------------

/// Actualiza la información de un contratista existente.
///
/// Incluye lógica inteligente para detectar cambios de empresa y sincronizar
/// los datos vinculados (Vehículos) si se proporcionan en el mismo formulario.
///
/// # Argumentos
/// * `search_service` - Referencia al servicio de indexación.
/// * `id_str` - ID del contratista.
/// * `input` - Campos a actualizar.
///
/// # Retorno
/// Perfil actualizado del contratista.
///
/// # Errores
/// - `ContratistaError::NotFound`: Contratista no existe.
/// - `ContratistaError::Validation`: Datos inválidos.
/// - `ContratistaError::EmpresaNotFound`: Nueva empresa no existe.
/// - `ContratistaError::Database`: Error de persistencia.
pub async fn update_contratista(
    search_service: &Arc<SearchService>,
    id_str: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    use crate::models::contratista::ContratistaUpdateDTO;
    use crate::models::vehiculo::VehiculoUpdateDTO;
    use crate::repositories::contratista::{
        SurrealContratistaRepository, SurrealEmpresaRepository, SurrealVehiculoRepository,
    };
    let repo = SurrealContratistaRepository;
    let empresa_repo = SurrealEmpresaRepository;
    let vehiculo_repo = SurrealVehiculoRepository;

    let id = parse_contratista_id(&id_str);
    debug!("Solicitud de actualización para Contratista ID: {}", id);

    domain::validar_update_input(&input)?;

    let existing = repo.find_by_id(&id).await.map_err(map_db_error)?.ok_or({
        warn!("Intento de actualizar contratista inexistente: {}", id);
        ContratistaError::NotFound
    })?;

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
            if empresa_repo.find_by_id(&empresa_id).await.map_err(map_db_error)?.is_none() {
                warn!("Empresa especificada no existe: {}", empresa_id);
                return Err(ContratistaError::EmpresaNotFound);
            }
            info!(
                "Cambiando empresa de contratista {}: {} -> {}",
                id, existing.empresa, empresa_id
            );
            dto.empresa = Some(empresa_id);
        }
    }

    if let Some(v) = input.fecha_vencimiento_praind {
        let fecha_naive = domain::validar_fecha(&v)?;
        let fecha: DateTime<Utc> =
            chrono::Utc.from_utc_datetime(&fecha_naive.and_hms_opt(0, 0, 0).unwrap());
        dto.fecha_vencimiento_praind = Some(surrealdb::Datetime::from(fecha));
        info!("Actualizando fecha PRAIND para {}: {:?}", id, fecha);
    }

    let updated = repo.update(&id, dto).await.map_err(|e| {
        error!("Error en DB al actualizar contratista {}: {}", id, e);
        map_db_error(e)
    })?;

    info!("Contratista actualizado correctamente: {}", id);

    // Gestión del vehículo vinculada a la actualización del perfil.
    if let Some(true) = input.tiene_vehiculo {
        debug!("Procesando actualización vehicular para {}", id);
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                let tipo_norm = crate::domain::vehiculo::validar_tipo_vehiculo(tipo)
                    .map_err(|e| ContratistaError::Validation(e.to_string()))?
                    .as_str()
                    .to_string();

                let placa_norm = crate::domain::vehiculo::normalizar_placa(placa);

                let existing_vehiculos =
                    vehiculo_repo.find_by_propietario(&id).await.map_err(map_db_error)?;
                let existing_vehiculo = existing_vehiculos.iter().find(|v| v.placa == placa_norm);

                // NOTA: Para mantener la compatibilidad con el código anterior de vehiculos
                // y dado que VehiculoRepository fue definido como solo lectura en traits.rs por simplicidad inicial,
                // usaremos imports directos de veh_db SOLO AQUI para mantener la logica de vehicle update
                // hasta que vehiculo_service sea refactorizado. Esto es una deuda tecnica aceptada.
                use crate::db::surrealdb_vehiculo_queries as veh_db_direct;

                if let Some(vehiculo) = existing_vehiculo {
                    let update_dto = VehiculoUpdateDTO {
                        tipo_vehiculo: Some(
                            tipo_norm
                                .parse::<crate::models::vehiculo::TipoVehiculo>()
                                .map_err(|e| ContratistaError::Validation(e))?,
                        ),
                        marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                        modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                        color: input.color.as_ref().map(|s| s.trim().to_string()),
                        ..Default::default()
                    };

                    let _ = veh_db_direct::update(&vehiculo.id, update_dto).await;
                    debug!("Vehículo {} actualizado para contratista {}", placa_norm, id);
                } else {
                    let dto_vehiculo = VehiculoCreateDTO {
                        propietario: updated.id.clone(),
                        tipo_vehiculo: tipo_norm
                            .parse::<crate::models::vehiculo::TipoVehiculo>()
                            .map_err(|e| ContratistaError::Validation(e))?,
                        placa: placa_norm.clone(),
                        marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                        modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                        color: input.color.as_ref().map(|s| s.trim().to_string()),
                        is_active: true,
                    };

                    let _ = veh_db_direct::insert(dto_vehiculo).await;
                    info!("Vehículo nuevo {} registrado para contratista {}", placa_norm, id);
                }
            }
        }
    }

    // Actualización del motor de búsqueda tras cambios de perfil.
    let empresa_nombre = updated.empresa.nombre.clone();
    if let Err(e) = search_service.update_contratista_fetched(&updated, &empresa_nombre).await {
        log::warn!("Aviso: Falló la sincronización del buscador: {}", e);
    }

    build_response_fetched(updated).await
}

// ==========================================
// CAMBIAR ESTADO
// ==========================================

/// Cambia el estado de un contratista (Activo, Inactivo, Bloqueado).
pub async fn cambiar_estado_contratista(
    _search_service: &Arc<SearchService>,
    id_str: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    let id = parse_contratista_id(&id_str);
    debug!("Solicitud cambio de estado manual para {}: {:?}", id, input.estado);

    repo.find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    // input.estado ya es EstadoContratista (Type-Driven Design)
    let updated = repo.update_status(&id, input.estado.clone()).await.map_err(map_db_error)?;

    info!("Estado actualizado para {}: {:?}", id, input.estado);
    build_response_fetched(updated).await
}

// ==========================================
// ELIMINAR CONTRATISTA
// ==========================================

/// Elimina a un contratista (Marcado como borrado lógico en SurrealDB).
pub async fn delete_contratista(
    _search_service: &Arc<SearchService>,
    id_str: String,
) -> Result<(), ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    let id = parse_contratista_id(&id_str);
    info!("Solicitud de archivado (Delete) para contratista {}", id);

    repo.find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    repo.delete(&id).await.map_err(|e| {
        error!("Fallo crítico al eliminar contratista {}: {}", id_str, e);
        map_db_error(e)
    })?;

    info!("Contratista {} archivado correctamente.", id);
    Ok(())
}

// ==========================================
// ACTUALIZAR PRAIND CON HISTORIAL
// ==========================================

/// Actualiza la fecha de vencimiento PRAIND y registra el evento en el historial de auditoría.
pub async fn actualizar_praind_con_historial(
    _search_service: &Arc<SearchService>,
    input: ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    use crate::models::contratista::ContratistaUpdateDTO;
    use crate::repositories::contratista::{SurrealAuditRepository, SurrealContratistaRepository};
    let repo = SurrealContratistaRepository;
    let audit_repo = SurrealAuditRepository;

    let id = parse_contratista_id(&input.contratista_id);
    debug!("Actualización auditada de PRAIND para {}. Usuario: {}", id, usuario_id);

    let contratista =
        repo.find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

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

    let updated = repo.update(&id, dto).await.map_err(map_db_error)?;

    audit_repo
        .insert_praind_historial(
            &input.contratista_id,
            Some(&fecha_anterior),
            &input.nueva_fecha_praind,
            &usuario_id,
            input.motivo.as_deref(),
        )
        .await
        .map_err(|e| {
            error!("Fallo al registrar auditoría PRAIND para {}: {}", id, e);
            map_db_error(e)
        })?;

    info!(
        "PRAIND actualizado con auditoría para {}: {} -> {}",
        id, fecha_anterior, input.nueva_fecha_praind
    );
    build_response_fetched(updated).await
}

// ==========================================
// CAMBIAR ESTADO CON HISTORIAL
// ==========================================

/// Cambia el estado (Ej. Activo -> Inactivo) y audita quién realizó el cambio y por qué.
pub async fn cambiar_estado_con_historial(
    _search_service: &Arc<SearchService>,
    input: CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    use crate::repositories::contratista::{SurrealAuditRepository, SurrealContratistaRepository};
    let repo = SurrealContratistaRepository;
    let audit_repo = SurrealAuditRepository;

    let id = parse_contratista_id(&input.contratista_id);
    debug!("Cambio de estado auditado para {}. Usuario: {}", id, usuario_id);

    let contratista =
        repo.find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    let estado_anterior = contratista.estado.as_str().to_string();

    // input.nuevo_estado ya es EstadoContratista (Type-Driven Design)
    let nuevo_estado = input.nuevo_estado;

    let updated = repo.update_status(&id, nuevo_estado.clone()).await.map_err(map_db_error)?;

    audit_repo
        .insert_historial_estado(
            &input.contratista_id,
            &estado_anterior,
            nuevo_estado.as_str(),
            Some(&usuario_id),
            &input.motivo,
        )
        .await
        .map_err(|e| {
            error!("Fallo al registrar auditoría de estado para {}: {}", id, e);
            map_db_error(e)
        })?;

    info!("Estado cambiado con auditoría para {}: {} -> {:?}", id, estado_anterior, nuevo_estado);
    build_response_fetched(updated).await
}

/// Construye el objeto de respuesta enriqueciendo los datos básicos con vehículos y estado de bloqueo.
async fn build_response_fetched(
    contratista: crate::models::contratista::ContratistaFetched,
) -> Result<ContratistaResponse, ContratistaError> {
    use crate::repositories::contratista::{SurrealSecurityRepository, SurrealVehiculoRepository};
    let veh_repo = SurrealVehiculoRepository;
    let sec_repo = SurrealSecurityRepository;

    let mut response = ContratistaResponse::from_fetched(contratista.clone());

    // El sistema intenta recuperar el vehículo principal para visualizarlo en listados y detalles.
    let vehiculos = veh_repo.find_by_propietario(&contratista.id).await.map_err(map_db_error)?;
    if let Some(v) = vehiculos.first() {
        response.vehiculo_tipo = Some(v.tipo_vehiculo.to_string());
        response.vehiculo_placa = Some(v.placa.clone());
        response.vehiculo_marca = v.marca.clone();
        response.vehiculo_modelo = v.modelo.clone();
        response.vehiculo_color = v.color.clone();
    }

    let block_status =
        sec_repo.check_if_blocked_by_cedula(&contratista.cedula).await.map_err(map_db_error)?;
    response.esta_bloqueado = block_status.is_blocked;

    if block_status.is_blocked {
        response.puede_ingresar = false;
    }

    Ok(response)
}

// ==========================================
// RESTORE & ARCHIVED
// ==========================================

/// Recupera a un contratista eliminado y lo re-indexa para que vuelva a estar visible en búsquedas.
pub async fn restore_contratista(
    search_service: &Arc<SearchService>,
    id_str: String,
) -> Result<(), ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    let id = parse_contratista_id(&id_str);

    let exists = repo.find_by_id(&id).await.map_err(map_db_error)?;
    if exists.is_none() {
        return Err(ContratistaError::NotFound);
    }

    repo.restore(&id).await.map_err(map_db_error)?;

    if let Some(contratista) = repo.find_by_id_fetched(&id).await.map_err(map_db_error)? {
        let empresa_nombre = contratista.empresa.nombre.clone();

        // Tantivy Best Effort: Intentamos re-indexar, si falla solo logueamos.
        let _ = search_service.add_contratista_fetched(&contratista, &empresa_nombre).await;
    }

    Ok(())
}

/// Consulta el historial de contratistas archivados (soft-deleted).
///
/// Permite auditar qué contratistas han sido eliminados del sistema principal.
///
/// # Retorno
/// Lista de perfiles que han sido marcados como borrados (`deleted_at` no nulo).
///
/// # Errores
/// - `ContratistaError::Database`: Fallo de conexión o consulta.
pub async fn get_archived_contratistas() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    use crate::repositories::contratista::SurrealContratistaRepository;
    let repo = SurrealContratistaRepository;

    let raw_list = repo.find_archived().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        contratistas.push(build_response_fetched(c).await?);
    }

    Ok(contratistas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_contratista_id_con_prefijo() {
        let id = parse_contratista_id("contratista:123");
        assert_eq!(id.to_string(), "contratista:123");
    }

    #[test]
    fn test_parse_contratista_id_sin_prefijo() {
        let id = parse_contratista_id("123");
        assert_eq!(id.to_string(), "contratista:123");
    }

    #[test]
    fn test_parse_empresa_id_con_prefijo() {
        let id = parse_empresa_id("empresa:abc");
        assert_eq!(id.to_string(), "empresa:abc");
    }

    #[test]
    fn test_parse_empresa_id_sin_prefijo() {
        let id = parse_empresa_id("abc");
        assert_eq!(id.to_string(), "empresa:abc");
    }
}
