/// Puertos de Entrada: Resiliencia y Mantenimiento de Datos.
///
/// Este módulo expone comandos para la gestión de copias de seguridad
/// y la preparación de restauraciones atómicas.
use crate::config::AppConfig;
use crate::domain::errors::BackupError;
use crate::services::backup;
use log::{error, info};
use tauri::{command, State};

// --------------------------------------------------------------------------
// COMANDOS DE MANTENIMIENTO
// --------------------------------------------------------------------------

/// [Comando Tauri] Realiza una copia de seguridad manual de la base de datos activa.
///
/// Ejecuta el comando `EXPORT FILE` de SurrealDB para generar un script SQL
/// con la estructura y los datos actuales.
///
/// # Argumentos
/// * `destination_path` - Ruta absoluta donde se guardará el archivo .surql.
///
/// # Retorno
/// Retorna `Ok(())` si la exportación es exitosa.
#[command]
pub async fn backup_database(destination_path: String) -> Result<(), BackupError> {
    info!("📦 Iniciando respaldo manual de base de datos a: {}", destination_path);

    // 1. Obtener cliente de BD
    let db = crate::services::surrealdb_service::get_db().await.map_err(|e| {
        error!("No se pudo obtener conexión a DB para respaldo: {}", e);
        BackupError::IO(format!("Error de conexión al motor de base de datos: {}", e))
    })?;

    // 2. Sanitizar ruta (Windows backslashes pueden causar problemas en cadenas SQL)
    // Convertimos backslashes a forward slashes que funcionan bien en rutas mixtas
    let clean_path = destination_path.replace('\\', "/");

    // 3. Ejecutar exportación
    // EXPORT FILE guarda todo (SCHEMA + DATA) en el archivo indicado
    let query = format!("EXPORT FILE '{}';", clean_path);

    info!("⚙️ Ejecutando query de exportación...");
    match db.query(query).await {
        Ok(_) => {
            info!("✅ Respaldo completado exitosamente en: {}", destination_path);
            Ok(())
        }
        Err(e) => {
            error!("❌ Falla crítica al exportar base de datos: {}", e);
            Err(BackupError::IO(format!(
                "Fallo al ejecutar exportación interna: {}. Verifique permisos de escritura.",
                e
            )))
        }
    }
}

/// [Comando Tauri] Prepara el sistema para una restauración de base de datos.
///
/// La restauración efectiva NO ocurre inmediatamente. Este comando coloca los datos
/// en un área de "staging" y el sistema los aplicará automáticamente en el próximo arranque.
///
/// # Argumentos
/// * `config` - Estado de la configuración de la aplicación.
/// * `source_path` - Ruta absoluta al backup a restaurar.
///
/// # Retorno
/// Retorna `Ok(())` si la preparación fue exitosa. Entrega `BackupError::NotFound`
/// si el origen no existe o `BackupError::IO` si falla la copia.
#[command]
pub async fn restore_database(
    config: State<'_, AppConfig>,
    source_path: String,
) -> Result<(), BackupError> {
    info!("🔄 Preparando protocolo de restauración desde: {source_path}");

    let db_path = crate::config::manager::get_database_path(&config);
    let restore_path = backup::get_restore_path(&db_path);

    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        error!("Fallo en restauración: Origen inexistente en {source_path}");
        return Err(BackupError::NotFound(source_path));
    }

    info!("📦 Copiando datos al área de preparación: {}", restore_path.display());

    // Asegurar que el destino esté limpio si es una restauración nueva
    if restore_path.exists() {
        if restore_path.is_dir() {
            let _ = std::fs::remove_dir_all(&restore_path);
        } else {
            let _ = std::fs::remove_file(&restore_path);
        }
    }

    backup::copy_recursive(source, &restore_path).map_err(|e| {
        error!("Error al preparar staging de restauración: {e}");
        BackupError::IO(format!("Fallo al copiar datos a staging: {e}"))
    })?;

    info!("✅ Protocolo listo. El sistema se restaurará en el próximo reinicio.");
    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS DE INTEGRACIÓN
// --------------------------------------------------------------------------
#[cfg(test)]
mod integration_tests {
    // use super::*;
    // use crate::services::surrealdb_service::{init_surrealdb, SurrealDbConfig};
    // use std::fs;

    // TODO: Habilitar test cuando se resuelva el error de runtime:
    // `STATUS_ENTRYPOINT_NOT_FOUND` (0xc0000139) en Windows al ejecutar tests de SurrealDB.
    // Parece ser un conflicto de DLLs en el entorno de pruebas vs ejecución normal.
    //
    // #[tokio::test]
    // async fn test_backup_database_demo() {
    //     // 1. Setup - Usar DB Demo (aislada)
    //     // Nota: Init es global (OnceCell), así que esto solo funciona si es el primer test
    //     // o si la configuración coincide. Para `cargo test` suele ser suficiente.
    //     let config = SurrealDbConfig::demo();
    //     let service = init_surrealdb(config.clone());

    //     // Conectar (ignorar error si ya estaba conectado)
    //     let _ = service.connect().await;

    //     // 2. Preparar ruta de prueba
    //     let mut backup_path = std::env::temp_dir();
    //     backup_path.push(format!("test_backup_{}.surql", chrono::Utc::now().timestamp()));
    //     let backup_path_str = backup_path.to_string_lossy().to_string();

    //     // Limpiar previo por si acaso
    //     if backup_path.exists() {
    //         let _ = fs::remove_file(&backup_path);
    //     }

    //     // 3. Ejecutar comando (debe crear el archivo)
    //     let result = backup_database(backup_path_str.clone()).await;

    //     // 4. Validaciones
    //     match result {
    //         Ok(_) => {
    //             assert!(backup_path.exists(), "El archivo de backup debería haberse creado");

    //             let metadata = fs::metadata(&backup_path).unwrap();
    //             assert!(metadata.len() > 0, "El archivo de backup no debería estar vacío");

    //             // Cleanup solo si fue exitoso (para dejar evidencia si falla)
    //             let _ = fs::remove_file(backup_path);
    //         }
    //         Err(e) => {
    //             panic!("El comando backup_database falló: {:?}", e);
    //         }
    //     }
    // }
}
