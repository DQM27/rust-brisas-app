// src-tauri/src/supabase/mod.rs

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::keyring_manager::SupabaseCredentials;

pub struct SupabaseClient {
    pool: PgPool,
}

impl SupabaseClient {
    /// Crea cliente desde credenciales del keyring
    pub async fn new(creds: &SupabaseCredentials) -> Result<Self, String> {
        if creds.url.is_empty() || creds.db_password.is_empty() {
            return Err("Credenciales de Supabase incompletas".to_string());
        }

        let project_ref = extract_project_ref(&creds.url)?;

        // ✅ FORMATO CORRECTO: Direct Connection
        let database_url = format!(
            "postgresql://postgres:{}@db.{}.supabase.co:5432/postgres",
            creds.db_password,
            project_ref
        );

        println!("🔌 Conectando a Supabase...");
        println!("📍 Project: {}", project_ref);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|e| format!("Error conectando a Supabase: {}", e))?;

        println!("✅ Conectado a Supabase");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

fn extract_project_ref(url: &str) -> Result<String, String> {
    let ref_str = url
        .trim_start_matches("https://")
        .trim_end_matches(".supabase.co")
        .to_string();
    
    if ref_str.is_empty() {
        return Err("URL de Supabase inválida".to_string());
    }
    
    Ok(ref_str)
}
