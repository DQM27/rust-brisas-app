/// Capa de Aceleración y Eficiencia Operativa (Cache In-Memory).
///
/// Este servicio reduce drásticamente la latencia y la carga sobre SurrealDB al
/// mantener en memoria RAM los datos de acceso más frecuente (Contratistas y Proveedores).
/// Utiliza una política de expiración (TTL) para garantizar que la información
/// no sea obsoleta, liberando al disco de consultas redundantes durante las "horas pico"
/// en garita.
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use crate::models::contratista::ContratistaFetched;
use crate::models::proveedor::ProveedorFetched;

// ==========================================
// ESTRUCTURAS DE ALMACENAMIENTO VOLÁTIL
// ==========================================

#[derive(Clone)]
pub struct CacheEntry<T: Clone> {
    pub data: T,
    pub expires_at: u64,
}

impl<T: Clone> CacheEntry<T> {
    pub fn new(data: T, ttl_secs: u64) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self { data, expires_at: now + ttl_secs }
    }

    /// Control de Obsolecencia: Determina si el dato debe ser refrescado desde DB.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        now >= self.expires_at
    }
}

// ==========================================
// INSTANCIAS DE CACHE (Singletons Dinámicos)
// ==========================================

/// Ventana de vigencia (5 minutos): Balance ideal entre rendimiento y frescura de datos.
pub const CACHE_TTL: u64 = 300;

/// Depósitos de alta velocidad protegidos por bloqueos de lectura/escritura (RwLock).
pub static CONTRATISTA_CACHE: Lazy<Arc<RwLock<HashMap<String, CacheEntry<ContratistaFetched>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

pub static PROVEEDOR_CACHE: Lazy<Arc<RwLock<HashMap<String, CacheEntry<ProveedorFetched>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

// ==========================================
// OPERACIONES GENÉRICAS
// ==========================================

/// Recuperación rápida: Retorna el dato si vive en RAM y no ha caducado.
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

/// Inserción en Cache.
pub async fn set_cached<T: Clone>(
    cache: &RwLock<HashMap<String, CacheEntry<T>>>,
    key: String,
    data: T,
    ttl_secs: u64,
) {
    let mut guard = cache.write().await;
    guard.insert(key, CacheEntry::new(data, ttl_secs));
}

/// Invalida manualmente una entrada (Ej: cuando se actualiza un registro en DB).
pub async fn invalidate_cached<T: Clone>(
    cache: &RwLock<HashMap<String, CacheEntry<T>>>,
    key: &str,
) {
    let mut guard = cache.write().await;
    guard.remove(key);
}

pub async fn clear_cache<T: Clone>(cache: &RwLock<HashMap<String, CacheEntry<T>>>) {
    let mut guard = cache.write().await;
    guard.clear();
}

// ==========================================
// CONTRATISTA-SPECIFIC HELPERS
// ==========================================

/// Gets a contratista from cache
pub async fn get_cached_contratista(id: &str) -> Option<ContratistaFetched> {
    get_cached(&*CONTRATISTA_CACHE, id).await
}

/// Caches a contratista
pub async fn cache_contratista(id: String, contratista: ContratistaFetched) {
    set_cached(&*CONTRATISTA_CACHE, id, contratista, CACHE_TTL).await;
}

/// Invalidates a contratista from cache
pub async fn invalidate_contratista(id: &str) {
    invalidate_cached(&*CONTRATISTA_CACHE, id).await;
}

// ==========================================
// PROVEEDOR-SPECIFIC HELPERS
// ==========================================

/// Gets a proveedor from cache
pub async fn get_cached_proveedor(id: &str) -> Option<ProveedorFetched> {
    get_cached(&*PROVEEDOR_CACHE, id).await
}

/// Caches a proveedor
pub async fn cache_proveedor(id: String, proveedor: ProveedorFetched) {
    set_cached(&*PROVEEDOR_CACHE, id, proveedor, CACHE_TTL).await;
}

/// Invalidates a proveedor from cache
pub async fn invalidate_proveedor(id: &str) {
    invalidate_cached(&*PROVEEDOR_CACHE, id).await;
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test".to_string(), 0);
        // With 0 TTL, should be expired immediately
        assert!(entry.is_expired());
    }

    #[tokio::test]
    async fn test_cache_entry_not_expired() {
        let entry = CacheEntry::new("test".to_string(), 300);
        assert!(!entry.is_expired());
    }
}
