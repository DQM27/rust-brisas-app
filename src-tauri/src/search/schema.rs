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
    schema_builder.add_text_field("email", TEXT | STORED);

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
    pub const EMAIL: &str = "email";
    pub const SEARCH_TEXT: &str = "search_text";
}

/// Handles pre-cargados de todos los campos del schema.
/// Se inicializa una vez al cargar el índice (fail-fast pattern).
#[derive(Clone, Copy)]
pub struct FieldHandles {
    pub id: Field,
    pub tipo: Field,
    pub cedula: Field,
    pub nombre: Field,
    pub segundo_nombre: Field,
    pub apellido: Field,
    pub segundo_apellido: Field,
    pub empresa_nombre: Field,
    pub email: Field,
    pub search_text: Field,
}

impl FieldHandles {
    /// Crea los handles desde el schema o retorna error descriptivo.
    /// Debe llamarse al inicializar el índice (fail-fast).
    pub fn new(schema: &Schema) -> Result<Self, String> {
        Ok(Self {
            id: schema
                .get_field(fields::ID)
                .map_err(|_| format!("Campo '{}' no encontrado en schema", fields::ID))?,
            tipo: schema
                .get_field(fields::TIPO)
                .map_err(|_| format!("Campo '{}' no encontrado en schema", fields::TIPO))?,
            cedula: schema
                .get_field(fields::CEDULA)
                .map_err(|_| format!("Campo '{}' no encontrado en schema", fields::CEDULA))?,
            nombre: schema
                .get_field(fields::NOMBRE)
                .map_err(|_| format!("Campo '{}' no encontrado en schema", fields::NOMBRE))?,
            segundo_nombre: schema.get_field(fields::SEGUNDO_NOMBRE).map_err(|_| {
                format!("Campo '{}' no encontrado en schema", fields::SEGUNDO_NOMBRE)
            })?,
            apellido: schema
                .get_field(fields::APELLIDO)
                .map_err(|_| format!("Campo '{}' no encontrado en schema", fields::APELLIDO))?,
            segundo_apellido: schema.get_field(fields::SEGUNDO_APELLIDO).map_err(|_| {
                format!(
                    "Campo '{}' no encontrado en schema",
                    fields::SEGUNDO_APELLIDO
                )
            })?,
            empresa_nombre: schema.get_field(fields::EMPRESA_NOMBRE).map_err(|_| {
                format!("Campo '{}' no encontrado en schema", fields::EMPRESA_NOMBRE)
            })?,
            email: schema
                .get_field(fields::EMAIL)
                .map_err(|_| format!("Campo '{}' no encontrado en schema", fields::EMAIL))?,
            search_text: schema
                .get_field(fields::SEARCH_TEXT)
                .map_err(|_| format!("Campo '{}' no encontrado en schema", fields::SEARCH_TEXT))?,
        })
    }
}
