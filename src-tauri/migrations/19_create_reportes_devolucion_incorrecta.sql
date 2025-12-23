-- ==========================================
-- Migration: Reportes de devolución incorrecta
-- ==========================================
-- Detecta fraudes vs errores de escritura en devolución de gafetes

CREATE TABLE IF NOT EXISTS reportes_devolucion_incorrecta (
    id TEXT PRIMARY KEY NOT NULL,
    ingreso_id TEXT NOT NULL,
    persona_id TEXT NOT NULL,
    persona_nombre TEXT NOT NULL,
    gafete_esperado TEXT NOT NULL,
    gafete_devuelto TEXT NOT NULL,
    tipo_incidente TEXT NOT NULL CHECK (tipo_incidente IN ('error_escritura', 'fraude')),
    gafete_dueno_real TEXT,      -- Si es fraude: nombre del dueño real
    gafete_dueno_id TEXT,         -- Si es fraude: ID del dueño real
    resolucion TEXT,
    fecha_incidente TEXT NOT NULL,
    guardia_id TEXT NOT NULL,
    supervisor_notificado INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (guardia_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_reportes_devolucion_fecha 
ON reportes_devolucion_incorrecta(fecha_incidente DESC);

CREATE INDEX IF NOT EXISTS idx_reportes_devolucion_tipo 
ON reportes_devolucion_incorrecta(tipo_incidente);
