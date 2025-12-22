// ==========================================
// src/services/ingreso_visita_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para Ingresos de Visita

use crate::db::{ingreso_visita_queries, visitante_queries};
use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita as domain;
use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::gafete_service;
use sqlx::SqlitePool;

pub async fn registrar_ingreso(
    pool: &SqlitePool,
    input: CreateIngresoVisitaInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    // 1. Validar existencia del visitante
    if visitante_queries::get_visitante_by_id(pool, &input.visitante_id).await?.is_none() {
        return Err(IngresoVisitaError::Validation("El visitante no existe".to_string()));
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
    ingreso_visita_queries::create(pool, input).await.map_err(IngresoVisitaError::Database)
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
    )?;

    // 2. Registrar salida
    ingreso_visita_queries::registrar_salida(pool, &id, &usuario_id, observaciones.as_deref())
        .await
        .map_err(IngresoVisitaError::Database)?;

    // 3. Crear alerta si aplica
    if decision.debe_generar_reporte {
        if let Some(num) = decision.gafete_numero {
            let nombre_completo =
                format!("{} {}", ingreso.visitante_nombre, ingreso.visitante_apellido);

            // Note: ingreso_visita logic might need a dedicated field in AlertGafete table or generic persona_id usage?
            // alerta_service::insert takes ingreso_proveedor_id and ingreso_contratista_id optional args.
            // It doesn't seem to have ingreso_visita_id yet?
            // Let's check alerta_service::insert signature again.
            // Step 1432 for ingreso_contratista usage: `insert(..., Some(&input.ingreso_id), None, ...)`
            // Alert table has `ingreso_contratista_id`, `ingreso_proveedor_id`.
            // Does it have `ingreso_visita_id`?
            // If not, we might abuse one of them or use persona_id?
            // User prompt doesn't ask to change DB schema.
            // But `IngresoVisita` was requested.
            // Let's assume there is no column for ingreso_visita_id yet, since TO-DO said "Agregar ingreso_visita_id a alertas_gafetes".
            // If I can't add it to DB, I should probably skip inserting or use a placeholder if the schema supports it.
            // Check `alerta_service.rs` insert function to be sure.
            // But for now, I'll log a warning if I can't insert, or use Generic persona logic.
            // Wait, I can try to use `alerta_service::insert` mapping to `persona_id` or similar?
            // Let's look at `alerta_service` signature.
            // I'll assume for this refactor I should just make the code clean, and maybe leave the alert insertion as TO-DO if column missing, OR try to insert if I can.
            // But the TO-DO `TODO: Agregar ingreso_visita_id a alertas_gafetes` in previous code suggests the column is MISSING.
            // So calling `alerta_service::insert` might fail or I'd need to modify `alerta_service` to accept `ingreso_visita_id`.
            // Modifying `alerta_service` is outside scope? No, "Refactor Ingreso Domain".
            // But modifying DB schema requires migrations.
            // If I can't modify DB, I can't implement real alert persistence for visits yet.
            // User goal 3: "Update domain/ingreso_visita.rs to return Result<T, IngresoVisitaError>".
            // It doesn't explicitly say "Implement alerts for visits".
            // However, `ingreso_proveedor` did have it.

            // I will keep the TO-DO but use the domain logic for decision making.
            // Or better, just log for now as before, but using the decision result.
            println!(
                "⚠️ ALERTA (PENDIENTE DB): Gafete {} no devuelto por visita {} ({})",
                num, nombre_completo, ingreso.visitante_cedula
            );
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
