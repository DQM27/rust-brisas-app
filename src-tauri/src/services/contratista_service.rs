// ==========================================
// src/services/contratista_service.rs
// ==========================================
// Capa de servicio: orquesta dominio y db
// Contiene la lógica de negocio completa

use crate::db::contratista_queries as db;
use crate::db::empresa_queries;
use crate::db::lista_negra_queries;
use crate::domain::contratista as domain;
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    EstadoContratista, UpdateContratistaInput,
};
use crate::services::search_service::SearchService;
use chrono::Utc;
use log::{error, info, warn};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

// ==========================================
// CREAR CONTRATISTA
// ==========================================

pub async fn create_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar datos
    let cedula_normalizada = domain::normalizar_cedula(&input.cedula);
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);
    let segundo_nombre_normalizado =
        domain::normalizar_segundo_nombre(input.segundo_nombre.as_ref());
    let apellido_normalizado = domain::normalizar_apellido(&input.apellido);
    let segundo_apellido_normalizado =
        domain::normalizar_segundo_apellido(input.segundo_apellido.as_ref());

    // 3. Verificar que NO esté en lista negra
    let block_status = lista_negra_queries::check_if_blocked_by_cedula(pool, &cedula_normalizada)
        .await
        .map_err(|e| ContratistaError::Validation(e.to_string()))?;

    if block_status.blocked {
        let motivo = block_status.motivo.unwrap_or_else(|| "Sin motivo especificado".to_string());
        return Err(ContratistaError::Validation(format!(
            "No se puede registrar. La persona con cédula {} está en lista negra. Motivo: {}",
            cedula_normalizada, motivo
        )));
    }

    // 4. Verificar que la cédula no exista
    let count = db::count_by_cedula(pool, &cedula_normalizada).await?;
    if count > 0 {
        return Err(ContratistaError::CedulaExists);
    }

    // 5. Verificar que la empresa exista
    info!(
        "Creando contratista con cédula {} para empresa {}",
        cedula_normalizada, input.empresa_id
    );
    let empresa_existe = empresa_queries::exists(pool, &input.empresa_id).await?;
    if !empresa_existe {
        return Err(ContratistaError::EmpresaNotFound);
    }

    // 6. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 7. Insertar en DB
    db::insert(
        pool,
        &id,
        &cedula_normalizada,
        &nombre_normalizado,
        segundo_nombre_normalizado.as_deref(),
        &apellido_normalizado,
        segundo_apellido_normalizado.as_deref(),
        &input.empresa_id,
        &input.fecha_vencimiento_praind,
        EstadoContratista::Activo.as_str(),
        &now,
        &now,
    )
    .await
    .map_err(|e| {
        error!("Error de base de datos al crear contratista {}: {}", cedula_normalizada, e);
        ContratistaError::Database(e)
    })?;

    info!("Contratista {} creado exitosamente con ID {}", cedula_normalizada, id);

    // 8. Obtener contratista creado
    let response = get_contratista_by_id(pool, &id).await?;

    // 9. Indexar en Tantivy
    if let Some(row) = db::find_by_id_with_empresa(pool, &id).await? {
        if let Err(e) = search_service.add_contratista(&row.contratista, &row.empresa_nombre).await
        {
            warn!("Error al indexar contratista {}: {}", id, e);
        }
    }

    Ok(response)
}

// ==========================================
// OBTENER CONTRATISTA POR ID
// ==========================================

pub async fn get_contratista_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    let row = db::find_by_id_with_empresa(pool, id).await?.ok_or(ContratistaError::NotFound)?;

    let mut response = ContratistaResponse::from(row.contratista);
    response.empresa_nombre = row.empresa_nombre;
    response.vehiculo_tipo = row.vehiculo_tipo;
    response.vehiculo_placa = row.vehiculo_placa;
    response.vehiculo_marca = row.vehiculo_marca;
    response.vehiculo_modelo = row.vehiculo_modelo;
    response.vehiculo_color = row.vehiculo_color;
    response.esta_bloqueado = row.is_blocked;

    if row.is_blocked {
        response.puede_ingresar = false;
    }

    Ok(response)
}

// ==========================================
// OBTENER CONTRATISTA POR CÉDULA
// ==========================================

pub async fn get_contratista_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    let row =
        db::find_by_cedula_with_empresa(pool, cedula).await?.ok_or(ContratistaError::NotFound)?;

    let mut response = ContratistaResponse::from(row.contratista);
    response.empresa_nombre = row.empresa_nombre;
    response.vehiculo_tipo = row.vehiculo_tipo;
    response.vehiculo_placa = row.vehiculo_placa;
    response.vehiculo_marca = row.vehiculo_marca;
    response.vehiculo_modelo = row.vehiculo_modelo;
    response.vehiculo_color = row.vehiculo_color;
    response.esta_bloqueado = row.is_blocked;

    if row.is_blocked {
        response.puede_ingresar = false;
    }

    Ok(response)
}

// ==========================================
// OBTENER TODOS LOS CONTRATISTAS
// ==========================================

pub async fn get_all_contratistas(
    pool: &SqlitePool,
) -> Result<ContratistaListResponse, ContratistaError> {
    let contratistas_with_empresa = db::find_all_with_empresa(pool).await?;

    let contratistas: Vec<ContratistaResponse> = contratistas_with_empresa
        .into_iter()
        .map(|(contratista, empresa_nombre, vehiculo_tipo, vehiculo_placa, is_blocked)| {
            let mut response = ContratistaResponse::from(contratista);
            response.empresa_nombre = empresa_nombre;
            response.vehiculo_tipo = vehiculo_tipo;
            response.vehiculo_placa = vehiculo_placa;
            response.esta_bloqueado = is_blocked;

            if is_blocked {
                response.puede_ingresar = false;
            }

            response
        })
        .collect();

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

// ==========================================
// OBTENER CONTRATISTAS ACTIVOS
// ==========================================

pub async fn get_contratistas_activos(
    pool: &SqlitePool,
) -> Result<Vec<ContratistaResponse>, ContratistaError> {
    let contratistas_with_empresa = db::find_activos_with_empresa(pool).await?;

    let contratistas = contratistas_with_empresa
        .into_iter()
        .map(|(contratista, empresa_nombre, vehiculo_tipo, vehiculo_placa, is_blocked)| {
            let mut response = ContratistaResponse::from(contratista);
            response.empresa_nombre = empresa_nombre;
            response.vehiculo_tipo = vehiculo_tipo;
            response.vehiculo_placa = vehiculo_placa;
            response.esta_bloqueado = is_blocked;

            if is_blocked {
                response.puede_ingresar = false;
            }

            response
        })
        .collect();

    Ok(contratistas)
}

// ==========================================
// ACTUALIZAR CONTRATISTA
// ==========================================

pub async fn update_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    info!("Actualizando contratista con ID {}", id);

    // 2. Verificar que el contratista existe
    let _ = db::find_by_id_with_empresa(pool, &id).await?.ok_or(ContratistaError::NotFound)?;

    // 3. Normalizar datos si vienen
    let nombre_normalizado = input.nombre.as_ref().map(|n| domain::normalizar_nombre(n));

    let segundo_nombre_normalizado =
        input.segundo_nombre.as_ref().and_then(|sn| domain::normalizar_segundo_nombre(Some(sn)));

    let apellido_normalizado = input.apellido.as_ref().map(|a| domain::normalizar_apellido(a));

    let segundo_apellido_normalizado = input
        .segundo_apellido
        .as_ref()
        .and_then(|sa| domain::normalizar_segundo_apellido(Some(sa)));

    // 4. Verificar que la empresa exista si viene
    if let Some(ref empresa_id) = input.empresa_id {
        let empresa_existe = empresa_queries::exists(pool, empresa_id).await?;
        if !empresa_existe {
            return Err(ContratistaError::EmpresaNotFound);
        }
    }

    // 5. Timestamp de actualización
    let now = Utc::now().to_rfc3339();

    // 6. Actualizar en DB
    db::update(
        pool,
        &id,
        nombre_normalizado.as_deref(),
        segundo_nombre_normalizado.as_deref(),
        apellido_normalizado.as_deref(),
        segundo_apellido_normalizado.as_deref(),
        input.empresa_id.as_deref(),
        input.fecha_vencimiento_praind.as_deref(),
        &now,
    )
    .await
    .map_err(|e| {
        error!("Error al actualizar contratista {}: {}", id, e);
        ContratistaError::Database(e)
    })?;

    info!("Contratista {} actualizado exitosamente", id);

    // 7. Gestionar Vehículo
    if let Some(tiene) = input.tiene_vehiculo {
        use crate::db::vehiculo_queries;
        let vehiculos = vehiculo_queries::find_by_contratista(pool, &id).await.unwrap_or_default();
        let vehiculo_existente = vehiculos.first();

        if tiene {
            if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
                if !tipo.is_empty() && !placa.is_empty() {
                    if let Some(v) = vehiculo_existente {
                        let _ = vehiculo_queries::update(
                            pool,
                            &v.id,
                            Some(tipo),
                            input.marca.as_deref(),
                            input.modelo.as_deref(),
                            input.color.as_deref(),
                            Some(true),
                            &now,
                        )
                        .await;
                    } else {
                        let vid = Uuid::new_v4().to_string();
                        let _ = vehiculo_queries::insert(
                            pool,
                            &vid,
                            Some(&id),
                            None,
                            tipo,
                            placa,
                            input.marca.as_deref(),
                            input.modelo.as_deref(),
                            input.color.as_deref(),
                            &now,
                            &now,
                        )
                        .await;
                    }
                }
            }
        } else if let Some(v) = vehiculo_existente {
            let _ = vehiculo_queries::delete(pool, &v.id).await;
        }
    }

    // 8. Obtener contratista actualizado
    let response = get_contratista_by_id(pool, &id).await?;

    // 9. Actualizar índice de Tantivy
    if let Some(row) = db::find_by_id_with_empresa(pool, &id).await? {
        if let Err(e) =
            search_service.update_contratista(&row.contratista, &row.empresa_nombre).await
        {
            warn!("Error al actualizar índice del contratista {}: {}", id, e);
        }
    }

    Ok(response)
}

// ==========================================
// CAMBIAR ESTADO
// ==========================================

pub async fn cambiar_estado_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Validar estado
    let estado = domain::validar_estado(&input.estado)?;

    info!("Cambiando estado de contratista {} a {}", id, input.estado);

    // 2. Verificar que el contratista existe
    let _ = db::find_by_id_with_empresa(pool, &id).await?.ok_or(ContratistaError::NotFound)?;

    // 3. Timestamp de actualización
    let now = Utc::now().to_rfc3339();

    // 4. Actualizar estado en DB
    db::update_estado(pool, &id, estado.as_str(), &now).await?;

    // 5. Obtener contratista actualizado
    let response = get_contratista_by_id(pool, &id).await?;

    // 6. Actualizar índice de Tantivy
    if let Some(row) = db::find_by_id_with_empresa(pool, &id).await? {
        if let Err(e) =
            search_service.update_contratista(&row.contratista, &row.empresa_nombre).await
        {
            warn!("Error al actualizar índice del contratista {}: {}", id, e);
        }
    }

    Ok(response)
}

// ==========================================
// ELIMINAR CONTRATISTA
// ==========================================

pub async fn delete_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
) -> Result<(), ContratistaError> {
    info!("Eliminando contratista con ID {}", id);

    // Verificar que existe
    let _ = db::find_by_id_with_empresa(pool, &id).await?.ok_or(ContratistaError::NotFound)?;

    // Eliminar de DB
    db::delete(pool, &id).await.map_err(|e| {
        error!("Error al eliminar contratista {}: {}", id, e);
        ContratistaError::Database(e)
    })?;

    info!("Contratista {} eliminado exitosamente", id);

    // Eliminar del índice de Tantivy
    if let Err(e) = search_service.delete_contratista(&id).await {
        warn!("Error al eliminar del índice el contratista {}: {}", id, e);
    }

    Ok(())
}

// ==========================================
// ACTUALIZAR PRAIND CON HISTORIAL
// ==========================================

use crate::db::audit_queries;

/// Input para actualizar fecha PRAIND
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActualizarPraindInput {
    pub contratista_id: String,
    pub nueva_fecha_praind: String,
    pub motivo: Option<String>,
}

/// Actualiza la fecha de vencimiento PRAIND y registra en historial
pub async fn actualizar_praind_con_historial(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    input: ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Obtener contratista actual
    let contratista_row = db::find_by_id_with_empresa(pool, &input.contratista_id)
        .await?
        .ok_or(ContratistaError::NotFound)?;

    let fecha_anterior = contratista_row.contratista.fecha_vencimiento_praind.clone();

    // 2. Validar nueva fecha
    crate::models::contratista::validaciones::validar_fecha(&input.nueva_fecha_praind)
        .map_err(|e| ContratistaError::Validation(e))?;

    // 3. Actualizar contratista
    let now = Utc::now().to_rfc3339();
    info!(
        "Actualizando PRAIND para contratista {} -> {}",
        input.contratista_id, input.nueva_fecha_praind
    );
    db::update_praind(pool, &input.contratista_id, &input.nueva_fecha_praind, &now).await?;

    // 4. Registrar en historial
    let historial_id = Uuid::new_v4().to_string();
    audit_queries::insert_praind_historial(
        pool,
        &historial_id,
        &input.contratista_id,
        Some(&fecha_anterior),
        &input.nueva_fecha_praind,
        &usuario_id,
        input.motivo.as_deref(),
        &now,
    )
    .await
    .map_err(|e| ContratistaError::Database(e))?;

    // 5. Si el PRAIND renovado y estaba suspendido por PRAIND, activar automáticamente
    let fecha_venc = chrono::NaiveDate::parse_from_str(&input.nueva_fecha_praind, "%Y-%m-%d")
        .unwrap_or_default();
    let nueva_fecha_valida = fecha_venc >= chrono::Utc::now().date_naive();

    if nueva_fecha_valida && contratista_row.contratista.estado.as_str() == "suspendido" {
        // Cambiar a activo automáticamente
        db::update_estado(pool, &input.contratista_id, "activo", &now).await?;

        // Registrar transición de estado
        let estado_historial_id = Uuid::new_v4().to_string();
        audit_queries::insert_historial_estado(
            pool,
            &estado_historial_id,
            &input.contratista_id,
            "suspendido",
            "activo",
            Some(&usuario_id),
            "Reactivación automática por renovación de PRAIND",
            &now,
        )
        .await
        .map_err(|e| ContratistaError::Database(e))?;
    }

    // 6. Obtener respuesta actualizada
    let updated = db::find_by_id_with_empresa(pool, &input.contratista_id)
        .await?
        .ok_or(ContratistaError::NotFound)?;

    // 7. Actualizar índice de búsqueda
    if let Err(e) =
        search_service.update_contratista(&updated.contratista, &updated.empresa_nombre).await
    {
        warn!("Error al actualizar índice del contratista: {}", e);
    }

    let response = build_response(updated, pool).await;
    Ok(response)
}

// ==========================================
// CAMBIAR ESTADO CON HISTORIAL
// ==========================================

/// Input para cambiar estado con auditoría
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CambiarEstadoConHistorialInput {
    pub contratista_id: String,
    pub nuevo_estado: String,
    pub motivo: String,
}

/// Cambia el estado del contratista y registra en historial
pub async fn cambiar_estado_con_historial(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    input: CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Obtener contratista actual
    let contratista_row = db::find_by_id_with_empresa(pool, &input.contratista_id)
        .await?
        .ok_or(ContratistaError::NotFound)?;

    let estado_anterior = contratista_row.contratista.estado.as_str().to_string();

    // 2. Validar nuevo estado
    let nuevo_estado: crate::models::contratista::EstadoContratista =
        input.nuevo_estado.parse().map_err(|e: String| ContratistaError::Validation(e))?;

    // 3. Actualizar estado
    let now = Utc::now().to_rfc3339();
    db::update_estado(pool, &input.contratista_id, nuevo_estado.as_str(), &now).await?;

    // 4. Registrar en historial
    let historial_id = Uuid::new_v4().to_string();
    audit_queries::insert_historial_estado(
        pool,
        &historial_id,
        &input.contratista_id,
        &estado_anterior,
        nuevo_estado.as_str(),
        Some(&usuario_id),
        &input.motivo,
        &now,
    )
    .await
    .map_err(|e| ContratistaError::Database(e))?;

    // 5. Obtener respuesta actualizada
    let updated = db::find_by_id_with_empresa(pool, &input.contratista_id)
        .await?
        .ok_or(ContratistaError::NotFound)?;

    // 6. Actualizar índice de búsqueda
    if let Err(e) =
        search_service.update_contratista(&updated.contratista, &updated.empresa_nombre).await
    {
        warn!("Error al actualizar índice del contratista: {}", e);
    }

    let response = build_response(updated, pool).await;
    Ok(response)
}

/// Helper para construir respuesta con datos completos
async fn build_response(row: db::ContratistaEnhancedRow, pool: &SqlitePool) -> ContratistaResponse {
    let vehiculo = crate::db::vehiculo_queries::find_by_contratista(pool, &row.contratista.id)
        .await
        .ok()
        .and_then(|v| v.into_iter().next());

    let mut response = ContratistaResponse::from(row.contratista);
    response.empresa_nombre = row.empresa_nombre;

    if let Some(v) = vehiculo {
        response.vehiculo_tipo = Some(v.tipo_vehiculo.to_string());
        response.vehiculo_placa = Some(v.placa);
        response.vehiculo_marca = v.marca;
        response.vehiculo_modelo = v.modelo;
        response.vehiculo_color = v.color;
    }

    response
}
