use crate::db::{ingreso_visita_queries, visitante_queries};
use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
};
use crate::domain::visitante::CreateVisitanteInput;
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
                return Err(format!("El gafete {} no está disponible", g));
            }
        }

        // 3. Crear ingreso
        ingreso_visita_queries::create(&self.pool, input)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn registrar_ingreso_full(
        &self,
        input: CreateIngresoVisitaFullInput,
    ) -> Result<IngresoVisita, String> {
        // 1. Buscar o Crear Visitante
        let visitante_id =
            match visitante_queries::get_visitante_by_cedula(&self.pool, &input.cedula)
                .await
                .map_err(|e| e.to_string())?
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
                        has_vehicle: false, // Default false por ahora o agregar al input full si se requiere
                    };
                    visitante_queries::create_visitante(&self.pool, create_input)
                        .await
                        .map_err(|e| e.to_string())?
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
        self.registrar_ingreso(ingreso_input).await
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

    pub async fn get_activos(&self) -> Result<Vec<IngresoVisitaPopulated>, String> {
        ingreso_visita_queries::find_actives(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
