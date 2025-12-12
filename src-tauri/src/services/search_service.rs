use crate::db::contratista_queries;
use crate::db::lista_negra_queries;
use crate::db::user_queries;
use crate::models::contratista::Contratista;
use crate::models::lista_negra::ListaNegra;
use crate::models::user::User;
use crate::search::{
    commit_index, delete_from_index, index_contratista, index_lista_negra, index_user,
    update_contratista_in_index, update_lista_negra_in_index, update_user_in_index,
};
use crate::search::{get_index_reader, get_index_writer, initialize_index};
use crate::search::{search_index, SearchResult};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::sync::Arc;
use tantivy::{Index, IndexReader};
use tokio::sync::Mutex;

/// Estado del servicio de búsqueda
pub struct SearchService {
    pub index: Arc<Index>,
    pub reader: Arc<IndexReader>,
    pub writer_mutex: Mutex<()>,
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
            writer_mutex: Mutex::new(()),
            index_path,
        })
    }

    /// Re-indexa todo (contratistas, usuarios y lista negra) desde la base de datos
    pub async fn reindex_all(&self, pool: &SqlitePool) -> Result<(), String> {
        // Obtener todos los contratistas con empresa (Async, sin lock)
        let contratistas = contratista_queries::find_all_with_empresa(pool).await?;

        // Obtener todos los usuarios
        let users = user_queries::find_all(pool).await?;

        // Obtener todos los registros de lista negra
        let lista_negra = lista_negra_queries::find_all(pool).await?;

        // Adquirir lock para escribir en el índice
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        // Limpiar índice existente
        writer
            .delete_all_documents()
            .map_err(|e| format!("Error al limpiar índice: {}", e))?;

        // Indexar contratistas
        for (contratista, empresa_nombre, _, _, _) in contratistas {
            index_contratista(&mut writer, &schema, &contratista, &empresa_nombre)?;
        }

        // Indexar usuarios
        for user in users {
            index_user(&mut writer, &schema, &user)?;
        }

        // Indexar lista negra
        for ln in lista_negra {
            index_lista_negra(&mut writer, &schema, &ln)?;
        }

        // Commit
        commit_index(&mut writer)?;

        // Reload reader
        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Método legado para compatibilidad si es necesario, ahora llama a reindex_all
    pub async fn reindex_all_contratistas(&self, pool: &SqlitePool) -> Result<(), String> {
        self.reindex_all(pool).await
    }

    /// Indexa un contratista nuevo
    pub async fn add_contratista(
        &self,
        contratista: &Contratista,
        empresa_nombre: &str,
    ) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

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
    pub async fn update_contratista(
        &self,
        contratista: &Contratista,
        empresa_nombre: &str,
    ) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

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
    pub async fn delete_contratista(&self, id: &str) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        delete_from_index(&mut writer, &schema, id)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Indexa un usuario nuevo
    pub async fn add_user(&self, user: &User) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        index_user(&mut writer, &schema, user)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Actualiza un usuario en el índice
    pub async fn update_user(&self, user: &User) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        update_user_in_index(&mut writer, &schema, user)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Elimina un usuario del índice
    pub async fn delete_user(&self, id: &str) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        delete_from_index(&mut writer, &schema, id)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Indexa un registro de lista negra
    pub async fn add_lista_negra(&self, lista_negra: &ListaNegra) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        index_lista_negra(&mut writer, &schema, lista_negra)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Actualiza un registro de lista negra en el índice
    pub async fn update_lista_negra(&self, lista_negra: &ListaNegra) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        update_lista_negra_in_index(&mut writer, &schema, lista_negra)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Elimina un registro de lista negra del índice
    pub async fn delete_lista_negra(&self, id: &str) -> Result<(), String> {
        let _lock = self.writer_mutex.lock().await;

        let schema = self.index.schema();
        let mut writer = get_index_writer(&self.index)?;

        delete_from_index(&mut writer, &schema, id)?;
        commit_index(&mut writer)?;

        self.reader
            .reload()
            .map_err(|e| format!("Error al recargar reader: {}", e))?;

        Ok(())
    }

    /// Busca en el índice (contratistas, usuarios y lista negra)
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, String> {
        search_index(&self.index, &self.reader, query, limit)
    }

    /// Verifica si el índice está vacío
    pub fn is_empty(&self) -> bool {
        let searcher = self.reader.searcher();
        searcher.num_docs() == 0
    }

    /// Obtiene la cantidad de documentos en el índice
    pub fn doc_count(&self) -> u64 {
        let searcher = self.reader.searcher();
        searcher.num_docs()
    }
}
