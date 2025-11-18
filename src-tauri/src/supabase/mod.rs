// src-tauri/src/supabase/mod.rs

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::config::AppConfig;

pub struct SupabaseClient {
    pool: PgPool,
}

impl SupabaseClient {
    pub async fn new(config: &AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        if config.supabase.url.is_empty() || config.supabase.anon_key.is_empty() || config.supabase.db_password.is_empty() {
            return Err("Configuración de Supabase incompleta".into());
        }

        let project_ref = extract_project_ref(&config.supabase.url)?;

        let database_url = format!(
            "postgresql://postgres:{}@db.{}.supabase.co:5432/postgres",
            config.supabase.db_password,
            project_ref
        );

        println!("🔌 Conectando a Supabase...");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        println!("✅ Conectado a Supabase");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

fn extract_project_ref(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ref_str = url
        .trim_start_matches("https://")
        .trim_end_matches(".supabase.co")
        .to_string();
    
    if ref_str.is_empty() {
        return Err("URL de Supabase inválida".into());
    }
    
    Ok(ref_str)
}