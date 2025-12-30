// ==========================================
// src-tauri/src/services/cache_service.rs
// ==========================================
// Memory cache layer for frequently accessed data
// TTL: 5 minutes (300 seconds) for contratistas

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use crate::models::contratista::ContratistaFetched;
use crate::models::proveedor::ProveedorFetched;

// ==========================================
// CACHE ENTRY TYPE
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

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        now >= self.expires_at
    }
}

// ==========================================
// CACHE INSTANCES (Singletons)
// ==========================================

/// TTL in seconds (5 minutes)
pub const CACHE_TTL: u64 = 300;

/// Cache for contratistas by ID
pub static CONTRATISTA_CACHE: Lazy<Arc<RwLock<HashMap<String, CacheEntry<ContratistaFetched>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

/// Cache for proveedores by ID
pub static PROVEEDOR_CACHE: Lazy<Arc<RwLock<HashMap<String, CacheEntry<ProveedorFetched>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

// ==========================================
// GENERIC CACHE OPERATIONS
// ==========================================

/// Gets a value from cache if it exists and is not expired
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

/// Sets a value in cache with TTL
pub async fn set_cached<T: Clone>(
    cache: &RwLock<HashMap<String, CacheEntry<T>>>,
    key: String,
    data: T,
    ttl_secs: u64,
) {
    let mut guard = cache.write().await;
    guard.insert(key, CacheEntry::new(data, ttl_secs));
}

/// Removes a value from cache
pub async fn invalidate_cached<T: Clone>(
    cache: &RwLock<HashMap<String, CacheEntry<T>>>,
    key: &str,
) {
    let mut guard = cache.write().await;
    guard.remove(key);
}

/// Clears all entries from a cache
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
