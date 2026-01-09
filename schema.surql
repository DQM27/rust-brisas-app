-- ============================================================================
-- 1. SEGURIDAD Y ACCESO
-- ============================================================================

DEFINE SCOPE sistema_scope SESSION 24h
    SIGNIN ( SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $password) )
    -- Solo admins crean usuarios, pero definimos el signup por si acaso
    SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password), role = 'portero' );

-- ============================================================================
-- 2. CATÁLOGOS BASE (Roles y Empresas)
-- ============================================================================

-- TABLA: ROLE
DEFINE TABLE role SCHEMAFULL
    PERMISSIONS FOR select, create, update, delete WHERE $auth.role = 'admin' OR $auth.id != NONE;

DEFINE FIELD name          ON TABLE role TYPE string ASSERT $value != "";
DEFINE FIELD description   ON TABLE role TYPE option<string>;
DEFINE FIELD is_system     ON TABLE role TYPE bool DEFAULT false;
DEFINE FIELD inherits_from ON TABLE role TYPE option<record<role>>;
DEFINE FIELD permissions   ON TABLE role TYPE option<array<string>>;
DEFINE INDEX idx_role_name ON TABLE role COLUMNS name UNIQUE;
-- Sync
DEFINE FIELD updated_at    ON TABLE role TYPE datetime DEFAULT time::now() VALUE time::now();

-- TABLA: EMPRESA
DEFINE TABLE empresa SCHEMAFULL
    PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;

DEFINE FIELD nombre        ON TABLE empresa TYPE string ASSERT $value != "";
DEFINE FIELD direccion     ON TABLE empresa TYPE option<string>;
DEFINE FIELD is_active     ON TABLE empresa TYPE bool DEFAULT true;
DEFINE INDEX idx_emp_nom   ON TABLE empresa COLUMNS nombre UNIQUE;
-- Sync
DEFINE FIELD updated_at    ON TABLE empresa TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD sync_status   ON TABLE empresa TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device ON TABLE empresa TYPE string;

-- ============================================================================
-- 3. USUARIOS
-- ============================================================================

DEFINE TABLE user SCHEMAFULL
    PERMISSIONS
        FOR select WHERE id = $auth.id OR $auth.role = 'admin'
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE FIELD email          ON TABLE user TYPE string ASSERT string::is::email($value);
DEFINE FIELD username       ON TABLE user TYPE string; -- Mapeo para login
DEFINE FIELD password       ON TABLE user TYPE string;
DEFINE FIELD nombre         ON TABLE user TYPE string;
DEFINE FIELD apellido       ON TABLE user TYPE string;
DEFINE FIELD role           ON TABLE user TYPE record<role>;
DEFINE FIELD is_active      ON TABLE user TYPE bool DEFAULT true;
DEFINE FIELD cedula         ON TABLE user TYPE string;
DEFINE FIELD must_change_password ON TABLE user TYPE bool DEFAULT false;
DEFINE FIELD avatar_path    ON TABLE user TYPE option<string>;
-- Campos Opcionales
DEFINE FIELD segundo_nombre ON TABLE user TYPE option<string>;
DEFINE FIELD segundo_apellido ON TABLE user TYPE option<string>;
DEFINE FIELD numero_gafete  ON TABLE user TYPE option<string>;
DEFINE FIELD fecha_inicio_labores ON TABLE user TYPE option<string>; -- O datetime si lo cambias en Rust
DEFINE FIELD fecha_nacimiento ON TABLE user TYPE option<string>;
DEFINE FIELD telefono       ON TABLE user TYPE option<string>;
DEFINE FIELD direccion      ON TABLE user TYPE option<string>;
DEFINE FIELD contacto_emergencia_nombre ON TABLE user TYPE option<string>;
DEFINE FIELD contacto_emergencia_telefono ON TABLE user TYPE option<string>;
-- Indices
DEFINE INDEX idx_user_email  ON TABLE user COLUMNS email UNIQUE;
DEFINE INDEX idx_user_cedula ON TABLE user COLUMNS cedula UNIQUE;
-- Sync
DEFINE FIELD updated_at      ON TABLE user TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD created_at      ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at      ON TABLE user TYPE option<datetime>;

-- ============================================================================
-- 4. PERSONAL EXTERNO (Contratistas, Proveedores, Visitas)
-- ============================================================================

-- TABLA: CONTRATISTA
DEFINE TABLE contratista SCHEMAFULL
    PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;

DEFINE FIELD cedula         ON TABLE contratista TYPE string ASSERT $value != "";
DEFINE FIELD nombre         ON TABLE contratista TYPE string;
DEFINE FIELD apellido       ON TABLE contratista TYPE string;
DEFINE FIELD empresa        ON TABLE contratista TYPE record<empresa>;
DEFINE FIELD fecha_vencimiento_praind ON TABLE contratista TYPE datetime;
DEFINE FIELD estado         ON TABLE contratista TYPE string ASSERT $value INSIDE ['activo', 'inactivo', 'bloqueado'];
DEFINE FIELD segundo_nombre ON TABLE contratista TYPE option<string>;
DEFINE FIELD segundo_apellido ON TABLE contratista TYPE option<string>;
DEFINE INDEX idx_cont_ced   ON TABLE contratista COLUMNS cedula UNIQUE;
-- Sync
DEFINE FIELD created_at     ON TABLE contratista TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at     ON TABLE contratista TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD deleted_at     ON TABLE contratista TYPE option<datetime>;
DEFINE FIELD sync_status    ON TABLE contratista TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device  ON TABLE contratista TYPE string;

-- TABLA: PROVEEDOR
DEFINE TABLE proveedor SCHEMAFULL
    PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;

DEFINE FIELD cedula         ON TABLE proveedor TYPE string ASSERT $value != "";
DEFINE FIELD nombre         ON TABLE proveedor TYPE string;
DEFINE FIELD apellido       ON TABLE proveedor TYPE string;
DEFINE FIELD empresa        ON TABLE proveedor TYPE record<empresa>;
DEFINE FIELD estado         ON TABLE proveedor TYPE string ASSERT $value INSIDE ['activo', 'inactivo', 'suspendido'];
DEFINE FIELD segundo_nombre ON TABLE proveedor TYPE option<string>;
DEFINE FIELD segundo_apellido ON TABLE proveedor TYPE option<string>;
DEFINE INDEX idx_prov_ced   ON TABLE proveedor COLUMNS cedula UNIQUE;
-- Sync
DEFINE FIELD created_at     ON TABLE proveedor TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at     ON TABLE proveedor TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD deleted_at     ON TABLE proveedor TYPE option<datetime>;
DEFINE FIELD sync_status    ON TABLE proveedor TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device  ON TABLE proveedor TYPE string;

-- TABLA: VISITANTE
DEFINE TABLE visitante SCHEMAFULL
    PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;

DEFINE FIELD cedula         ON TABLE visitante TYPE string ASSERT $value != "";
DEFINE FIELD nombre         ON TABLE visitante TYPE string;
DEFINE FIELD apellido       ON TABLE visitante TYPE string;
DEFINE FIELD empresa        ON TABLE visitante TYPE option<record<empresa>>;
DEFINE FIELD has_vehicle    ON TABLE visitante TYPE bool DEFAULT false;
DEFINE FIELD segundo_nombre ON TABLE visitante TYPE option<string>;
DEFINE FIELD segundo_apellido ON TABLE visitante TYPE option<string>;
DEFINE INDEX idx_vis_ced    ON TABLE visitante COLUMNS cedula UNIQUE;
-- Sync
DEFINE FIELD created_at     ON TABLE visitante TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at     ON TABLE visitante TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD deleted_at     ON TABLE visitante TYPE option<datetime>;
DEFINE FIELD sync_status    ON TABLE visitante TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device  ON TABLE visitante TYPE string;

-- ============================================================================
-- 5. SEGURIDAD Y CONTROL (Lista Negra, Gafetes)
-- ============================================================================

-- TABLA: LISTA NEGRA
DEFINE TABLE lista_negra SCHEMAFULL
    PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;

DEFINE FIELD cedula         ON TABLE lista_negra TYPE string;
DEFINE FIELD nombre         ON TABLE lista_negra TYPE string;
DEFINE FIELD apellido       ON TABLE lista_negra TYPE string;
DEFINE FIELD nivel_severidad ON TABLE lista_negra TYPE string ASSERT $value INSIDE ['ALTO', 'MEDIO', 'BAJO'];
DEFINE FIELD bloqueado_por  ON TABLE lista_negra TYPE string; -- ID string o record
DEFINE FIELD is_active      ON TABLE lista_negra TYPE bool DEFAULT true;
DEFINE FIELD motivo_bloqueo ON TABLE lista_negra TYPE option<string>;
DEFINE FIELD empresa_id     ON TABLE lista_negra TYPE option<record<empresa>>;
DEFINE INDEX idx_ln_ced     ON TABLE lista_negra COLUMNS cedula; -- No unique, puede bloquearse varias veces
-- Sync
DEFINE FIELD created_at     ON TABLE lista_negra TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at     ON TABLE lista_negra TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD sync_status    ON TABLE lista_negra TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device  ON TABLE lista_negra TYPE string;

-- TABLA: GAFETE
DEFINE TABLE gafete SCHEMAFULL
    PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;

DEFINE FIELD numero         ON TABLE gafete TYPE int;
DEFINE FIELD tipo           ON TABLE gafete TYPE string; -- 'contratista', 'proveedor', etc
DEFINE FIELD estado         ON TABLE gafete TYPE string ASSERT $value INSIDE ['activo', 'danado', 'extraviado'];
DEFINE FIELD en_uso         ON TABLE gafete TYPE bool DEFAULT false;
DEFINE INDEX idx_gafete_num ON TABLE gafete COLUMNS numero UNIQUE;
-- Sync
DEFINE FIELD updated_at     ON TABLE gafete TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD sync_status    ON TABLE gafete TYPE string DEFAULT 'pending';

-- ============================================================================
-- 6. OPERACIONES (INGRESOS)
-- ============================================================================

-- TABLA: INGRESO_CONTRATISTA
DEFINE TABLE ingreso_contratista SCHEMAFULL PERMISSIONS FOR select, create WHERE $auth.id != NONE;
DEFINE FIELD contratista      ON TABLE ingreso_contratista TYPE record<contratista>;
DEFINE FIELD fecha_hora_ingreso ON TABLE ingreso_contratista TYPE datetime DEFAULT time::now();
DEFINE FIELD usuario_ingreso  ON TABLE ingreso_contratista TYPE record<user>;
DEFINE FIELD tipo_autorizacion ON TABLE ingreso_contratista TYPE string;
DEFINE FIELD modo_ingreso     ON TABLE ingreso_contratista TYPE string;
DEFINE FIELD nombre           ON TABLE ingreso_contratista TYPE string; -- Snapshot
DEFINE FIELD apellido         ON TABLE ingreso_contratista TYPE string; -- Snapshot
DEFINE FIELD cedula           ON TABLE ingreso_contratista TYPE string; -- Snapshot
-- Opcionales
DEFINE FIELD fecha_hora_salida ON TABLE ingreso_contratista TYPE option<datetime>;
DEFINE FIELD usuario_salida   ON TABLE ingreso_contratista TYPE option<record<user>>;
DEFINE FIELD placa_vehiculo   ON TABLE ingreso_contratista TYPE option<string>;
DEFINE FIELD gafete_numero    ON TABLE ingreso_contratista TYPE option<int>;
DEFINE FIELD observaciones    ON TABLE ingreso_contratista TYPE option<string>;
-- Sync
DEFINE FIELD sync_status      ON TABLE ingreso_contratista TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device    ON TABLE ingreso_contratista TYPE string;
DEFINE FIELD updated_at       ON TABLE ingreso_contratista TYPE datetime DEFAULT time::now() VALUE time::now();

-- TABLA: INGRESO_PROVEEDOR
DEFINE TABLE ingreso_proveedor SCHEMAFULL PERMISSIONS FOR select, create WHERE $auth.id != NONE;
DEFINE FIELD proveedor        ON TABLE ingreso_proveedor TYPE record<proveedor>;
DEFINE FIELD fecha_hora_ingreso ON TABLE ingreso_proveedor TYPE datetime DEFAULT time::now();
DEFINE FIELD usuario_ingreso  ON TABLE ingreso_proveedor TYPE record<user>;
DEFINE FIELD motivo           ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD modo_ingreso     ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD nombre           ON TABLE ingreso_proveedor TYPE string; -- Snapshot
DEFINE FIELD apellido         ON TABLE ingreso_proveedor TYPE string; -- Snapshot
DEFINE FIELD cedula           ON TABLE ingreso_proveedor TYPE string; -- Snapshot
-- Opcionales
DEFINE FIELD fecha_hora_salida ON TABLE ingreso_proveedor TYPE option<datetime>;
DEFINE FIELD usuario_salida   ON TABLE ingreso_proveedor TYPE option<record<user>>;
DEFINE FIELD placa_vehiculo   ON TABLE ingreso_proveedor TYPE option<string>;
DEFINE FIELD gafete_numero    ON TABLE ingreso_proveedor TYPE option<int>;
DEFINE FIELD observaciones    ON TABLE ingreso_proveedor TYPE option<string>;
-- Sync
DEFINE FIELD sync_status      ON TABLE ingreso_proveedor TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device    ON TABLE ingreso_proveedor TYPE string;
DEFINE FIELD updated_at       ON TABLE ingreso_proveedor TYPE datetime DEFAULT time::now() VALUE time::now();

-- TABLA: INGRESO_VISITA
DEFINE TABLE ingreso_visita SCHEMAFULL PERMISSIONS FOR select, create WHERE $auth.id != NONE;
-- Nota: Visita no tiene link fuerte obligatorio, es snapshot puro muchas veces
DEFINE FIELD fecha_hora_ingreso ON TABLE ingreso_visita TYPE datetime DEFAULT time::now();
DEFINE FIELD usuario_ingreso  ON TABLE ingreso_visita TYPE record<user>;
DEFINE FIELD motivo           ON TABLE ingreso_visita TYPE string;
DEFINE FIELD anfitrion        ON TABLE ingreso_visita TYPE string;
DEFINE FIELD nombre           ON TABLE ingreso_visita TYPE string;
DEFINE FIELD apellido         ON TABLE ingreso_visita TYPE string;
DEFINE FIELD cedula           ON TABLE ingreso_visita TYPE string;
-- Opcionales
DEFINE FIELD fecha_hora_salida ON TABLE ingreso_visita TYPE option<datetime>;
DEFINE FIELD usuario_salida   ON TABLE ingreso_visita TYPE option<record<user>>;
DEFINE FIELD placa_vehiculo   ON TABLE ingreso_visita TYPE option<string>;
DEFINE FIELD gafete_numero    ON TABLE ingreso_visita TYPE option<int>;
DEFINE FIELD observaciones    ON TABLE ingreso_visita TYPE option<string>;
-- Sync
DEFINE FIELD sync_status      ON TABLE ingreso_visita TYPE string DEFAULT 'pending';
DEFINE FIELD origin_device    ON TABLE ingreso_visita TYPE string;
DEFINE FIELD updated_at       ON TABLE ingreso_visita TYPE datetime DEFAULT time::now() VALUE time::now();

-- ============================================================================
-- 7. LOGÍSTICA (Vehículos, Citas, Alertas)
-- ============================================================================

-- TABLA: VEHICULO
DEFINE TABLE vehiculo SCHEMAFULL PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;
DEFINE FIELD placa          ON TABLE vehiculo TYPE string ASSERT $value != "";
DEFINE FIELD tipo_vehiculo  ON TABLE vehiculo TYPE string;
-- Polimorfismo: Propietario puede ser cualquiera de estos 3
DEFINE FIELD propietario    ON TABLE vehiculo TYPE record<contratista | proveedor | visitante>;
DEFINE FIELD is_active      ON TABLE vehiculo TYPE bool DEFAULT true;
DEFINE INDEX idx_placa      ON TABLE vehiculo COLUMNS placa UNIQUE;
-- Sync
DEFINE FIELD updated_at     ON TABLE vehiculo TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD sync_status    ON TABLE vehiculo TYPE string DEFAULT 'pending';

-- TABLA: CITA
DEFINE TABLE cita SCHEMAFULL PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;
DEFINE FIELD usuario_id     ON TABLE cita TYPE record<user>;
DEFINE FIELD visitante_id   ON TABLE cita TYPE option<record<visitante>>;
DEFINE FIELD motivo         ON TABLE cita TYPE string;
DEFINE FIELD fecha_inicio   ON TABLE cita TYPE datetime;
DEFINE FIELD fecha_fin      ON TABLE cita TYPE datetime;
DEFINE FIELD estado         ON TABLE cita TYPE string;
DEFINE FIELD activa         ON TABLE cita TYPE bool DEFAULT true;
-- Sync
DEFINE FIELD updated_at     ON TABLE cita TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD sync_status    ON TABLE cita TYPE string DEFAULT 'pending';

-- TABLA: ALERTA_GAFETE
DEFINE TABLE alerta_gafete SCHEMAFULL PERMISSIONS FOR select, create, update WHERE $auth.id != NONE;
DEFINE FIELD cedula         ON TABLE alerta_gafete TYPE string;
DEFINE FIELD gafete_numero  ON TABLE alerta_gafete TYPE int;
DEFINE FIELD resuelto       ON TABLE alerta_gafete TYPE bool DEFAULT false;
DEFINE FIELD reportado_por  ON TABLE alerta_gafete TYPE record<user>;
-- Sync
DEFINE FIELD updated_at     ON TABLE alerta_gafete TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD sync_status    ON TABLE alerta_gafete TYPE string DEFAULT 'pending';