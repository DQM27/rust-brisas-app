use crate::db::visitante_queries;
use crate::domain::visitante::{CreateVisitanteInput, Visitante};
use sqlx::SqlitePool;

pub struct VisitanteService {
    pool: SqlitePool,
}

impl VisitanteService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_visitante(&self, input: CreateVisitanteInput) -> Result<Visitante, String> {
        // Validar si ya existe cédula
        if let Ok(Some(_)) =
            visitante_queries::get_visitante_by_cedula(&self.pool, &input.cedula).await
        {
            return Err("Ya existe un visitante con esa cédula".to_string());
        }

        visitante_queries::create_visitante(&self.pool, input)
            .await
            .map_err(|e| format!("Error creando visitante: {}", e))
    }

    pub async fn search_visitantes(&self, term: String) -> Result<Vec<Visitante>, String> {
        visitante_queries::search_visitantes(&self.pool, &term)
            .await
            .map_err(|e| format!("Error buscando visitantes: {}", e))
    }
}
