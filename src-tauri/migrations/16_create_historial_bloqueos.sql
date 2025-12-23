-- ==========================================
-- Migration: Historial de bloqueos (Lista Negra)
-- ==========================================
-- Registra todas las acciones de bloqueo/desbloqueo

CREATE TABLE IF NOT EXISTS historial_bloqueos (
    id TEXT PRIMARY KEY NOT NULL,
    lista_negra_id TEXT NOT NULL,
    accion TEXT NOT NULL CHECK (accion IN ('bloqueado', 'desbloqueado')),
    usuario_id TEXT NOT NULL,
    motivo TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (lista_negra_id) REFERENCES lista_negra(id) ON DELETE CASCADE,
    FOREIGN KEY (usuario_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_historial_bloqueos_lista_negra 
ON historial_bloqueos(lista_negra_id);

CREATE INDEX IF NOT EXISTS idx_historial_bloqueos_fecha 
ON historial_bloqueos(created_at DESC);
