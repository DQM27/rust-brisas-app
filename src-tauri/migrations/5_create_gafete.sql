-- ==========================================
-- Migration: Gafetes
-- Fecha: 2025-11-15
-- Descripción: Tabla para gafetes físicos
-- ==========================================

CREATE TABLE IF NOT EXISTS gafetes (
    id TEXT PRIMARY KEY NOT NULL,
    numero TEXT NOT NULL UNIQUE,
    estado TEXT NOT NULL DEFAULT 'disponible' CHECK(estado IN ('disponible', 'asignado', 'perdido')),
    contratista_asignado_id TEXT,
    ingreso_actual_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (contratista_asignado_id) REFERENCES contratistas(id) ON DELETE SET NULL
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_gafetes_numero ON gafetes(numero);
CREATE INDEX IF NOT EXISTS idx_gafetes_estado ON gafetes(estado);
CREATE INDEX IF NOT EXISTS idx_gafetes_contratista ON gafetes(contratista_asignado_id);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_gafetes_timestamp 
AFTER UPDATE ON gafetes
BEGIN
    UPDATE gafetes SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Insertar gafete especial "S/G" (Sin Gafete)
INSERT OR IGNORE INTO gafetes (id, numero, estado, created_at, updated_at)
VALUES (
    'sin-gafete-default',
    'S/G',
    'disponible',
    datetime('now'),
    datetime('now')
);