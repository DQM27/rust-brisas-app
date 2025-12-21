// ==========================================
// src/export/pdf/engine.rs
// ==========================================
// Wrapper de Typst 0.14 para compilación de PDF
// Con soporte para packages locales (offline)

use crate::export::errors::{ExportError, ExportResult};

use crate::models::export::{PdfConfig, PdfDesign};
use chrono::Datelike;
use std::collections::HashMap;
use std::path::PathBuf;

use super::templates;

// IMPORTS DE TYPST 0.14
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

// ==========================================
// FUNCIÓN PRINCIPAL
// ==========================================

pub fn generate_pdf(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &PdfConfig,
    design: &PdfDesign, // ✅ USA PDF DESIGN
) -> ExportResult<Vec<u8>> {
    let markup = templates::generate_typst_markup(headers, rows, config, design)?;

    templates::validate_markup(&markup)?;
    let pdf_bytes = compile_typst_to_pdf(&markup)?;

    if pdf_bytes.is_empty() {
        return Err(ExportError::TypstCompilationError(
            "PDF resultante está vacío".to_string(),
        ));
    }

    Ok(pdf_bytes)
}

// ==========================================
// COMPILACIÓN TYPST
// ==========================================

fn compile_typst_to_pdf(markup: &str) -> ExportResult<Vec<u8>> {
    let world = TypstWorld::new(markup)?;

    // ✅ FIX: compile() ahora retorna Warned<T>, usar .output
    let result = typst::compile(&world);
    let document = result.output.map_err(|errors| {
        let error_messages: Vec<String> = errors.iter().map(|e| format!("{}", e.message)).collect();

        if error_messages.is_empty() {
            ExportError::TypstCompilationError("Error desconocido".to_string())
        } else {
            ExportError::TypstCompilationError(error_messages.join("; "))
        }
    })?;

    // ✅ FIX: pdf() ahora toma (&document, &PdfOptions) y retorna Result
    let options = typst_pdf::PdfOptions::default();
    let pdf_bytes = typst_pdf::pdf(&document, &options)
        .map_err(|e| ExportError::TypstCompilationError(format!("Error generando PDF: {:?}", e)))?;

    Ok(pdf_bytes)
}

// ==========================================
// TYPST WORLD IMPLEMENTATION
// ==========================================

struct TypstWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    source: Source,
    main_id: FileId,
    packages_root: PathBuf, // Carpeta de packages locales
}

impl TypstWorld {
    fn new(markup: &str) -> ExportResult<Self> {
        let library = LazyHash::new(Library::default());

        let fonts = Self::load_system_fonts();
        let book = LazyHash::new(FontBook::from_fonts(&fonts));

        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(main_id, markup.to_string());

        // Buscar carpeta de packages relativa al ejecutable
        let packages_root = Self::find_packages_root();

        Ok(Self {
            library,
            book,
            fonts,
            source,
            main_id,
            packages_root,
        })
    }

    /// Busca la carpeta de packages locales
    fn find_packages_root() -> PathBuf {
        // Intentar primero en el directorio del ejecutable (para producción)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let packages_dir = exe_dir.join("packages");
                if packages_dir.exists() {
                    return packages_dir;
                }
            }
        }

        // Fallback: buscar en src-tauri/packages (para desarrollo)
        let dev_paths = [
            PathBuf::from("packages"),
            PathBuf::from("src-tauri/packages"),
            PathBuf::from("../src-tauri/packages"),
        ];

        for path in &dev_paths {
            if path.exists() {
                return path.clone();
            }
        }

        // Default: carpeta packages en el directorio actual
        PathBuf::from("packages")
    }

    /// Resuelve la ruta de un archivo dentro de un package
    fn resolve_package_file(&self, id: FileId) -> Option<PathBuf> {
        let package = id.package()?;
        let namespace = package.namespace.as_str();
        let name = package.name.as_str();
        let version = package.version.to_string();

        let package_dir = self.packages_root.join(namespace).join(name).join(&version);

        let file_path = package_dir.join(id.vpath().as_rootless_path());

        if file_path.exists() {
            Some(file_path)
        } else {
            None
        }
    }

    /// Carga fuentes del sistema (Windows) + fuentes bundleadas (Inter)
    fn load_system_fonts() -> Vec<Font> {
        let mut fonts = Vec::new();

        // 1. Cargar fuentes bundleadas (Inter) - PRIORIDAD
        fonts.extend(Self::load_bundled_fonts());

        // 2. Cargar fuentes del sistema (fallback)
        let font_dir = "C:\\Windows\\Fonts";
        let target_fonts = vec![
            "arial.ttf",
            "arialbd.ttf",
            "arialbi.ttf",
            "ariali.ttf",
            "segoeui.ttf",
            "segoeuib.ttf",
            "segoeuil.ttf",
            "times.ttf",
            "timesbd.ttf",
            "calibri.ttf",
            "calibrib.ttf",
        ];

        for filename in target_fonts {
            let path = std::path::Path::new(font_dir).join(filename);
            if path.exists() {
                if let Ok(data) = std::fs::read(&path) {
                    let buffer = Bytes::new(data);
                    let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);

                    for face_index in 0..face_count {
                        if let Some(font) = Font::new(buffer.clone(), face_index) {
                            fonts.push(font);
                        }
                    }
                }
            }
        }

        if fonts.is_empty() {
            eprintln!("⚠️ WARN: No se encontraron fuentes. El PDF podría no mostrar texto.");
        }

        fonts
    }

    /// Carga fuentes bundleadas desde fonts/
    fn load_bundled_fonts() -> Vec<Font> {
        let mut fonts = Vec::new();

        // Buscar carpeta fonts/ en ubicación relativa a packages/
        let fonts_root = Self::find_fonts_root();

        // Inter - solo los esenciales para no usar demasiada RAM
        let inter_fonts = vec![
            "inter/Inter-Regular.ttf",
            "inter/Inter-Bold.ttf",
            "inter/Inter-Medium.ttf",
            "inter/Inter-SemiBold.ttf",
            "inter/Inter-Light.ttf",
            "inter/Inter-Italic.ttf",
        ];

        for font_path in inter_fonts {
            let full_path = fonts_root.join(font_path);
            if full_path.exists() {
                if let Ok(data) = std::fs::read(&full_path) {
                    let buffer = Bytes::new(data);
                    let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);

                    for face_index in 0..face_count {
                        if let Some(font) = Font::new(buffer.clone(), face_index) {
                            fonts.push(font);
                        }
                    }
                }
            }
        }

        fonts
    }

    /// Encuentra la carpeta fonts/ (similar a packages/)
    fn find_fonts_root() -> PathBuf {
        // Primero buscar en dev (src-tauri/fonts/)
        let dev_path = std::env::current_dir().unwrap_or_default().join("fonts");

        if dev_path.exists() {
            return dev_path;
        }

        // En producción buscar junto al ejecutable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let prod_path = exe_dir.join("fonts");
                if prod_path.exists() {
                    return prod_path;
                }
            }
        }

        // Fallback
        PathBuf::from("fonts")
    }
}

impl World for TypstWorld {
    // ✅ FIX: Retornar &LazyHash<Library>
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    // ✅ FIX: Retornar &LazyHash<FontBook>
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    // ✅ FIX: Retornar FileId, no Source
    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            Ok(self.source.clone())
        } else if let Some(path) = self.resolve_package_file(id) {
            // Cargar archivo de package local
            let content =
                std::fs::read_to_string(&path).map_err(|_| FileError::NotFound(path.clone()))?;
            Ok(Source::new(id, content))
        } else {
            Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(path) = self.resolve_package_file(id) {
            // Cargar archivo binario de package local
            let data = std::fs::read(&path).map_err(|_| FileError::NotFound(path))?;
            Ok(Bytes::new(data))
        } else {
            Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let now = chrono::Local::now();
        Datetime::from_ymd(now.year(), now.month() as u8, now.day() as u8)
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_markup() {
        let markup = r#"
#set page(paper: "us-letter")
#set text(font: "Liberation Sans")

= Test Document

This is a test.
"#;

        let result = compile_typst_to_pdf(markup);
        assert!(result.is_ok());

        let pdf_bytes = result.unwrap();
        assert!(!pdf_bytes.is_empty());
        assert_eq!(&pdf_bytes[0..4], b"%PDF");
    }
}
