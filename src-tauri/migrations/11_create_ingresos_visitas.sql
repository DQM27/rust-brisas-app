-- ==========================================
-- Migration: Tabla Independiente Ingresos Visitas
-- Fecha: 2025-12-13
-- Descripción: Tabla exclusiva para el flujo de visitas, separada de contratistas.
-- ==========================================

CREATE TABLE IF NOT EXISTS ingresos_visitas (
    id TEXT PRIMARY KEY,
    
    -- Relaciones
    visitante_id TEXT NOT NULL,
    cita_id TEXT, -- Puede ser NULL si llega sin cita (registro espontáneo)
    
    -- Datos del ingreso
    anfitrion TEXT NOT NULL,
    area_visitada TEXT NOT NULL,
    motivo TEXT NOT NULL,
    
    -- Gafete (FK compuesta a gafetes)
    gafete TEXT,
    gafete_tipo TEXT DEFAULT 'visita' CHECK (gafete_tipo IN ('contratista', 'proveedor', 'visita', 'otro')),
    razon_sin_gafete TEXT, -- Motivo si es "S/G"
    
    -- Tiempos y Estado
    fecha_ingreso DATETIME NOT NULL,
    fecha_salida DATETIME,
    estado TEXT NOT NULL DEFAULT 'ADENTRO', -- ADENTRO, SALIO
    
    -- Auditoría
    usuario_ingreso_id TEXT NOT NULL,
    usuario_salida_id TEXT,
    observaciones TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (visitante_id) REFERENCES visitantes(id),
    FOREIGN KEY (cita_id) REFERENCES citas(id),
    FOREIGN KEY (usuario_ingreso_id) REFERENCES users(id),
    FOREIGN KEY (usuario_salida_id) REFERENCES users(id),
    FOREIGN KEY (gafete, gafete_tipo) REFERENCES gafetes(numero, tipo)
);

-- Índices para búsquedas rápidas
CREATE INDEX IF NOT EXISTS idx_ingresos_visitas_visitante ON ingresos_visitas(visitante_id);
CREATE INDEX IF NOT EXISTS idx_ingresos_visitas_estado ON ingresos_visitas(estado);
CREATE INDEX IF NOT EXISTS idx_ingresos_visitas_fecha ON ingresos_visitas(fecha_ingreso);
CREATE INDEX IF NOT EXISTS idx_ingresos_visitas_gafete ON ingresos_visitas(gafete, gafete_tipo);
