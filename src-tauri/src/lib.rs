// ==========================================
// src/lib.rs
// ==========================================
pub mod models;
pub mod services;
pub mod commands;
pub mod db;
pub mod config;  // ← NUEVO MÓDULO

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(not(mobile))]
    {
        #[tokio::main]
        async fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
            dotenvy::dotenv().ok();
            
            // ✅ Cargar configuración
            let app_config = config::load_config()?;
            println!("🏢 Terminal: {} (ID: {})", app_config.terminal.nombre, app_config.terminal.id);
            
            // ✅ Inicializar DB con la configuración
            let pool = db::init_db(&app_config).await?;
            
            tauri::Builder::default()
                .manage(pool)
                .manage(app_config)  // ← Compartir config con comandos
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
                ])
                .run(tauri::generate_context!())?;
            Ok(())
        }
        main_inner().expect("error");
    }
}