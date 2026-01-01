/// Sistema de Resiliencia de Datos y Recuperación ante Desastres.
///
/// Este módulo no es solo para copias de seguridad; es el guardián de la integridad
/// de la base de datos durante el arranque. Implementa una lógica de auto-recuperación
/// que detecta estados de restauración pendientes y asegura que siempre haya un
/// "Rollback" disponible en caso de falla crítica.
use crate::config::AppConfig;
use log::info;
use std::fs;
use std::path::{Path, PathBuf};

/// Orquestador de Restauración Reactiva.
///
/// Se ejecuta EN ANTES de que SurrealDB tome control del archivo de base de datos.
/// Pasos:
/// 1. Detección: Busca una señal de restauración (.restore).
/// 2. Salvaguarda: Crea un backup de "último minuto" del estado actual antes de sobreescribir.
/// 3. Aplicación: Reemplaza atómicamente la DB vieja por la nueva versión solicitada.
pub fn check_and_restore_database(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = crate::config::manager::get_database_path(config);
    let verify_restore_path = get_restore_path(&db_path);

    if verify_restore_path.exists() {
        info!("🔴 ALERTA DE SISTEMA: Restauración pendiente detectada. Iniciando protocolo de recuperación...");

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let safety_backup = db_path.with_extension(format!("bkp.{}", timestamp));

        if db_path.exists() {
            info!(
                "🛡️  Seguridad: Creando punto de restauración de emergencia en {}",
                safety_backup.display()
            );
            fs::copy(&db_path, &safety_backup)?;
        }

        info!("⚙️  Actualizando motor: Aplicando nueva base de datos...");
        if let Err(_) = fs::rename(&verify_restore_path, &db_path) {
            fs::copy(&verify_restore_path, &db_path)?;
            fs::remove_file(&verify_restore_path)?;
        }

        info!("✅ ÉXITO: Sistema restaurado y listo para operación.");
    }

    Ok(())
}

/// Genera el nombre del activo de intercambio para la señalización de restauración.
pub fn get_restore_path(db_path: &Path) -> PathBuf {
    let mut path = db_path.to_path_buf();
    if let Some(filename) = path.file_name() {
        let new_name = format!("{}.restore", filename.to_string_lossy());
        path.set_file_name(new_name);
    }
    path
}
