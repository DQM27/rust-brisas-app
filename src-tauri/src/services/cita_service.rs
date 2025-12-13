use crate::db::{cita_queries, visitante_queries};
use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use crate::domain::visitante::CreateVisitanteInput;

use sqlx::SqlitePool;

pub struct CitaService {
    pool: SqlitePool,
}

impl CitaService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn agendar_cita(
        &self,
        cita_input: CreateCitaInput,
        visitante_input: Option<CreateVisitanteInput>,
    ) -> Result<Cita, String> {
        let mut visitante_id = cita_input.visitante_id.clone();

        // 1. Si viene input de visitante, crear o buscar
        if let Some(v_input) = visitante_input {
            // Verificar si existe por cédula
            let existente = visitante_queries::get_visitante_by_cedula(&self.pool, &v_input.cedula)
                .await
                .map_err(|e| e.to_string())?;

            match existente {
                Some(v) => {
                    visitante_id = v.id;
                }
                None => {
                    let nuevo = visitante_queries::create_visitante(&self.pool, v_input)
                        .await
                        .map_err(|e| e.to_string())?;
                    visitante_id = nuevo.id;
                }
            }
        }

        if visitante_id.is_empty() {
            return Err("Se requiere un visitante ID o datos para crear uno nuevo".to_string());
        }

        // 2. Crear cita
        let mut input_final = cita_input;
        input_final.visitante_id = visitante_id;

        cita_queries::create_cita(&self.pool, input_final)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_citas_hoy(&self) -> Result<Vec<CitaPopulated>, String> {
        let now = chrono::Local::now();
        let fecha = now.format("%Y-%m-%d").to_string();
        cita_queries::get_citas_pendientes_del_dia(&self.pool, &fecha)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn procesar_ingreso_cita(
        &self,
        cita_id: String,
        gafete: String,
        usuario_id: String,
    ) -> Result<String, String> {
        // 1. Obtener la cita
        let cita = cita_queries::find_by_id(&self.pool, &cita_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Cita no encontrada".to_string())?;

        // 2. Obtener el visitante
        let visitante = visitante_queries::get_visitante_by_id(&self.pool, &cita.visitante_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Visitante asociado no encontrado".to_string())?;

        // 3. Preparar input para crear ingreso visita (NUEVO MODULO)
        let input_ingreso = crate::domain::ingreso_visita::CreateIngresoVisitaInput {
            visitante_id: visitante.id,
            cita_id: Some(cita.id.clone()),
            anfitrion: cita.anfitrion,
            area_visitada: cita.area_visitada,
            motivo: cita.motivo,
            gafete: Some(gafete),
            observaciones: Some("Ingreso desde Cita Pre-registrada".to_string()),
            usuario_ingreso_id: usuario_id,
        };

        // 4. Delegar creación al NUEVO servicio de ingreso visita
        let service =
            crate::services::ingreso_visita_service::IngresoVisitaService::new(self.pool.clone());
        let ingreso = service.registrar_ingreso(input_ingreso).await?;

        // 5. Marcar cita como completada
        cita_queries::marcar_cita_completada(&self.pool, &cita_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(ingreso.id)
    }
}
