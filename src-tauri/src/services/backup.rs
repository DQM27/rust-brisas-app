/// Servicio: Resiliencia y Recuperación de Datos.
///
/// Orquestador para la detección y aplicación de restauraciones de base de datos.
/// Actúa como un guardián previo a la inicialización del motor SurrealDB.
///
/// **SurrealDB Note**: A diferencia de SQLite, SurrealDB (SurrealKv) usa directorios.
/// Este servicio maneja recursivamente tanto archivos como carpetas según sea necesario.
///
/// Responsabilidades:
/// - Detectar señales de restauración (.restore).
/// - Asegurar la integridad mediante backups de emergencia (directorio o archivo).
/// - Aplicar reemplazos de base de datos de forma atómica.
use crate::config::AppConfig;
use crate::domain::errors::BackupError;
use log::{error, info, warn};
use std::fs;
use std::path::{Path, PathBuf};

/// Orquestador de Restauración Reactiva.
///
/// Verifica si existe un archivo de señalización (.restore) y procede a
/// reemplazar la base de datos operativa (directorio o archivo) por la versión solicitada.
///
/// # Arguments
///
/// * `config` - Referencia a la configuración global de la aplicación.
///
/// # Returns
///
/// `Ok(())` si no hay restauración pendiente o si se completó con éxito.
///
/// # Errors
///
/// * `BackupError::IO`: Si fallan las operaciones de copia o eliminación de archivos.
/// * `BackupError::AtomicFailure`: Si el renombramiento de archivos/directorios falla críticamente.
pub fn check_and_restore_database(config: &AppConfig) -> Result<(), BackupError> {
    let db_path = crate::config::manager::get_database_path(config);
    let verify_restore_path = get_restore_path(&db_path);

    if verify_restore_path.exists() {
        info!(
            "🔴 ALERTA DE SISTEMA: Restauración pendiente detectada ({}).",
            verify_restore_path.display()
        );

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let safety_backup = db_path.with_extension(format!("bkp.{}", timestamp));

        // 1. Crear backup de seguridad del estado actual
        if db_path.exists() {
            info!("🛡️  Seguridad: Creando punto de salvaguarda en {}", safety_backup.display());
            copy_recursive(&db_path, &safety_backup).map_err(|e| {
                error!("Fallo crítico al crear salvaguarda de emergencia: {}", e);
                BackupError::IO(format!("Fallo al crear salvaguarda: {}", e))
            })?;
        }

        info!("⚙️  Migración: Aplicando nueva estructura de datos...");

        // 2. Limpiar destino antes de aplicar (asegura éxito de rename en Windows)
        if db_path.exists() {
            if db_path.is_dir() {
                let _ = fs::remove_dir_all(&db_path);
            } else {
                let _ = fs::remove_file(&db_path);
            }
        }

        // 3. Intento de movimiento atómico (solo funciona en mismo filesystem)
        if let Err(e) = fs::rename(&verify_restore_path, &db_path) {
            warn!("Rename fallido ({}), intentando transplante manual...", e);

            // Fallback: Copia recursiva y limpieza
            copy_recursive(&verify_restore_path, &db_path).map_err(|e| {
                error!("Fallo atómico en transplante de datos: {}", e);
                BackupError::AtomicFailure(format!("Fallo al copiar restauración: {}", e))
            })?;

            // Limpieza de staging
            if verify_restore_path.is_dir() {
                fs::remove_dir_all(&verify_restore_path).ok();
            } else {
                fs::remove_file(&verify_restore_path).ok();
            }
        }

        info!("✅ ÉXITO: Sistema restaurado. El motor SurrealDB puede iniciar ahora.");
    }

    Ok(())
}

/// Genera la ruta del archivo de señalización de restauración basado en la ruta de la DB.
///
/// # Arguments
///
/// * `db_path` - Ruta al archivo/directorio principal de SurrealDB.
pub fn get_restore_path(db_path: &Path) -> PathBuf {
    let mut path = db_path.to_path_buf();
    if let Some(filename) = path.file_name() {
        let new_name = format!("{}.restore", filename.to_string_lossy());
        path.set_file_name(new_name);
    } else {
        // Fallback para rutas raíz
        path.push("db.restore");
    }
    path
}

/// Utilidad de copia recursiva compatible con archivos y directorios.
///
/// Fundamental para SurrealDB ya que usa estructuras de directorios K/V.
pub fn copy_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                copy_recursive(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_get_restore_path_logic() {
        let db_path = Path::new("/data/brisas.db");
        let restore_path = get_restore_path(db_path);
        let path_str = restore_path.to_string_lossy().replace("\\", "/");
        assert!(path_str.ends_with("/data/brisas.db.restore"));
    }

    #[test]
    fn test_restore_path_with_no_filename() {
        let db_path = Path::new("/");
        let restore_path = get_restore_path(db_path);
        assert!(restore_path.to_string_lossy().ends_with("db.restore"));
    }
}
