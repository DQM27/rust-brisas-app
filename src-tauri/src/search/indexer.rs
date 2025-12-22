// ==========================================
// src/search/indexer.rs
// ==========================================
// Funciones para indexar documentos en Tantivy

use crate::models::contratista::Contratista;
use crate::models::lista_negra::ListaNegra;
use crate::models::proveedor::Proveedor;
use crate::models::user::User;
use crate::search::schema::{build_search_schema, fields, FieldHandles};
use std::path::Path;
use tantivy::schema::Schema;
use tantivy::{Index, IndexWriter, TantivyDocument};

/// Inicializa el índice de Tantivy
pub fn initialize_index(index_path: &Path) -> Result<Index, String> {
    let schema = build_search_schema();

    // Crear o abrir índice existente
    if index_path.exists() {
        let index =
            Index::open_in_dir(index_path).map_err(|e| format!("Error al abrir índice: {}", e))?;

        // Verificar si existe el campo "email" (indicador simple de migración)
        if index.schema().get_field(fields::EMAIL).is_err() {
            // Intentar eliminar directorio
            std::fs::remove_dir_all(index_path)
                .map_err(|e| format!("Error al eliminar índice obsoleto: {}", e))?;
            std::fs::create_dir_all(index_path)
                .map_err(|e| format!("Error al crear directorio de índice: {}", e))?;

            Index::create_in_dir(index_path, schema)
                .map_err(|e| format!("Error al crear índice: {}", e))
        } else {
            Ok(index)
        }
    } else {
        std::fs::create_dir_all(index_path)
            .map_err(|e| format!("Error al crear directorio de índice: {}", e))?;

        Index::create_in_dir(index_path, schema)
            .map_err(|e| format!("Error al crear índice: {}", e))
    }
}

/// Crea los FieldHandles desde el schema del índice.
/// Debe llamarse una vez al inicializar y pasarse a las funciones de indexación.
pub fn create_field_handles(schema: &Schema) -> Result<FieldHandles, String> {
    FieldHandles::new(schema)
}

/// Obtiene un writer para el índice
pub fn get_index_writer(index: &Index) -> Result<IndexWriter, String> {
    // Budget ajustado a 15MB (Mínimo requerido por Tantivy)
    index
        .writer(15_000_000)
        .map_err(|e| format!("Error al crear writer: {}", e))
}

/// Indexa un contratista con su nombre de empresa
pub fn index_contratista(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    contratista: &Contratista,
    empresa_nombre: &str,
) -> Result<(), String> {
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
    doc.add_text(handles.id, &contratista.id);
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
        .map_err(|e| format!("Error al agregar documento: {}", e))?;

    Ok(())
}

/// Indexa un usuario
pub fn index_user(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    user: &User,
) -> Result<(), String> {
    // Construir texto de búsqueda concatenado
    let mut search_text_parts = vec![
        user.cedula.clone(),
        user.nombre.clone(),
        user.apellido.clone(),
        user.email.clone(),
    ];

    if let Some(ref segundo_nombre) = user.segundo_nombre {
        search_text_parts.push(segundo_nombre.clone());
    }

    if let Some(ref segundo_apellido) = user.segundo_apellido {
        search_text_parts.push(segundo_apellido.clone());
    }

    let search_text = search_text_parts.join(" ");

    // Crear documento usando handles pre-cargados
    let mut doc = TantivyDocument::default();
    doc.add_text(handles.id, &user.id);
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
        .map_err(|e| format!("Error al agregar usuario: {}", e))?;

    Ok(())
}

/// Elimina un documento del índice por ID
pub fn delete_from_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    id: &str,
) -> Result<(), String> {
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
) -> Result<(), String> {
    // Eliminar el documento viejo
    delete_from_index(writer, handles, &contratista.id)?;

    // Agregar el documento actualizado
    index_contratista(writer, handles, contratista, empresa_nombre)?;

    Ok(())
}

/// Actualiza un usuario (delete + insert)
pub fn update_user_in_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    user: &User,
) -> Result<(), String> {
    // Eliminar el documento viejo
    delete_from_index(writer, handles, &user.id)?;

    // Agregar el documento actualizado
    index_user(writer, handles, user)?;

    Ok(())
}

/// Commit de los cambios al índice
pub fn commit_index(writer: &mut IndexWriter) -> Result<(), String> {
    writer
        .commit()
        .map_err(|e| format!("Error al hacer commit: {}", e))?;
    Ok(())
}

/// Indexa una entrada de lista negra
pub fn index_lista_negra(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    lista_negra: &ListaNegra,
) -> Result<(), String> {
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
        .map_err(|e| format!("Error al agregar lista negra: {}", e))?;

    Ok(())
}

/// Actualiza una entrada de lista negra (delete + insert)
pub fn update_lista_negra_in_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    lista_negra: &ListaNegra,
) -> Result<(), String> {
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
) -> Result<(), String> {
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
    doc.add_text(handles.id, &proveedor.id);
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
        .map_err(|e| format!("Error al agregar proveedor: {}", e))?;

    Ok(())
}

/// Actualiza un proveedor en el índice (delete + insert)
pub fn update_proveedor_in_index(
    writer: &mut IndexWriter,
    handles: &FieldHandles,
    proveedor: &Proveedor,
    empresa_nombre: &str,
) -> Result<(), String> {
    delete_from_index(writer, handles, &proveedor.id)?;
    index_proveedor(writer, handles, proveedor, empresa_nombre)?;
    Ok(())
}
