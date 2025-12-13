// ==========================================
// src/domain/ingreso/strategy/mod.rs
// ==========================================
// Patrón Strategy para diferentes tipos de ingreso

use crate::domain::ingreso::tipos::{DatosIngreso, DatosValidacion, ResultadoValidacion};
use crate::models::ingreso::TipoIngreso;
use async_trait::async_trait;
use sqlx::SqlitePool;

// Módulos de estrategias
pub mod contratista_strategy;
pub mod proveedor_strategy;
pub mod visita_strategy;

// Re-exports
pub use contratista_strategy::ContratistaStrategy;
pub use proveedor_strategy::ProveedorStrategy;
pub use visita_strategy::VisitaStrategy;

// ==========================================
// TRAIT PRINCIPAL
// ==========================================

#[async_trait]
pub trait IngresoStrategy: Send + Sync {
    /// Retorna el tipo de ingreso que maneja esta estrategia
    fn tipo(&self) -> TipoIngreso;

    /// Indica si este tipo de ingreso requiere PRAIND
    fn requiere_praind(&self) -> bool;

    /// Lista de campos requeridos específicos de este tipo
    fn campos_requeridos(&self) -> Vec<&str>;

    /// Valida si la persona puede ingresar
    /// Esta es la función principal que orquesta validaciones compartidas + específicas
    async fn validar_entrada(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
    ) -> Result<ResultadoValidacion, String> {
        let mut alertas = Vec::new();

        // 1. Validaciones compartidas (todos los tipos)
        self.validar_compartidas(pool, datos, &mut alertas)
            .await?;

        // 2. Validaciones específicas (cada estrategia implementa)
        self.validar_especificas(pool, datos, &mut alertas)
            .await?;

        // 3. Retornar resultado exitoso
        Ok(ResultadoValidacion {
            puede_ingresar: true,
            motivo_rechazo: None,
            alertas,
            datos_adicionales: None,
        })
    }

    /// Validaciones compartidas (implementación por defecto)
    async fn validar_compartidas(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
        alertas: &mut Vec<String>,
    ) -> Result<(), String> {
        use crate::domain::ingreso::validaciones_comunes;

        // Ejecutar todas las validaciones compartidas
        let (block_status, tiene_ingreso_abierto, alertas_gafetes) =
            validaciones_comunes::validar_compartidas(pool, &datos.cedula).await?;

        // 1. Lista negra (BLOQUEANTE)
        if block_status.blocked {
            return Err(format!(
                "Bloqueado en lista negra: {}",
                block_status.motivo.unwrap_or_else(|| "Sin motivo especificado".to_string())
            ));
        }

        // 2. Ingreso duplicado (BLOQUEANTE)
        if tiene_ingreso_abierto {
            return Err("Ya tiene un ingreso abierto".to_string());
        }

        // 3. Alertas de gafetes (WARNING, no bloqueante)
        if !alertas_gafetes.is_empty() {
            alertas.push(format!(
                "Tiene {} gafete(s) sin devolver de ingresos anteriores",
                alertas_gafetes.len()
            ));
        }

        // 4. Validar gafete si se proporciona
        if let Some(ref gafete) = datos.gafete_numero {
            let disponible =
                validaciones_comunes::verificar_disponibilidad_gafete(pool, gafete).await?;
            if !disponible {
                return Err(format!("Gafete {} no está disponible", gafete));
            }
        }

        Ok(())
    }

    /// Validaciones específicas de cada tipo (cada estrategia implementa)
    async fn validar_especificas(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
        alertas: &mut Vec<String>,
    ) -> Result<(), String>;

    /// Prepara los datos para inserción en BD
    async fn preparar_datos_ingreso(
        &self,
        pool: &SqlitePool,
        datos: &DatosValidacion,
        usuario_id: &str,
    ) -> Result<DatosIngreso, String>;
}

// ==========================================
// FACTORY PATTERN
// ==========================================

/// Factory para obtener la estrategia correcta según el tipo de ingreso
pub fn get_strategy(tipo: &TipoIngreso) -> Box<dyn IngresoStrategy> {
    match tipo {
        TipoIngreso::Contratista => Box::new(ContratistaStrategy),
        TipoIngreso::Visita => Box::new(VisitaStrategy),
        TipoIngreso::Proveedor => Box::new(ProveedorStrategy),
    }
}
