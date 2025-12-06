// ==========================================
// src/export/pdf/templates.rs
// ==========================================
// Generación de markup Typst dinámico
// Crea el código Typst que será compilado a PDF

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::{PageOrientation, PdfConfig};
use std::collections::HashMap;

// ==========================================
// FUNCIÓN PRINCIPAL
// ==========================================

/// Genera el markup Typst completo para el PDF
pub fn generate_typst_markup(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &PdfConfig,
) -> ExportResult<String> {
    let mut markup = String::new();

    // 1. Imports y configuración de página
    markup.push_str(&generate_page_setup(config)?);

    // 2. Título del documento
    markup.push_str(&generate_title(&config.title));

    // 3. Tabla con datos
    markup.push_str(&generate_table(headers, rows)?);

    // 4. Footer con metadata
    markup.push_str(&generate_footer());

    Ok(markup)
}

// ==========================================
// SECCIONES DEL TEMPLATE
// ==========================================
/// Genera configuración de página y imports
fn generate_page_setup(config: &PdfConfig) -> ExportResult<String> {
    let flipped = match config.orientation {
        PageOrientation::Portrait => "false",
        PageOrientation::Landscape => "true",
    };

    // ✅ ALTERNATIVA: format! simple sin raw strings
    let setup = format!(
        "#set page(\n\
  paper: \"us-letter\",\n\
  flipped: {},\n\
  margin: (x: 1.5cm, y: 2cm),\n\
)\n\n\
#set text(\n\
  font: \"New Computer Modern\",\n\
  size: 10pt,\n\
  lang: \"es\",\n\
)\n\n\
#set table(\n\
  stroke: 0.5pt + black,\n\
  fill: (x, y) => if y == 0 {{ rgb(\"#e8e8e8\") }} else {{ none }},\n\
  align: (x, y) => if y == 0 {{ center }} else {{ left }},\n\
)\n\n",
        flipped
    );

    Ok(setup)
}

/// Genera el título del documento
fn generate_title(title: &str) -> String {
    let escaped_title = escape_typst_string(title);

    format!(
        r#"#align(center)[
  #text(size: 16pt, weight: "bold")[{}]
]

#v(0.5cm)

"#,
        escaped_title
    )
}

/// Genera la tabla con headers y datos
fn generate_table(headers: &[String], rows: &[HashMap<String, String>]) -> ExportResult<String> {
    if headers.is_empty() {
        return Err(ExportError::TemplateGenerationError(
            "Headers vacíos".to_string(),
        ));
    }

    let mut markup = String::from("#table(\n");
    markup.push_str(&format!("  columns: {},\n", headers.len()));
    markup.push_str("  inset: 8pt,\n");

    // Headers
    for header in headers {
        let escaped_header = escape_typst_string(header);
        markup.push_str(&format!("  [*{}*],\n", escaped_header));
    }

    // Rows
    for row in rows {
        for header in headers {
            let value = row.get(header).map(|s| s.as_str()).unwrap_or("");
            let escaped_value = escape_typst_string(value);
            markup.push_str(&format!("  [{}],\n", escaped_value));
        }
    }

    markup.push_str(")\n\n");

    Ok(markup)
}

/// Genera footer con metadata
fn generate_footer() -> String {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

    format!(
        r#"#v(1cm)

#align(right)[
  #text(size: 8pt, fill: gray)[
    Generado: {}
  ]
]
"#,
        now
    )
}

// ==========================================
// UTILIDADES DE ESCAPE
// ==========================================

/// Escapa caracteres especiales de Typst
fn escape_typst_string(input: &str) -> String {
    input
        .replace('\\', r"\\")
        .replace('#', r"\#")
        .replace('[', r"\[")
        .replace(']', r"\]")
        .replace('*', r"\*")
        .replace('_', r"\_")
        .replace('$', r"\$")
        .replace('<', r"\<")
        .replace('>', r"\>")
        .replace('`', r"\`")
}

// ==========================================
// VALIDACIONES
// ==========================================

/// Valida que el markup generado sea válido
pub fn validate_markup(markup: &str) -> ExportResult<()> {
    if markup.is_empty() {
        return Err(ExportError::TemplateGenerationError(
            "Markup vacío".to_string(),
        ));
    }

    // Verificar balance de brackets
    let open_brackets = markup.matches('[').count();
    let close_brackets = markup.matches(']').count();

    if open_brackets != close_brackets {
        return Err(ExportError::TemplateGenerationError(format!(
            "Brackets desbalanceados: {} abiertos, {} cerrados",
            open_brackets, close_brackets
        )));
    }

    Ok(())
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_typst_string() {
        assert_eq!(escape_typst_string("hello"), "hello");
        assert_eq!(escape_typst_string("hello#world"), r"hello\#world");
        assert_eq!(escape_typst_string("[test]"), r"\[test\]");
        assert_eq!(escape_typst_string("*bold*"), r"\*bold\*");
        assert_eq!(escape_typst_string("_italic_"), r"\_italic\_");
        assert_eq!(escape_typst_string("$math$"), r"\$math\$");
    }

    #[test]
    fn test_generate_page_setup() {
        let config = PdfConfig {
            title: "Test".to_string(),
            orientation: PageOrientation::Landscape,
            headers: vec![],
            show_preview: false,
        };

        let setup = generate_page_setup(&config).unwrap();
        assert!(setup.contains("true"));
        assert!(setup.contains("us-letter"));
    }

    #[test]
    fn test_generate_title() {
        let title = generate_title("Test Report");
        assert!(title.contains("Test Report"));
        assert!(title.contains("16pt"));
        assert!(title.contains("bold"));
    }

    #[test]
    fn test_generate_table() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let mut row1 = HashMap::new();
        row1.insert("Name".to_string(), "John".to_string());
        row1.insert("Age".to_string(), "30".to_string());

        let rows = vec![row1];

        let table = generate_table(&headers, &rows).unwrap();
        assert!(table.contains("columns: 2"));
        assert!(table.contains("[*Name*]"));
        assert!(table.contains("[*Age*]"));
        assert!(table.contains("[John]"));
        assert!(table.contains("[30]"));
    }

    #[test]
    fn test_generate_table_empty_headers() {
        let headers: Vec<String> = vec![];
        let rows = vec![];

        let result = generate_table(&headers, &rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_markup_balanced() {
        let markup = "[hello] [world]";
        assert!(validate_markup(markup).is_ok());
    }

    #[test]
    fn test_validate_markup_unbalanced() {
        let markup = "[hello [world]";
        assert!(validate_markup(markup).is_err());
    }
}
