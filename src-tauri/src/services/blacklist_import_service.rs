// ==========================================
// src/services/blacklist_import_service.rs
// ==========================================
// Capa de servicios: orquestación de lógica de negocio
// Coordina validaciones, queries y transformaciones

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Local;

use crate::models::blacklist_import::{
    BlacklistImportTest,
    CreateBlacklistImportInput,
    UpdateBlacklistImportInput,
    BlacklistImportResponse,
    BlacklistImportStats,
    EmpresaStats,
    ValidationStatus,
};
use crate::domain::blacklist_import::{
    validar_create_input,
    validar_update_input,
    normalizar_cedula,
    normalizar_texto,
};
use crate::db::blacklist_import_queries;

// ==========================================
// CREAR ENTRADA
// ==========================================

pub async fn create_blacklist_import(
    pool: &SqlitePool,
    input: CreateBlacklistImportInput,
    imported_by: String,
) -> Result<BlacklistImportResponse, String> {
    // 1. Validar input
    validar_create_input(&input)?;

    // 2. Verificar si la cédula ya existe
    let cedula_normalizada = normalizar_cedula(&input.cedula);
    if blacklist_import_queries::exists_cedula(pool, &cedula_normalizada)
        .await
        .map_err(|e| format!("Error verificando cédula: {}", e))?
    {
        return Err(format!("La cédula {} ya existe en la tabla de prueba", cedula_normalizada));
    }

    // 3. Aplicar defaults y normalizar
    let motivo_bloqueo = input.motivo_bloqueo
        .filter(|m| !m.trim().is_empty())
        .unwrap_or_else(|| "No especificado".to_string());

    let fecha_inicio_bloqueo = input.fecha_inicio_bloqueo
        .filter(|f| !f.trim().is_empty())
        .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 4. Construir modelo
    let entry = BlacklistImportTest {
        id: Uuid::new_v4().to_string(),
        cedula: cedula_normalizada,
        primer_nombre: normalizar_texto(&input.primer_nombre),
        segundo_nombre: input.segundo_nombre.as_ref().map(|s| normalizar_texto(s)),
        primer_apellido: normalizar_texto(&input.primer_apellido),
        segundo_apellido: input.segundo_apellido.as_ref().map(|s| normalizar_texto(s)),
        nombre_completo: String::new(), // Se genera automáticamente en BD
        empresa: normalizar_texto(&input.empresa),
        motivo_bloqueo,
        fecha_inicio_bloqueo,
        observaciones: input.observaciones.map(|s| s.trim().to_string()),
        imported_at: now.clone(),
        imported_by: imported_by.clone(),
        created_at: now.clone(),
        updated_at: now,
    };

    // 5. Insertar en BD
    blacklist_import_queries::insert_blacklist_import(pool, &entry)
        .await
        .map_err(|e| format!("Error insertando entrada: {}", e))?;

    // 6. Recuperar el registro con nombre_completo generado
    let saved = blacklist_import_queries::get_blacklist_import_by_id(pool, &entry.id)
        .await
        .map_err(|e| format!("Error recuperando entrada: {}", e))?
        .ok_or("Entrada no encontrada después de insertar")?;

    // 7. Construir response
    let mut response = BlacklistImportResponse::from(saved);
    response.validation_status = ValidationStatus::Valid;
    response.validation_message = Some("Entrada creada exitosamente".to_string());

    Ok(response)
}

// ==========================================
// LEER ENTRADAS
// ==========================================

pub async fn get_blacklist_import_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<BlacklistImportResponse, String> {
    let entry = blacklist_import_queries::get_blacklist_import_by_id(pool, id)
        .await
        .map_err(|e| format!("Error consultando entrada: {}", e))?
        .ok_or("Entrada no encontrada")?;

    Ok(BlacklistImportResponse::from(entry))
}

pub async fn get_blacklist_import_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<BlacklistImportResponse, String> {
    let cedula_normalizada = normalizar_cedula(cedula);
    
    let entry = blacklist_import_queries::get_blacklist_import_by_cedula(pool, &cedula_normalizada)
        .await
        .map_err(|e| format!("Error consultando entrada: {}", e))?
        .ok_or(format!("No se encontró entrada con cédula {}", cedula_normalizada))?;

    Ok(BlacklistImportResponse::from(entry))
}

pub async fn get_all_blacklist_imports(
    pool: &SqlitePool,
) -> Result<Vec<BlacklistImportResponse>, String> {
    let entries = blacklist_import_queries::get_all_blacklist_imports(pool)
        .await
        .map_err(|e| format!("Error consultando entradas: {}", e))?;

    Ok(entries.into_iter().map(BlacklistImportResponse::from).collect())
}

pub async fn get_blacklist_imports_by_empresa(
    pool: &SqlitePool,
    empresa: &str,
) -> Result<Vec<BlacklistImportResponse>, String> {
    let entries = blacklist_import_queries::get_blacklist_imports_by_empresa(pool, empresa)
        .await
        .map_err(|e| format!("Error consultando entradas: {}", e))?;

    Ok(entries.into_iter().map(BlacklistImportResponse::from).collect())
}

// ==========================================
// ACTUALIZAR ENTRADA
// ==========================================

pub async fn update_blacklist_import(
    pool: &SqlitePool,
    id: &str,
    input: UpdateBlacklistImportInput,
) -> Result<BlacklistImportResponse, String> {
    // 1. Validar input
    validar_update_input(&input)?;

    // 2. Verificar que existe
    let exists = blacklist_import_queries::get_blacklist_import_by_id(pool, id)
        .await
        .map_err(|e| format!("Error verificando entrada: {}", e))?
        .is_some();

    if !exists {
        return Err("Entrada no encontrada".to_string());
    }

    // 3. Normalizar campos opcionales
    let primer_nombre = input.primer_nombre.as_ref().map(|s| normalizar_texto(s));
    let segundo_nombre = input.segundo_nombre.as_ref().map(|s| normalizar_texto(s));
    let primer_apellido = input.primer_apellido.as_ref().map(|s| normalizar_texto(s));
    let segundo_apellido = input.segundo_apellido.as_ref().map(|s| normalizar_texto(s));
    let empresa = input.empresa.as_ref().map(|s| normalizar_texto(s));
    let motivo_bloqueo = input.motivo_bloqueo.as_ref().map(|s| s.trim().to_string());
    let observaciones = input.observaciones.as_ref().map(|s| s.trim().to_string());

    // 4. Actualizar en BD
    let updated = blacklist_import_queries::update_blacklist_import(
        pool,
        id,
        primer_nombre.as_deref(),
        segundo_nombre.as_deref(),
        primer_apellido.as_deref(),
        segundo_apellido.as_deref(),
        empresa.as_deref(),
        motivo_bloqueo.as_deref(),
        observaciones.as_deref(),
    )
    .await
    .map_err(|e| format!("Error actualizando entrada: {}", e))?;

    if !updated {
        return Err("No se pudo actualizar la entrada".to_string());
    }

    // 5. Recuperar entrada actualizada
    let entry = blacklist_import_queries::get_blacklist_import_by_id(pool, id)
        .await
        .map_err(|e| format!("Error recuperando entrada: {}", e))?
        .ok_or("Entrada no encontrada después de actualizar")?;

    Ok(BlacklistImportResponse::from(entry))
}

// ==========================================
// ELIMINAR ENTRADA
// ==========================================

pub async fn delete_blacklist_import(
    pool: &SqlitePool,
    id: &str,
) -> Result<(), String> {
    let deleted = blacklist_import_queries::delete_blacklist_import(pool, id)
        .await
        .map_err(|e| format!("Error eliminando entrada: {}", e))?;

    if !deleted {
        return Err("Entrada no encontrada".to_string());
    }

    Ok(())
}

pub async fn delete_all_blacklist_imports(
    pool: &SqlitePool,
) -> Result<u64, String> {
    let count = blacklist_import_queries::delete_all_blacklist_imports(pool)
        .await
        .map_err(|e| format!("Error eliminando entradas: {}", e))?;

    Ok(count)
}

// ==========================================
// ESTADÍSTICAS
// ==========================================

pub async fn get_blacklist_import_stats(
    pool: &SqlitePool,
) -> Result<BlacklistImportStats, String> {
    // Total de entradas
    let total_entries = blacklist_import_queries::count_all_imports(pool)
        .await
        .map_err(|e| format!("Error contando entradas: {}", e))? as usize;

    // Conteo por empresa
    let empresa_counts = blacklist_import_queries::count_imports_by_empresa(pool)
        .await
        .map_err(|e| format!("Error contando por empresa: {}", e))?;

    let by_empresa: Vec<EmpresaStats> = empresa_counts
        .into_iter()
        .map(|(empresa, count)| EmpresaStats {
            empresa,
            count: count as usize,
        })
        .collect();

    // Últimas 10 importaciones
    let recent = blacklist_import_queries::get_recent_imports(pool, 10)
        .await
        .map_err(|e| format!("Error obteniendo importaciones recientes: {}", e))?;

    let recent_imports: Vec<BlacklistImportResponse> = recent
        .into_iter()
        .map(BlacklistImportResponse::from)
        .collect();

    Ok(BlacklistImportStats {
        total_entries,
        by_empresa,
        recent_imports,
    })
}

// ==========================================
// VERIFICACIÓN DE DUPLICADOS
// ==========================================

pub async fn check_duplicate_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<bool, String> {
    let cedula_normalizada = normalizar_cedula(cedula);
    
    blacklist_import_queries::exists_cedula(pool, &cedula_normalizada)
        .await
        .map_err(|e| format!("Error verificando duplicado: {}", e))
}