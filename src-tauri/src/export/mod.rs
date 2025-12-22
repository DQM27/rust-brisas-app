// ==========================================
// src/export/mod.rs
// ==========================================
// Módulo público de export - API mínima y desacoplada

// ==========================================
// SUBMÓDULOS SIEMPRE DISPONIBLES
// ==========================================
pub mod errors;

// CSV siempre disponible (sin deps externas)
pub mod csv;

// ==========================================
// SUBMÓDULOS CONDICIONALES
// ==========================================
#[cfg(feature = "export-pdf")]
pub mod pdf;

#[cfg(feature = "export-excel")]
pub mod excel;

// ==========================================
// RE-EXPORTS PÚBLICOS
// ==========================================

// Errors
pub use errors::{ExportError, ExportResult};

// CSV (siempre disponible)
pub use csv::generate_csv;

// PDF (condicional)
#[cfg(feature = "export-pdf")]
pub use pdf::generate_pdf;

// Excel (condicional)
#[cfg(feature = "export-excel")]
pub use excel::generate_excel;

// ==========================================
// FUNCIÓN DE DISPONIBILIDAD
// ==========================================

/// Verifica qué formatos de export están disponibles
pub fn available_formats() -> Vec<&'static str> {
    let mut formats = vec!["csv"]; // CSV siempre disponible

    #[cfg(feature = "export-pdf")]
    formats.push("pdf");

    #[cfg(feature = "export-excel")]
    formats.push("excel");

    formats
}

/// Verifica si un formato específico está disponible
pub fn is_format_available(format: &str) -> bool {
    match format.to_lowercase().as_str() {
        "csv" => true, // Siempre disponible

        #[cfg(feature = "export-pdf")]
        "pdf" => true,

        #[cfg(feature = "export-excel")]
        "excel" => true,

        _ => false,
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_siempre_disponible() {
        assert!(is_format_available("csv"));
        assert!(available_formats().contains(&"csv"));
    }

    #[test]
    fn test_formato_invalido() {
        assert!(!is_format_available("word"));
        assert!(!is_format_available(""));
    }
}
