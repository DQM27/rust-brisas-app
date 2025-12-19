// ==========================================
// src/search/indexer.rs
// ==========================================
// Funciones para indexar documentos en Tantivy

use crate::models::contratista::Contratista;
use crate::models::lista_negra::ListaNegra;
use crate::models::proveedor::Proveedor;
use crate::models::user::User;
use crate::search::schema::{build_search_schema, fields};
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

/// Obtiene un writer para el índice
pub fn get_index_writer(index: &Index) -> Result<IndexWriter, String> {
    // Budget reducido a 3MB para ahorrar memoria (Optimización final)
    index
        .writer(3_000_000)
        .map_err(|e| format!("Error al crear writer: {}", e))
}

/// Indexa un contratista con su nombre de empresa
pub fn index_contratista(
    writer: &mut IndexWriter,
    schema: &Schema,
    contratista: &Contratista,
    empresa_nombre: &str,
) -> Result<(), String> {
    // Obtener handles de campos
    let id_field = schema.get_field(fields::ID).unwrap();
    let tipo_field = schema.get_field(fields::TIPO).unwrap();
    let cedula_field = schema.get_field(fields::CEDULA).unwrap();
    let nombre_field = schema.get_field(fields::NOMBRE).unwrap();
    let segundo_nombre_field = schema.get_field(fields::SEGUNDO_NOMBRE).unwrap();
    let apellido_field = schema.get_field(fields::APELLIDO).unwrap();
    let segundo_apellido_field = schema.get_field(fields::SEGUNDO_APELLIDO).unwrap();
    let empresa_nombre_field = schema.get_field(fields::EMPRESA_NOMBRE).unwrap();
    let search_text_field = schema.get_field(fields::SEARCH_TEXT).unwrap();

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

    // Crear documento
    let mut doc = TantivyDocument::default();
    doc.add_text(id_field, &contratista.id);
    doc.add_text(tipo_field, "contratista");
    doc.add_text(cedula_field, &contratista.cedula);
    doc.add_text(nombre_field, &contratista.nombre);

    if let Some(ref segundo_nombre) = contratista.segundo_nombre {
        doc.add_text(segundo_nombre_field, segundo_nombre);
    }

    doc.add_text(apellido_field, &contratista.apellido);

    if let Some(ref segundo_apellido) = contratista.segundo_apellido {
        doc.add_text(segundo_apellido_field, segundo_apellido);
    }

    doc.add_text(empresa_nombre_field, empresa_nombre);
    doc.add_text(search_text_field, &search_text);

    // Agregar al índice
    writer
        .add_document(doc)
        .map_err(|e| format!("Error al agregar documento: {}", e))?;

    Ok(())
}

/// Indexa un usuario
pub fn index_user(writer: &mut IndexWriter, schema: &Schema, user: &User) -> Result<(), String> {
    // Obtener handles de campos
    let id_field = schema.get_field(fields::ID).unwrap();
    let tipo_field = schema.get_field(fields::TIPO).unwrap();
    let cedula_field = schema.get_field(fields::CEDULA).unwrap();
    let nombre_field = schema.get_field(fields::NOMBRE).unwrap();
    let segundo_nombre_field = schema.get_field(fields::SEGUNDO_NOMBRE).unwrap();
    let apellido_field = schema.get_field(fields::APELLIDO).unwrap();
    let segundo_apellido_field = schema.get_field(fields::SEGUNDO_APELLIDO).unwrap();
    let email_field = schema.get_field(fields::EMAIL).unwrap();
    let search_text_field = schema.get_field(fields::SEARCH_TEXT).unwrap();

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

    // Crear documento
    let mut doc = TantivyDocument::default();
    doc.add_text(id_field, &user.id);
    doc.add_text(tipo_field, "usuario"); // Usamos "usuario" para users
    doc.add_text(cedula_field, &user.cedula);
    doc.add_text(nombre_field, &user.nombre);
    doc.add_text(email_field, &user.email);

    if let Some(ref segundo_nombre) = user.segundo_nombre {
        doc.add_text(segundo_nombre_field, segundo_nombre);
    }

    doc.add_text(apellido_field, &user.apellido);

    if let Some(ref segundo_apellido) = user.segundo_apellido {
        doc.add_text(segundo_apellido_field, segundo_apellido);
    }

    doc.add_text(search_text_field, &search_text);

    // Agregar al índice
    writer
        .add_document(doc)
        .map_err(|e| format!("Error al agregar usuario: {}", e))?;

    Ok(())
}

/// Elimina un documento del índice por ID
pub fn delete_from_index(
    writer: &mut IndexWriter,
    schema: &Schema,
    id: &str,
) -> Result<(), String> {
    let id_field = schema.get_field(fields::ID).unwrap();
    let term = tantivy::Term::from_field_text(id_field, id);

    writer.delete_term(term);

    Ok(())
}

/// Actualiza un documento (delete + insert)
pub fn update_contratista_in_index(
    writer: &mut IndexWriter,
    schema: &Schema,
    contratista: &Contratista,
    empresa_nombre: &str,
) -> Result<(), String> {
    // Eliminar el documento viejo
    delete_from_index(writer, schema, &contratista.id)?;

    // Agregar el documento actualizado
    index_contratista(writer, schema, contratista, empresa_nombre)?;

    Ok(())
}

/// Actualiza un usuario (delete + insert)
pub fn update_user_in_index(
    writer: &mut IndexWriter,
    schema: &Schema,
    user: &User,
) -> Result<(), String> {
    // Eliminar el documento viejo
    delete_from_index(writer, schema, &user.id)?;

    // Agregar el documento actualizado
    index_user(writer, schema, user)?;

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
    schema: &Schema,
    lista_negra: &ListaNegra,
) -> Result<(), String> {
    let id_field = schema.get_field(fields::ID).unwrap();
    let tipo_field = schema.get_field(fields::TIPO).unwrap();
    let cedula_field = schema.get_field(fields::CEDULA).unwrap();
    let nombre_field = schema.get_field(fields::NOMBRE).unwrap();
    let segundo_nombre_field = schema.get_field(fields::SEGUNDO_NOMBRE).unwrap();
    let apellido_field = schema.get_field(fields::APELLIDO).unwrap();
    let segundo_apellido_field = schema.get_field(fields::SEGUNDO_APELLIDO).unwrap();
    let search_text_field = schema.get_field(fields::SEARCH_TEXT).unwrap();

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

    // Crear documento
    let mut doc = TantivyDocument::default();
    doc.add_text(id_field, &lista_negra.id);
    doc.add_text(tipo_field, "lista_negra");
    doc.add_text(cedula_field, &lista_negra.cedula);
    doc.add_text(nombre_field, &lista_negra.nombre);

    if let Some(ref segundo_nombre) = lista_negra.segundo_nombre {
        doc.add_text(segundo_nombre_field, segundo_nombre);
    }

    doc.add_text(apellido_field, &lista_negra.apellido);

    if let Some(ref segundo_apellido) = lista_negra.segundo_apellido {
        doc.add_text(segundo_apellido_field, segundo_apellido);
    }

    doc.add_text(search_text_field, &search_text);

    writer
        .add_document(doc)
        .map_err(|e| format!("Error al agregar lista negra: {}", e))?;

    Ok(())
}

/// Actualiza una entrada de lista negra (delete + insert)
pub fn update_lista_negra_in_index(
    writer: &mut IndexWriter,
    schema: &Schema,
    lista_negra: &ListaNegra,
) -> Result<(), String> {
    delete_from_index(writer, schema, &lista_negra.id)?;
    index_lista_negra(writer, schema, lista_negra)?;
    Ok(())
}

/// Indexa un proveedor con su nombre de empresa
pub fn index_proveedor(
    writer: &mut IndexWriter,
    schema: &Schema,
    proveedor: &Proveedor,
    empresa_nombre: &str,
) -> Result<(), String> {
    // Obtener handles de campos
    let id_field = schema.get_field(fields::ID).unwrap();
    let tipo_field = schema.get_field(fields::TIPO).unwrap();
    let cedula_field = schema.get_field(fields::CEDULA).unwrap();
    let nombre_field = schema.get_field(fields::NOMBRE).unwrap();
    let segundo_nombre_field = schema.get_field(fields::SEGUNDO_NOMBRE).unwrap();
    let apellido_field = schema.get_field(fields::APELLIDO).unwrap();
    let segundo_apellido_field = schema.get_field(fields::SEGUNDO_APELLIDO).unwrap();
    let empresa_nombre_field = schema.get_field(fields::EMPRESA_NOMBRE).unwrap();
    let search_text_field = schema.get_field(fields::SEARCH_TEXT).unwrap();

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

    // Crear documento
    let mut doc = TantivyDocument::default();
    doc.add_text(id_field, &proveedor.id);
    doc.add_text(tipo_field, "proveedor");
    doc.add_text(cedula_field, &proveedor.cedula);
    doc.add_text(nombre_field, &proveedor.nombre);

    if let Some(ref segundo_nombre) = proveedor.segundo_nombre {
        doc.add_text(segundo_nombre_field, segundo_nombre);
    }

    doc.add_text(apellido_field, &proveedor.apellido);

    if let Some(ref segundo_apellido) = proveedor.segundo_apellido {
        doc.add_text(segundo_apellido_field, segundo_apellido);
    }

    doc.add_text(empresa_nombre_field, empresa_nombre);
    doc.add_text(search_text_field, &search_text);

    // Agregar al índice
    writer
        .add_document(doc)
        .map_err(|e| format!("Error al agregar proveedor: {}", e))?;

    Ok(())
}

/// Actualiza un proveedor en el índice (delete + insert)
pub fn update_proveedor_in_index(
    writer: &mut IndexWriter,
    schema: &Schema,
    proveedor: &Proveedor,
    empresa_nombre: &str,
) -> Result<(), String> {
    delete_from_index(writer, schema, &proveedor.id)?;
    index_proveedor(writer, schema, proveedor, empresa_nombre)?;
    Ok(())
}
