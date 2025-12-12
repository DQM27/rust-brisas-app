-- ==========================================
-- Migration: Reportes de Errores/Sugerencias
-- ==========================================

-- 1. Creacion de la Tabla
CREATE TABLE IF NOT EXISTS reportes (
    id TEXT PRIMARY KEY NOT NULL,
    tipo TEXT NOT NULL,  -- 'error', 'sugerencia', 'mejora'
    asunto TEXT NOT NULL,
    mensaje TEXT NOT NULL,
    contacto TEXT,
    tiene_adjunto INTEGER NOT NULL DEFAULT 0,
    nombre_adjunto TEXT,
    estado TEXT NOT NULL DEFAULT 'pendiente',  -- 'pendiente', 'enviado', 'fallido'
    error_envio TEXT,
    enviado_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 2. Indices
CREATE INDEX IF NOT EXISTS idx_reportes_tipo ON reportes(tipo);
CREATE INDEX IF NOT EXISTS idx_reportes_estado ON reportes(estado);
CREATE INDEX IF NOT EXISTS idx_reportes_created ON reportes(created_at);

-- 3. Trigger para actualizar updated_at automaticamente
CREATE TRIGGER IF NOT EXISTS update_reportes_timestamp
AFTER UPDATE ON reportes
BEGIN
    UPDATE reportes SET updated_at = datetime('now') WHERE id = NEW.id;
END;
