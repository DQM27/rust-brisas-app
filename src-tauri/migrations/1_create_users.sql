-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'guardia',
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
    deleted_at TEXT DEFAULT NULL
);

-- Create index for email lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Create index for role
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);