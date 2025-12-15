-- ==========================================
-- Migration: Vehiculos
-- Fecha: 2025-11-21
-- Descripción: Tabla para vehículos de contratistas
-- ==========================================

CREATE TABLE IF NOT EXISTS vehiculos (
    id TEXT PRIMARY KEY NOT NULL,
    contratista_id TEXT,
    proveedor_id TEXT,
    tipo_vehiculo TEXT NOT NULL CHECK(tipo_vehiculo IN ('motocicleta', 'automovil')),
    placa TEXT NOT NULL UNIQUE,
    marca TEXT,
    modelo TEXT,
    color TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE CASCADE,
    FOREIGN KEY (proveedor_id) REFERENCES proveedores(id) ON DELETE CASCADE,
    CHECK (contratista_id IS NOT NULL OR proveedor_id IS NOT NULL)
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_vehiculos_contratista ON vehiculos(contratista_id);
CREATE INDEX IF NOT EXISTS idx_vehiculos_placa ON vehiculos(placa);
CREATE INDEX IF NOT EXISTS idx_vehiculos_is_active ON vehiculos(is_active);
CREATE INDEX IF NOT EXISTS idx_vehiculos_tipo ON vehiculos(tipo_vehiculo);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_vehiculos_timestamp 
AFTER UPDATE ON vehiculos
BEGIN
    UPDATE vehiculos SET updated_at = datetime('now') WHERE id = NEW.id;
END;