// ==========================================
// src/domain/ingreso/validaciones_comunes.rs
// ==========================================
// Validaciones compartidas por TODOS los tipos de ingreso

use crate::db::{alerta_gafete_queries, ingreso_queries, lista_negra_queries};
use crate::models::ingreso::AlertaGafete;

use sqlx::SqlitePool;

// ==========================================
// RE-EXPORT DE BLOCKSTATUS
// ==========================================

// Usamos el tipo de lista_negra_queries directamente
pub use lista_negra_queries::BlockStatus;

// ==========================================
// VALIDACIONES COMPARTIDAS
// ==========================================

/// Verifica si una persona está en la lista negra (por cédula)
pub async fn verificar_lista_negra(pool: &SqlitePool, cedula: &str) -> Result<BlockStatus, String> {
    lista_negra_queries::check_if_blocked_by_cedula(pool, cedula).await
}

/// Verifica si una persona ya tiene un ingreso abierto (por cédula)
pub async fn verificar_ingreso_duplicado(pool: &SqlitePool, cedula: &str) -> Result<bool, String> {
    let ingreso_opt = ingreso_queries::find_ingreso_abierto_by_cedula(pool, cedula).await?;
    Ok(ingreso_opt.is_some())
}

/// Obtiene alertas de gafetes pendientes (por cédula)
pub async fn verificar_alertas_gafetes(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Vec<AlertaGafete>, String> {
    alerta_gafete_queries::find_pendientes_by_cedula(pool, cedula).await
}

// ==========================================
// VALIDACIÓN COMPUESTA
// ==========================================

/// Ejecuta todas las validaciones compartidas y retorna resultado
pub async fn validar_compartidas(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<(BlockStatus, bool, Vec<AlertaGafete>), String> {
    // 1. Lista negra
    let block_status = verificar_lista_negra(pool, cedula).await?;

    // 2. Ingreso duplicado
    let tiene_ingreso_abierto = verificar_ingreso_duplicado(pool, cedula).await?;

    // 3. Alertas de gafetes
    let alertas_gafetes = verificar_alertas_gafetes(pool, cedula).await?;

    Ok((block_status, tiene_ingreso_abierto, alertas_gafetes))
}
