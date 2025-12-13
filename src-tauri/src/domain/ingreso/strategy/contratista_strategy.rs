// ==========================================
// src/domain/ingreso/strategy/contratista_strategy.rs
// ==========================================
// Estrategia de validación para ingresos de CONTRATISTAS

use super::IngresoStrategy;
use crate::db::contratista_queries;
use crate::domain::ingreso::tipos::{DatosIngreso, DatosValidacion};
use crate::domain::ingreso::validaciones_entrada::{
    normalizar_numero_gafete, verificar_praind_vigente,
};
use crate::models::ingreso::TipoIngreso;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct ContratistaStrategy;

#[async_trait]
impl IngresoStrategy for ContratistaStrategy {
    fn tipo(&self) -> TipoIngreso {
        TipoIngreso::Contratista
    }

    fn requiere_praind(&self) -> bool {
        true
    }

    fn campos_requeridos(&self) -> Vec<&str> {
        vec!["contratista_id"]
    }

    async fn validar_especificas(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
        _alertas: &mut Vec<String>,
    ) -> Result<(), String> {
        // 1. Contratista ID es requerido
        let contratista_id = datos
            .contratista_id
            .as_ref()
            .ok_or("ID de contratista es requerido")?;

        // 2. Verificar que el contratista existe
        let contratista = contratista_queries::find_basic_info_by_id(pool, contratista_id)
            .await?
            .ok_or("Contratista no encontrado")?;

        // 3. Verificar estado activo
        if contratista.estado.to_lowercase() != "activo" {
            return Err(format!(
                "Contratista con estado: {}",
                contratista.estado
            ));
        }

        // 4. Verificar PRAIND vigente
        let praind_vigente = verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;
        if !praind_vigente {
            return Err("PRAIND vencido".to_string());
        }

        Ok(())
    }

    async fn preparar_datos_ingreso(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
        _usuario_id: &str,
    ) -> Result<DatosIngreso, String> {
        let contratista_id = datos
            .contratista_id
            .as_ref()
            .ok_or("ID de contratista es requerido")?;

        // Obtener datos del contratista
        let contratista = contratista_queries::find_basic_info_by_id(pool, contratista_id)
            .await?
            .ok_or("Contratista no encontrado")?;

        // Verificar PRAIND
        let praind_vigente = verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;

        // Normalizar gafete si existe
        let gafete_normalizado = datos
            .gafete_numero
            .as_ref()
            .map(|g| normalizar_numero_gafete(g));

        Ok(DatosIngreso {
            id: Uuid::new_v4().to_string(),
            contratista_id: Some(contratista_id.clone()),
            cedula: contratista.cedula.clone(),
            nombre: contratista.nombre.clone(),
            apellido: contratista.apellido.clone(),
            empresa_nombre: contratista.empresa_nombre.clone(),
            empresa_proveedor_id: None,
            anfitrion: None,
            area_visitada: None,
            motivo_visita: None,
            motivo_proveedor: None,
            tipo_ingreso: TipoIngreso::Contratista.as_str().to_string(),
            tipo_autorizacion: datos.tipo_autorizacion.clone(),
            modo_ingreso: datos.modo_ingreso.clone(),
            vehiculo_id: datos.vehiculo_id.clone(),
            placa_temporal: None,
            gafete_numero: gafete_normalizado,
            praind_vigente_al_ingreso: Some(praind_vigente),
            estado_contratista_al_ingreso: Some(contratista.estado.clone()),
            observaciones: None,
        })
    }
}
