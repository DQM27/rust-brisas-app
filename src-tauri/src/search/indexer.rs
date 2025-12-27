// ==========================================
// src/search/indexer.rs
// ==========================================
// Funciones para indexar documentos en Tantivy

use crate::models::contratista::Contratista;
use crate::models::lista_negra::ListaNegra;
use crate::models::proveedor::Proveedor;
use crate::models::user::User;
use crate::search::errors::SearchError;
use crate::search::schema::{build_search_schema, fields, FieldHandles};
use std::path::Path;
use tantivy::schema::Schema;
use tantivy::{Index, IndexWriter, TantivyDocument};

/// Inicializa el índice de Tantivy
pub fn initialize_index(index_path: &Path) -> Result<Index, SearchError> {
    let schema = build_search_schema();

    // Crear o abrir índice existente
    if index_path.exists() {
        let index = Index::open_in_dir(index_path)
            .map_err(|e| SearchError::TantivyError(format!("Error al abrir índice: {}", e)))?;

        // Verificar si existe el campo "email" (indicador simple de migración)
        if index.schema().get_field(fields::EMAIL).is_err() {
            // Intentar eliminar directorio
            std::fs::remove_dir_all(index_path).map_err(|e| {
                SearchError::IoError(format!("Error al eliminar índice obsoleto: {}", e))
            })?;
            std::fs::create_dir_all(index_path).map_err(|e| {
                SearchError::IoError(format!("Error al crear directorio de índice: {}", e))
            })?;

            Index::create_in_dir(index_path, schema)
                .map_err(|e| SearchError::TantivyError(format!("Error al crear índice: {}", e)))
        } else {
            Ok(index)
        }
    } else {
        std::fs::create_dir_all(index_path).map_err(|e| {
            SearchError::IoError(format!("Error al crear directorio de índice: {}", e))
        })?;

        Index::create_in_dir(index_path, schema)
            .map_err(|e| SearchError::TantivyError(format!("Error al crear índice: {}", e)))
    }
}

/// Crea los FieldHandles desde el schema del índice.
pub fn create_field_handles(schema: &Schema) -> Result<FieldHandles, SearchError> {
    FieldHandles::new(schema)
}

/// Obtiene un writer para el índice
pub fn get_index_writer(index: &Index) -> Result<IndexWriter, SearchError> {
    // Budget ajustado a 15MB (Mínimo requerido por Tantivy)
    index
        .writer(15_000_000)
        .map_err(|e| SearchError::TantivyError(format!("Error al crear writer: {}", e)))
}

/// Indexa un contratista con su nombre de empresa
pub fn index_contratista(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    contratista: &Contratista,
    empresa_nombre: &str,
) -> Result<(), SearchError> {
    // Construir texto de búsqueda concatenado
    let mut search_text_parts = vec![
        contratista.cedula.clone(),
        contratista.nombre.clone(),
        contratista.apellido.clone(),
        empresa_nombre.to_string(),
    ];

    if let Some(ref segundo_nombre) = contratista.segundo_nombre {
        search_text_parts.push(segundo_nombre.clone());
    }

    if let Some(ref segundo_apellido) = contratista.segundo_apellido {
        search_text_parts.push(segundo_apellido.clone());
    }

    let search_text = search_text_parts.join(" ");

    // Crear documento usando handles pre-cargados
    let mut doc = TantivyDocument::default();
    doc.add_text(handles.id, &contratista.id.to_string());
    doc.add_text(handles.tipo, "contratista");
    doc.add_text(handles.cedula, &contratista.cedula);
    doc.add_text(handles.nombre, &contratista.nombre);

    if let Some(ref segundo_nombre) = contratista.segundo_nombre {
        doc.add_text(handles.segundo_nombre, segundo_nombre);
    }

    doc.add_text(handles.apellido, &contratista.apellido);

    if let Some(ref segundo_apellido) = contratista.segundo_apellido {
        doc.add_text(handles.segundo_apellido, segundo_apellido);
    }

    doc.add_text(handles.empresa_nombre, empresa_nombre);
    doc.add_text(handles.search_text, &search_text);

    // Agregar al índice
    writer
        .add_document(doc)
        .map_err(|e| SearchError::TantivyError(format!("Error al agregar documento: {}", e)))?;

    Ok(())
}

/// Indexa un usuario
pub fn index_user(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    user: &User,
) -> Result<(), SearchError> {
    // Construir texto de búsqueda concatenado
    let mut search_text_parts =
        vec![user.cedula.clone(), user.nombre.clone(), user.apellido.clone(), user.email.clone()];

    if let Some(ref segundo_nombre) = user.segundo_nombre {
        search_text_parts.push(segundo_nombre.clone());
    }

    if let Some(ref segundo_apellido) = user.segundo_apellido {
        search_text_parts.push(segundo_apellido.clone());
    }

    let search_text = search_text_parts.join(" ");

    // Crear documento usando handles pre-cargados
    let mut doc = TantivyDocument::default();
    doc.add_text(handles.id, &user.id.to_string());
    doc.add_text(handles.tipo, "usuario");
    doc.add_text(handles.cedula, &user.cedula);
    doc.add_text(handles.nombre, &user.nombre);
    doc.add_text(handles.email, &user.email);

    if let Some(ref segundo_nombre) = user.segundo_nombre {
        doc.add_text(handles.segundo_nombre, segundo_nombre);
    }

    doc.add_text(handles.apellido, &user.apellido);

    if let Some(ref segundo_apellido) = user.segundo_apellido {
        doc.add_text(handles.segundo_apellido, segundo_apellido);
    }

    doc.add_text(handles.search_text, &search_text);

    // Agregar al índice
    writer
        .add_document(doc)
        .map_err(|e| SearchError::TantivyError(format!("Error al agregar usuario: {}", e)))?;

    Ok(())
}

/// Elimina un documento del índice por ID
pub fn delete_from_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    id: &str,
) -> Result<(), SearchError> {
    let term = tantivy::Term::from_field_text(handles.id, id);
    writer.delete_term(term);
    Ok(())
}

/// Actualiza un documento (delete + insert)
pub fn update_contratista_in_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    contratista: &Contratista,
    empresa_nombre: &str,
) -> Result<(), SearchError> {
    delete_from_index(writer, handles, &contratista.id.to_string())?;
    index_contratista(writer, handles, contratista, empresa_nombre)?;
    Ok(())
}

/// Actualiza un usuario (delete + insert)
pub fn update_user_in_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    user: &User,
) -> Result<(), SearchError> {
    delete_from_index(writer, handles, &user.id.to_string())?;
    index_user(writer, handles, user)?;
    Ok(())
}

/// Commit de los cambios al índice
pub fn commit_index(writer: &mut IndexWriter) -> Result<(), SearchError> {
    writer
        .commit()
        .map_err(|e| SearchError::TantivyError(format!("Error al hacer commit: {}", e)))?;
    Ok(())
}

/// Indexa una entrada de lista negra
pub fn index_lista_negra(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    lista_negra: &ListaNegra,
) -> Result<(), SearchError> {
    // Construir texto de búsqueda concatenado
    let mut search_text_parts = vec![
        lista_negra.cedula.clone(),
        lista_negra.nombre.clone(),
        lista_negra.apellido.clone(),
        lista_negra.motivo_bloqueo.clone(),
    ];

    if let Some(ref segundo_nombre) = lista_negra.segundo_nombre {
        search_text_parts.push(segundo_nombre.clone());
    }

    if let Some(ref segundo_apellido) = lista_negra.segundo_apellido {
        search_text_parts.push(segundo_apellido.clone());
    }

    let search_text = search_text_parts.join(" ");

    // Crear documento usando handles pre-cargados
    let mut doc = TantivyDocument::default();
    doc.add_text(handles.id, &lista_negra.id);
    doc.add_text(handles.tipo, "lista_negra");
    doc.add_text(handles.cedula, &lista_negra.cedula);
    doc.add_text(handles.nombre, &lista_negra.nombre);

    if let Some(ref segundo_nombre) = lista_negra.segundo_nombre {
        doc.add_text(handles.segundo_nombre, segundo_nombre);
    }

    doc.add_text(handles.apellido, &lista_negra.apellido);

    if let Some(ref segundo_apellido) = lista_negra.segundo_apellido {
        doc.add_text(handles.segundo_apellido, segundo_apellido);
    }

    doc.add_text(handles.search_text, &search_text);

    writer
        .add_document(doc)
        .map_err(|e| SearchError::TantivyError(format!("Error al agregar lista negra: {}", e)))?;

    Ok(())
}

/// Actualiza una entrada de lista negra (delete + insert)
pub fn update_lista_negra_in_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    lista_negra: &ListaNegra,
) -> Result<(), SearchError> {
    delete_from_index(writer, handles, &lista_negra.id)?;
    index_lista_negra(writer, handles, lista_negra)?;
    Ok(())
}

/// Indexa un proveedor con su nombre de empresa
pub fn index_proveedor(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    proveedor: &Proveedor,
    empresa_nombre: &str,
) -> Result<(), SearchError> {
    // Construir texto de búsqueda concatenado
    let mut search_text_parts = vec![
        proveedor.cedula.clone(),
        proveedor.nombre.clone(),
        proveedor.apellido.clone(),
        empresa_nombre.to_string(),
    ];

    if let Some(ref segundo_nombre) = proveedor.segundo_nombre {
        search_text_parts.push(segundo_nombre.clone());
    }

    if let Some(ref segundo_apellido) = proveedor.segundo_apellido {
        search_text_parts.push(segundo_apellido.clone());
    }

    let search_text = search_text_parts.join(" ");

    // Crear documento usando handles pre-cargados
    let mut doc = TantivyDocument::default();
    doc.add_text(handles.id, &proveedor.id.to_string());
    doc.add_text(handles.tipo, "proveedor");
    doc.add_text(handles.cedula, &proveedor.cedula);
    doc.add_text(handles.nombre, &proveedor.nombre);

    if let Some(ref segundo_nombre) = proveedor.segundo_nombre {
        doc.add_text(handles.segundo_nombre, segundo_nombre);
    }

    doc.add_text(handles.apellido, &proveedor.apellido);

    if let Some(ref segundo_apellido) = proveedor.segundo_apellido {
        doc.add_text(handles.segundo_apellido, segundo_apellido);
    }

    doc.add_text(handles.empresa_nombre, empresa_nombre);
    doc.add_text(handles.search_text, &search_text);

    // Agregar al índice
    writer
        .add_document(doc)
        .map_err(|e| SearchError::TantivyError(format!("Error al agregar proveedor: {}", e)))?;

    Ok(())
}

/// Actualiza un proveedor en el índice (delete + insert)
pub fn update_proveedor_in_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    proveedor: &Proveedor,
    empresa_nombre: &str,
) -> Result<(), SearchError> {
    delete_from_index(writer, handles, &proveedor.id.to_string())?;
    index_proveedor(writer, handles, proveedor, empresa_nombre)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::contratista::EstadoContratista;
    use crate::search::schema::build_search_schema;
    use chrono::Utc;
    use surrealdb::sql::Thing;
    use tantivy::Index;

    fn setup_test_index() -> (Index, FieldHandles) {
        let schema = build_search_schema();
        let index = Index::create_in_ram(schema.clone());
        let handles = FieldHandles::new(&schema).unwrap();
        (index, handles)
    }

    #[test]
    fn test_index_contratista() {
        let (index, handles) = setup_test_index();
        let mut writer = get_index_writer(&index).unwrap();

        let contratista = Contratista {
            id: Thing::from(("contratista", "1")),
            cedula: "123".to_string(),
            nombre: "John".to_string(),
            segundo_nombre: None,
            apellido: "Doe".to_string(),
            segundo_apellido: None,
            empresa: Thing::from(("empresa", "emp-1")),
            fecha_vencimiento_praind: Utc::now(),
            estado: EstadoContratista::Activo,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        index_contratista(&mut writer, &handles, &contratista, "Empresa A").unwrap();
        commit_index(&mut writer).unwrap();

        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 1);
    }

    #[test]
    fn test_index_and_delete() {
        let (index, handles) = setup_test_index();
        let mut writer = get_index_writer(&index).unwrap();

        let user = User {
            id: Thing::from(("user", "user-1")),
            cedula: "456".to_string(),
            nombre: "Jane".to_string(),
            segundo_nombre: None,
            apellido: "Doe".to_string(),
            segundo_apellido: None,
            email: "jane@example.com".to_string(),
            role: Thing::from(("role", "role-1")),
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
            avatar_path: None,
        };

        index_user(&mut writer, &handles, &user).unwrap();
        commit_index(&mut writer).unwrap();

        let reader = index.reader().unwrap();
        assert_eq!(reader.searcher().num_docs(), 1);

        // Delete
        delete_from_index(&mut writer, &handles, &user.id.to_string()).unwrap();
        commit_index(&mut writer).unwrap();

        reader.reload().unwrap();
        assert_eq!(reader.searcher().num_docs(), 0);
    }
}
