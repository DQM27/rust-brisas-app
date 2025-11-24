-- ==========================================
-- Migration: Blacklist Import Test Table
-- Tabla piloto para probar importación de Excel
-- ==========================================

-- 1. Creación de la Tabla de Prueba
CREATE TABLE IF NOT EXISTS blacklist_import_test (
    id TEXT PRIMARY KEY NOT NULL,
    cedula TEXT NOT NULL,
    
    -- Nombres estructurados (nuevo formato)
    primer_nombre TEXT NOT NULL,
    segundo_nombre TEXT,
    
    -- Apellidos estructurados (nuevo formato)
    primer_apellido TEXT NOT NULL,
    segundo_apellido TEXT,
    
    -- Nombre completo generado automáticamente
    nombre_completo TEXT GENERATED ALWAYS AS (
        primer_nombre || 
        COALESCE(' ' || segundo_nombre, '') || ' ' ||
        primer_apellido || 
        COALESCE(' ' || segundo_apellido, '')
    ) STORED,
    
    -- Datos del bloqueo
    empresa TEXT NOT NULL,
    motivo_bloqueo TEXT NOT NULL DEFAULT 'No especificado',
    fecha_inicio_bloqueo TEXT NOT NULL DEFAULT (date('now')),
    observaciones TEXT,
    
    -- Metadata de importación
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    imported_by TEXT NOT NULL,
    
    -- Campos de auditoría (compatibles con lista_negra final)
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 2. Índices para pruebas
CREATE INDEX IF NOT EXISTS idx_blacklist_test_cedula 
ON blacklist_import_test(cedula);

CREATE INDEX IF NOT EXISTS idx_blacklist_test_imported_at 
ON blacklist_import_test(imported_at);

-- 3. Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_blacklist_test_timestamp 
AFTER UPDATE ON blacklist_import_test
BEGIN
    UPDATE blacklist_import_test 
    SET updated_at = datetime('now') 
    WHERE id = NEW.id;
END;