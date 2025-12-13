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
            commands::user_commands::change_password,
            // Comandos de empresa
            commands::empresa_commands::create_empresa,
            commands::empresa_commands::get_empresa_by_id,
            commands::empresa_commands::get_all_empresas,
            commands::empresa_commands::get_empresas_activas,
            commands::empresa_commands::update_empresa,
            commands::empresa_commands::delete_empresa,
            // Comandos de contratista
            commands::contratista_commands::create_contratista,
            commands::contratista_commands::get_contratista,
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
            // COMANDOS DE CITAS (Pre-registro)
            // ==========================================
            commands::cita_commands::create_cita,
            commands::cita_commands::get_citas_hoy,
            commands::cita_commands::procesar_ingreso_cita,
            // ==========================================
            // COMANDOS DE ENTRADA (Fase 1)
            // ==========================================
            commands::entrada_commands::validar_ingreso_contratista,
            commands::entrada_commands::crear_ingreso_contratista,
            commands::entrada_commands::crear_ingreso_visita,
            commands::entrada_commands::crear_ingreso_proveedor,
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
            commands::salida_commands::get_salidas_en_rango,
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
            // Comandos de búsqueda
            commands::search_commands::search_contratistas,
            commands::search_commands::reindex_all_contratistas,
            // ==========================================
            // COMANDOS DE EXPORTACIÓN (NUEVO)
            // ==========================================

            // Comando principal unificado
            commands::export_commands::export_data,
            // Comandos de verificación de disponibilidad
            commands::export_commands::check_export_available,
            commands::export_commands::get_available_export_formats,
            commands::export_commands::is_export_format_available,
            // Comandos específicos por formato (opcionales)
            commands::export_commands::export_to_pdf,
            commands::export_commands::export_to_excel,
            commands::export_commands::export_to_csv,
            // ==========================================
            // COMANDOS DE TEMPLATES
            // ==========================================
            commands::templates::get_templates,
            commands::templates::save_template,
            commands::templates::delete_template,
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
            commands::backup::restore_database,
            // ==========================================
            // COMANDOS DE CONFIGURACIÓN
            // ==========================================
            commands::config_commands::get_app_config,
            commands::config_commands::update_terminal_config,
            // ==========================================
            // COMANDOS DE PREFERENCIAS (NUEVO)
            // ==========================================
            commands::preferences_commands::get_user_preferences,
            commands::preferences_commands::set_user_preference,
            commands::shortcuts_commands::get_shortcuts,
            commands::shortcuts_commands::update_shortcuts,
            commands::shortcuts_commands::reset_shortcuts,
            // ==========================================
            // COMANDOS DE VENTANA
            // ==========================================
            commands::window_commands::show_main_window,
            // ==========================================
            // COMANDOS DE EMAIL/REPORTES
            // ==========================================
            commands::email_commands::send_suggestion,
            commands::email_commands::send_error_report,
            commands::email_commands::create_reporte,
            commands::email_commands::get_all_reportes,
            commands::email_commands::get_reporte,
            commands::email_commands::get_reportes_by_tipo,
            commands::email_commands::retry_reporte,
            // ==========================================
            // COMANDOS DE KEYRING/CREDENCIALES
            // ==========================================
            commands::keyring_commands::get_credential_status,
            commands::keyring_commands::is_app_configured,
            commands::keyring_commands::needs_setup,
            commands::keyring_commands::setup_credentials,
            commands::keyring_commands::get_smtp_config,
            commands::keyring_commands::update_smtp_credentials,
            commands::keyring_commands::test_smtp_connection,
            commands::keyring_commands::test_smtp_connection_with_creds,
            commands::keyring_commands::get_argon2_config,
            commands::keyring_commands::update_argon2_params,
            commands::keyring_commands::generate_argon2_secret,
            commands::keyring_commands::generate_random_secret,
            commands::keyring_commands::test_keyring,
            commands::keyring_commands::reset_all_credentials,
        ]
    };
}
