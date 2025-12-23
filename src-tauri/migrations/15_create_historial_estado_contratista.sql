-- ==========================================
-- Migration: Historial de cambios de estado contratista
-- ==========================================
-- Registra transiciones de estado (Activo, Inactivo, Suspendido)

CREATE TABLE IF NOT EXISTS historial_estado_contratista (
    id TEXT PRIMARY KEY NOT NULL,
    contratista_id TEXT NOT NULL,
    estado_anterior TEXT NOT NULL,
    estado_nuevo TEXT NOT NULL,
    cambiado_por TEXT,  -- NULL cuando es cambio automático del sistema
    motivo TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE CASCADE,
    FOREIGN KEY (cambiado_por) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_historial_estado_contratista_id 
ON historial_estado_contratista(contratista_id);

CREATE INDEX IF NOT EXISTS idx_historial_estado_fecha 
ON historial_estado_contratista(created_at DESC);
