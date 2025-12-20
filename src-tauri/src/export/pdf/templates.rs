// ==========================================
// src/export/pdf/templates.rs
// ==========================================
// Generación de markup Typst dinámico con showybox
// Usa el package showybox para cajas decorativas

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

/// Genera configuración de página
fn generate_page_setup(config: &PdfConfig, design: &PdfDesign) -> ExportResult<String> {
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
  fill: rgb(\"#0d1117\"),\n\
)\n\n\
#set text(\n\
  font: \"{}\",\n\
  size: {}pt,\n\
  lang: \"es\",\n\
  fill: rgb(\"#e6edf3\"),\n\
)\n\n",
        design.page_size, orientation, margin_x, margin_y, design.fonts.family, design.fonts.size,
    );

    Ok(setup)
}

/// Genera el contenido dentro de un showybox decorativo
fn generate_showybox_content(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &PdfConfig,
    design: &PdfDesign,
) -> ExportResult<String> {
    if headers.is_empty() {
        return Err(ExportError::TemplateGenerationError(
            "Headers vacíos".to_string(),
        ));
    }

    let escaped_title = escape_typst_string(&config.title);
    let now = chrono::Local::now().format("%d/%m/%Y %H:%M");
    let row_count = rows.len();

    // Generar la tabla
    let table_content = generate_table(headers, rows, design)?;

    let content = format!(
        "#showybox(\n\
  title-style: (\n\
    weight: \"bold\",\n\
    color: white,\n\
    sep-thickness: 0pt,\n\
  ),\n\
  frame: (\n\
    title-color: rgb(\"#2563eb\"),\n\
    border-color: rgb(\"#3b82f6\"),\n\
    body-color: rgb(\"#161b22\"),\n\
    thickness: 1.5pt,\n\
    radius: 6pt,\n\
    inset: (x: 12pt, y: 10pt),\n\
  ),\n\
  title: [\n\
    #text(size: 14pt)[{}]\n\
    #h(1fr)\n\
    #text(size: 9pt, weight: \"regular\")[{} registros]\n\
  ],\n\
  breakable: true,\n\
)[\n\
  {}\n\n\
  #v(8pt)\n\
  \n\
  #align(right)[\n\
    #text(size: 8pt, fill: rgb(\"#8b949e\"))[\n\
      Generado: {}\n\
    ]\n\
  ]\n\
]\n",
        escaped_title, row_count, table_content, now
    );

    Ok(content)
}

/// Genera la tabla con headers y datos
fn generate_table(
    headers: &[String],
    rows: &[HashMap<String, String>],
    _design: &PdfDesign,
) -> ExportResult<String> {
    let col_count = headers.len();

    let mut markup = format!(
        "#table(\n\
    columns: {},\n\
    inset: 8pt,\n\
    stroke: 0.5pt + rgb(\"#30363d\"),\n\
    fill: (x, y) => if y == 0 {{ rgb(\"#21262d\") }} else if calc.odd(y) {{ rgb(\"#161b22\") }} else {{ rgb(\"#0d1117\") }},\n\
    align: (x, y) => if y == 0 {{ center }} else {{ left }},\n",
        col_count
    );

    // Headers
    for header in headers {
        let escaped_header = escape_typst_string(header);
        markup.push_str(&format!(
            "    [*#text(fill: rgb(\"#58a6ff\"), size: 10pt)[{}]*],\n",
            escaped_header
        ));
    }

    // Rows
    for row in rows {
        for header in headers {
            let value = row.get(header).map(|s| s.as_str()).unwrap_or("-");
            let escaped_value = escape_typst_string(value);
            markup.push_str(&format!(
                "    [#text(fill: rgb(\"#e6edf3\"), size: 9pt)[{}]],\n",
                escaped_value
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
