// ==========================================
// Migración SQL
// ==========================================

pub const CONTRATISTAS_MIGRATION: &str = r#"
CREATE TABLE IF NOT EXISTS contratistas (
    id TEXT PRIMARY KEY NOT NULL,
    cedula TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    empresa TEXT NOT NULL,
    fecha_vencimiento_praind TEXT NOT NULL,
    estado TEXT NOT NULL DEFAULT 'activo',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_contratistas_cedula ON contratistas(cedula);
CREATE INDEX IF NOT EXISTS idx_contratistas_estado ON contratistas(estado);
CREATE INDEX IF NOT EXISTS idx_contratistas_fecha_vencimiento ON contratistas(fecha_vencimiento_praind);
"#;