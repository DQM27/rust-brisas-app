-- ==========================================
-- Migration: Catálogo de Proveedores
-- Fecha: 2025-12-13
-- Descripción: Tabla maestra de proveedores (separada de ingresos).
-- ==========================================

CREATE TABLE IF NOT EXISTS proveedores (
    id TEXT PRIMARY KEY,
    cedula TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    segundo_nombre TEXT,
    apellido TEXT NOT NULL,
    segundo_apellido TEXT,
    empresa_id TEXT NOT NULL,
    estado TEXT NOT NULL DEFAULT 'ACTIVO', -- ACTIVO, INACTIVO, SUSPENDIDO
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (empresa_id) REFERENCES empresas(id)
);

CREATE INDEX IF NOT EXISTS idx_proveedores_cedula ON proveedores(cedula);
CREATE INDEX IF NOT EXISTS idx_proveedores_empresa ON proveedores(empresa_id);
CREATE INDEX IF NOT EXISTS idx_proveedores_estado ON proveedores(estado);

-- Agregar columna proveedor_id a ingresos_proveedores
-- Nota: En SQLite, ALTER TABLE ADD COLUMN es limitado.
ALTER TABLE ingresos_proveedores ADD COLUMN proveedor_id TEXT REFERENCES proveedores(id);
CREATE INDEX IF NOT EXISTS idx_ingresos_proveedores_proveedor_id ON ingresos_proveedores(proveedor_id);
