// ==========================================
// src/supabase/connection.rs
// ==========================================

use crate::supabase::SupabaseClient;
use crate::keyring_manager;
use crate::SupabaseState;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Inicializa el cliente de Supabase si hay credenciales
pub async fn init_supabase() -> Arc<RwLock<SupabaseState>> {
    let supabase_client = match keyring_manager::load_credentials() {
        Ok(creds) => {
            println!("🔐 Credenciales encontradas en keyring");
            match SupabaseClient::new(&creds).await {
                Ok(client) => {
                    println!("✅ Cliente de Supabase inicializado");
                    Some(client)
                }
                Err(e) => {
                    println!("⚠️ No se pudo conectar a Supabase: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("⚠️ No hay credenciales de Supabase guardadas: {}", e);
            println!("💡 Configura las credenciales desde la UI");
            None
        }
    };

    Arc::new(RwLock::new(SupabaseState {
        client: supabase_client,
    }))
}