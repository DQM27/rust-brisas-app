// ==========================================
// src/domain/ingreso/strategy/visita_strategy.rs
// ==========================================
// Estrategia de validación para ingresos de VISITAS

use super::IngresoStrategy;
use crate::domain::ingreso::tipos::{DatosIngreso, DatosValidacion};
use crate::domain::ingreso::validaciones_entrada::normalizar_numero_gafete;
use crate::models::ingreso::TipoIngreso;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct VisitaStrategy;

#[async_trait]
impl IngresoStrategy for VisitaStrategy {
    fn tipo(&self) -> TipoIngreso {
        TipoIngreso::Visita
    }

    fn requiere_praind(&self) -> bool {
        false
    }

    fn campos_requeridos(&self) -> Vec<&str> {
        vec!["cedula", "nombre", "apellido", "anfitrion", "area_visitada", "motivo_visita"]
    }

    async fn validar_especificas(
        &self,
        _pool: &SqlitePool,
        datos: &DatosValidacion,
        _alertas: &mut Vec<String>,
    ) -> Result<(), String> {
        // 1. Anfitrión requerido
        let anfitrion = datos
            .anfitrion
            .as_ref()
            .ok_or("Anfitrión es requerido")?;

        if anfitrion.trim().is_empty() {
            return Err("Anfitrión no puede estar vacío".to_string());
        }

        // 2. Área visitada requerida
        let area = datos
            .area_visitada
            .as_ref()
            .ok_or("Área visitada es requerida")?;

        if area.trim().is_empty() {
            return Err("Área visitada no puede estar vacía".to_string());
        }

        // 3. Motivo de visita requerido
        let motivo = datos
            .motivo_visita
            .as_ref()
            .ok_or("Motivo de visita es requerido")?;

        if motivo.trim().is_empty() {
            return Err("Motivo de visita no puede estar vacío".to_string());
        }

        // 4. Tipo de autorización válido (solo validamos que sea correo o praind)
        if !["correo", "praind"].contains(&datos.tipo_autorizacion.to_lowercase().as_str()) {
            return Err("Tipo de autorización inválido para visitas".to_string());
        }

        Ok(())
    }

    async fn preparar_datos_ingreso(
        &self,
        _pool: &SqlitePool,
        datos: &DatosValidacion,
        _usuario_id: &str,
    ) -> Result<DatosIngreso, String> {
        // Normalizar gafete si existe
        let gafete_normalizado = datos
            .gafete_numero
            .as_ref()
            .map(|g| normalizar_numero_gafete(g));

        Ok(DatosIngreso {
            id: Uuid::new_v4().to_string(),
            contratista_id: None,
            cedula: datos.cedula.clone(),
            nombre: datos.nombre.clone().unwrap_or_default(),
            apellido: datos.apellido.clone().unwrap_or_default(),
            empresa_nombre: "N/A".to_string(),
            empresa_proveedor_id: None,
            anfitrion: datos.anfitrion.clone(),
            area_visitada: datos.area_visitada.clone(),
            motivo_visita: datos.motivo_visita.clone(),
            motivo_proveedor: None,
            tipo_ingreso: TipoIngreso::Visita.as_str().to_string(),
            tipo_autorizacion: datos.tipo_autorizacion.clone(),
            modo_ingreso: datos.modo_ingreso.clone(),
            vehiculo_id: None,
            placa_temporal: datos.placa_temporal.clone(),
            gafete_numero: gafete_normalizado,
            praind_vigente_al_ingreso: None,
            estado_contratista_al_ingreso: None,
            observaciones: None,
        })
    }
}
