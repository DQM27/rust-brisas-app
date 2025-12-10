use crate::config::AppConfig;
use crate::services::backup;
use std::fs;
use tauri::{command, State};

/// Crea una copia de seguridad de la base de datos usando VACUUM INTO
#[command]
pub async fn backup_database(
    pool: State<'_, sqlx::SqlitePool>,
    destination_path: String,
) -> Result<(), String> {
    println!("💾 Iniciando backup manual a: {}", destination_path);

    // Usar SQL directo para el backup en caliente
    // VACUUM INTO es la forma segura en SQLite de copiar mientras está en uso
    let query = format!("VACUUM INTO '{}'", destination_path);

    // Verificar si el archivo destino ya existe y borrarlo (VACUUM INTO falla si existe)
    let dest_path = std::path::Path::new(&destination_path);
    if dest_path.exists() {
        fs::remove_file(dest_path)
            .map_err(|e| format!("No se pudo sobrescribir el archivo: {}", e))?;
    }

    sqlx::query(&query)
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Error al ejecutar backup SQL: {}", e))?;

    println!("✅ Backup completado exitosamente");
    Ok(())
}

/// Prepara la restauración de una base de datos (copia a .restore y pide reinicio)
#[command]
pub async fn restore_database(
    config: State<'_, AppConfig>,
    source_path: String,
) -> Result<(), String> {
    println!("🔄 Preparando restauración desde: {}", source_path);

    let db_path = crate::config::manager::get_database_path(&config);
    let restore_path = backup::get_restore_path(&db_path);

    // Validar origen
    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        return Err("El archivo de origen no existe".to_string());
    }

    // Copiar archivo fuente a *.restore
    println!("📄 Copiando a área de staging: {}", restore_path.display());
    fs::copy(source, &restore_path)
        .map_err(|e| format!("Error al preparar archivo de restauración: {}", e))?;

    println!("✅ Archivo de restauración listo. Se requiere reinicio.");
    Ok(())
}
