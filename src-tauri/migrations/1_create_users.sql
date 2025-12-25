-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    role_id TEXT NOT NULL DEFAULT 'role-guardia',  -- FK a roles
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    -- Nuevos campos agregados
    cedula TEXT NOT NULL DEFAULT '',
    segundo_nombre TEXT,
    segundo_apellido TEXT,
    fecha_inicio_labores TEXT,
    numero_gafete TEXT,
    fecha_nacimiento TEXT,
    telefono TEXT,
    direccion TEXT,
    contacto_emergencia_nombre TEXT,
    contacto_emergencia_telefono TEXT,
    must_change_password INTEGER NOT NULL DEFAULT 0,
    deleted_at TEXT DEFAULT NULL,
    avatar_path TEXT DEFAULT NULL,
    
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

-- Tabla de roles
CREATE TABLE IF NOT EXISTS roles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Tabla de permisos
CREATE TABLE IF NOT EXISTS permissions (
    id TEXT PRIMARY KEY,
    module TEXT NOT NULL,
    action TEXT NOT NULL,
    description TEXT,
    UNIQUE(module, action)
);

-- Relación role -> permissions
CREATE TABLE IF NOT EXISTS role_permissions (
    role_id TEXT NOT NULL,
    permission_id TEXT NOT NULL,
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role_id ON users(role_id);
CREATE INDEX IF NOT EXISTS idx_role_permissions_role ON role_permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_permissions_module ON permissions(module);