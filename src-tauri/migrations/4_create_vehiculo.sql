-- ==========================================
-- Migration: Vehiculos
-- Fecha: 2025-11-15
-- Descripción: Tabla para vehículos de contratistas
-- ==========================================

CREATE TABLE IF NOT EXISTS vehiculos (
    id TEXT PRIMARY KEY NOT NULL,
    contratista_id TEXT NOT NULL,
    placa TEXT NOT NULL UNIQUE,
    marca TEXT,
    modelo TEXT,
    color TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE CASCADE
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_vehiculos_contratista ON vehiculos(contratista_id);
CREATE INDEX IF NOT EXISTS idx_vehiculos_placa ON vehiculos(placa);
CREATE INDEX IF NOT EXISTS idx_vehiculos_is_active ON vehiculos(is_active);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_vehiculos_timestamp 
AFTER UPDATE ON vehiculos
BEGIN
    UPDATE vehiculos SET updated_at = datetime('now') WHERE id = NEW.id;
END;