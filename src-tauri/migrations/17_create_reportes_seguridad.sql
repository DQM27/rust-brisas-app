-- ==========================================
-- Migration: Reportes de Seguridad
-- ==========================================
-- Eventos que requieren atención de seguridad

CREATE TABLE IF NOT EXISTS reportes_seguridad (
    id TEXT PRIMARY KEY NOT NULL,
    tipo TEXT NOT NULL CHECK (tipo IN (
        'ingreso_duplicado',
        'tiempo_excedido', 
        'cierre_manual', 
        'gafete_no_devuelto', 
        'ingreso_excepcional'
    )),
    contratista_id TEXT,
    ingreso_id TEXT,
    descripcion TEXT NOT NULL,
    generado_por TEXT,  -- NULL = sistema automático
    resolucion TEXT,
    resuelto_por TEXT,
    fecha_resolucion TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE SET NULL,
    FOREIGN KEY (ingreso_id) REFERENCES ingresos(id) ON DELETE SET NULL,
    FOREIGN KEY (generado_por) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (resuelto_por) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_reportes_seguridad_tipo 
ON reportes_seguridad(tipo);

CREATE INDEX IF NOT EXISTS idx_reportes_seguridad_contratista 
ON reportes_seguridad(contratista_id);

CREATE INDEX IF NOT EXISTS idx_reportes_seguridad_sin_resolver 
ON reportes_seguridad(resolucion) WHERE resolucion IS NULL;

CREATE INDEX IF NOT EXISTS idx_reportes_seguridad_fecha 
ON reportes_seguridad(created_at DESC);
