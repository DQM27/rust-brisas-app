-- ==========================================
-- Migration: Lista Negra
-- Fecha: 2025-11-14
-- Descripción: Tabla para control de accesos bloqueados
-- ==========================================

CREATE TABLE IF NOT EXISTS lista_negra (
    id TEXT PRIMARY KEY NOT NULL,
    contratista_id TEXT,
    cedula TEXT NOT NULL,
    nombre TEXT,
    apellido TEXT,
    motivo_bloqueo TEXT NOT NULL,
    fecha_inicio_bloqueo TEXT NOT NULL,
    fecha_fin_bloqueo TEXT,
    bloqueado_por TEXT NOT NULL,
    observaciones TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE SET NULL,
    UNIQUE(cedula, is_active) -- Solo un bloqueo activo por cédula
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_lista_negra_cedula ON lista_negra(cedula);
CREATE INDEX IF NOT EXISTS idx_lista_negra_is_active ON lista_negra(is_active);
CREATE INDEX IF NOT EXISTS idx_lista_negra_contratista ON lista_negra(contratista_id);
CREATE INDEX IF NOT EXISTS idx_lista_negra_activos ON lista_negra(cedula, is_active);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_lista_negra_timestamp 
AFTER UPDATE ON lista_negra
BEGIN
    UPDATE lista_negra SET updated_at = datetime('now') WHERE id = NEW.id;
END;