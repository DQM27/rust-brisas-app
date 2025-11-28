-- ==========================================
-- Migración: Tabla de Gafetes
-- ==========================================
-- Inventario físico de gafetes

CREATE TABLE IF NOT EXISTS gafetes (
    numero TEXT PRIMARY KEY,
    tipo TEXT NOT NULL CHECK (tipo IN ('contratista', 'proveedor', 'visita', 'otro')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Índice para búsquedas por tipo
CREATE INDEX IF NOT EXISTS idx_gafetes_tipo ON gafetes(tipo);

-- ==========================================
-- Datos iniciales (opcional)
-- ==========================================
-- Puedes descomentar esto para crear algunos gafetes de prueba

-- INSERT INTO gafetes (numero, tipo, created_at, updated_at) VALUES
-- ('001', 'contratista', datetime('now'), datetime('now')),
-- ('002', 'contratista', datetime('now'), datetime('now')),
-- ('003', 'contratista', datetime('now'), datetime('now')),
-- ('P-001', 'proveedor', datetime('now'), datetime('now')),
-- ('P-002', 'proveedor', datetime('now'), datetime('now')),
-- ('V-001', 'visita', datetime('now'), datetime('now')),
-- ('V-002', 'visita', datetime('now'), datetime('now'));