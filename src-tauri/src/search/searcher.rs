// ==========================================
// src/search/searcher.rs
// ==========================================
// Funciones para buscar en el índice de Tantivy

use tantivy::{Index, IndexReader, ReloadPolicy};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Value}; 
use crate::search::schema::fields;
use serde::{Deserialize, Serialize};

/// Resultado de búsqueda
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: String,
    pub tipo: String,
    pub score: f32,
}

/// Inicializa un reader para búsquedas
pub fn get_index_reader(index: &Index) -> Result<IndexReader, String> {
    index.reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|e| format!("Error al crear reader: {}", e))
}

/// Busca en el índice con fuzzy search
pub fn search_index(
    index: &Index,
    reader: &IndexReader,
    query_str: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    let schema = index.schema();
    let searcher = reader.searcher();
    
    // Campos en los que buscar
    let search_fields = vec![
        schema.get_field(fields::CEDULA).unwrap(),
        schema.get_field(fields::NOMBRE).unwrap(),
        schema.get_field(fields::SEGUNDO_NOMBRE).unwrap(),
        schema.get_field(fields::APELLIDO).unwrap(),
        schema.get_field(fields::SEGUNDO_APELLIDO).unwrap(),
        schema.get_field(fields::EMPRESA_NOMBRE).unwrap(),
        schema.get_field(fields::SEARCH_TEXT).unwrap(),
    ];
    
    // Crear query parser con fuzzy search habilitado
    let mut query_parser = QueryParser::for_index(index, search_fields);
    query_parser.set_field_fuzzy(
        schema.get_field(fields::SEARCH_TEXT).unwrap(),
        true,
        2, // distancia de edición máxima
        true, // prefix
    );
    
    // Parsear query (con manejo de errores de sintaxis)
    let query = query_parser
        .parse_query(query_str)
        .map_err(|e| format!("Error al parsear query: {}", e))?;
    
    // Ejecutar búsqueda
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|e| format!("Error al buscar: {}", e))?;
    
    // Extraer resultados
    let mut results = Vec::new();
    
    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher
            .doc(doc_address)
            .map_err(|e| format!("Error al recuperar documento: {}", e))?;
        
        // Extraer campos
        let id = get_field_value(&retrieved_doc, &schema, fields::ID)
            .unwrap_or_default();
        let tipo = get_field_value(&retrieved_doc, &schema, fields::TIPO)
            .unwrap_or_default();
        
        results.push(SearchResult {
            id,
            tipo,
            score,
        });
    }
    
    Ok(results)
}

/// Busca solo en contratistas
pub fn search_contratistas(
    index: &Index,
    reader: &IndexReader,
    query_str: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    // Agregar filtro de tipo
    let filtered_query = format!("{} AND tipo:contratista", query_str);
    search_index(index, reader, &filtered_query, limit)
}

/// Helper: extrae el valor de un campo del documento
fn get_field_value(
    doc: &tantivy::TantivyDocument,
    schema: &Schema,
    field_name: &str,
) -> Option<String> {
    let field = schema.get_field(field_name).ok()?;
    doc.get_first(field)?.as_str().map(|s| s.to_string())
}