-- ==========================================
-- Migration: Soporte multi-tipo de ingresos
-- Fecha: 2025-12-13
-- Descripción: Agrega campos para soportar Visitas y Proveedores
-- ==========================================

-- Agregar columnas para VISITAS
ALTER TABLE ingresos ADD COLUMN anfitrion TEXT NULL;
ALTER TABLE ingresos ADD COLUMN motivo_visita TEXT NULL;

-- Agregar columna compartida: área visitada (Visitas y Proveedores)
ALTER TABLE ingresos ADD COLUMN area_visitada TEXT NULL;

-- Agregar columnas para PROVEEDORES
ALTER TABLE ingresos ADD COLUMN empresa_proveedor_id TEXT NULL;
ALTER TABLE ingresos ADD COLUMN motivo_proveedor TEXT NULL;

-- Agregar constraint de foreign key para empresa proveedora
-- NOTA: SQLite no soporta ADD CONSTRAINT después de CREATE TABLE
-- Pero podemos crear un índice para mejorar el rendimiento
CREATE INDEX IF NOT EXISTS idx_ingresos_empresa_proveedor
ON ingresos(empresa_proveedor_id);

-- Crear índices para mejorar búsquedas
CREATE INDEX IF NOT EXISTS idx_ingresos_tipo
ON ingresos(tipo_ingreso);

CREATE INDEX IF NOT EXISTS idx_ingresos_anfitrion
ON ingresos(anfitrion)
WHERE anfitrion IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ingresos_area
ON ingresos(area_visitada)
WHERE area_visitada IS NOT NULL;
