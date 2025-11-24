// src-tauri/src/lib.rs

pub mod models;
pub mod services;
pub mod commands;
pub mod db;
pub mod domain;
pub mod config; 
pub mod supabase;
pub mod keyring_manager;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SupabaseState {
    pub client: Option<supabase::SupabaseClient>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(not(mobile))]
    {
        #[tokio::main]
        async fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
            dotenvy::dotenv().ok();
            
            let app_config = config::load_config()?;
            println!("🏢 Terminal: {} (ID: {})", app_config.terminal.nombre, app_config.terminal.id);
            
            let pool = db::init_db(&app_config).await?;
            
            let supabase_client = match keyring_manager::load_credentials() {
                Ok(creds) => {
                    println!("🔐 Credenciales encontradas en keyring");
                    match supabase::SupabaseClient::new(&creds).await {
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

            let supabase_state = Arc::new(RwLock::new(SupabaseState {
                client: supabase_client,
            }));
            
            // Envolver el pool en Arc para que coincida con los comandos
            
            
            tauri::Builder::default()
                .manage(pool)
                .manage(app_config)
                .manage(supabase_state)
                .plugin(tauri_plugin_dialog::init())
                .plugin(tauri_plugin_opener::init())
                .invoke_handler(tauri::generate_handler![
                    // Comandos de usuario
                    commands::user_commands::create_user,
                    commands::user_commands::get_user_by_id,  
                    commands::user_commands::get_all_users,
                    commands::user_commands::update_user,
                    commands::user_commands::delete_user,
                    commands::user_commands::login,
                    // Comandos de empresa
                    commands::empresa_commands::create_empresa,
                    commands::empresa_commands::get_empresa_by_id,
                    commands::empresa_commands::get_all_empresas,
                    commands::empresa_commands::get_empresas_activas,
                    commands::empresa_commands::update_empresa,
                    commands::empresa_commands::delete_empresa,
                    // Comandos de contratista
                    commands::contratista_commands::create_contratista,
                    commands::contratista_commands::get_contratista_by_id,
                    commands::contratista_commands::get_contratista_by_cedula,
                    commands::contratista_commands::get_all_contratistas,
                    commands::contratista_commands::get_contratistas_activos,
                    commands::contratista_commands::update_contratista,
                    commands::contratista_commands::cambiar_estado_contratista,
                    commands::contratista_commands::delete_contratista,
                    // Comandos de lista negra
                    commands::lista_negra_commands::add_to_lista_negra,
                    commands::lista_negra_commands::get_lista_negra_by_id,
                    commands::lista_negra_commands::get_all_lista_negra,
                    commands::lista_negra_commands::get_lista_negra_activos,
                    commands::lista_negra_commands::check_is_blocked,
                    commands::lista_negra_commands::get_blocked_by_cedula,
                    commands::lista_negra_commands::remove_from_lista_negra,
                    commands::lista_negra_commands::update_lista_negra,
                    commands::lista_negra_commands::delete_lista_negra,
                    // Comandos de vehículo
                    commands::vehiculo_commands::create_vehiculo,
                    commands::vehiculo_commands::get_vehiculo_by_id,
                    commands::vehiculo_commands::get_vehiculo_by_placa,
                    commands::vehiculo_commands::get_all_vehiculos,
                    commands::vehiculo_commands::get_vehiculos_activos,
                    commands::vehiculo_commands::get_vehiculos_by_contratista,
                    commands::vehiculo_commands::update_vehiculo,
                    commands::vehiculo_commands::delete_vehiculo,
                    // Gafetes
                    commands::gafete_commands::create_gafete,
                    commands::gafete_commands::get_gafete_by_id,
                    commands::gafete_commands::get_gafete_by_numero,
                    commands::gafete_commands::get_all_gafetes,
                    commands::gafete_commands::get_gafetes_disponibles,
                    commands::gafete_commands::get_stock_gafetes,
                    commands::gafete_commands::asignar_gafete,
                    commands::gafete_commands::liberar_gafete,
                    commands::gafete_commands::update_gafete,
                    commands::gafete_commands::delete_gafete,
                    // Gafetes Perdidos
                    commands::gafete_perdido_commands::reportar_gafete_perdido,
                    commands::gafete_perdido_commands::get_gafete_perdido_by_id,
                    commands::gafete_perdido_commands::get_all_gafetes_perdidos,
                    commands::gafete_perdido_commands::get_gafetes_perdidos_pendientes,
                    commands::gafete_perdido_commands::get_deudas_by_contratista,
                    commands::gafete_perdido_commands::registrar_pago_gafete,
                    commands::gafete_perdido_commands::condonar_deuda_gafete,
                    commands::gafete_perdido_commands::delete_gafete_perdido,
                    // Ingresos
                    commands::ingreso_commands::validar_ingreso_contratista,
                    commands::ingreso_commands::create_ingreso_contratista,
                    commands::ingreso_commands::create_ingreso_temporal,
                    commands::ingreso_commands::get_ingreso_by_id,
                    commands::ingreso_commands::get_all_ingresos,
                    commands::ingreso_commands::get_ingresos_abiertos,
                    commands::ingreso_commands::get_ingreso_by_gafete,
                    commands::ingreso_commands::registrar_salida,
                    commands::ingreso_commands::registrar_salida_con_gafete_perdido,
                    commands::ingreso_commands::cerrar_ingreso_anterior,
                    // Comandos de Supabase
                    commands::supabase_commands::test_supabase_connection,
                    commands::supabase_commands::get_supabase_config,
                    commands::supabase_commands::reinitialize_supabase,
                    // Comandos de Keyring
                    commands::keyring_commands::keyring_save,
                    commands::keyring_commands::keyring_load,
                    commands::keyring_commands::keyring_delete,
                    commands::keyring_commands::keyring_check,
                    commands::keyring_commands::keyring_info,
                    
                    // Comandos CRUD de blacklist_import
                    commands::blacklist_import_commands::create_blacklist_import_entry,
                    commands::blacklist_import_commands::get_blacklist_import_by_id,
                    commands::blacklist_import_commands::get_blacklist_import_by_cedula,
                    commands::blacklist_import_commands::get_all_blacklist_imports,
                    commands::blacklist_import_commands::get_blacklist_imports_by_empresa,
                    commands::blacklist_import_commands::update_blacklist_import_entry,
                    commands::blacklist_import_commands::delete_blacklist_import_entry,
                    commands::blacklist_import_commands::delete_all_blacklist_imports,
                    
                    // Comandos de estadísticas
                    commands::blacklist_import_commands::get_blacklist_import_stats,
                    commands::blacklist_import_commands::check_duplicate_cedula,
                    
                    // Comandos de importación Excel
                    commands::blacklist_import_commands::preview_excel_import,
                    commands::blacklist_import_commands::parse_excel_file,
                    commands::blacklist_import_commands::import_excel_to_database,
                    commands::blacklist_import_commands::import_reviewed_entries,
                    
                    // Comandos de utilidad
                    commands::blacklist_import_commands::validate_and_split_name,
                    commands::blacklist_import_commands::check_name_requires_validation,
                    commands::blacklist_import_commands::normalize_cedula,
                    commands::blacklist_import_commands::capitalize_name,
                ])
                .run(tauri::generate_context!())?;
            Ok(())
        }
        main_inner().expect("error");
    }
}