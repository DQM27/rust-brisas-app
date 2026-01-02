//! # Servicio: Motor de Búsqueda Global (Tantivy)
//!
//! Proporciona capacidades de búsqueda full-text sobre los datos de la aplicación.
//! Mientras que `SurrealDB` es excelente para relaciones y persistencia, Tantivy
//! permite búsquedas rápidas, fuzzy y segmentadas sobre múltiples entidades.
//!
//! ## Responsabilidades
//! - Inicialización y gestión del índice de búsqueda
//! - Indexación de entidades (Contratistas, Usuarios, Proveedores, `ListaNegra`)
//! - Reindexación completa desde `SurrealDB`
//! - Búsqueda multi-entidad de alto rendimiento
//!
//! ## Arquitectura
//! - **Index**: Índice persistido en disco
//! - **Reader**: Lector para consultas (recargable tras commits)
//! - **Writer**: Escritor protegido por Mutex
//! - **`FieldHandles`**: Cache de campos para acceso O(1)

use crate::db::{
    surrealdb_contratista_queries as contratista_queries,
    surrealdb_lista_negra_queries as lista_negra_queries,
    surrealdb_proveedor_queries as proveedor_queries, surrealdb_user_queries as user_queries,
};
use crate::models::contratista::ContratistaFetched;
use crate::models::lista_negra::ListaNegra;
use crate::models::proveedor::ProveedorFetched;
use crate::models::user::{User, UserFetched};
use crate::search::errors::SearchError;
use crate::search::schema::FieldHandles;
use crate::search::searcher::SearchResultDto;
use crate::search::{
    commit_index, create_field_handles, delete_from_index, index_contratista_fetched,
    index_lista_negra, index_proveedor_fetched, index_user, index_user_fetched,
    update_contratista_fetched_in_index, update_lista_negra_in_index,
    update_proveedor_fetched_in_index, update_user_fetched_in_index, update_user_in_index,
};
use crate::search::{
    get_index_reader, get_index_writer, initialize_index, search_index, SearchFields,
};
use log::{debug, error, info};
use std::path::PathBuf;
use std::sync::Arc;
use tantivy::{Index, IndexReader};
use tokio::sync::Mutex;
use tokio::sync::RwLock;

pub struct SearchState(pub RwLock<Arc<SearchService>>);

/// Orquestador del motor de búsqueda.
pub struct SearchService {
    /// Instancia principal del índice en disco.
    pub index: Arc<Index>,
    /// Lector para realizar consultas. Se recarga tras cada 'commit'.
    pub reader: Arc<IndexReader>,
    /// Cache de identificadores de campos (schema) para optimizar el acceso en caliente.
    pub fields: Arc<SearchFields>,
    /// Handles pre-calculados para evitar errores de mapeo durante la escritura.
    pub handles: FieldHandles,
    /// Mutex de escritura: Solo un hilo puede escribir o modificar el índice a la vez.
    pub writer_mutex: Mutex<()>,
    #[allow(dead_code)]
    index_path: PathBuf,
}

impl SearchService {
    /// Inicializa el motor de búsqueda en el directorio especificado.
    pub fn new(index_dir_path: &str) -> Result<Self, SearchError> {
        let index_path = PathBuf::from(index_dir_path);

        // Garantizamos que el directorio del índice exista para evitar fallos de inicialización.
        if !index_path.exists() {
            std::fs::create_dir_all(&index_path).map_err(|e| {
                SearchError::TantivyError(format!(
                    "No se pudo crear el directorio del índice de búsqueda: {e}"
                ))
            })?;
        }

        let index = initialize_index(&index_path)?;
        let reader = get_index_reader(&index)?;

        // Pre-calculamos los campos una sola vez al inicio para maximizar la velocidad de búsqueda.
        let fields = Arc::new(SearchFields::new(&index.schema()));
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

    /// Crea una instancia volátil en memoria, exclusiva para pruebas unitarias.
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
            index_path: PathBuf::from("memory"),
        }
    }

    /// Reconstruye el índice completo sincronizándolo con los datos actuales de `SurrealDB`.
    ///
    /// Este proceso es intensivo en recursos. Se utiliza principalmente en el arranque inicial
    /// o cuando se detecta una inconsistencia grave entre la base de datos y el motor de búsqueda.
    pub async fn reindex_all(&self) -> Result<(), SearchError> {
        info!("🔄 Iniciando reindexación completa del motor de búsqueda");

        // Obtenemos una fotografía actual de todas las entidades relevantes de la DB.
        let contratistas = contratista_queries::find_all_fetched().await.map_err(|e| {
            error!("❌ Error al cargar contratistas para reindexación: {e}");
            SearchError::DatabaseError(e.to_string())
        })?;

        let users = user_queries::find_all_fetched(None).await.map_err(|e| {
            error!("❌ Error al cargar usuarios para reindexación: {e}");
            SearchError::DatabaseError(e.to_string())
        })?;

        let lista_negra = lista_negra_queries::find_all().await.map_err(|e| {
            error!("❌ Error al cargar lista negra para reindexación: {e}");
            SearchError::DatabaseError(e.to_string())
        })?;

        let proveedores = proveedor_queries::find_all_fetched().await.map_err(|e| {
            error!("❌ Error al cargar proveedores para reindexación: {e}");
            SearchError::DatabaseError(e.to_string())
        })?;

        debug!(
            "📊 Entidades a indexar: {} contratistas, {} usuarios, {} lista_negra, {} proveedores",
            contratistas.len(),
            users.len(),
            lista_negra.len(),
            proveedores.len()
        );

        // Adquirimos el lock de escritura para evitar que otras actualizaciones parciales
        // interfieran con el vaciado y reconstrucción total del índice.
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            // Vaciamos el índice para asegurar una reconstrucción limpia y sin duplicados.
            writer.delete_all_documents().map_err(|e| {
                error!("❌ Error al limpiar índice: {e}");
                SearchError::TantivyError(format!("Error al limpiar el índice: {e}"))
            })?;

            // Procesamos e indexamos cada tipo de entidad secuencialmente.
            for c in &contratistas {
                index_contratista_fetched(&mut writer, &self.handles, c, &c.empresa.nombre)?;
            }
            for user in &users {
                index_user_fetched(&mut writer, &self.handles, user)?;
            }
            for ln in &lista_negra {
                index_lista_negra(&mut writer, &self.handles, ln)?;
            }
            for p in &proveedores {
                index_proveedor_fetched(&mut writer, &self.handles, p, &p.empresa.nombre)?;
            }

            // El commit persiste los cambios en disco.
            commit_index(&mut writer)?;
        }

        // Obligamos al lector a recargarse para que las búsquedas reflejen los nuevos datos de inmediato.
        self.reader.reload().map_err(|e| {
            error!("❌ Error al recargar lector de búsqueda: {e}");
            SearchError::TantivyError(format!("Error al recargar el lector de búsqueda: {e}"))
        })?;

        let total = contratistas.len() + users.len() + lista_negra.len() + proveedores.len();
        info!("✅ Reindexación completa: {total} documentos indexados");

        Ok(())
    }

    pub async fn reindex_all_contratistas(&self) -> Result<(), SearchError> {
        self.reindex_all().await
    }

    /// Agrega un nuevo contratista al índice de búsqueda.
    /// Agrega un nuevo contratista al índice de búsqueda.
    pub async fn add_contratista_fetched(
        &self,
        contratista: &ContratistaFetched,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        debug!("➕ Indexando contratista: {}", contratista.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_contratista_fetched(&mut writer, &self.handles, contratista, empresa_nombre)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    /// Actualiza la información de un contratista en el índice basándose en su ID único.
    /// Actualiza la información de un contratista en el índice basándose en su ID único.
    pub async fn update_contratista_fetched(
        &self,
        contratista: &ContratistaFetched,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        debug!("✏️ Actualizando índice contratista: {}", contratista.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_contratista_fetched_in_index(
                &mut writer,
                &self.handles,
                contratista,
                empresa_nombre,
            )?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    /// Elimina a un contratista del motor de búsqueda (generalmente por eliminación o archivado).
    /// Elimina a un contratista del motor de búsqueda (generalmente por eliminación o archivado).
    pub async fn delete_contratista(&self, id: &str) -> Result<(), SearchError> {
        debug!("🗑️ Eliminando contratista del índice: {id}");
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    /// Indexa a un nuevo usuario del sistema.
    /// Indexa a un nuevo usuario del sistema.
    pub async fn add_user(&self, user: &User) -> Result<(), SearchError> {
        debug!("➕ Indexando usuario: {}", user.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_user(&mut writer, &self.handles, user)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    /// Sincroniza los cambios de perfil de un usuario con el motor de búsqueda.
    /// Sincroniza los cambios de perfil de un usuario con el motor de búsqueda.
    pub async fn update_user(&self, user: &User) -> Result<(), SearchError> {
        debug!("✏️ Actualizando índice usuario: {}", user.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_user_in_index(&mut writer, &self.handles, user)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    /// Revoca la visibilidad de un usuario en las búsquedas globales.
    /// Revoca la visibilidad de un usuario en las búsquedas globales.
    pub async fn delete_user(&self, id: &str) -> Result<(), SearchError> {
        debug!("🗑️ Eliminando usuario del índice: {id}");
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    pub async fn add_user_fetched(&self, user: &UserFetched) -> Result<(), SearchError> {
        debug!("➕ Indexando usuario (fetched): {}", user.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_user_fetched(&mut writer, &self.handles, user)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    pub async fn update_user_fetched(&self, user: &UserFetched) -> Result<(), SearchError> {
        debug!("✏️ Actualizando índice usuario (fetched): {}", user.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_user_fetched_in_index(&mut writer, &self.handles, user)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    /// Registra un ingreso en la lista negra para bloquear el acceso visual mediante búsquedas.
    /// Registra un ingreso en la lista negra para bloquear el acceso visual mediante búsquedas.
    pub async fn add_lista_negra(&self, lista_negra: &ListaNegra) -> Result<(), SearchError> {
        debug!("➕ Indexando lista negra: {}", lista_negra.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_lista_negra(&mut writer, &self.handles, lista_negra)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    pub async fn update_lista_negra(&self, lista_negra: &ListaNegra) -> Result<(), SearchError> {
        debug!("✏️ Actualizando índice lista negra: {}", lista_negra.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_lista_negra_in_index(&mut writer, &self.handles, lista_negra)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    pub async fn delete_lista_negra(&self, id: &str) -> Result<(), SearchError> {
        debug!("🗑️ Eliminando lista negra del índice: {id}");
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    /// Realiza una búsqueda multitabla.
    ///
    /// Se optimiza pasando los campos pre-calculados (Cache), permitiendo devolver
    /// resultados relevantes en milisegundos incluso con miles de registros.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResultDto>, SearchError> {
        search_index(&self.index, &self.reader, &self.fields, query, limit)
    }

    pub fn is_empty(&self) -> bool {
        let searcher = self.reader.searcher();
        searcher.num_docs() == 0
    }

    pub fn doc_count(&self) -> u64 {
        let searcher = self.reader.searcher();
        searcher.num_docs()
    }

    pub async fn delete_proveedor(&self, id: &str) -> Result<(), SearchError> {
        debug!("🗑️ Eliminando proveedor del índice: {id}");
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            delete_from_index(&mut writer, &self.handles, id)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    pub async fn add_proveedor_fetched(
        &self,
        proveedor: &ProveedorFetched,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        debug!("➕ Indexando proveedor: {}", proveedor.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            index_proveedor_fetched(&mut writer, &self.handles, proveedor, empresa_nombre)?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }

    pub async fn update_proveedor_fetched(
        &self,
        proveedor: &ProveedorFetched,
        empresa_nombre: &str,
    ) -> Result<(), SearchError> {
        debug!("✏️ Actualizando índice proveedor: {}", proveedor.id);
        let _lock = self.writer_mutex.lock().await;

        {
            let mut writer = get_index_writer(&self.index)?;

            update_proveedor_fetched_in_index(
                &mut writer,
                &self.handles,
                proveedor,
                empresa_nombre,
            )?;
            commit_index(&mut writer)?;
        }

        self.reader.reload().map_err(|e| SearchError::TantivyError(e.to_string()))?;
        Ok(())
    }
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use surrealdb::RecordId;

    /// Verifica que se pueda crear una instancia en memoria sin errores.
    #[test]
    fn test_initialization() {
        let service = SearchService::test_instance();
        assert_eq!(service.doc_count(), 0);
        assert!(service.is_empty());
    }

    /// Verifica el flujo completo de indexación y búsqueda de un usuario.
    #[tokio::test]
    async fn test_index_and_search_user() {
        let service = SearchService::test_instance();

        let user = User {
            id: RecordId::from_table_key("user", "test-1"),
            cedula: "111222333".to_string(),
            nombre: "Test".to_string(),
            segundo_nombre: None,
            apellido: "User".to_string(),
            segundo_apellido: None,
            email: "test@example.com".to_string(),
            // Campos opcionales / defaults
            role: RecordId::from_table_key("role", "admin"),
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
            created_at: surrealdb::Datetime::from(Utc::now()),
            updated_at: surrealdb::Datetime::from(Utc::now()),
            avatar_path: None,
        };

        // 1. Indexar
        service.add_user(&user).await.expect("Fallo al indexar usuario");

        // 2. Verificar conteo
        assert_eq!(service.doc_count(), 1);

        // 3. Buscar
        let results = service.search("111222333", 10).expect("Fallo en búsqueda");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "test-1");
        assert_eq!(results[0].cedula.as_deref(), Some("111222333"));
    }
}
