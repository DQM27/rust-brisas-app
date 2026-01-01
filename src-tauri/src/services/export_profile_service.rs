/// Gestión de Perfiles de Exportación.
///
/// Este servicio permite a los usuarios definir y persistir configuraciones personalizadas
/// para generar reportes (PDF, Excel, CSV). Los perfiles se guardan en un archivo JSON
/// independiente para que sean fáciles de respaldar o editar manualmente si fuera necesario.
use crate::config::manager::{get_database_path, load_config};
use crate::export::errors::ExportError;
use crate::models::export::{CsvOptions, ExportProfile, PdfColors, PdfDesign, PdfFonts};
use std::fs;
use std::path::PathBuf;

const PROFILE_FILE_NAME: &str = "export_profiles.json";

/// Determina la ubicación física del archivo de perfiles.
/// Se guarda junto a la base de datos para mantener toda la información del usuario unificada.
fn get_profiles_path() -> Result<PathBuf, ExportError> {
    let config = load_config().map_err(|e| ExportError::IoError(e.to_string()))?;
    let db_path = get_database_path(&config);

    let parent = db_path.parent().ok_or(ExportError::FileSystemError(
        "No se pudo localizar el directorio de configuración del sistema.".to_string(),
    ))?;
    Ok(parent.join(PROFILE_FILE_NAME))
}

/// Define el catálogo inicial de perfiles.
/// Esto asegura que el usuario siempre tenga opciones listas para usar tras la instalación.
fn get_default_profiles() -> Vec<ExportProfile> {
    vec![
        ExportProfile {
            id: "default-pdf-landscape".to_string(),
            name: "PDF Horizontal Rápido".to_string(), // Ideal para tablas con muchas columnas.
            format: "pdf".to_string(),
            is_default: true,
            title: Some("Reporte".to_string()),
            show_preview: Some(false),
            pdf_design: Some(PdfDesign {
                page_size: "us-letter".to_string(),
                orientation: "landscape".to_string(),
                margin_x: 1.5,
                margin_x_unit: "cm".to_string(),
                margin_y: 2.0,
                margin_y_unit: "cm".to_string(),
                colors: PdfColors {
                    header_fill: "#2da44e".to_string(), // Verde institucional.
                    header_text: "#ffffff".to_string(),
                    row_text: "#000000".to_string(),
                    border: "#cccccc".to_string(),
                },
                fonts: PdfFonts {
                    family: "New Computer Modern".to_string(), // Fuente legible y profesional.
                    size: 10,
                    header_size: 12,
                },
            }),
            csv_options: None,
        },
        ExportProfile {
            id: "default-pdf-portrait".to_string(),
            name: "PDF Vertical".to_string(), // Preferido para listados cortos o individuales.
            format: "pdf".to_string(),
            is_default: false,
            title: Some("Reporte".to_string()),
            show_preview: Some(false),
            pdf_design: Some(PdfDesign {
                page_size: "us-letter".to_string(),
                orientation: "portrait".to_string(),
                margin_x: 1.5,
                margin_x_unit: "cm".to_string(),
                margin_y: 2.0,
                margin_y_unit: "cm".to_string(),
                colors: PdfColors {
                    header_fill: "#2da44e".to_string(),
                    header_text: "#ffffff".to_string(),
                    row_text: "#000000".to_string(),
                    border: "#cccccc".to_string(),
                },
                fonts: PdfFonts {
                    family: "New Computer Modern".to_string(),
                    size: 10,
                    header_size: 12,
                },
            }),
            csv_options: None,
        },
        ExportProfile {
            id: "default-excel".to_string(),
            name: "Excel Estándar".to_string(),
            format: "excel".to_string(),
            is_default: false,
            title: None,
            show_preview: None,
            pdf_design: None,
            csv_options: None,
        },
        ExportProfile {
            id: "default-csv".to_string(),
            name: "CSV para Excel".to_string(), // Configurado con BOM para evitar problemas de acentos en Excel.
            format: "csv".to_string(),
            is_default: false,
            title: None,
            show_preview: None,
            pdf_design: None,
            csv_options: Some(CsvOptions { delimiter: "comma".to_string(), include_bom: true }),
        },
    ]
}

/// Recupera la colección completa de perfiles.
/// Si el archivo no existe, crea uno nuevo con los valores predeterminados.
pub fn get_all_profiles() -> Result<Vec<ExportProfile>, ExportError> {
    let path = get_profiles_path()?;

    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| ExportError::IoError(e.to_string()))?;
        let profiles: Vec<ExportProfile> = serde_json::from_str(&content).map_err(|e| {
            ExportError::ProfileSerializationError(format!(
                "Error de lectura en el archivo de perfiles: {}",
                e
            ))
        })?;
        Ok(profiles)
    } else {
        // Inicialización automática para asegurar que el sistema siempre sea funcional.
        let defaults = get_default_profiles();
        save_all_profiles(&defaults)?;
        Ok(defaults)
    }
}

/// Sincroniza la lista de perfiles con el almacenamiento físico.
fn save_all_profiles(profiles: &[ExportProfile]) -> Result<(), ExportError> {
    let path = get_profiles_path()?;
    let json = serde_json::to_string_pretty(profiles)
        .map_err(|e| ExportError::ProfileSerializationError(e.to_string()))?;
    fs::write(path, json).map_err(|e| ExportError::IoError(e.to_string()))?;
    Ok(())
}

/// Agrega un nuevo perfil o actualiza uno existente mediante su ID (Upsert).
pub fn save_profile(profile: ExportProfile) -> Result<(), ExportError> {
    let mut profiles = get_all_profiles()?;

    if let Some(idx) = profiles.iter().position(|p| p.id == profile.id) {
        profiles[idx] = profile;
    } else {
        profiles.push(profile);
    }

    save_all_profiles(&profiles)?;
    Ok(())
}

/// Elimina un perfil de la lista.
/// Bloqueamos la eliminación si el perfil es el único marcado como predeterminado,
/// garantizando que el sistema siempre tenga una opción válida de exportación por defecto.
pub fn delete_profile(id: String) -> Result<(), ExportError> {
    let mut profiles = get_all_profiles()?;

    let default_count = profiles.iter().filter(|p| p.is_default).count();
    if default_count == 1 {
        if let Some(profile) = profiles.iter().find(|p| p.id == id) {
            if profile.is_default {
                return Err(ExportError::InvalidProfileOperation(
                    "Operación denegada: Debe existir al menos un perfil predeterminado activo."
                        .to_string(),
                ));
            }
        }
    }

    let initial_len = profiles.len();
    profiles.retain(|p| p.id != id);

    if profiles.len() != initial_len {
        save_all_profiles(&profiles)?;
    }

    Ok(())
}

/// Cambia la preferencia global del usuario sobre qué perfil usar por defecto.
pub fn set_default_profile(id: String) -> Result<(), ExportError> {
    let mut profiles = get_all_profiles()?;

    if !profiles.iter().any(|p| p.id == id) {
        return Err(ExportError::ProfileNotFound);
    }

    // Desmarcamos todos los anteriores para asegurar que solo haya uno predeterminado.
    for profile in profiles.iter_mut() {
        profile.is_default = profile.id == id;
    }

    save_all_profiles(&profiles)?;
    Ok(())
}

/// Recupera el perfil que está configurado como predeterminado.
pub fn get_default_profile() -> Option<ExportProfile> {
    get_all_profiles().ok()?.into_iter().find(|p| p.is_default)
}

/// Busca un perfil de exportación específico mediante su identificador.
pub fn get_profile_by_id(id: &str) -> Option<ExportProfile> {
    get_all_profiles().ok()?.into_iter().find(|p| p.id == id)
}
