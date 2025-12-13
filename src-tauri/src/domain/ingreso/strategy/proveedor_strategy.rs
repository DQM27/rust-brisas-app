// ==========================================
// src/domain/ingreso/strategy/proveedor_strategy.rs
// ==========================================
// Estrategia de validación para ingresos de PROVEEDORES

use super::IngresoStrategy;
use crate::db::empresa_queries;
use crate::domain::ingreso::tipos::{DatosIngreso, DatosValidacion};
use crate::domain::ingreso::validaciones_entrada::normalizar_numero_gafete;
use crate::models::ingreso::TipoIngreso;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct ProveedorStrategy;

#[async_trait]
impl IngresoStrategy for ProveedorStrategy {
    fn tipo(&self) -> TipoIngreso {
        TipoIngreso::Proveedor
    }

    fn requiere_praind(&self) -> bool {
        false
    }

    fn campos_requeridos(&self) -> Vec<&str> {
        vec!["cedula", "nombre", "apellido", "empresa_id", "area_visitada", "motivo"]
    }

    async fn validar_especificas(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
        _alertas: &mut Vec<String>,
    ) -> Result<(), String> {
        // 1. Empresa proveedora requerida
        let empresa_id = datos
            .empresa_id
            .as_ref()
            .ok_or("ID de empresa proveedora es requerido")?;

        if empresa_id.trim().is_empty() {
            return Err("ID de empresa no puede estar vacío".to_string());
        }

        // 2. Verificar que la empresa existe
        let _empresa = empresa_queries::find_by_id(pool, empresa_id)
            .await?
            .ok_or("Empresa proveedora no encontrada")?;

        // 3. Área visitada requerida
        let area = datos
            .area_visitada
            .as_ref()
            .ok_or("Área visitada es requerida")?;

        if area.trim().is_empty() {
            return Err("Área visitada no puede estar vacía".to_string());
        }

        // 4. Motivo requerido
        let motivo = datos
            .motivo_proveedor
            .as_ref()
            .ok_or("Motivo de ingreso es requerido")?;

        if motivo.trim().is_empty() {
            return Err("Motivo de ingreso no puede estar vacío".to_string());
        }

        Ok(())
    }

    async fn preparar_datos_ingreso(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
        _usuario_id: &str,
    ) -> Result<DatosIngreso, String> {
        let empresa_id = datos
            .empresa_id
            .as_ref()
            .ok_or("ID de empresa proveedora es requerido")?;

        // Obtener nombre de empresa
        let empresa = empresa_queries::find_by_id(pool, empresa_id)
            .await?
            .ok_or("Empresa no encontrada")?;

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
            empresa_nombre: empresa.nombre.clone(),
            empresa_proveedor_id: Some(empresa_id.clone()),
            anfitrion: None,
            area_visitada: datos.area_visitada.clone(),
            motivo_visita: None,
            motivo_proveedor: datos.motivo_proveedor.clone(),
            tipo_ingreso: TipoIngreso::Proveedor.as_str().to_string(),
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
