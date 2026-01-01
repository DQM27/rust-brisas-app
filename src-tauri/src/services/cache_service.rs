/// Capa de Aceleración: Gestión de Cache In-Memory.
///
/// Este servicio reduce la latencia y la carga sobre SurrealDB al mantener en
/// memoria RAM los datos de acceso más frecuente (Contratistas y Proveedores).
///
/// Utiliza una política de expiración (TTL) para garantizar que la información
/// no sea obsoleta, liberando al disco de consultas redundantes durante los
/// periodos de alta demanda operativa.
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use crate::models::contratista::ContratistaFetched;
use crate::models::proveedor::ProveedorFetched;

// --------------------------------------------------------------------------
// ESTRUCTURAS DE ALMACENAMIENTO VOLÁTIL
// --------------------------------------------------------------------------

/// Entrada individual en el sistema de cache con marca de tiempo de expiración.
#[derive(Clone, Debug)]
pub struct CacheEntry<T: Clone> {
    /// Los datos reales almacenados.
    pub data: T,
    /// Timestamp (segundos desde UNIX_EPOCH) en el que esta entrada deja de ser válida.
    pub expires_at: u64,
}

impl<T: Clone> CacheEntry<T> {
    /// Crea una nueva entrada de cache calculando el tiempo de expiración.
    pub fn new(data: T, ttl_secs: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("El tiempo del sistema es anterior a UNIX_EPOCH")
            .as_secs();
        Self { data, expires_at: now + ttl_secs }
    }

    /// Determina si el dato ha excedido su tiempo de vida configurado.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("El tiempo del sistema es anterior a UNIX_EPOCH")
            .as_secs();
        now >= self.expires_at
    }
}

// --------------------------------------------------------------------------
// CACHES ESTÁTICOS (Singletons Protegidos)
// --------------------------------------------------------------------------

/// Tiempo de vida por defecto para las entradas de cache (5 minutos).
pub const CACHE_TTL: u64 = 300;

/// Cache de alta velocidad para Contratistas.
pub static CONTRATISTA_CACHE: Lazy<Arc<RwLock<HashMap<String, CacheEntry<ContratistaFetched>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

/// Cache de alta velocidad para Proveedores.
pub static PROVEEDOR_CACHE: Lazy<Arc<RwLock<HashMap<String, CacheEntry<ProveedorFetched>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

// --------------------------------------------------------------------------
// OPERACIONES GENÉRICAS DE CACHE
// --------------------------------------------------------------------------

/// Recupera un valor del cache si existe y es válido.
///
/// # Argumentos
/// * `cache` - El mapa de cache protegido por RwLock.
/// * `key` - La clave única del registro.
///
/// # Retorno
/// Retorna `Some(T)` si el dato está en cache y no ha expirado, o `None` en caso contrario.
pub async fn get_cached<T: Clone>(
    cache: &RwLock<HashMap<String, CacheEntry<T>>>,
    key: &str,
) -> Option<T> {
    let guard = cache.read().await;
    if let Some(entry) = guard.get(key) {
        if !entry.is_expired() {
            return Some(entry.data.clone());
        }
    }
    None
}

/// Inserta o actualiza un valor en el cache con un TTL específico.
///
/// # Argumentos
/// * `cache` - El mapa de cache protegido por RwLock.
/// * `key` - Clave única para el registro.
/// * `data` - Los datos a almacenar.
/// * `ttl_secs` - Tiempo de vida en segundos.
///
/// # Retorno
/// No retorna valor (operación en memoria).
pub async fn set_cached<T: Clone>(
    cache: &RwLock<HashMap<String, CacheEntry<T>>>,
    key: String,
    data: T,
    ttl_secs: u64,
) {
    let mut guard = cache.write().await;
    guard.insert(key, CacheEntry::new(data, ttl_secs));
}

/// Elimina una entrada específica del cache.
///
/// Se debe invocar tras cualquier operación de escritura (Update/Delete) en la DB.
///
/// # Argumentos
/// * `cache` - El mapa de cache.
/// * `key` - La clave a invalidar.
///
/// # Retorno
/// No retorna valor.
pub async fn invalidate_cached<T: Clone>(
    cache: &RwLock<HashMap<String, CacheEntry<T>>>,
    key: &str,
) {
    let mut guard = cache.write().await;
    guard.remove(key);
}

/// Limpia todas las entradas de un cache específico.
///
/// # Argumentos
/// * `cache` - El mapa de cache a vaciar.
///
/// # Retorno
/// No retorna valor.
pub async fn clear_cache<T: Clone>(cache: &RwLock<HashMap<String, CacheEntry<T>>>) {
    let mut guard = cache.write().await;
    guard.clear();
}

// --------------------------------------------------------------------------
// AYUDANTES ESPECÍFICOS PARA CONTRATISTAS
// --------------------------------------------------------------------------

/// Intenta obtener un contratista del cache.
pub async fn get_cached_contratista(id: &str) -> Option<ContratistaFetched> {
    get_cached(&*CONTRATISTA_CACHE, id).await
}

/// Almacena un contratista en el cache con el TTL estándar.
pub async fn cache_contratista(id: String, contratista: ContratistaFetched) {
    set_cached(&*CONTRATISTA_CACHE, id, contratista, CACHE_TTL).await;
}

/// Invalida la entrada de un contratista en el cache.
pub async fn invalidate_contratista(id: &str) {
    invalidate_cached(&*CONTRATISTA_CACHE, id).await;
}

// --------------------------------------------------------------------------
// AYUDANTES ESPECÍFICOS PARA PROVEEDORES
// --------------------------------------------------------------------------

/// Intenta obtener un proveedor del cache.
pub async fn get_cached_proveedor(id: &str) -> Option<ProveedorFetched> {
    get_cached(&*PROVEEDOR_CACHE, id).await
}

/// Almacena un proveedor en el cache con el TTL estándar.
pub async fn cache_proveedor(id: String, proveedor: ProveedorFetched) {
    set_cached(&*PROVEEDOR_CACHE, id, proveedor, CACHE_TTL).await;
}

/// Invalida la entrada de un proveedor en el cache.
pub async fn invalidate_proveedor(id: &str) {
    invalidate_cached(&*PROVEEDOR_CACHE, id).await;
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_entry_expiration_logic() {
        let entry = CacheEntry::new("test".to_string(), 0);
        // Con TTL 0, debería considerarse expirado inmediatamente.
        assert!(entry.is_expired());
    }

    #[tokio::test]
    async fn test_cache_entry_persistence() {
        let entry = CacheEntry::new("test".to_string(), 3600);
        assert!(!entry.is_expired());
        assert_eq!(entry.data, "test");
    }

    #[tokio::test]
    async fn test_generic_cache_operations() {
        let cache = RwLock::new(HashMap::new());
        let key = "key1".to_string();
        let value = "value1".to_string();

        set_cached(&cache, key.clone(), value.clone(), 300).await;

        let retrieved = get_cached(&cache, &key).await;
        assert_eq!(retrieved, Some(value));

        invalidate_cached(&cache, &key).await;
        let expired = get_cached(&cache, &key).await;
        assert_eq!(expired, None);
    }

    #[tokio::test]
    async fn test_clear_all_cache() {
        let cache = RwLock::new(HashMap::new());
        set_cached(&cache, "a".to_string(), 1, 300).await;
        set_cached(&cache, "b".to_string(), 2, 300).await;

        clear_cache(&cache).await;

        let guard = cache.read().await;
        assert!(guard.is_empty());
    }
}
