-- ==========================================
-- Migration: Gafetes Perdidos
-- Fecha: 2025-11-15
-- Descripción: Tabla para registro de gafetes perdidos y deudas
-- ==========================================

CREATE TABLE IF NOT EXISTS gafetes_perdidos (
    id TEXT PRIMARY KEY NOT NULL,
    gafete_id TEXT NOT NULL,
    contratista_id TEXT NOT NULL,
    ingreso_id TEXT,
    fecha_perdida TEXT NOT NULL,
    monto_cobro REAL NOT NULL,
    estado_pago TEXT NOT NULL DEFAULT 'pendiente' CHECK(estado_pago IN ('pendiente', 'pagado', 'condonado')),
    fecha_pago TEXT,
    observaciones TEXT,
    reportado_por TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (gafete_id) REFERENCES gafetes(id) ON DELETE CASCADE,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE CASCADE,
    FOREIGN KEY (reportado_por) REFERENCES users(id)
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_gafetes_perdidos_gafete ON gafetes_perdidos(gafete_id);
CREATE INDEX IF NOT EXISTS idx_gafetes_perdidos_contratista ON gafetes_perdidos(contratista_id);
CREATE INDEX IF NOT EXISTS idx_gafetes_perdidos_estado_pago ON gafetes_perdidos(estado_pago);
CREATE INDEX IF NOT EXISTS idx_gafetes_perdidos_fecha ON gafetes_perdidos(fecha_perdida);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_gafetes_perdidos_timestamp 
AFTER UPDATE ON gafetes_perdidos
BEGIN
    UPDATE gafetes_perdidos SET updated_at = datetime('now') WHERE id = NEW.id;
END;