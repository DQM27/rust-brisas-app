// ==========================================
// src/search/schema.rs
// ==========================================
// Definición del schema de Tantivy

use tantivy::schema::*;

/// Crea el schema para el índice de búsqueda
pub fn build_search_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    
    // ID único del documento
    schema_builder.add_text_field("id", STRING | STORED);
    
    // Tipo de entidad
    schema_builder.add_text_field("tipo", STRING | STORED);
    
    // Campos de búsqueda para contratistas
    schema_builder.add_text_field("cedula", TEXT | STORED);
    schema_builder.add_text_field("nombre", TEXT | STORED);
    schema_builder.add_text_field("segundo_nombre", TEXT | STORED);
    schema_builder.add_text_field("apellido", TEXT | STORED);
    schema_builder.add_text_field("segundo_apellido", TEXT | STORED);
    schema_builder.add_text_field("empresa_nombre", TEXT | STORED);
    
    // Campo de búsqueda general (concatenación de todos los campos)
    schema_builder.add_text_field("search_text", TEXT);
    
    schema_builder.build()
}

/// Nombres de campos como constantes para evitar typos
pub mod fields {
    pub const ID: &str = "id";
    pub const TIPO: &str = "tipo";
    pub const CEDULA: &str = "cedula";
    pub const NOMBRE: &str = "nombre";
    pub const SEGUNDO_NOMBRE: &str = "segundo_nombre";
    pub const APELLIDO: &str = "apellido";
    pub const SEGUNDO_APELLIDO: &str = "segundo_apellido";
    pub const EMPRESA_NOMBRE: &str = "empresa_nombre";
    pub const SEARCH_TEXT: &str = "search_text";
}