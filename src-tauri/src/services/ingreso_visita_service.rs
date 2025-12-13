use crate::db::{ingreso_visita_queries, visitante_queries};
use crate::domain::ingreso_visita::{CreateIngresoVisitaInput, IngresoVisita};
use crate::services::gafete_service;
use sqlx::SqlitePool;

pub struct IngresoVisitaService {
    pool: SqlitePool,
}

impl IngresoVisitaService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn registrar_ingreso(
        &self,
        input: CreateIngresoVisitaInput,
    ) -> Result<IngresoVisita, String> {
        // 1. Validar existencia del visitante
        if visitante_queries::get_visitante_by_id(&self.pool, &input.visitante_id)
            .await
            .map_err(|e| e.to_string())?
            .is_none()
        {
            return Err("El visitante no existe".to_string());
        }

        // 2. Validar disponibilidad de gafete (si aplica)
        if let Some(ref g) = input.gafete {
            let disponible = gafete_service::is_gafete_disponible(&self.pool, g)
                .await
                .map_err(|e| e.to_string())?;
            if !disponible {
                // Nota: Podríamos ser más flexibles aquí, pero por seguridad validamos
                return Err(format!("El gafete {} no está disponible", g));
            }
        }

        // 3. Crear ingreso
        ingreso_visita_queries::create(&self.pool, input)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn registrar_salida(
        &self,
        id: String,
        usuario_id: String,
        observaciones: Option<String>,
    ) -> Result<(), String> {
        ingreso_visita_queries::registrar_salida(
            &self.pool,
            &id,
            &usuario_id,
            observaciones.as_deref(),
        )
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_activos(&self) -> Result<Vec<IngresoVisita>, String> {
        ingreso_visita_queries::find_actives(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
