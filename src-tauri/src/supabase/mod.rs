// src-tauri/src/supabase/mod.rs

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::config::AppConfig;

/// Cliente de Supabase
pub struct SupabaseClient {
    pool: PgPool,
}

impl SupabaseClient {
    /// Crea una nueva conexión a Supabase
    pub async fn new(config: &AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        if config.supabase.url.is_empty() || config.supabase.anon_key.is_empty() {
            return Err("Configuración de Supabase incompleta".into());
        }

        // Construir connection string de PostgreSQL
        let database_url = format!(
            "postgresql://postgres.{}:{}@aws-0-us-east-1.pooler.supabase.com:6543/postgres",
            extract_project_ref(&config.supabase.url)?,
            config.supabase.anon_key
        );

        println!("🔌 Conectando a Supabase...");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        println!("✅ Conectado a Supabase");

        Ok(Self { pool })
    }

    /// Obtiene referencia al pool de conexiones
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

/// Extrae el ref del proyecto de la URL de Supabase
fn extract_project_ref(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // URL ejemplo: https://myygjkofrdxhubjoppnz.supabase.co
    let ref_str = url
        .trim_start_matches("https://")
        .trim_end_matches(".supabase.co")
        .to_string();
    
    if ref_str.is_empty() {
        return Err("URL de Supabase inválida".into());
    }
    
    Ok(ref_str)
}