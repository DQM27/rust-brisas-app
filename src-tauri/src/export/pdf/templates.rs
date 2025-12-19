// ==========================================
// src/export/pdf/templates.rs
// ==========================================
// Generación de markup Typst dinámico
// Crea el código Typst que será compilado a PDF

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::{PageOrientation, PdfConfig, PdfDesign};
use std::collections::HashMap;

// ==========================================
// FUNCIÓN PRINCIPAL
// ==========================================

/// Genera el markup Typst completo para el PDF
pub fn generate_typst_markup(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &PdfConfig,
    design: &PdfDesign, // ✅ USA PDF DESIGN (Perfil)
) -> ExportResult<String> {
    let mut markup = String::new();

    // 1. Imports y configuración de página
    markup.push_str(&generate_page_setup(config, design)?);

    // 2. Título del documento
    markup.push_str(&generate_title(&config.title, design));

    // 3. Tabla con datos
    markup.push_str(&generate_table(headers, rows, design)?);

    // 4. Footer con metadata
    markup.push_str(&generate_footer(design));

    Ok(markup)
}

// ==========================================
// SECCIONES DEL TEMPLATE
// ==========================================
/// Genera configuración de página y imports
fn generate_page_setup(config: &PdfConfig, design: &PdfDesign) -> ExportResult<String> {
    // La orientación del config tiene prioridad
    let orientation = match config.orientation {
        PageOrientation::Portrait => "false",
        PageOrientation::Landscape => "true",
    };

    let margin_x = format!("{}{}", design.margin_x, design.margin_x_unit);
    let margin_y = format!("{}{}", design.margin_y, design.margin_y_unit);

    let setup = format!(
        "#set page(\n\
  paper: \"{}\",\n\
  flipped: {},\n\
  margin: (x: {}, y: {}),\n\
)\n\n\
#set text(\n\
  font: \"{}\",\n\
  size: {}pt,\n\
  lang: \"es\",\n\
)\n\n\
#set table(\n\
  stroke: 0.5pt + rgb(\"{}\"),\n\
  fill: (x, y) => if y == 0 {{ rgb(\"{}\") }} else {{ none }},\n\
  align: (x, y) => if y == 0 {{ center }} else {{ left }},\n\
)\n\n",
        design.page_size,
        orientation,
        margin_x,
        margin_y,
        design.fonts.family,
        design.fonts.size,
        design.colors.border,
        design.colors.header_fill
    );

    Ok(setup)
}

/// Genera el título del documento
fn generate_title(title: &str, _design: &PdfDesign) -> String {
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
fn generate_table(
    headers: &[String],
    rows: &[HashMap<String, String>],
    design: &PdfDesign,
) -> ExportResult<String> {
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
        markup.push_str(&format!(
            "  [*#text(fill: rgb(\"{}\"))[{}]*],\n",
            design.colors.header_text, escaped_header
        ));
    }

    // Rows
    for row in rows {
        for header in headers {
            let value = row.get(header).map(|s| s.as_str()).unwrap_or("");
            let escaped_value = escape_typst_string(value);
            markup.push_str(&format!(
                "  [#text(fill: rgb(\"{}\"))[{}]],\n",
                design.colors.row_text, escaped_value
            ));
        }
    }

    markup.push_str(")\n\n");

    Ok(markup)
}

/// Genera footer con metadata
fn generate_footer(_design: &PdfDesign) -> String {
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
