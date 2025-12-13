// ==========================================
// src/services/entrada_service.rs
// ==========================================
// Orquesta DB + Dominio para la fase de ENTRADA

use crate::db::lista_negra_queries;
use crate::db::{alerta_gafete_queries as alerta_db, contratista_queries, ingreso_queries as db};

use crate::domain::ingreso::validaciones_entrada as domain;
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, ModoIngreso, TipoAutorizacion, TipoIngreso,
    ValidacionIngresoResponse,
};
use crate::services::gafete_service;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// VALIDAR INGRESO CONTRATISTA
// ==========================================

/// Valida si un contratista puede ingresar
///
/// Orquesta:
/// 1. Consultas a DB (lista negra, ingreso abierto, datos contratista, alertas)
/// 2. Validaciones de dominio (elegibilidad)
/// 3. Construye respuesta con toda la información
pub async fn validar_ingreso_contratista(
    pool: &SqlitePool,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, String> {
    // 1. Verificar lista negra
    let block_status = lista_negra_queries::check_if_blocked(pool, &contratista_id).await?;

    // 2. Verificar ingreso abierto
    let tiene_ingreso_abierto = db::find_ingreso_abierto_by_contratista(pool, &contratista_id)
        .await
        .is_ok();

    // Si tiene ingreso abierto, retornar inmediatamente con la info
    if tiene_ingreso_abierto {
        let ingreso = db::find_ingreso_abierto_by_contratista(pool, &contratista_id).await?;
        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("El contratista ya tiene un ingreso abierto".to_string()),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(IngresoResponse::from(ingreso)),
        });
    }

    // 3. Obtener datos del contratista
    let contratista_opt = contratista_queries::find_basic_info_by_id(pool, &contratista_id).await?;

    let contratista = match contratista_opt {
        None => {
            return Ok(ValidacionIngresoResponse {
                puede_ingresar: false,
                motivo_rechazo: Some("Contratista no encontrado".to_string()),
                alertas: vec![],
                contratista: None,
                tiene_ingreso_abierto: false,
                ingreso_abierto: None,
            });
        }
        Some(c) => c,
    };

    // 4. Verificar PRAIND con dominio
    let praind_vigente = domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;

    // 5. Verificar alertas de gafetes pendientes
    let alertas_db = alerta_db::find_pendientes_by_cedula(pool, &contratista.cedula).await?;
    let cantidad_alertas = alertas_db.len();

    // 6. APLICAR REGLAS DE DOMINIO (función core)
    let resultado = domain::evaluar_elegibilidad_entrada(
        block_status.blocked,
        block_status.motivo,
        tiene_ingreso_abierto,
        &contratista.estado,
        praind_vigente,
        cantidad_alertas,
    );

    // 7. Obtener vehículos del contratista (si es elegible o tiene rechazo pero existe)
    // Esto es necesario para que el frontend pueda mostrar el selector de vehículos
    let vehiculos = crate::db::vehiculo_queries::find_by_contratista(pool, &contratista_id)
        .await
        .unwrap_or_default(); // Si falla, simplemente lista vacía

    // 8. Construir respuesta
    let contratista_json = if resultado.puede_ingresar || resultado.motivo_rechazo.is_some() {
        Some(serde_json::json!({
            "id": contratista.id,
            "cedula": contratista.cedula,
            "nombre": contratista.nombre,
            "apellido": contratista.apellido,
            "empresa_nombre": contratista.empresa_nombre,
            "estado": contratista.estado,
            "praind_vigente": praind_vigente,
            "fecha_vencimiento_praind": contratista.fecha_vencimiento_praind,
            "vehiculos": vehiculos, // Incluimos los vehículos al finalizar
            "alertas": alertas_db.iter().cloned().map(crate::models::ingreso::AlertaGafeteResponse::from).collect::<Vec<_>>()
        }))
    } else {
        None
    };

    Ok(ValidacionIngresoResponse {
        puede_ingresar: resultado.puede_ingresar,
        motivo_rechazo: resultado.motivo_rechazo,
        alertas: resultado.alertas,
        contratista: contratista_json,
        tiene_ingreso_abierto: false,
        ingreso_abierto: None,
    })
}

// ==========================================
// CREAR INGRESO CONTRATISTA
// ==========================================

/// Crea un nuevo registro de ingreso para un contratista
///
/// Orquesta:
/// 1. Validaciones de dominio (input, praind, gafete)
/// 2. Verificación de duplicados
/// 3. Inserción en DB
/// 4. Retorno del ingreso creado
pub async fn crear_ingreso_contratista(
    pool: &SqlitePool,
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    // 1. Validar input con dominio
    domain::validar_input_entrada(&input)?;

    // 2. Verificar que NO tenga ingreso abierto (evita duplicados)
    if db::find_ingreso_abierto_by_contratista(pool, &input.contratista_id)
        .await
        .is_ok()
    {
        return Err("El contratista ya tiene un ingreso abierto".to_string());
    }

    // 3. Obtener datos del contratista
    let contratista = contratista_queries::find_basic_info_by_id(pool, &input.contratista_id)
        .await?
        .ok_or_else(|| "Contratista no encontrado".to_string())?;

    // 4. Validar PRAIND con dominio
    let praind_vigente = domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;

    // 5. Validar y normalizar gafete si se proporciona
    let gafete_normalizado = if let Some(ref g) = input.gafete_numero {
        let normalizado = domain::normalizar_numero_gafete(g);

        // Validar disponibilidad en DB
        let disponible = gafete_service::is_gafete_disponible(pool, &normalizado).await?;
        if !disponible {
            return Err(format!("Gafete {} no está disponible", normalizado));
        }

        Some(normalizado)
    } else {
        None
    };

    // 6. Parsear enums
    let tipo_autorizacion = TipoAutorizacion::from_str(&input.tipo_autorizacion)?;
    let modo_ingreso = ModoIngreso::from_str(&input.modo_ingreso)?;

    // 7. Crear registro en DB usando insert_extended
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    db::insert_extended(
        pool,
        &id,
        Some(input.contratista_id.as_str()),
        &contratista.cedula,
        &contratista.nombre,
        &contratista.apellido,
        &contratista.empresa_nombre,
        None, // empresa_proveedor_id
        None, // anfitrion
        None, // area_visitada
        None, // motivo_visita
        None, // motivo_proveedor
        TipoIngreso::Contratista.as_str(),
        tipo_autorizacion.as_str(),
        modo_ingreso.as_str(),
        input.vehiculo_id.as_deref(),
        None, // placa_temporal
        gafete_normalizado.as_deref(),
        &now,
        &usuario_id,
        Some(praind_vigente),
        Some(contratista.estado.as_str()),
        input.observaciones.as_deref(),
        &now,
        &now,
    )
    .await?;

    // 8. Retornar ingreso creado
    get_ingreso_by_id(pool, id).await
}

// ==========================================
// CONSULTAS AUXILIARES
// ==========================================

/// Obtiene un ingreso por ID con detalles completos
async fn get_ingreso_by_id(pool: &SqlitePool, id: String) -> Result<IngresoResponse, String> {
    let ingreso = db::find_by_id(pool, &id).await?;
    let details = db::find_details_by_id(pool, &id).await?;

    let mut response = IngresoResponse::from(ingreso);
    response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    response.usuario_salida_nombre = details.usuario_salida_nombre;
    response.vehiculo_placa = details.vehiculo_placa;

    Ok(response)
}
