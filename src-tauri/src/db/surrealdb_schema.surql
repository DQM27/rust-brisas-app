-- =========================================================
-- SurrealDB Schema for Brisas - ENTERPRISE NATIVE
-- =========================================================

-- =========================================================
-- 1. SISTEMA Y ACCESO (User, Role)
-- =========================================================

-- =========================================================
-- Tabla ROLE
-- =========================================================
DEFINE TABLE role SCHEMAFULL;
DEFINE FIELD name ON TABLE role TYPE string;
DEFINE FIELD description ON TABLE role TYPE option<string>;
DEFINE FIELD is_system ON TABLE role TYPE bool DEFAULT false;
DEFINE FIELD inherits_from ON TABLE role TYPE option<record<role>>;
DEFINE FIELD permissions ON TABLE role TYPE option<array<string>>;
DEFINE FIELD created_at ON TABLE role TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE role TYPE datetime VALUE time::now();
DEFINE INDEX idx_role_name ON role COLUMNS name UNIQUE;

-- =========================================================
-- Tabla USUARIO (user)
-- =========================================================
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON TABLE user TYPE string ASSERT string::is::email($value);
DEFINE INDEX idx_user_email ON user COLUMNS email UNIQUE;
DEFINE FIELD password_hash ON TABLE user TYPE string;
DEFINE FIELD is_active ON TABLE user TYPE bool DEFAULT true;
DEFINE FIELD role ON TABLE user TYPE record<role>; 
DEFINE FIELD nombre ON TABLE user TYPE string;
DEFINE FIELD apellido ON TABLE user TYPE string;
DEFINE FIELD cedula ON TABLE user TYPE string;
DEFINE INDEX idx_user_cedula ON user COLUMNS cedula UNIQUE;
DEFINE FIELD segundo_nombre ON TABLE user TYPE option<string>;
DEFINE FIELD segundo_apellido ON TABLE user TYPE option<string>;
DEFINE FIELD fecha_inicio_labores ON TABLE user TYPE option<string>;
DEFINE FIELD fecha_nacimiento ON TABLE user TYPE option<string>;
DEFINE FIELD numero_gafete ON TABLE user TYPE option<string>;
DEFINE FIELD telefono ON TABLE user TYPE option<string>;
DEFINE FIELD direccion ON TABLE user TYPE option<string>;
DEFINE FIELD contacto_emergencia_nombre ON TABLE user TYPE option<string>;
DEFINE FIELD contacto_emergencia_telefono ON TABLE user TYPE option<string>;
DEFINE FIELD avatar_path ON TABLE user TYPE option<string>;
DEFINE FIELD must_change_password ON TABLE user TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE user TYPE datetime VALUE time::now();
DEFINE FIELD deleted_at ON TABLE user TYPE option<datetime>;
DEFINE FIELD nombre_completo ON TABLE user VALUE 
    string::trim(nombre + ' ' + (segundo_nombre OR '') + ' ' + apellido + ' ' + (segundo_apellido OR ''));

-- =========================================================
-- 2. ORGANIZACIÓN (Empresa)
-- =========================================================

-- =========================================================
-- Tabla EMPRESA
-- =========================================================
DEFINE TABLE empresa SCHEMAFULL;
DEFINE FIELD nombre ON TABLE empresa TYPE string;
DEFINE FIELD direccion ON TABLE empresa TYPE option<string>;
DEFINE FIELD is_active ON TABLE empresa TYPE bool DEFAULT true;
DEFINE FIELD created_at ON TABLE empresa TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE empresa TYPE datetime VALUE time::now();
DEFINE INDEX idx_empresa_nombre ON empresa COLUMNS nombre UNIQUE;

-- =========================================================
-- 3. ACTORES EXTERNOS (Contratista, Proveedor, Visitante)
-- =========================================================

-- =========================================================
-- Tabla CONTRATISTA
-- =========================================================
DEFINE TABLE contratista SCHEMAFULL;
DEFINE FIELD cedula ON TABLE contratista TYPE string;
DEFINE FIELD nombre ON TABLE contratista TYPE string;
DEFINE FIELD segundo_nombre ON TABLE contratista TYPE option<string>;
DEFINE FIELD apellido ON TABLE contratista TYPE string;
DEFINE FIELD segundo_apellido ON TABLE contratista TYPE option<string>;
DEFINE FIELD nombre_completo ON TABLE contratista VALUE 
    string::trim(nombre + ' ' + (segundo_nombre OR '') + ' ' + apellido + ' ' + (segundo_apellido OR ''));
DEFINE FIELD empresa ON TABLE contratista TYPE record<empresa>;
DEFINE FIELD estado ON TABLE contratista TYPE string ASSERT $value INSIDE ['activo', 'inactivo', 'bloqueado'];
DEFINE FIELD fecha_vencimiento_praind ON TABLE contratista TYPE option<string>;
DEFINE FIELD created_at ON TABLE contratista TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE contratista TYPE datetime VALUE time::now();
DEFINE FIELD deleted_at ON TABLE contratista TYPE option<datetime>;
DEFINE INDEX idx_contratista_cedula ON contratista COLUMNS cedula UNIQUE;

-- =========================================================
-- Tabla PROVEEDOR
-- =========================================================
DEFINE TABLE proveedor SCHEMAFULL;
DEFINE FIELD cedula ON TABLE proveedor TYPE string;
DEFINE FIELD nombre ON TABLE proveedor TYPE string;
DEFINE FIELD segundo_nombre ON TABLE proveedor TYPE option<string>;
DEFINE FIELD apellido ON TABLE proveedor TYPE string;
DEFINE FIELD segundo_apellido ON TABLE proveedor TYPE option<string>;
DEFINE FIELD nombre_completo ON TABLE proveedor VALUE 
    string::trim(nombre + ' ' + (segundo_nombre OR '') + ' ' + apellido + ' ' + (segundo_apellido OR ''));
DEFINE FIELD empresa ON TABLE proveedor TYPE record<empresa>;
DEFINE FIELD estado ON TABLE proveedor TYPE string ASSERT $value INSIDE ['ACTIVO', 'INACTIVO', 'SUSPENDIDO'] DEFAULT 'ACTIVO';
DEFINE FIELD created_at ON TABLE proveedor TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE proveedor TYPE datetime VALUE time::now();
DEFINE FIELD deleted_at ON TABLE proveedor TYPE option<datetime>;
DEFINE INDEX idx_proveedor_cedula ON proveedor COLUMNS cedula UNIQUE;

-- =========================================================
-- Tabla VISITANTE
-- =========================================================
DEFINE TABLE visitante SCHEMAFULL;
DEFINE FIELD cedula ON TABLE visitante TYPE string;
DEFINE FIELD nombre ON TABLE visitante TYPE string;
DEFINE FIELD segundo_nombre ON TABLE visitante TYPE option<string>;
DEFINE FIELD apellido ON TABLE visitante TYPE string;
DEFINE FIELD segundo_apellido ON TABLE visitante TYPE option<string>;
DEFINE FIELD nombre_completo ON TABLE visitante VALUE 
    string::trim(nombre + ' ' + (segundo_nombre OR '') + ' ' + apellido + ' ' + (segundo_apellido OR ''));
DEFINE FIELD empresa ON TABLE visitante TYPE option<record<empresa>>;
DEFINE FIELD created_at ON TABLE visitante TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE visitante TYPE datetime VALUE time::now();
DEFINE FIELD deleted_at ON TABLE visitante TYPE option<datetime>;
DEFINE INDEX idx_visitante_cedula ON visitante COLUMNS cedula UNIQUE;

-- =========================================================
-- 4. OPERACIONES (INGRESOS Y CITAS)
-- =========================================================

-- =========================================================
-- Tabla INGRESO_CONTRATISTA
-- =========================================================
DEFINE TABLE ingreso_contratista SCHEMAFULL;
DEFINE FIELD contratista ON TABLE ingreso_contratista TYPE record<contratista>;
DEFINE FIELD nombre ON TABLE ingreso_contratista TYPE string;
DEFINE FIELD apellido ON TABLE ingreso_contratista TYPE string;
DEFINE FIELD cedula ON TABLE ingreso_contratista TYPE string;
DEFINE FIELD tipo_autorizacion ON TABLE ingreso_contratista TYPE string ASSERT $value INSIDE ['praind', 'correo'];
DEFINE FIELD modo_ingreso ON TABLE ingreso_contratista TYPE string ASSERT $value INSIDE ['caminando', 'vehiculo'];
DEFINE FIELD placa_vehiculo ON TABLE ingreso_contratista TYPE option<string>;
DEFINE FIELD gafete_numero ON TABLE ingreso_contratista TYPE option<int>;
DEFINE FIELD fecha_hora_ingreso ON TABLE ingreso_contratista TYPE datetime DEFAULT time::now();
DEFINE FIELD usuario_ingreso ON TABLE ingreso_contratista TYPE record<user>;
DEFINE FIELD fecha_hora_salida ON TABLE ingreso_contratista TYPE option<datetime>;
DEFINE FIELD usuario_salida ON TABLE ingreso_contratista TYPE option<record<user>>;
DEFINE FIELD observaciones ON TABLE ingreso_contratista TYPE option<string>;
DEFINE FIELD created_at ON TABLE ingreso_contratista TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE ingreso_contratista TYPE datetime VALUE time::now();

-- =========================================================
-- Tabla INGRESO_PROVEEDOR
-- =========================================================
DEFINE TABLE ingreso_proveedor SCHEMAFULL;
DEFINE FIELD proveedor ON TABLE ingreso_proveedor TYPE record<proveedor>;
DEFINE FIELD nombre ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD apellido ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD cedula ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD area_visitada ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD motivo ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD modo_ingreso ON TABLE ingreso_proveedor TYPE string ASSERT $value INSIDE ['caminando', 'vehiculo'];
DEFINE FIELD placa_vehiculo ON TABLE ingreso_proveedor TYPE option<string>;
DEFINE FIELD gafete_numero ON TABLE ingreso_proveedor TYPE option<int>;
DEFINE FIELD fecha_hora_ingreso ON TABLE ingreso_proveedor TYPE datetime DEFAULT time::now();
DEFINE FIELD usuario_ingreso ON TABLE ingreso_proveedor TYPE record<user>;
DEFINE FIELD fecha_hora_salida ON TABLE ingreso_proveedor TYPE option<datetime>;
DEFINE FIELD usuario_salida ON TABLE ingreso_proveedor TYPE option<record<user>>;
DEFINE FIELD observaciones ON TABLE ingreso_proveedor TYPE option<string>;
DEFINE FIELD created_at ON TABLE ingreso_proveedor TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE ingreso_proveedor TYPE datetime VALUE time::now();

-- =========================================================
-- Tabla INGRESO_VISITA
-- =========================================================
DEFINE TABLE ingreso_visita SCHEMAFULL;
DEFINE FIELD nombre ON TABLE ingreso_visita TYPE string;
DEFINE FIELD apellido ON TABLE ingreso_visita TYPE string;
DEFINE FIELD cedula ON TABLE ingreso_visita TYPE string;
DEFINE FIELD anfitrion ON TABLE ingreso_visita TYPE string;
DEFINE FIELD area_visitada ON TABLE ingreso_visita TYPE string;
DEFINE FIELD motivo ON TABLE ingreso_visita TYPE string;
DEFINE FIELD modo_ingreso ON TABLE ingreso_visita TYPE string ASSERT $value INSIDE ['caminando', 'vehiculo'];
DEFINE FIELD placa_vehiculo ON TABLE ingreso_visita TYPE option<string>;
DEFINE FIELD gafete_numero ON TABLE ingreso_visita TYPE option<int>;
DEFINE FIELD fecha_hora_ingreso ON TABLE ingreso_visita TYPE datetime DEFAULT time::now();
DEFINE FIELD usuario_ingreso ON TABLE ingreso_visita TYPE record<user>;
DEFINE FIELD fecha_hora_salida ON TABLE ingreso_visita TYPE option<datetime>;
DEFINE FIELD usuario_salida ON TABLE ingreso_visita TYPE option<record<user>>;
DEFINE FIELD observaciones ON TABLE ingreso_visita TYPE option<string>;
DEFINE FIELD created_at ON TABLE ingreso_visita TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE ingreso_visita TYPE datetime VALUE time::now();

-- =========================================================
-- Tabla CITA
-- =========================================================
DEFINE TABLE cita SCHEMAFULL;
DEFINE FIELD visitante_id ON TABLE cita TYPE option<record<visitante>>;
DEFINE FIELD usuario_id ON TABLE cita TYPE record<user>;
DEFINE FIELD motivo ON TABLE cita TYPE string;
DEFINE FIELD fecha_inicio ON TABLE cita TYPE datetime;
DEFINE FIELD fecha_fin ON TABLE cita TYPE datetime;
DEFINE FIELD estado ON TABLE cita TYPE string ASSERT $value INSIDE ['pendiente', 'completada', 'cancelada'] DEFAULT 'pendiente';
DEFINE FIELD activa ON TABLE cita TYPE bool DEFAULT true;
DEFINE FIELD visitante_nombre ON TABLE cita TYPE option<string>;
DEFINE FIELD visitante_cedula ON TABLE cita TYPE option<string>;
DEFINE FIELD created_at ON TABLE cita TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE cita TYPE datetime VALUE time::now();

-- =========================================================
-- 5. RECURSOS (Vehículos, Gafetes)
-- =========================================================

-- =========================================================
-- Tabla GAFETE
-- =========================================================
DEFINE TABLE gafete SCHEMAFULL;
DEFINE FIELD numero ON TABLE gafete TYPE int;
DEFINE FIELD tipo ON TABLE gafete TYPE string ASSERT $value INSIDE ['contratista', 'proveedor', 'visita'];
DEFINE FIELD estado ON TABLE gafete TYPE string ASSERT $value INSIDE ['activo', 'inactivo', 'perdido'] DEFAULT 'activo';
DEFINE FIELD en_uso ON TABLE gafete TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE gafete TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE gafete TYPE datetime VALUE time::now();
DEFINE INDEX idx_gafete_numero_tipo ON gafete COLUMNS numero, tipo UNIQUE;

-- =========================================================
-- Tabla VEHICULO
-- =========================================================
DEFINE TABLE vehiculo SCHEMAFULL;
DEFINE FIELD propietario ON TABLE vehiculo TYPE record<contratista | proveedor | visitante>;
DEFINE FIELD tipo_vehiculo ON TABLE vehiculo TYPE string ASSERT $value INSIDE ['automovil', 'motocicleta', 'camion', 'otro'];
DEFINE FIELD placa ON TABLE vehiculo TYPE string;
DEFINE FIELD marca ON TABLE vehiculo TYPE option<string>;
DEFINE FIELD modelo ON TABLE vehiculo TYPE option<string>;
DEFINE FIELD color ON TABLE vehiculo TYPE option<string>;
DEFINE FIELD is_active ON TABLE vehiculo TYPE bool DEFAULT true;
DEFINE FIELD created_at ON TABLE vehiculo TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE vehiculo TYPE datetime VALUE time::now();
DEFINE INDEX idx_vehiculo_placa ON vehiculo COLUMNS placa UNIQUE;

-- =========================================================
-- 6. SEGURIDAD Y AUDITORÍA (Lista Negra, Logs)
-- =========================================================

-- =========================================================
-- Tabla LISTA_NEGRA
-- =========================================================
DEFINE TABLE lista_negra SCHEMAFULL;
DEFINE FIELD cedula ON TABLE lista_negra TYPE string;
DEFINE FIELD nombre ON TABLE lista_negra TYPE string;
DEFINE FIELD apellido ON TABLE lista_negra TYPE string;
DEFINE FIELD nivel_severidad ON TABLE lista_negra TYPE string ASSERT $value INSIDE ['BAJO', 'MEDIO', 'ALTO'];
DEFINE FIELD motivo_bloqueo ON TABLE lista_negra TYPE string;
DEFINE FIELD bloqueado_por ON TABLE lista_negra TYPE option<string>;
DEFINE FIELD is_active ON TABLE lista_negra TYPE bool DEFAULT true;
DEFINE FIELD created_at ON TABLE lista_negra TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE lista_negra TYPE datetime VALUE time::now();
DEFINE INDEX idx_lista_negra_cedula ON lista_negra COLUMNS cedula UNIQUE;

-- =========================================================
-- Tabla AUDIT_LOG
-- =========================================================
-- =========================================================
-- CONFIGURACIÓN DEL SISTEMA
-- =========================================================
DEFINE TABLE system_config SCHEMAFULL;
DEFINE FIELD audit_tables ON TABLE system_config TYPE array<string> DEFAULT ['user', 'role', 'lista_negra'];
DEFINE FIELD created_at ON TABLE system_config TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE system_config TYPE datetime VALUE time::now();

-- =========================================================
-- Tabla AUDIT_LOG (Estilo Simple History con Configuración Dinámica)
-- =========================================================
DEFINE TABLE audit_log SCHEMAFULL;
DEFINE FIELD action ON TABLE audit_log TYPE string;
DEFINE FIELD target_id ON TABLE audit_log TYPE record;
DEFINE FIELD target_table ON TABLE audit_log TYPE string;
DEFINE FIELD actor ON TABLE audit_log TYPE option<record<user>>;
DEFINE FIELD change_reason ON TABLE audit_log TYPE option<string>;
DEFINE FIELD previous_state ON TABLE audit_log TYPE option<object>;
DEFINE FIELD new_state ON TABLE audit_log TYPE option<object>;
DEFINE FIELD timestamp ON TABLE audit_log TYPE datetime VALUE time::now();

-- Evento: CREACIÓN DE USUARIO (Condicional)
DEFINE EVENT log_user_creation ON TABLE user WHEN $event = "CREATE" THEN {
    LET $config = (SELECT VALUE audit_tables FROM system_config LIMIT 1);
    IF "user" IN $config THEN
        CREATE audit_log SET 
            action = "CREATE", 
            target_id = $after.id,
            target_table = "user",
            actor = $auth.id,
            new_state = $after,
            timestamp = time::now()
    END
};

-- Evento: ACTUALIZACIÓN DE USUARIO (Condicional)
DEFINE EVENT log_user_update ON TABLE user WHEN $event = "UPDATE" THEN {
    LET $config = (SELECT VALUE audit_tables FROM system_config LIMIT 1);
    IF "user" IN $config THEN
        CREATE audit_log SET 
            action = "UPDATE", 
            target_id = $after.id,
            target_table = "user",
            actor = $auth.id,
            previous_state = $before,
            new_state = $after,
            timestamp = time::now()
    END
};

-- Evento: ELIMINACIÓN DE USUARIO (Condicional)
DEFINE EVENT log_user_delete ON TABLE user WHEN $event = "DELETE" THEN {
    LET $config = (SELECT VALUE audit_tables FROM system_config LIMIT 1);
    IF "user" IN $config THEN
        CREATE audit_log SET 
            action = "DELETE", 
            target_id = $before.id,
            target_table = "user",
            actor = $auth.id,
            previous_state = $before,
            timestamp = time::now()
    END
};
