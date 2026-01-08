//! # Schema: Definición de Estructura del Índice
//!
//! Define la estructura de los documentos en Tantivy, especificando qué campos son
//! indexados, almacenados (stored) o buscables.
//!
//! ## Estructura
//! - `id`: Identificador único (String)
//! - `tipo`: Tipo de entidad (contratista, usuario, etc.)
//! - `search_text`: Campo unificado para búsquedas globales
//! - Otros metadatos específicos (cedula, nombre, empresa, etc.)

use crate::search::errors::SearchError;
use tantivy::schema::{Field, Schema, STORED, STRING, TEXT};

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
    pub fn new(schema: &Schema) -> Result<Self, SearchError> {
        Ok(Self {
            id: schema
                .get_field(fields::ID)
                .map_err(|_| SearchError::FieldNotFound(fields::ID.to_string()))?,
            tipo: schema
                .get_field(fields::TIPO)
                .map_err(|_| SearchError::FieldNotFound(fields::TIPO.to_string()))?,
            cedula: schema
                .get_field(fields::CEDULA)
                .map_err(|_| SearchError::FieldNotFound(fields::CEDULA.to_string()))?,
            nombre: schema
                .get_field(fields::NOMBRE)
                .map_err(|_| SearchError::FieldNotFound(fields::NOMBRE.to_string()))?,
            segundo_nombre: schema
                .get_field(fields::SEGUNDO_NOMBRE)
                .map_err(|_| SearchError::FieldNotFound(fields::SEGUNDO_NOMBRE.to_string()))?,
            apellido: schema
                .get_field(fields::APELLIDO)
                .map_err(|_| SearchError::FieldNotFound(fields::APELLIDO.to_string()))?,
            segundo_apellido: schema
                .get_field(fields::SEGUNDO_APELLIDO)
                .map_err(|_| SearchError::FieldNotFound(fields::SEGUNDO_APELLIDO.to_string()))?,
            empresa_nombre: schema
                .get_field(fields::EMPRESA_NOMBRE)
                .map_err(|_| SearchError::FieldNotFound(fields::EMPRESA_NOMBRE.to_string()))?,
            email: schema
                .get_field(fields::EMAIL)
                .map_err(|_| SearchError::FieldNotFound(fields::EMAIL.to_string()))?,
            search_text: schema
                .get_field(fields::SEARCH_TEXT)
                .map_err(|_| SearchError::FieldNotFound(fields::SEARCH_TEXT.to_string()))?,
        })
    }
}
