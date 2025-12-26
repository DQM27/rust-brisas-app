-- ==========================================
-- SurrealDB Schema for Brisas
-- ==========================================
-- Ejecutar con: surreal import schema.surql

-- ==========================================
-- NAMESPACE Y DATABASE
-- ==========================================
DEFINE NAMESPACE IF NOT EXISTS brisas;
USE NS brisas;

DEFINE DATABASE IF NOT EXISTS produccion;
USE DB produccion;

-- ==========================================
-- SCOPES (para autenticación de usuarios)
-- ==========================================
DEFINE SCOPE user SESSION 24h
    SIGNUP (
        CREATE user SET 
            email = $email, 
            password = crypto::argon2::generate($password),
            nombre = $nombre,
            apellido = $apellido
    )
    SIGNIN (
        SELECT * FROM user 
        WHERE email = $email 
        AND crypto::argon2::compare(password, $password)
    );

-- ==========================================
-- TABLA: roles
-- ==========================================
DEFINE TABLE roles SCHEMAFULL
    PERMISSIONS
        FOR select WHERE true
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE FIELD id ON roles TYPE string;
DEFINE FIELD nombre ON roles TYPE string;
DEFINE FIELD descripcion ON roles TYPE option<string>;
DEFINE FIELD permisos ON roles TYPE array DEFAULT [];

-- Índice único por nombre
DEFINE INDEX idx_roles_nombre ON roles COLUMNS nombre UNIQUE;

-- ==========================================
-- TABLA: usuarios
-- ==========================================
DEFINE TABLE usuarios SCHEMAFULL
    PERMISSIONS
        FOR select WHERE true
        FOR create, update, delete WHERE $auth.role IN ['admin', 'supervisor'];

-- Campos principales
DEFINE FIELD id ON usuarios TYPE string;
DEFINE FIELD email ON usuarios TYPE string 
    ASSERT string::is::email($value);
DEFINE FIELD password ON usuarios TYPE string;
DEFINE FIELD nombre ON usuarios TYPE string;
DEFINE FIELD apellido ON usuarios TYPE string;
DEFINE FIELD role ON usuarios TYPE record<roles>;
DEFINE FIELD is_active ON usuarios TYPE bool DEFAULT true;
DEFINE FIELD created_at ON usuarios TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON usuarios TYPE datetime DEFAULT time::now();

-- Campos adicionales
DEFINE FIELD cedula ON usuarios TYPE string;
DEFINE FIELD segundo_nombre ON usuarios TYPE option<string>;
DEFINE FIELD segundo_apellido ON usuarios TYPE option<string>;
DEFINE FIELD fecha_inicio_labores ON usuarios TYPE option<datetime>;
DEFINE FIELD numero_gafete ON usuarios TYPE option<string>;
DEFINE FIELD fecha_nacimiento ON usuarios TYPE option<datetime>;
DEFINE FIELD telefono ON usuarios TYPE option<string>;
DEFINE FIELD direccion ON usuarios TYPE option<string>;
DEFINE FIELD contacto_emergencia_nombre ON usuarios TYPE option<string>;
DEFINE FIELD contacto_emergencia_telefono ON usuarios TYPE option<string>;
DEFINE FIELD must_change_password ON usuarios TYPE bool DEFAULT false;
DEFINE FIELD deleted_at ON usuarios TYPE option<datetime>;

-- Índices
DEFINE INDEX idx_usuarios_email ON usuarios COLUMNS email UNIQUE;
DEFINE INDEX idx_usuarios_cedula ON usuarios COLUMNS cedula UNIQUE;

-- ==========================================
-- EVENTOS (triggers automáticos)
-- ==========================================
DEFINE EVENT usuario_updated ON TABLE usuarios WHEN $event = "UPDATE" THEN (
    UPDATE $after.id SET updated_at = time::now()
);

-- ==========================================
-- DATOS INICIALES: Roles
-- ==========================================
-- Estos se ejecutan una sola vez al inicializar

-- CREATE roles:admin SET
--     id = 'admin',
--     nombre = 'Administrador',
--     descripcion = 'Control total del sistema',
--     permisos = ['all'];

-- CREATE roles:supervisor SET
--     id = 'supervisor',
--     nombre = 'Supervisor',
--     descripcion = 'Gestión de personal y reportes',
--     permisos = ['read', 'write', 'reports'];

-- CREATE roles:guardia SET
--     id = 'guardia',
--     nombre = 'Guardia',
--     descripcion = 'Operaciones de caseta',
--     permisos = ['read', 'write:ingresos'];
