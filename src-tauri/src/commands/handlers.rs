// src/commands/handlers.rs

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
            commands::user_commands::change_password,
            commands::user_commands::upload_user_avatar,
            commands::user_commands::get_user_avatar,
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
            commands::contratista_commands::restore_contratista,
            commands::contratista_commands::get_archived_contratistas,
            commands::contratista_commands::delete_contratista,
            // Auditoría de contratista
            commands::contratista_commands::actualizar_praind_con_historial,
            commands::contratista_commands::cambiar_estado_con_historial,
            // Comandos de lista negra
            commands::lista_negra_commands::add_to_lista_negra,
            commands::lista_negra_commands::get_lista_negra_by_id,
            commands::lista_negra_commands::get_all_lista_negra,
            commands::lista_negra_commands::check_is_blocked,
            commands::lista_negra_commands::search_personas_for_block,
            commands::lista_negra_commands::update_lista_negra,
            commands::lista_negra_commands::delete_from_lista_negra,
            commands::lista_negra_commands::restore_lista_negra,
            // Comandos de vehículo
            commands::vehiculo_commands::create_vehiculo,
            commands::vehiculo_commands::get_vehiculo_by_id,
            commands::vehiculo_commands::get_vehiculo_by_placa,
            commands::vehiculo_commands::get_all_vehiculos,
            commands::vehiculo_commands::get_vehiculos_activos,
            commands::vehiculo_commands::get_vehiculos_by_propietario,
            commands::vehiculo_commands::update_vehiculo,
            commands::vehiculo_commands::delete_vehiculo,
            // Comandos de gafetes
            commands::gafete_commands::create_gafete,
            commands::gafete_commands::create_gafete_range,
            commands::gafete_commands::get_gafete,
            commands::gafete_commands::get_all_gafetes,
            commands::gafete_commands::get_gafetes_disponibles,
            commands::gafete_commands::is_gafete_disponible,
            commands::gafete_commands::update_gafete,
            commands::gafete_commands::update_gafete_status,
            commands::gafete_commands::delete_gafete,
            // ==========================================
            // COMANDOS DE VISITANTE
            // ==========================================
            commands::visitante_commands::create_visitante,
            commands::visitante_commands::search_visitantes_catalog,
            commands::visitante_commands::get_visitante_by_cedula,
            commands::visitante_commands::get_visitante_by_id,
            commands::visitante_commands::update_visitante,
            commands::visitante_commands::delete_visitante,
            commands::visitante_commands::restore_visitante,
            commands::visitante_commands::get_archived_visitantes,
            commands::visitante_commands::list_visitantes,
            // ==========================================
            // COMANDOS DE INGRESO VISITAS
            // ==========================================
            commands::ingreso_visita_commands::get_ingresos_visita_activos,
            commands::ingreso_visita_commands::get_ingresos_visita_historial,
            commands::ingreso_visita_commands::crear_ingreso_visita,
            commands::ingreso_visita_commands::validar_ingreso_visita,
            commands::ingreso_visita_commands::registrar_salida_visita,
            // ==========================================
            // COMANDOS DE INGRESO PROVEEDORES
            // ==========================================
            commands::ingreso_proveedor_commands::crear_ingreso_proveedor_v2,
            commands::ingreso_proveedor_commands::get_ingresos_proveedores_activos,
            commands::ingreso_proveedor_commands::registrar_salida_proveedor,
            commands::ingreso_proveedor_commands::validar_ingreso_proveedor,
            commands::ingreso_proveedor_commands::search_proveedores_by_cedula,
            // ==========================================
            // COMANDOS DE PROVEEDORES (CATALOGO)
            // ==========================================
            commands::proveedor_commands::create_proveedor,
            commands::proveedor_commands::search_proveedores_catalog,
            commands::proveedor_commands::get_proveedor_by_cedula,
            commands::proveedor_commands::change_proveedor_status,
            commands::proveedor_commands::update_proveedor,
            commands::proveedor_commands::get_proveedor_by_id,
            commands::proveedor_commands::delete_proveedor,
            commands::proveedor_commands::restore_proveedor,
            commands::proveedor_commands::get_archived_proveedores,
            // ==========================================
            // COMANDOS DE INGRESO CONTRATISTA (UNIFICADO)
            // ==========================================
            commands::ingreso_contratista_commands::validate_ingreso_contratista,
            commands::ingreso_contratista_commands::create_ingreso_contratista,
            commands::ingreso_contratista_commands::validate_exit_contratista,
            commands::ingreso_contratista_commands::register_exit_contratista,
            // Monitoreo
            commands::ingreso_contratista_commands::get_ingresos_contratistas_activos,
            commands::ingreso_contratista_commands::get_ingresos_contratistas_historial,
            commands::ingreso_contratista_commands::check_time_alerts,
            // ==========================================
            // COMANDOS GENERALES DE CONSULTA DE INGRESOS
            // ==========================================
            // "Legacy" queries genéricas
            commands::ingreso_commands::get_ingreso_by_id,
            commands::ingreso_commands::get_all_ingresos,
            // commands::ingreso_commands::get_ingresos_abiertos, // Eliminado por modularización
            commands::ingreso_commands::get_ingreso_by_gafete,
            commands::ingreso_commands::get_salidas_en_rango,
            commands::ingreso_commands::get_salidas_del_dia,
            // ==========================================
            // COMANDOS DE ALERTAS DE GAFETES
            // ==========================================
            commands::ingreso_commands::get_alertas_pendientes_by_cedula,
            commands::ingreso_commands::get_all_alertas_gafetes,
            commands::ingreso_commands::resolver_alerta_gafete,
            // Comandos de búsqueda
            commands::search_commands::search_omnibox,
            commands::search_commands::search_global,
            commands::search_commands::reindex_global_search,
            // ==========================================
            // COMANDOS DE EXPORTACIÓN
            // ==========================================
            commands::export_commands::export_data,
            commands::export_commands::check_export_available,
            commands::export_commands::get_available_export_formats,
            commands::export_commands::is_export_format_available,
            commands::export_commands::export_to_pdf,
            commands::export_commands::export_to_excel,
            commands::export_commands::export_to_csv,
            commands::export_commands::export_preview,
            // ==========================================
            // COMANDOS DE PERFILES DE EXPORTACIÓN
            // ==========================================
            commands::export_profiles::get_export_profiles,
            commands::export_profiles::save_export_profile,
            commands::export_profiles::delete_export_profile,
            commands::export_profiles::set_default_export_profile,
            commands::export_profiles::get_default_export_profile,
            // ==========================================
            // COMANDOS DE BACKUP/RESTORE
            // ==========================================
            commands::backup::backup_database,
            commands::backup::backup_database_auto,
            commands::backup::backup_database_portable,
            commands::backup::list_backups,
            commands::backup::delete_backup,
            commands::backup::restore_database,
            commands::backup::restore_from_auto_backup,
            commands::backup::restore_portable_backup,
            commands::backup::cleanup_old_backups,
            // ==========================================
            // COMANDOS DE CONFIGURACIÓN
            // ==========================================
            commands::config_commands::get_app_config,
            commands::config_commands::update_terminal_config,
            commands::config_commands::update_audio_config,
            commands::config_commands::get_backup_config,
            commands::config_commands::update_backup_config,
            // ==========================================
            // COMANDOS DE VENTANA
            // ==========================================
            commands::window_commands::show_main_window,
            commands::app_commands::exit_app,
            commands::app_commands::set_window_decorations,
            commands::app_commands::set_window_size,
            // ==========================================
            // COMANDOS DE KEYRING/CREDENCIALES
            // ==========================================
            commands::keyring_commands::get_credential_status,
            commands::keyring_commands::is_app_configured,
            commands::keyring_commands::needs_setup,
            commands::keyring_commands::setup_credentials,
            commands::keyring_commands::get_argon2_config,
            commands::keyring_commands::update_argon2_params,
            commands::keyring_commands::generate_argon2_secret,
            commands::keyring_commands::generate_random_secret,
            commands::keyring_commands::export_master_key_cmd,
            commands::keyring_commands::import_master_key_cmd,
            commands::keyring_commands::test_keyring,
            commands::keyring_commands::reset_all_credentials,
            commands::keyring_commands::save_secret,
            commands::keyring_commands::get_secret,
            commands::keyring_commands::delete_secret,
            commands::keyring_commands::generate_recovery_fragments,
            commands::keyring_commands::recover_from_fragments,
            // ==========================================
            // COMANDOS DE SISTEMA
            // ==========================================
            commands::system_commands::get_system_idle_time,
            // ==========================================
            // COMANDOS DE GESTIÓN DE MÓDULOS
            // ==========================================
            commands::module_commands::get_modules_status,
            commands::module_commands::update_module_status,
            // ==========================================
            // COMANDOS DE ROLES
            // ==========================================
            commands::role_commands::get_all_roles,
            commands::role_commands::get_role_by_id,
            commands::role_commands::get_all_permissions,
            commands::role_commands::get_visible_modules,
            commands::role_commands::create_role,
            commands::role_commands::update_role,
            commands::role_commands::delete_role,
            // ==========================================
            // COMANDOS DE AUDIO
            // ==========================================
            commands::audio_commands::play_alert_sound,
            commands::audio_commands::upload_custom_sound,
            commands::audio_commands::set_use_custom_sound,
            // ==========================================
            // COMANDOS DE VALIDACIÓN
            // ==========================================
            commands::validation_commands::check_unique,
        ]
    };
}
