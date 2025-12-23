// ==========================================
// src/services/ingreso_visita_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para Ingresos de Visita

use crate::db::ingreso_visita_queries;
use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita as domain;
use crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse;
use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
};
use crate::domain::motor_validacion::{self as motor, ContextoIngreso};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::{alerta_service, gafete_service, lista_negra_service, visitante_service};
use log::{error, info};
use sqlx::SqlitePool;

pub async fn registrar_ingreso(
    pool: &SqlitePool,
    input: CreateIngresoVisitaInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    // 1. Validar existencia del visitante y reglas de ingreso
    info!("Validando ingreso para visitante con ID {}", input.visitante_id);
    let validacion = validar_ingreso(pool, &input.visitante_id).await?; // validando_ingreso already returns Result<..., IngresoVisitaError>
                                                                        // Wait, looking at the code, it uses map_err. I will clean it up too.

    if !validacion.puede_ingresar {
        return Err(IngresoVisitaError::Validation(
            validacion
                .motivo_rechazo
                .unwrap_or("Ingreso rechazado por reglas de negocio".to_string()),
        ));
    }

    // 2. Validar disponibilidad de gafete (si aplica)
    if let Some(ref g) = input.gafete {
        let disponible = gafete_service::is_gafete_disponible(pool, g, "visita")
            .await
            .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?; // Map GafeteError
        if !disponible {
            return Err(IngresoVisitaError::Validation(format!(
                "El gafete {} no está disponible",
                g
            )));
        }
    }

    // 3. Crear ingreso
    let ingreso = ingreso_visita_queries::create(pool, input).await.map_err(|e| {
        error!("Error de base de datos al registrar ingreso de visita: {}", e);
        IngresoVisitaError::Database(e)
    })?;

    info!("Ingreso de visita {} registrado exitosamente", ingreso.id);
    Ok(ingreso)
}

pub async fn registrar_ingreso_full(
    pool: &SqlitePool,
    input: CreateIngresoVisitaFullInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    // 1. Buscar o Crear Visitante
    let visitante_id = match visitante_service::get_visitante_by_cedula(pool, &input.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
    {
        Some(v) => v.id,
        None => {
            // Crear nuevo visitante
            let create_input = CreateVisitanteInput {
                cedula: input.cedula,
                nombre: input.nombre,
                apellido: input.apellido,
                segundo_nombre: None,
                segundo_apellido: None,
                empresa: input.empresa,
                has_vehicle: false, // Default false por ahora
            };
            visitante_service::create_visitante(pool, create_input)
                .await
                .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
                .id
        }
    };

    // 2. Preparar input de ingreso
    let ingreso_input = CreateIngresoVisitaInput {
        visitante_id,
        cita_id: input.cita_id,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        gafete: input.gafete,
        observaciones: input.observaciones,
        usuario_ingreso_id: input.usuario_ingreso_id,
    };

    // 3. Registrar ingreso (reusing validaciones)
    registrar_ingreso(pool, ingreso_input).await
}

pub async fn registrar_salida(
    pool: &SqlitePool,
    id: String,
    usuario_id: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<(), IngresoVisitaError> {
    // 1. Obtener el ingreso
    let actives =
        ingreso_visita_queries::find_actives(pool).await.map_err(IngresoVisitaError::Database)?;

    let ingreso = actives.iter().find(|i| i.id == id).ok_or(IngresoVisitaError::NotFound)?;

    domain::validar_ingreso_abierto(&ingreso.fecha_salida)?;

    // Evaluar gafete
    // The IngresoVisitaPopulated struct uses `gafete` Option<String> too.
    let decision = domain::evaluar_devolucion_gafete(
        ingreso.gafete.is_some(),
        ingreso.gafete.as_deref(),
        devolvio_gafete,
        if devolvio_gafete { ingreso.gafete.as_deref() } else { None },
    ); // Retorna DecisionReporteGafete directamente

    info!("Registrando salida para ingreso de visita {}", id);
    // 2. Registrar salida
    ingreso_visita_queries::registrar_salida(pool, &id, &usuario_id, observaciones.as_deref())
        .await
        .map_err(|e| {
            error!("Error al registrar salida de visita {}: {}", id, e);
            IngresoVisitaError::Database(e)
        })?;

    info!("Salida de visita {} registrada exitosamente", id);

    // 3. Crear alerta si aplica
    if decision.debe_generar_reporte {
        if let Some(num) = decision.gafete_numero {
            let alerta_id = uuid::Uuid::new_v4().to_string();
            let nombre_completo =
                format!("{} {}", ingreso.visitante_nombre, ingreso.visitante_apellido);
            let now = chrono::Utc::now().to_rfc3339();

            alerta_service::insert(
                pool,
                &alerta_id,
                None, // No persona_id generico
                &ingreso.visitante_cedula,
                &nombre_completo,
                &num,
                None,      // ingreso_contratista_id
                None,      // ingreso_proveedor_id
                Some(&id), // ingreso_visita_id
                &now,
                decision.motivo.as_deref(),
                &usuario_id,
                &now,
                &now,
            )
            .await
            .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?; // Map correctly
        }
    }

    Ok(())
}

pub async fn get_activos(
    pool: &SqlitePool,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    ingreso_visita_queries::find_actives(pool).await.map_err(IngresoVisitaError::Database)
}

pub async fn get_historial(
    pool: &SqlitePool,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    ingreso_visita_queries::find_historial(pool).await.map_err(IngresoVisitaError::Database)
}

pub async fn validar_ingreso(
    pool: &SqlitePool,
    visitante_id: &str,
) -> Result<ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    // A. Buscar Visitante
    let visitante = visitante_service::get_visitante_by_id(pool, visitante_id)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
        .ok_or(IngresoVisitaError::Validation("Visitante no encontrado".to_string()))?;

    // B. Verificar Bloqueo
    let block_response = lista_negra_service::check_is_blocked(pool, visitante.cedula.clone())
        .await
        .unwrap_or(crate::models::lista_negra::BlockCheckResponse {
            is_blocked: false,
            motivo: None,
            bloqueado_desde: None,
            bloqueado_hasta: None,
            bloqueado_por: None,
        });

    // C. Verificar Ingreso Abierto
    let ingreso_abierto = ingreso_visita_queries::find_active_by_visitante_id(pool, visitante_id)
        .await
        .map_err(IngresoVisitaError::Database)?;

    // D. Alertas (Gafetes)
    let alertas_db = alerta_service::find_pendientes_by_cedula(pool, &visitante.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Validation(format!("Error alertas: {}", e)))?;

    // E. Motor
    let nombre_completo = format!("{} {}", visitante.nombre, visitante.apellido);
    let contexto = ContextoIngreso::new_visita(
        visitante.cedula.clone(),
        nombre_completo,
        None, // Autorización correo no trackeada en DB aún
        block_response.is_blocked,
        block_response.motivo,
        ingreso_abierto.is_some(),
        alertas_db.len(),
    );

    let resultado_motor = motor::validar_ingreso(&contexto);

    // F. Construir JSON
    let visitante_json = if resultado_motor.puede_ingresar || !resultado_motor.bloqueos.is_empty() {
        Some(serde_json::json!({
            "id": visitante.id,
            "cedula": visitante.cedula,
            "nombre": visitante.nombre,
            "apellido": visitante.apellido,
            "empresa": visitante.empresa,
            "alertas": alertas_db.iter().cloned().map(crate::models::ingreso::AlertaGafeteResponse::from).collect::<Vec<_>>()
        }))
    } else {
        None
    };

    Ok(ValidacionIngresoVisitaResponse {
        puede_ingresar: resultado_motor.puede_ingresar,
        motivo_rechazo: resultado_motor.mensaje_bloqueo(),
        alertas: resultado_motor.alertas,
        visitante: visitante_json,
        tiene_ingreso_abierto: false,
        ingreso_abierto: ingreso_abierto,
    })
}
