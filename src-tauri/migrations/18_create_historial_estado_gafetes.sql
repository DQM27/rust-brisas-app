-- ==========================================
-- Migration: Historial de estados de gafetes
-- ==========================================
-- Registra cambios de estado: Activo, Dañado, Extraviado, Desactivado

CREATE TABLE IF NOT EXISTS historial_estado_gafetes (
    id TEXT PRIMARY KEY NOT NULL,
    gafete_numero TEXT NOT NULL,
    gafete_tipo TEXT NOT NULL,
    estado_anterior TEXT NOT NULL,
    estado_nuevo TEXT NOT NULL,
    cambiado_por TEXT NOT NULL,
    motivo TEXT,
    fecha_cambio TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (cambiado_por) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_historial_gafetes_numero_tipo 
ON historial_estado_gafetes(gafete_numero, gafete_tipo);

CREATE INDEX IF NOT EXISTS idx_historial_gafetes_fecha 
ON historial_estado_gafetes(fecha_cambio DESC);
