// ==========================================
// src/commands/handlers.rs
// ==========================================
// Macro centralizada para registrar todos los comandos

#[macro_export]
macro_rules! register_handlers {
    () => {
        tauri::generate_handler![
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
            commands::lista_negra_commands::reactivate_lista_negra,
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
            
            // Comandos de gafetes
            commands::gafete_commands::create_gafete,
            commands::gafete_commands::get_gafete,
            commands::gafete_commands::get_all_gafetes,
            commands::gafete_commands::get_gafetes_disponibles,
            commands::gafete_commands::is_gafete_disponible,
            commands::gafete_commands::update_gafete,
            commands::gafete_commands::delete_gafete,
            
            // ==========================================
            // COMANDOS DE ENTRADA (Fase 1)
            // ==========================================
            commands::entrada_commands::validar_ingreso_contratista,
            commands::entrada_commands::crear_ingreso_contratista,
            
            // ==========================================
            // COMANDOS DE PERMANENCIA (Fase 2)
            // ==========================================
            commands::permanencia_commands::get_ingreso_con_estado,
            commands::permanencia_commands::get_ingresos_abiertos_con_alertas,
            commands::permanencia_commands::verificar_tiempos_excedidos,
            commands::permanencia_commands::verificar_alertas_tempranas,
            commands::permanencia_commands::verificar_cambio_lista_negra,
            commands::permanencia_commands::verificar_cambios_lista_negra_masivo,
            commands::permanencia_commands::get_resumen_permanencias,
            
            // ==========================================
            // COMANDOS DE SALIDA (Fase 3)
            // ==========================================
            commands::salida_commands::validar_puede_salir,
            commands::salida_commands::registrar_salida,
            commands::salida_commands::registrar_salida_con_verificacion_gafete,
            commands::salida_commands::get_salidas_del_dia,
            commands::salida_commands::get_estadisticas_salidas,
            
            // ==========================================
            // COMANDOS GENERALES DE CONSULTA DE INGRESOS
            // ==========================================
            commands::ingreso_commands::get_ingreso_by_id,
            commands::ingreso_commands::get_all_ingresos,
            commands::ingreso_commands::get_ingresos_abiertos,
            commands::ingreso_commands::get_ingreso_by_gafete,
            
            // ==========================================
            // COMANDOS DE ALERTAS DE GAFETES
            // ==========================================
            commands::ingreso_commands::get_alertas_pendientes_by_cedula,
            commands::ingreso_commands::get_all_alertas_gafetes,
            commands::ingreso_commands::resolver_alerta_gafete,
            
            // Comandos de blacklist import
            commands::blacklist_import_commands::create_blacklist_import_entry,
            commands::blacklist_import_commands::get_blacklist_import_by_id,
            commands::blacklist_import_commands::get_blacklist_import_by_cedula,
            commands::blacklist_import_commands::get_all_blacklist_imports,
            commands::blacklist_import_commands::get_blacklist_imports_by_empresa,
            commands::blacklist_import_commands::update_blacklist_import_entry,
            commands::blacklist_import_commands::delete_blacklist_import_entry,
            commands::blacklist_import_commands::delete_all_blacklist_imports,
            commands::blacklist_import_commands::get_blacklist_import_stats,
            commands::blacklist_import_commands::check_duplicate_cedula,
            commands::blacklist_import_commands::preview_excel_import,
            commands::blacklist_import_commands::parse_excel_file,
            commands::blacklist_import_commands::import_excel_to_database,
            commands::blacklist_import_commands::import_reviewed_entries,
            commands::blacklist_import_commands::validate_and_split_name,
            commands::blacklist_import_commands::check_name_requires_validation,
            commands::blacklist_import_commands::normalize_cedula,
            commands::blacklist_import_commands::capitalize_name,
            
            // Comandos de búsqueda
            commands::search_commands::search_contratistas,
            commands::search_commands::reindex_all_contratistas,
        ]
    };
}