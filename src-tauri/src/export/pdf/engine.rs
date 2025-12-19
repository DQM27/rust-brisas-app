// ==========================================
// src/export/pdf/engine.rs
// ==========================================
// Wrapper de Typst 0.14 para compilación de PDF

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::PdfConfig;
use crate::models::template::PdfTemplate;
use chrono::Datelike;
use std::collections::HashMap;

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
    template: &PdfTemplate, // ✅ RECIBE EL TEMPLATE
) -> ExportResult<Vec<u8>> {
    let markup = templates::generate_typst_markup(headers, rows, config, template)?;

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
}

impl TypstWorld {
    fn new(markup: &str) -> ExportResult<Self> {
        // ✅ FIX: Usar LibraryExt para default()
        let library = LazyHash::new(Library::default());

        let fonts = Self::load_system_fonts();
        let book = LazyHash::new(FontBook::from_fonts(&fonts));

        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(main_id, markup.to_string());

        Ok(Self {
            library,
            book,
            fonts,
            source,
            main_id,
        })
    }

    /// Carga fuentes del sistema (Windows) manualmente para ahorrar memoria
    /// Solo carga las esenciales (Arial, Segoe UI) en lugar de todas las disponibles o assets embebidos
    fn load_system_fonts() -> Vec<Font> {
        let mut fonts = Vec::new();
        let font_dir = "C:\\Windows\\Fonts";

        // Lista curada de fuentes para UI y reportes
        // Minimiza el uso de RAM cargando solo lo necesario
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
            eprintln!("⚠️ WARN: No se encontraron fuentes del sistema (Arial/SegoeUI). El PDF podría no mostrar texto.");
        }

        fonts
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
        } else {
            Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
        }
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound("file not available".into()))
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
