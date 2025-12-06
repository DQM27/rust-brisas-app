// ==========================================
// src/services/search_service.rs
// ==========================================
// Servicio que maneja el índice de búsqueda de Tantivy

use crate::db::contratista_queries;
use crate::models::contratista::Contratista;
use crate::search::{
    commit_index, delete_from_index, index_contratista, update_contratista_in_index,
};
use crate::search::{get_index_reader, get_index_writer, initialize_index};
use crate::search::{search_contratistas, SearchResult};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::sync::Arc;
use tantivy::{Index, IndexReader};

/// Estado del servicio de búsqueda
pub struct SearchService {
    pub index: Arc<Index>,
    pub reader: Arc<IndexReader>,
    #[allow(dead_code)]
    index_path: PathBuf,
}

impl SearchService {
    /// Inicializa el servicio de búsqueda
    pub fn new(data_dir: &str) -> Result<Self, String> {
        let index_path = PathBuf::from(data_dir).join("search_index");

        // Inicializar índice
        let index = initialize_index(&index_path)?;
        let reader = get_index_reader(&index)?;

        Ok(Self {
            index: Arc::new(index),
            reader: Arc::new(reader),
            index_path,
        })
    }

    /// Re-indexa todos los contratistas desde la base de datos
    pub async fn reindex_all_contratistas(&self, pool: &SqlitePool) -> Result<(), String> {
        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        // Limpiar índice existente
        writer
            .delete_all_documents()
            .map_err(|e| format!("Error al limpiar índice: {}", e))?;

        // Obtener todos los contratistas con empresa
        let contratistas = contratista_queries::find_all_with_empresa(pool).await?;

        // Indexar cada uno
        for (contratista, empresa_nombre, _, _) in contratistas {
            index_contratista(&mut writer, &schema, &contratista, &empresa_nombre)?;
        }

        // Commit
        commit_index(&mut writer)?;

        // Reload reader
        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Indexa un contratista nuevo
    pub fn add_contratista(
        &self,
        contratista: &Contratista,
        empresa_nombre: &str,
    ) -> Result<(), String> {
        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        index_contratista(&mut writer, &schema, contratista, empresa_nombre)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Actualiza un contratista en el índice
    pub fn update_contratista(
        &self,
        contratista: &Contratista,
        empresa_nombre: &str,
    ) -> Result<(), String> {
        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        update_contratista_in_index(&mut writer, &schema, contratista, empresa_nombre)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Elimina un contratista del índice
    pub fn delete_contratista(&self, id: &str) -> Result<(), String> {
        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        delete_from_index(&mut writer, &schema, id)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Busca contratistas
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, String> {
        search_contratistas(&self.index, &self.reader, query, limit)
    }
}
