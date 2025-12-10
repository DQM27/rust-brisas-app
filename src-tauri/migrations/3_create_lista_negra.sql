-- ==========================================
-- Migration: Lista Negra (FINAL y Corregido para máxima compatibilidad SQLite)
-- ==========================================

-- 1. Creación de la Tabla
CREATE TABLE IF NOT EXISTS lista_negra (
    id TEXT PRIMARY KEY NOT NULL,
    contratista_id TEXT,
    cedula TEXT NOT NULL,
    nombre TEXT,
    segundo_nombre TEXT,
    apellido TEXT,
    segundo_apellido TEXT,
    motivo_bloqueo TEXT NOT NULL,
    fecha_inicio_bloqueo TEXT NOT NULL,
    fecha_fin_bloqueo TEXT,
    bloqueado_por TEXT NOT NULL,
    observaciones TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE SET NULL
);

-- 2. Índice para optimizar el TRIGGER
-- Índice compuesto básico para búsquedas rápidas por cédula y estado.
CREATE INDEX IF NOT EXISTS idx_lista_negra_cedula_activos 
ON lista_negra(cedula, is_active);


-- 3. Trigger para forzar la unicidad (Máxima compatibilidad)
-- Este TRIGGER usa una subconsulta COUNT() simple en lugar de EXISTS y evita condiciones complejas.
CREATE TRIGGER IF NOT EXISTS unique_active_cedula
BEFORE INSERT ON lista_negra
WHEN NEW.is_active = 1 AND (
    SELECT COUNT(id) FROM lista_negra 
    WHERE cedula = NEW.cedula AND is_active = 1
) > 0
BEGIN
    SELECT RAISE(ABORT, 'UNIQUE constraint failed: Solo puede haber un bloqueo activo (is_active = 1) por cedula.');
END;

-- 4. Trigger de unicidad adicional para UPDATES (obligatorio si se usa el TRIGGER de arriba)
-- Maneja el caso en el que se intenta activar un registro existente cuando ya hay otro activo.
CREATE TRIGGER IF NOT EXISTS unique_active_cedula_update
BEFORE UPDATE ON lista_negra
WHEN NEW.is_active = 1 AND (
    SELECT COUNT(id) FROM lista_negra 
    WHERE cedula = NEW.cedula 
    AND is_active = 1
    -- Asegura que no se cuente el registro que se está actualizando (NEW.id)
    AND id != NEW.id 
) > 0
BEGIN
    SELECT RAISE(ABORT, 'UNIQUE constraint failed: Solo puede haber un bloqueo activo (is_active = 1) por cedula.');
END;


-- 5. Índices complementarios
CREATE INDEX IF NOT EXISTS idx_lista_negra_cedula ON lista_negra(cedula);
CREATE INDEX IF NOT EXISTS idx_lista_negra_contratista ON lista_negra(contratista_id);

-- 6. Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_lista_negra_timestamp 
AFTER UPDATE ON lista_negra
BEGIN
    UPDATE lista_negra SET updated_at = datetime('now') WHERE id = NEW.id;
END;