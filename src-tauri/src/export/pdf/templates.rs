// ==========================================
// src/export/pdf/templates.rs
// ==========================================
// Generación de markup Typst dinámico
// Crea el código Typst que será compilado a PDF

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::{PageOrientation, PdfConfig};
use crate::models::template::PdfTemplate;
use std::collections::HashMap;

// ==========================================
// FUNCIÓN PRINCIPAL
// ==========================================

/// Genera el markup Typst completo para el PDF
pub fn generate_typst_markup(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &PdfConfig,
    template: &PdfTemplate, // ✅ RECIBE EL TEMPLATE
) -> ExportResult<String> {
    let mut markup = String::new();

    // 1. Imports y configuración de página
    markup.push_str(&generate_page_setup(config, template)?);

    // 2. Título del documento
    markup.push_str(&generate_title(&config.title, template));

    // 3. Tabla con datos
    markup.push_str(&generate_table(headers, rows, template)?);

    // 4. Footer con metadata
    markup.push_str(&generate_footer(template));

    Ok(markup)
}

// ==========================================
// SECCIONES DEL TEMPLATE
// ==========================================
/// Genera configuración de página y imports
fn generate_page_setup(config: &PdfConfig, template: &PdfTemplate) -> ExportResult<String> {
    // La orientación del config tiene prioridad, si no usa la del template
    // (Aunque en este caso la UI probablemente fuerce a usar la del config)
    let orientation = match config.orientation {
        PageOrientation::Portrait => "false",
        PageOrientation::Landscape => "true",
    };

    // Si la configuración del template dice "portrait" pero el usuario pidió "landscape",
    // el config manda.
    // Pero si el template define margenes, etc, los usamos.

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
        template.layout.page_size,
        orientation,
        template.layout.margin_x,
        template.layout.margin_y,
        template.fonts.family,
        template.fonts.size,
        template.colors.border,
        template.colors.header_fill
    );

    Ok(setup)
}

/// Genera el título del documento
fn generate_title(title: &str, _template: &PdfTemplate) -> String {
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
    template: &PdfTemplate,
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
        // ✅ Usar color de texto para headers del template
        markup.push_str(&format!(
            "  [*#text(fill: rgb(\"{}\"))[{}]*],\n",
            template.colors.header_text, escaped_header
        ));
    }

    // Rows
    for row in rows {
        for header in headers {
            let value = row.get(header).map(|s| s.as_str()).unwrap_or("");
            let escaped_value = escape_typst_string(value);
            // ✅ Usar color de texto para filas
            markup.push_str(&format!(
                "  [#text(fill: rgb(\"{}\"))[{}]],\n",
                template.colors.row_text, escaped_value
            ));
        }
    }

    markup.push_str(")\n\n");

    Ok(markup)
}

/// Genera footer con metadata
fn generate_footer(_template: &PdfTemplate) -> String {
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
