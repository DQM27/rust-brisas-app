// src-tauri/src/supabase/mod.rs

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::keyring_manager::SupabaseCredentials;

pub struct SupabaseClient {
    pool: PgPool,
}

impl SupabaseClient {
    /// Crea cliente desde credenciales del keyring
    pub async fn new(creds: &SupabaseCredentials) -> Result<Self, String> {  // ✅ String en lugar de Box<dyn Error>
        if creds.url.is_empty() || creds.db_password.is_empty() {
            return Err("Credenciales de Supabase incompletas".to_string());
        }

        let project_ref = extract_project_ref(&creds.url)?;

        let database_url = format!(
            "postgresql://postgres.{}:{}@aws-0-us-east-1.pooler.supabase.com:6543/postgres",
            project_ref,
            creds.db_password
        );

        println!("🔌 Conectando a Supabase...");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|e| format!("Error conectando a Supabase: {}", e))?;  // ✅ Convertir a String

        println!("✅ Conectado a Supabase");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

fn extract_project_ref(url: &str) -> Result<String, String> {  // ✅ String en lugar de Box<dyn Error>
    let ref_str = url
        .trim_start_matches("https://")
        .trim_end_matches(".supabase.co")
        .to_string();
    
    if ref_str.is_empty() {
        return Err("URL de Supabase inválida".to_string());
    }
    
    Ok(ref_str)
}