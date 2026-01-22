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

/// Crea un pre-registro de visita asegurando que el visitante exista en el catálogo.
///
/// Sigue el patrón de "doble registro" para que si el visitante es nuevo,
/// se cree automáticamente su perfil, permitiendo búsquedas futuras.
pub async fn create_pre_registro(
    input: CreatePreRegistroInput,
    registrado_por: RecordId,
) -> Result<PreRegistroVisitaFetched, String> {
    debug!("Iniciando creación de pre-registro para cédula: {}", input.cedula);

    // 1. Asegurar que el perfil del visitante existe (Doble registro inteligente)
    let visitante_id = match visitante_db::get_visitante_by_cedula(&input.cedula).await {
        Ok(Some(v)) => {
            debug!("Visitante existente encontrado: {}", v.id);
            // Nota: Aquí se podría implementar una actualización parcial si los nombres cambiaron
            // pero por simplicidad y seguridad de datos, usamos el registro existente.
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
                empresa: None, // En pre-registro solo tenemos el nombre de la empresa como String
                has_vehicle: input.modo_ingreso == "vehiculo",
            };

            match visitante_db::create_visitante(v_dto).await {
                Ok(v) => Some(v.id),
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
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        modo_ingreso: input.modo_ingreso,
        observaciones: input.observaciones,
        estado: PreRegistroEstado::Pendiente.to_string(),
        visitante: visitante_id,
        registrado_por,
        created_at: now.clone(),
        updated_at: now,
    };

    db::create(dto).await.map_err(|e| e.to_string())
}
