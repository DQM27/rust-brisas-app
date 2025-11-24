// ==========================================
// src/commands/blacklist_import_commands.rs
// ==========================================
// Comandos Tauri para importación de lista negra
// Expone la funcionalidad al frontend

use tauri::State;
use sqlx::SqlitePool;

use crate::models::blacklist_import::{
    CreateBlacklistImportInput,
    UpdateBlacklistImportInput,
    BlacklistImportResponse,
    BlacklistImportStats,
    ImportResultResponse,
    ExcelPreviewResponse,
};
use crate::services::{blacklist_import_service, excel_parser};

// ==========================================
// COMANDOS CRUD BÁSICOS
// ==========================================

/// Crea una entrada manual en la tabla de prueba
#[tauri::command]
pub async fn create_blacklist_import_entry(
    pool: State<'_, SqlitePool>,
    input: CreateBlacklistImportInput,
    user_id: String,
) -> Result<BlacklistImportResponse, String> {
    blacklist_import_service::create_blacklist_import(&pool, input, user_id).await
}

/// Obtiene una entrada por ID
#[tauri::command]
pub async fn get_blacklist_import_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<BlacklistImportResponse, String> {
    blacklist_import_service::get_blacklist_import_by_id(&pool, &id).await
}

/// Obtiene una entrada por cédula
#[tauri::command]
pub async fn get_blacklist_import_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<BlacklistImportResponse, String> {
    blacklist_import_service::get_blacklist_import_by_cedula(&pool, &cedula).await
}

/// Obtiene todas las entradas
#[tauri::command]
pub async fn get_all_blacklist_imports(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<BlacklistImportResponse>, String> {
    blacklist_import_service::get_all_blacklist_imports(&pool).await
}

/// Obtiene entradas por empresa
#[tauri::command]
pub async fn get_blacklist_imports_by_empresa(
    pool: State<'_, SqlitePool>,
    empresa: String,
) -> Result<Vec<BlacklistImportResponse>, String> {
    blacklist_import_service::get_blacklist_imports_by_empresa(&pool, &empresa).await
}

/// Actualiza una entrada
#[tauri::command]
pub async fn update_blacklist_import_entry(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateBlacklistImportInput,
) -> Result<BlacklistImportResponse, String> {
    blacklist_import_service::update_blacklist_import(&pool, &id, input).await
}

/// Elimina una entrada
#[tauri::command]
pub async fn delete_blacklist_import_entry(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    blacklist_import_service::delete_blacklist_import(&pool, &id).await
}

/// Elimina todas las entradas (limpia la tabla de prueba)
#[tauri::command]
pub async fn delete_all_blacklist_imports(
    pool: State<'_, SqlitePool>,
) -> Result<u64, String> {
    blacklist_import_service::delete_all_blacklist_imports(&pool).await
}

// ==========================================
// COMANDOS DE ESTADÍSTICAS
// ==========================================

/// Obtiene estadísticas de la tabla de prueba
#[tauri::command]
pub async fn get_blacklist_import_stats(
    pool: State<'_, SqlitePool>,
) -> Result<BlacklistImportStats, String> {
    blacklist_import_service::get_blacklist_import_stats(&pool).await
}

/// Verifica si una cédula ya existe
#[tauri::command]
pub async fn check_duplicate_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<bool, String> {
    blacklist_import_service::check_duplicate_cedula(&pool, &cedula).await
}

// ==========================================
// COMANDOS DE IMPORTACIÓN EXCEL
// ==========================================

/// Genera un preview del archivo Excel sin importar
#[tauri::command]
pub async fn preview_excel_import(
    file_path: String,
    skip_header: bool,
) -> Result<ExcelPreviewResponse, String> {
    let mapping = excel_parser::ColumnMapping::default();
    excel_parser::preview_excel_file(&file_path, &mapping, skip_header, 10)
}

/// Lee y normaliza el archivo Excel completo (sin insertar en BD)
#[tauri::command]
pub async fn parse_excel_file(
    file_path: String,
    skip_header: bool,
) -> Result<ImportResultResponse, String> {
    let mapping = excel_parser::ColumnMapping::default();
    let parsed_rows = excel_parser::read_excel_file(&file_path, &mapping, skip_header)?;
    Ok(excel_parser::normalize_excel_rows(parsed_rows))
}

/// Importa el archivo Excel a la base de datos
#[tauri::command]
pub async fn import_excel_to_database(
    pool: State<'_, SqlitePool>,
    file_path: String,
    user_id: String,
    skip_header: bool,
) -> Result<ImportResultResponse, String> {
    // 1. Parsear y normalizar el Excel
    let mapping = excel_parser::ColumnMapping::default();
    let parsed_rows = excel_parser::read_excel_file(&file_path, &mapping, skip_header)?;
    let mut import_result = excel_parser::normalize_excel_rows(parsed_rows);

    // 2. Filtrar solo las entradas válidas (excluir las que necesitan review y las inválidas)
    let valid_entries: Vec<_> = import_result.entries
        .iter()
        .filter(|e| e.validation_status == crate::models::blacklist_import::ValidationStatus::Valid)
        .cloned()
        .collect();

    // 3. Insertar entradas válidas en la BD
    let mut successfully_inserted = 0;
    let mut insertion_errors = Vec::new();

    for entry in valid_entries {
        let input = CreateBlacklistImportInput {
            cedula: entry.cedula.clone(),
            primer_nombre: entry.primer_nombre.clone(),
            segundo_nombre: entry.segundo_nombre.clone(),
            primer_apellido: entry.primer_apellido.clone(),
            segundo_apellido: entry.segundo_apellido.clone(),
            empresa: entry.empresa.clone(),
            motivo_bloqueo: Some(entry.motivo_bloqueo.clone()),
            fecha_inicio_bloqueo: Some(entry.fecha_inicio_bloqueo.clone()),
            observaciones: entry.observaciones.clone(),
        };

        match blacklist_import_service::create_blacklist_import(&pool, input, user_id.clone()).await {
            Ok(_) => {
                successfully_inserted += 1;
            }
            Err(e) => {
                insertion_errors.push(crate::models::blacklist_import::ImportError {
                    row_number: 0, // No tenemos el número de fila aquí
                    cedula: Some(entry.cedula.clone()),
                    error_type: "INSERTION_ERROR".to_string(),
                    message: e,
                });
            }
        }
    }

    // 4. Actualizar estadísticas del resultado
    import_result.successful = successfully_inserted;
    import_result.errors.extend(insertion_errors);

    Ok(import_result)
}

/// Importa solo las entradas que requieren revisión (después de corrección manual)
#[tauri::command]
pub async fn import_reviewed_entries(
    pool: State<'_, SqlitePool>,
    entries: Vec<CreateBlacklistImportInput>,
    user_id: String,
) -> Result<ImportResultResponse, String> {
    let total_rows = entries.len();
    let mut successful = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    let mut imported_entries = Vec::new();

    for (idx, input) in entries.into_iter().enumerate() {
        match blacklist_import_service::create_blacklist_import(&pool, input.clone(), user_id.clone()).await {
            Ok(response) => {
                successful += 1;
                imported_entries.push(response);
            }
            Err(e) => {
                failed += 1;
                errors.push(crate::models::blacklist_import::ImportError {
                    row_number: idx + 1,
                    cedula: Some(input.cedula.clone()),
                    error_type: "INSERTION_ERROR".to_string(),
                    message: e,
                });
            }
        }
    }

    Ok(ImportResultResponse {
        total_rows,
        successful,
        needs_review: 0,
        failed,
        entries: imported_entries,
        errors,
    })
}

// ==========================================
// COMANDOS DE UTILIDAD
// ==========================================

/// Valida un nombre completo y sugiere separación
#[tauri::command]
pub fn validate_and_split_name(
    nombre_completo: String,
) -> Result<(String, Option<String>, String, Option<String>), String> {
    use crate::domain::blacklist_import::{capitalizar_nombre, separar_nombre_automatico};
    
    let normalizado = capitalizar_nombre(&nombre_completo);
    separar_nombre_automatico(&normalizado)
}

/// Detecta si un nombre requiere validación manual
#[tauri::command]
pub fn check_name_requires_validation(
    nombre_completo: String,
) -> Result<bool, String> {
    use crate::domain::blacklist_import::requiere_validacion_manual;
    
    Ok(requiere_validacion_manual(&nombre_completo))
}

/// Normaliza una cédula
#[tauri::command]
pub fn normalize_cedula(
    cedula: String,
) -> Result<String, String> {
    use crate::domain::blacklist_import::normalizar_cedula;
    
    Ok(normalizar_cedula(&cedula))
}

/// Capitaliza un nombre
#[tauri::command]
pub fn capitalize_name(
    nombre: String,
) -> Result<String, String> {
    use crate::domain::blacklist_import::capitalizar_nombre;
    
    Ok(capitalizar_nombre(&nombre))
}