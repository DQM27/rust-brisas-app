use crate::config::AppConfig;
use std::fs;
use std::path::{Path, PathBuf};

/// Comprueba si hay una restauración pendiente y la ejecuta antes de iniciar la DB
pub fn check_and_restore_database(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = crate::config::manager::get_database_path(config);
    let verify_restore_path = get_restore_path(&db_path);

    // Si existe archivo .restore, proceder con la restauración
    if verify_restore_path.exists() {
        println!(
            "🔄 Restauración pendiente detectada: {}",
            verify_restore_path.display()
        );

        // 1. Crear backup de seguridad de la actual (rollback)
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let safety_backup = db_path.with_extension(format!("bkp.{}", timestamp));

        if db_path.exists() {
            println!(
                "🔒 Creando backup de seguridad pre-restauración: {}",
                safety_backup.display()
            );
            fs::copy(&db_path, &safety_backup)?;
        }

        // 2. Reemplazar la base de datos
        // Intentar renombrar primero (atómico), si falla (diferentes discos), copiar y borrar
        println!("🚀 Aplicando restauración...");
        if let Err(_) = fs::rename(&verify_restore_path, &db_path) {
            fs::copy(&verify_restore_path, &db_path)?;
            fs::remove_file(&verify_restore_path)?;
        }

        println!("✅ Base de datos restaurada correctamente.");
    }

    Ok(())
}

/// Genera la ruta del archivo de restauración pendiente (ej: brisas.db.restore)
pub fn get_restore_path(db_path: &Path) -> PathBuf {
    let mut path = db_path.to_path_buf();
    if let Some(filename) = path.file_name() {
        let new_name = format!("{}.restore", filename.to_string_lossy());
        path.set_file_name(new_name);
    }
    path
}
