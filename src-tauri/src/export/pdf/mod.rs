// ==========================================
// src/export/pdf/mod.rs
// ==========================================
// Re-export del generador PDF

mod engine;
mod templates;

pub use engine::generate_pdf;