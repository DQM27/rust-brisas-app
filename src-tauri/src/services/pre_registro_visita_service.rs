// ==========================================
// src/services/pre_registro_visita_service.rs
// ==========================================

use crate::db::surrealdb_pre_registro_visita_queries as db;
use crate::db::surrealdb_visitante_queries as visitante_db;
use crate::models::ingreso::{
    CreatePreRegistroInput, PreRegistroEstado, PreRegistroVisitaCreateDTO, PreRegistroVisitaFetched,
};
use crate::models::visitante::VisitanteCreateDTO;
use log::{debug, info, warn};
use surrealdb::RecordId;

use crate::services::search_service::SearchService;
use std::sync::Arc;

/// Crea un pre-registro de visita asegurando que el visitante exista en el catálogo.
pub async fn create_pre_registro(
    search_service: &Arc<SearchService>,
    input: CreatePreRegistroInput,
    registrado_por: RecordId,
) -> Result<PreRegistroVisitaFetched, String> {
    debug!("Iniciando creación de pre-registro para cédula: {}", input.cedula);

    // 1. Asegurar que el perfil del visitante existe (Doble registro inteligente)
    let visitante_id = match visitante_db::get_visitante_by_cedula(&input.cedula).await {
        Ok(Some(v)) => {
            debug!("Visitante existente encontrado: {}", v.id);
            Some(v.id)
        }
        Ok(None) => {
            info!("Creando perfil automático para nuevo visitante: {}", input.cedula);
            let v_dto = VisitanteCreateDTO {
                cedula: input.cedula.clone(),
                nombre: input.nombre.clone(),
                apellido: input.apellido.clone(),
                segundo_nombre: input.segundo_nombre.clone(),
                segundo_apellido: input.segundo_apellido.clone(),
                empresa: None,
                has_vehicle: input.modo_ingreso == "vehiculo",
            };

            match visitante_db::create_visitante(v_dto).await {
                Ok(v) => {
                    // Indexar nuevo visitante creado automáticamente
                    if let Ok(Some(fetched)) = visitante_db::find_by_id_fetched(&v.id).await {
                        let _ = search_service.add_visitante_fetched(&fetched, "Sin Empresa").await;
                    }
                    Some(v.id)
                }
                Err(e) => {
                    warn!("No se pudo crear perfil de visitante (no fatal): {e}");
                    None
                }
            }
        }
        Err(e) => {
            warn!("Error al verificar existencia de visitante: {e}");
            None
        }
    };

    let now = surrealdb::Datetime::from(chrono::Utc::now());

    // 2. Crear DTO de Pre-Registro con el enlace al visitante
    let dto = PreRegistroVisitaCreateDTO {
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        segundo_nombre: input.segundo_nombre,
        segundo_apellido: input.segundo_apellido,
        empresa_nombre: input.empresa_nombre,
        fecha_esperada: input.fecha_esperada,
        hora_esperada: input.hora_esperada,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        modo_ingreso: input.modo_ingreso,
        observaciones: None, // Campo removido del formulario
        estado: PreRegistroEstado::Pendiente.to_string(),
        visitante: visitante_id,
        registrado_por,
        created_at: now.clone(),
        updated_at: now,
    };

    db::create(dto).await.map_err(|e| e.to_string())
}
