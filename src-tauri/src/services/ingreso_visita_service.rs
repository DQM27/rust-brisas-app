// ==========================================
// src/services/ingreso_visita_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para Ingresos de Visita

use crate::db::{ingreso_visita_queries, visitante_queries};
use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
};
use crate::domain::visitante::CreateVisitanteInput;
use crate::services::gafete_service;
use sqlx::SqlitePool;

pub async fn registrar_ingreso(
    pool: &SqlitePool,
    input: CreateIngresoVisitaInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    // 1. Validar existencia del visitante
    if visitante_queries::get_visitante_by_id(pool, &input.visitante_id)
        .await?
        .is_none()
    {
        return Err(IngresoVisitaError::Validation(
            "El visitante no existe".to_string(),
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
    ingreso_visita_queries::create(pool, input)
        .await
        .map_err(IngresoVisitaError::Database)
}

pub async fn registrar_ingreso_full(
    pool: &SqlitePool,
    input: CreateIngresoVisitaFullInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    // 1. Buscar o Crear Visitante
    let visitante_id = match visitante_queries::get_visitante_by_cedula(pool, &input.cedula)
        .await
        .map_err(IngresoVisitaError::Database)?
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
            visitante_queries::create_visitante(pool, create_input)
                .await
                .map_err(IngresoVisitaError::Database)?
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
    // 1. Obtener el ingreso para saber el gafete y visitante
    let actives = ingreso_visita_queries::find_actives(pool)
        .await
        .map_err(IngresoVisitaError::Database)?;

    let ingreso = actives
        .iter()
        .find(|i| i.id == id)
        .ok_or(IngresoVisitaError::NotFound)?;

    let gafete = ingreso.gafete.clone();
    let visitante_nombre = format!(
        "{} {}",
        ingreso.visitante_nombre, ingreso.visitante_apellido
    );
    let visitante_cedula = ingreso.visitante_cedula.clone();

    // 2. Registrar salida
    ingreso_visita_queries::registrar_salida(pool, &id, &usuario_id, observaciones.as_deref())
        .await
        .map_err(IngresoVisitaError::Database)?;

    // 3. Si no devolvió el gafete, crear alerta
    if !devolvio_gafete {
        if let Some(ref gafete_num) = gafete {
            // TODO: Agregar ingreso_visita_id a alertas_gafetes y crear la alerta real
            // Por ahora solo log
            #[cfg(feature = "logging")]
            log::warn!(
                "⚠️ ALERTA: Gafete {} no devuelto por visita {} ({})",
                gafete_num,
                visitante_nombre,
                visitante_cedula
            );
            println!(
                "⚠️ ALERTA: Gafete {} no devuelto por visita {} ({})",
                gafete_num, visitante_nombre, visitante_cedula
            );
        }
    }

    Ok(())
}

pub async fn get_activos(
    pool: &SqlitePool,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    ingreso_visita_queries::find_actives(pool)
        .await
        .map_err(IngresoVisitaError::Database)
}

pub async fn get_historial(
    pool: &SqlitePool,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    ingreso_visita_queries::find_historial(pool)
        .await
        .map_err(IngresoVisitaError::Database)
}
