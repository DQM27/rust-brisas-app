// ==========================================
// src/search/searcher.rs
// ==========================================
// Funciones para buscar en el índice de Tantivy

use crate::search::schema::fields;
use serde::{Deserialize, Serialize};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, Value};
use tantivy::{Index, IndexReader, ReloadPolicy, TantivyDocument};

/// Cache de campos para evitar lookups por string en cada búsqueda
pub struct SearchFields {
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

impl SearchFields {
    pub fn new(schema: &Schema) -> Self {
        Self {
            id: schema.get_field(fields::ID).expect("Falta campo id"),
            tipo: schema.get_field(fields::TIPO).expect("Falta campo tipo"),
            cedula: schema
                .get_field(fields::CEDULA)
                .expect("Falta campo cedula"),
            nombre: schema
                .get_field(fields::NOMBRE)
                .expect("Falta campo nombre"),
            segundo_nombre: schema
                .get_field(fields::SEGUNDO_NOMBRE)
                .expect("Falta campo segundo_nombre"),
            apellido: schema
                .get_field(fields::APELLIDO)
                .expect("Falta campo apellido"),
            segundo_apellido: schema
                .get_field(fields::SEGUNDO_APELLIDO)
                .expect("Falta campo segundo_apellido"),
            empresa_nombre: schema
                .get_field(fields::EMPRESA_NOMBRE)
                .expect("Falta campo empresa_nombre"),
            email: schema.get_field(fields::EMAIL).expect("Falta campo email"),
            search_text: schema
                .get_field(fields::SEARCH_TEXT)
                .expect("Falta campo search_text"),
        }
    }
}

/// Resultado de búsqueda
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: String,
    pub tipo: String,
    pub score: f32,
    pub cedula: Option<String>,
    pub nombre_completo: Option<String>,
    pub empresa_nombre: Option<String>,
    pub email: Option<String>,
}

/// Inicializa un reader para búsquedas
pub fn get_index_reader(index: &Index) -> Result<IndexReader, String> {
    index
        .reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into()
        .map_err(|e| format!("Error al crear reader: {}", e))
}

/// Busca en el índice con fuzzy search (Optimizado)
pub fn search_index(
    index: &Index,
    reader: &IndexReader,
    fields: &SearchFields, // Cache de campos inyectada
    query_str: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    let searcher = reader.searcher();

    // 1. Usar campos pre-calculated (Cero costo de lookup)
    let search_target_fields = vec![
        fields.cedula,
        fields.nombre,
        fields.segundo_nombre,
        fields.apellido,
        fields.segundo_apellido,
        fields.empresa_nombre,
        fields.email,
        fields.search_text,
    ];

    // 2. Parser
    let mut query_parser = QueryParser::for_index(index, search_target_fields);
    // Usamos el ID numérico directo:
    query_parser.set_field_fuzzy(fields.search_text, true, 2, true);

    // Parsear query
    let query = query_parser
        .parse_query(query_str)
        .map_err(|e| format!("Query inválido: {}", e))?;

    // Ejecutar búsqueda
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|e| format!("Error búsqueda: {}", e))?;

    let mut results = Vec::with_capacity(top_docs.len());

    for (score, doc_addr) in top_docs {
        let doc: TantivyDocument = searcher.doc(doc_addr).map_err(|e| e.to_string())?;

        // 3. Extracción optimizada usando los IDs cacheados
        results.push(SearchResult {
            id: get_val(&doc, fields.id).unwrap_or_default(),
            tipo: get_val(&doc, fields.tipo).unwrap_or_default(),
            score,
            cedula: get_val(&doc, fields.cedula),
            nombre_completo: build_full_name(&doc, fields),
            empresa_nombre: get_val(&doc, fields.empresa_nombre),
            email: get_val(&doc, fields.email),
        });
    }

    Ok(results)
}

/// Busca solo en contratistas (Optimizado)
pub fn search_contratistas(
    index: &Index,
    reader: &IndexReader,
    fields: &SearchFields, // Cache de campos inyectada
    query_str: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    // Agregar filtro de tipo
    let filtered_query = format!("({}) AND tipo:contratista", query_str);
    search_index(index, reader, fields, &filtered_query, limit)
}

// Helpers optimizados (reciben Field u32, no string name)
fn get_val(doc: &TantivyDocument, field: Field) -> Option<String> {
    // Acceso directo sin lookup
    doc.get_first(field)?.as_str().map(|s| s.to_string())
}

fn build_full_name(doc: &TantivyDocument, fields: &SearchFields) -> Option<String> {
    let nombre = get_val(doc, fields.nombre)?;
    let apellido = get_val(doc, fields.apellido)?;

    let mut nombre_completo = format!("{} {}", nombre, apellido);

    if let Some(segundo_nombre) = get_val(doc, fields.segundo_nombre) {
        nombre_completo = format!("{} {} {}", nombre, segundo_nombre, apellido);
    }

    if let Some(segundo_apellido) = get_val(doc, fields.segundo_apellido) {
        nombre_completo.push(' ');
        nombre_completo.push_str(&segundo_apellido);
    }

    Some(nombre_completo)
}
