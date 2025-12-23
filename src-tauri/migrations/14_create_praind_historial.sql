-- ==========================================
-- Migration: Historial de cambios PRAIND
-- ==========================================
-- Registra cada actualización de fecha de vencimiento PRAIND

CREATE TABLE IF NOT EXISTS praind_historial (
    id TEXT PRIMARY KEY NOT NULL,
    contratista_id TEXT NOT NULL,
    fecha_anterior TEXT,
    fecha_nueva TEXT NOT NULL,
    actualizado_por TEXT NOT NULL,
    motivo TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE CASCADE,
    FOREIGN KEY (actualizado_por) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_praind_historial_contratista 
ON praind_historial(contratista_id);

CREATE INDEX IF NOT EXISTS idx_praind_historial_fecha 
ON praind_historial(created_at DESC);
