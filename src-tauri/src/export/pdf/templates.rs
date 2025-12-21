// ==========================================
// src/export/pdf/templates.rs
// ==========================================
// Generación de markup Typst dinámico con showybox
// Tema claro optimizado para impresión (ahorro de tinta)

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
    design: &PdfDesign,
) -> ExportResult<String> {
    let mut markup = String::new();

    // 1. Imports (showybox)
    markup.push_str(&generate_imports());

    // 2. Configuración de página
    markup.push_str(&generate_page_setup(config, design)?);

    // 3. Contenido dentro de showybox
    markup.push_str(&generate_showybox_content(headers, rows, config, design)?);

    Ok(markup)
}

// ==========================================
// SECCIONES DEL TEMPLATE
// ==========================================

/// Genera imports de packages
fn generate_imports() -> String {
    "#import \"@preview/showybox:2.0.4\": showybox\n\n".to_string()
}

/// Genera configuración de página (tema claro para impresión)
fn generate_page_setup(config: &PdfConfig, design: &PdfDesign) -> ExportResult<String> {
    let orientation = match config.orientation {
        PageOrientation::Portrait => "false",
        PageOrientation::Landscape => "true",
    };

    // Usar márgenes del config (seleccionados por usuario) o fallback a design
    let margin_top = format!("{}cm", config.margin_top);
    let margin_bottom = format!("{}cm", config.margin_bottom);
    let margin_left = format!("{}cm", config.margin_left);
    let margin_right = format!("{}cm", config.margin_right);

    let setup = format!(
        "#set page(\n\
  paper: \"{}\",\n\
  flipped: {},\n\
  margin: (top: {}, bottom: {}, left: {}, right: {}),\n\
)\n\n\
#set text(\n\
  font: \"{}\",\n\
  size: {}pt,\n\
  lang: \"es\",\n\
  fill: rgb(\"#1f2328\"),\n\
)\n\n",
        design.page_size,
        orientation,
        margin_top,
        margin_bottom,
        margin_left,
        margin_right,
        config.font_family,
        config.font_size,
    );

    Ok(setup)
}

/// Genera el contenido dentro de un showybox decorativo (tema claro)
fn generate_showybox_content(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &PdfConfig,
    _design: &PdfDesign,
) -> ExportResult<String> {
    if headers.is_empty() {
        return Err(ExportError::TemplateGenerationError(
            "Headers vacíos".to_string(),
        ));
    }

    let escaped_title = escape_typst_string(&config.title);
    let now = chrono::Local::now().format("%d/%m/%Y %H:%M");

    // Generar la tabla con font_size
    let table_content = generate_table(headers, rows, config)?;

    // Tema claro con borde azul - centrado
    let content = format!(
        "#align(center)[\n\
#showybox(\n\
  title-style: (\n\
    weight: \"bold\",\n\
    color: white,\n\
    sep-thickness: 0pt,\n\
  ),\n\
  frame: (\n\
    title-color: rgb(\"#2563eb\"),\n\
    border-color: rgb(\"#d0d7de\"),\n\
    body-color: white,\n\
    thickness: 1pt,\n\
    radius: 4pt,\n\
    inset: (x: 12pt, y: 10pt),\n\
  ),\n\
  title: [\n\
    #text(size: 12pt)[{}]\n\
  ],\n\
  breakable: true,\n\
)[\n\
  #align(center)[\n\
    {}\n\
  ]\n\n\
  #v(6pt)\n\
  \n\
  #align(right)[\n\
    #text(size: 7pt, fill: rgb(\"#656d76\"))[\n\
      Generado: {}\n\
    ]\n\
  ]\n\
]\n\
]\n",
        escaped_title, table_content, now
    );

    Ok(content)
}

/// Genera la tabla con headers y datos (tema claro)
fn generate_table(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &PdfConfig,
) -> ExportResult<String> {
    let col_count = headers.len();

    // Usar el font_size numérico del config (header es 1pt más grande)
    let body_size = format!("{}pt", config.font_size);
    let header_size = format!("{}pt", config.font_size + 1);

    // Columnas fraccionadas para distribuir el espacio uniformemente
    let columns_spec = vec!["1fr"; col_count].join(", ");

    // Tabla con tema claro - filas alternadas suaves
    // Usamos columns con fracciones para evitar desbordamiento
    let mut markup = format!(
        "#table(\n\
    columns: ({}),\n\
    inset: 6pt,\n\
    stroke: 0.5pt + rgb(\"#d0d7de\"),\n\
    fill: (x, y) => if y == 0 {{ rgb(\"#f6f8fa\") }} else if calc.odd(y) {{ white }} else {{ rgb(\"#f6f8fa\") }},\n\
    align: (x, y) => if y == 0 {{ center }} else {{ left }},\n",
        columns_spec
    );

    // Headers con word-break
    for header in headers {
        let escaped_header = escape_typst_string(header);
        markup.push_str(&format!(
            "    [#set par(justify: false); *#text(fill: rgb(\"#1f2328\"), size: {}, hyphenate: true)[{}]*],\n",
            header_size, escaped_header
        ));
    }

    // Rows con word-break
    for row in rows {
        for header in headers {
            let value = row.get(header).map(|s| s.as_str()).unwrap_or("-");
            let escaped_value = escape_typst_string(value);
            markup.push_str(&format!(
                "    [#set par(justify: false); #text(fill: rgb(\"#1f2328\"), size: {}, hyphenate: true)[{}]],\n",
                body_size, escaped_value
            ));
        }
    }

    markup.push_str("  )");

    Ok(markup)
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
