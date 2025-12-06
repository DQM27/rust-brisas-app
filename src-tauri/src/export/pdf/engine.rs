// ==========================================
// src/export/pdf/engine.rs
// ==========================================
// Wrapper de Typst 0.14 para compilación de PDF

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::PdfConfig;
use crate::models::template::PdfTemplate;
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

    // templates::validate_markup(&markup)?; // Validation moved inside debug block if needed, or kept before compile.
    // Keeping validate_markup active but removing print.

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

        let fonts = Self::load_embedded_fonts();
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

    fn load_embedded_fonts() -> Vec<Font> {
        let mut fonts = Vec::new();
        println!("🔍 Loading Typst embedded fonts...");

        let assets = typst_assets::fonts();
        let mut count = 0;

        // ✅ Cargar TODAS las fuentes embebidas de typst-assets
        for data in assets {
            count += 1;
            let buffer = Bytes::new(data.to_vec()); // Converted to Vec for Bytes::new
                                                    // Actually, let's check what data is.
            println!(
                "  Processing font asset #{} (size: {} bytes)",
                count,
                data.len()
            );

            let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);
            println!("    Face count: {}", face_count);

            // Cargar todas las fuentes en la colección (para TTC files)
            for face_index in 0..face_count {
                match Font::new(buffer.clone(), face_index) {
                    Some(font) => {
                        let family = font.info().family.clone();
                        println!("    ✓ Loaded font: {} (face {})", family, face_index);
                        fonts.push(font);
                    }
                    None => {
                        println!("    ❌ Failed to load font face {}", face_index);
                    }
                }
            }
        }

        if count == 0 {
            println!("❌ typst_assets::fonts() returned no items!");
        }

        println!("🔍 Total fonts loaded: {}", fonts.len());

        if fonts.is_empty() {
            eprintln!("⚠️  WARNING: No fonts were loaded! PDF text will not render.");
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

    // ✅ FIX: from_ymd retorna Datetime directamente (no Option)
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Datetime::from_ymd(2024, 12, 1)
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
