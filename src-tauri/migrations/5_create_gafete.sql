-- migrations/20251201_create_gafetes_final.sql

-- Tabla gafetes con clave compuesta (numero + tipo)
CREATE TABLE IF NOT EXISTS gafetes (
    numero TEXT NOT NULL,
    tipo TEXT NOT NULL CHECK (tipo IN ('contratista', 'proveedor', 'visita', 'otro')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (numero, tipo)
);

-- Índice útil
CREATE INDEX IF NOT EXISTS idx_gafetes_tipo ON gafetes(tipo);

-- Opcional: si quieres borrar datos viejos al recrear
-- DELETE FROM gafetes;