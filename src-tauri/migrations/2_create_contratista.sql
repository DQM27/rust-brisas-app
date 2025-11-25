-- Crear tabla empresas primero (dependencia)
CREATE TABLE IF NOT EXISTS empresas (
    id TEXT PRIMARY KEY NOT NULL,
    nombre TEXT NOT NULL UNIQUE,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_empresas_nombre ON empresas(nombre);

-- Crear tabla contratistas con FK a empresas
CREATE TABLE IF NOT EXISTS contratistas (
    id TEXT PRIMARY KEY NOT NULL,
    cedula TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    segundo_nombre TEXT,
    apellido TEXT NOT NULL,
    segundo_apellido TEXT,
    empresa_id TEXT NOT NULL,
    fecha_vencimiento_praind TEXT NOT NULL,
    estado TEXT NOT NULL DEFAULT 'activo',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (empresa_id) REFERENCES empresas(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_contratistas_cedula ON contratistas(cedula);
CREATE INDEX IF NOT EXISTS idx_contratistas_empresa_id ON contratistas(empresa_id);
CREATE INDEX IF NOT EXISTS idx_contratistas_estado ON contratistas(estado);
CREATE INDEX IF NOT EXISTS idx_contratistas_fecha_vencimiento ON contratistas(fecha_vencimiento_praind);