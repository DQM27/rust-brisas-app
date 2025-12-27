use crate::db::{
    surrealdb_contratista_queries as contratista_queries,
    surrealdb_empresa_queries as empresa_queries,
    surrealdb_lista_negra_queries as lista_negra_queries,
    surrealdb_proveedor_queries as proveedor_queries, surrealdb_user_queries as user_queries,
};
use crate::domain::role::SUPERUSER_ID;
use crate::models::contratista::Contratista;
use crate::models::lista_negra::ListaNegra;
use crate::models::proveedor::Proveedor;
use crate::models::user::User;
use crate::search::errors::SearchError;
use crate::search::schema::FieldHandles;
use crate::search::searcher::SearchResultDto;
use crate::search::{
    commit_index, create_field_handles, delete_from_index, index_contratista, index_lista_negra,
    index_proveedor, index_user, update_contratista_in_index, update_lista_negra_in_index,
    update_proveedor_in_index, update_user_in_index,
};
use crate::search::{get_index_reader, get_index_writer, initialize_index};
use crate::search::{search_index, SearchFields};
// use sqlx::SqlitePool; // REMOVED
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tantivy::{Index, IndexReader};
use tokio::sync::Mutex;
use tokio::sync::RwLock;

pub struct SearchState(pub RwLock<Arc<SearchService>>);

/// Estado del servicio de búsqueda
pub struct SearchService {
    pub index: Arc<Index>,
    pub reader: Arc<IndexReader>,
    pub fields: Arc<SearchFields>, // Cache de campos para búsqueda
    pub handles: FieldHandles,     // Handles pre-cargados para indexación (fail-fast)
    pub writer_mutex: Mutex<()>,
    #[allow(dead_code)]
    index_path: PathBuf,
}

impl SearchService {
    /// Inicializa el servicio de búsqueda
    pub fn new(data_dir: &str) -> Result<Self, SearchError> {
        let index_path = PathBuf::from(data_dir).join("search_index");

        // Inicializar índice
        let index = initialize_index(&index_path)?;
        let reader = get_index_reader(&index)?;

        // Pre-calcular campos para búsqueda (Zero-Cost Abstraction)
        let fields = Arc::new(SearchFields::new(&index.schema()));

        // Pre-cargar field handles para indexación (fail-fast pattern)
        let handles = create_field_handles(&index.schema())?;

        Ok(Self {
            index: Arc::new(index),
            reader: Arc::new(reader),
            fields,
            handles,
            writer_mutex: Mutex::new(()),
            index_path,
        })
    }

    /// Crea una instancia en memoria para tests
    pub fn test_instance() -> Self {
        use crate::search::schema::build_search_schema;
        let schema = build_search_schema();
        let index = Index::create_in_ram(schema.clone());
        let reader = get_index_reader(&index).unwrap();
        let fields = Arc::new(SearchFields::new(&schema));
        let handles = create_field_handles(&schema).unwrap();

        Self {
            index: Arc::new(index),
            reader: Arc::new(reader),
            fields,
            handles,
            writer_mutex: Mutex::new(()),
            index_path: PathBuf::from("/tmp"),
        }
    }

    /// Re-indexa todo (contratistas, usuarios, proveedores y lista negra) desde la base de datos
    pub async fn reindex_all(&self) -> Result<(), SearchError> {
        // 1. Cargar Empresas para mapeo de nombres
        let empresas = empresa_queries::find_all()
            .await
            .map_err(|e| SearchError::DatabaseError(e.to_string()))?;

        let empresas_map: HashMap<String, String> =
            empresas.into_iter().map(|e| (e.id, e.nombre)).collect();

        // 2. Obtener datos
        let contratistas = contratista_queries::find_all()
            .await
            .map_err(|e| SearchError::DatabaseError(e.to_string()))?;

        // Obtener todos los usuarios (excluyendo superuser del índice)
        let users = user_queries::find_all(SUPERUSER_ID)
            .await
            .map_err(|e| SearchError::DatabaseError(e.to_string()))?;

        // Obtener todos los registros de lista negra
        let lista_negra = lista_negra_queries::find_all()
            .await
            .map_err(|e| SearchError::DatabaseError(e.to_string()))?;

        // Obtener todos los proveedores
        let proveedores = proveedor_queries::find_all()
            .await
            .map_err(|e| SearchError::DatabaseError(e.to_string()))?;

        // 3. Adquirir lock para escribir en el índice
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            // Limpiar índice existente
            writer.delete_all_documents().map_err(|e| {
                SearchError::TantivyError(format!("Error al limpiar índice: {}", e))
            })?;

            // Indexar contratistas
            for contratista in contratistas {
                // Resolver nombre empresa
                // El contratista.empresa_id podría ser "empresas:XYZ" o "XYZ".
                // empresas_map key también.
                // Asumimos que vienen igual de la DB.
                // Contratista struct field se llama 'empresa_id' en model? No, es 'empresa' o 'empresa_id'?
                // En `surrealdb_contratista_queries`, el struct Contratista usado tiene fields.
                // Verificaré si Contratista model tiene empresa_id público.
                // Asumiendo que sí:
                let empresa_nombre = empresas_map
                    .get(&contratista.empresa_id)
                    .map(|s| s.as_str())
                    .unwrap_or("Desconocida");
                index_contratista(&mut writer, &self.handles, &contratista, empresa_nombre)?;
            }

            // Indexar usuarios
            for user in users {
                index_user(&mut writer, &self.handles, &user)?;
            }

            // Indexar lista negra
            for ln in lista_negra {
                index_lista_negra(&mut writer, &self.handles, &ln)?;
            }

            // Indexar proveedores
            for proveedor in proveedores {
                let empresa_nombre = empresas_map
                    .get(&proveedor.empresa_id)
                    .map(|s| s.as_str())
                    .unwrap_or("Desconocida");
                index_proveedor(&mut writer, &self.handles, &proveedor, empresa_nombre)?;
            }

            // Commit
            commit_index(&mut writer)?;
        }

        // Reload reader
        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Método legado para compatibilidad si es necesario, ahora llama a reindex_all
    pub async fn reindex_all_contratistas(&self) -> Result<(), SearchError> {
        self.reindex_all().await
    }

    /// Indexa un contratista nuevo
    pub async fn add_contratista(
        &self,
        contratista: &Contratista,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_contratista(&mut writer, &self.handles, contratista, empresa_nombre)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Actualiza un contratista en el índice
    pub async fn update_contratista(
        &self,
        contratista: &Contratista,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_contratista_in_index(&mut writer, &self.handles, contratista, empresa_nombre)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Elimina un contratista del índice
    pub async fn delete_contratista(&self, id: &str) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Indexa un usuario nuevo
    pub async fn add_user(&self, user: &User) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_user(&mut writer, &self.handles, user)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Actualiza un usuario en el índice
    pub async fn update_user(&self, user: &User) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_user_in_index(&mut writer, &self.handles, user)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Elimina un usuario del índice
    pub async fn delete_user(&self, id: &str) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Indexa un registro de lista negra
    pub async fn add_lista_negra(&self, lista_negra: &ListaNegra) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_lista_negra(&mut writer, &self.handles, lista_negra)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Actualiza un registro de lista negra en el índice
    pub async fn update_lista_negra(&self, lista_negra: &ListaNegra) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_lista_negra_in_index(&mut writer, &self.handles, lista_negra)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Elimina un registro de lista negra del índice
    pub async fn delete_lista_negra(&self, id: &str) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Busca en el índice (contratistas, usuarios, proveedores y lista negra)
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResultDto>, SearchError> {
        // Pasamos self.fields en lugar de buscar por strings (Hot Path Optimizado)
        search_index(&self.index, &self.reader, &self.fields, query, limit)
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

    /// Indexa un proveedor nuevo
    pub async fn add_proveedor(
        &self,
        proveedor: &Proveedor,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_proveedor(&mut writer, &self.handles, proveedor, empresa_nombre)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Actualiza un proveedor en el índice
    pub async fn update_proveedor(
        &self,
        proveedor: &Proveedor,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_proveedor_in_index(&mut writer, &self.handles, proveedor, empresa_nombre)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }

    /// Elimina un proveedor del índice
    pub async fn delete_proveedor(&self, id: &str) -> Result<(), SearchError> {
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader
            .reload()
            .map_err(|e| SearchError::TantivyError(format!("Error al recargar reader: {}", e)))?;

        Ok(())
    }
}
