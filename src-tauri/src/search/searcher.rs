// ==========================================
// src/search/searcher.rs
// ==========================================
// Funciones para buscar en el índice de Tantivy

use crate::search::errors::SearchError;
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
            id: schema
                .get_field(fields::ID)
                .expect("Falta campo id en schema (debe validarse al inicio)"),
            tipo: schema
                .get_field(fields::TIPO)
                .expect("Falta campo tipo en schema (debe validarse al inicio)"),
            cedula: schema
                .get_field(fields::CEDULA)
                .expect("Falta campo cedula en schema (debe validarse al inicio)"),
            nombre: schema
                .get_field(fields::NOMBRE)
                .expect("Falta campo nombre en schema (debe validarse al inicio)"),
            segundo_nombre: schema
                .get_field(fields::SEGUNDO_NOMBRE)
                .expect("Falta campo segundo_nombre en schema (debe validarse al inicio)"),
            apellido: schema
                .get_field(fields::APELLIDO)
                .expect("Falta campo apellido en schema (debe validarse al inicio)"),
            segundo_apellido: schema
                .get_field(fields::SEGUNDO_APELLIDO)
                .expect("Falta campo segundo_apellido en schema (debe validarse al inicio)"),
            empresa_nombre: schema
                .get_field(fields::EMPRESA_NOMBRE)
                .expect("Falta campo empresa_nombre en schema (debe validarse al inicio)"),
            email: schema
                .get_field(fields::EMAIL)
                .expect("Falta campo email en schema (debe validarse al inicio)"),
            search_text: schema
                .get_field(fields::SEARCH_TEXT)
                .expect("Falta campo search_text en schema (debe validarse al inicio)"),
        }
    }
}

/// Resultado de búsqueda
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultDto {
    pub id: String,
    pub tipo: String,
    pub score: f32,
    pub cedula: Option<String>,
    pub nombre_completo: Option<String>,
    pub empresa_nombre: Option<String>,
    pub email: Option<String>,
}

/// Inicializa un reader para búsquedas
pub fn get_index_reader(index: &Index) -> Result<IndexReader, SearchError> {
    index
        .reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into()
        .map_err(|e| SearchError::TantivyError(format!("Error al crear reader: {}", e)))
}

/// Busca en el índice con fuzzy search (Optimizado)
pub fn search_index(
    index: &Index,
    reader: &IndexReader,
    fields: &SearchFields, // Cache de campos inyectada
    query_str: &str,
    limit: usize,
) -> Result<Vec<SearchResultDto>, SearchError> {
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
        fields.tipo,
    ];

    // 2. Parser
    let mut query_parser = QueryParser::for_index(index, search_target_fields);
    // Usamos el ID numérico directo:
    query_parser.set_field_fuzzy(fields.search_text, true, 1, true);

    // Parsear query
    let query = query_parser
        .parse_query(query_str)
        .map_err(|e| SearchError::QueryError(format!("Query inválido: {}", e)))?;

    // Ejecutar búsqueda
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|e| SearchError::TantivyError(format!("Error búsqueda: {}", e)))?;

    let mut results = Vec::with_capacity(top_docs.len());

    for (score, doc_addr) in top_docs {
        let doc: TantivyDocument =
            searcher.doc(doc_addr).map_err(|e| SearchError::TantivyError(e.to_string()))?;

        // 3. Extracción optimizada usando los IDs cacheados
        results.push(SearchResultDto {
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
) -> Result<Vec<SearchResultDto>, SearchError> {
    // Agregar filtro de tipo
    let filtered_query = format!("+tipo:contratista +({})", query_str);
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::contratista::{Contratista, EstadoContratista};
    use crate::models::user::User;
    use crate::search::indexer::{
        commit_index, create_field_handles, get_index_writer, index_contratista, index_user,
    };
    use crate::search::schema::build_search_schema;

    fn setup_test_index_with_data() -> (Index, IndexReader, SearchFields) {
        let schema = build_search_schema();
        let index = Index::create_in_ram(schema.clone());
        let handles = create_field_handles(&schema).unwrap();
        let mut writer = get_index_writer(&index).unwrap();

        // 1. Contratista
        let c = Contratista {
            id: "c1".into(),
            cedula: "123".into(),
            nombre: "Alexander".into(),
            segundo_nombre: None,
            apellido: "Gomez".into(),
            segundo_apellido: None,
            empresa_id: "emp-1".into(),
            fecha_vencimiento_praind: "2025-01-01".into(),
            estado: EstadoContratista::Activo,
            created_at: "".into(),
            updated_at: "".into(),
        };
        index_contratista(&mut writer, &handles, &c, "Intel").unwrap();

        // 2. Usuario
        let u = User {
            id: "u1".into(),
            cedula: "456".into(),
            nombre: "John".into(),
            segundo_nombre: None,
            apellido: "Doe".into(),
            segundo_apellido: None,
            email: "john@doe.com".into(),
            role_id: "role-1".into(),
            is_active: true,
            must_change_password: false,
            fecha_inicio_labores: None,
            numero_gafete: None,
            fecha_nacimiento: None,
            telefono: None,
            direccion: None,
            contacto_emergencia_nombre: None,
            contacto_emergencia_telefono: None,
            deleted_at: None,
            created_at: "".into(),
            updated_at: "".into(),
        };
        index_user(&mut writer, &handles, &u).unwrap();

        commit_index(&mut writer).unwrap();

        let reader = get_index_reader(&index).unwrap();
        let fields = SearchFields::new(&schema);
        (index, reader, fields)
    }

    #[test]
    fn test_fuzzy_search() {
        let (index, reader, fields) = setup_test_index_with_data();

        // Exact
        let res = search_index(&index, &reader, &fields, "Alexander", 10).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, "c1");

        // Fuzzy (Alexande -> Alexander)
        let res = search_index(&index, &reader, &fields, "Alexande", 10).unwrap();
        assert!(res.len() >= 1);
    }

    #[test]
    fn test_search_contratistas_filter() {
        let (index, reader, fields) = setup_test_index_with_data();

        // "Doe" matches user, but search_contratistas should filter it out
        println!("Searching for 'Doe' with contractor filter...");
        let res = search_contratistas(&index, &reader, &fields, "Doe", 10).unwrap();
        for r in &res {
            println!("MATCH FOUND - ID: {}, Tipo: {}, Name: {:?}", r.id, r.tipo, r.nombre_completo);
        }
        assert_eq!(res.len(), 0, "Should NOT find User 'Doe' when filtering for contractors");

        // "Alexander" matches contratista
        println!("Searching for 'Alexander' with contractor filter...");
        let res = search_contratistas(&index, &reader, &fields, "Alexander", 10).unwrap();
        assert_eq!(res.len(), 1, "Should find Contractor 'Alexander'");
    }
}
