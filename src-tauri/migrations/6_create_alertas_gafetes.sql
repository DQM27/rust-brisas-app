-- ==========================================
-- Migración: Tabla de Alertas de Gafetes
-- ==========================================
-- Registro de gafetes no devueltos (antes "gafetes_perdidos")

CREATE TABLE IF NOT EXISTS alertas_gafetes (
    id TEXT PRIMARY KEY,
    
    -- Persona responsable
    persona_id TEXT,  -- contratista_id si existe, NULL si es temporal
    cedula TEXT NOT NULL,
    nombre_completo TEXT NOT NULL,
    
    -- Gafete no devuelto
    gafete_numero TEXT NOT NULL,
    ingreso_id TEXT NOT NULL,
    
    -- Estado de la alerta
    fecha_reporte TEXT NOT NULL,
    resuelto INTEGER DEFAULT 0,  -- 0 = pendiente, 1 = resuelto
    fecha_resolucion TEXT,
    
    -- Observaciones
    notas TEXT,
    
    -- Auditoría
    reportado_por TEXT NOT NULL,  -- ID del usuario (guardia)
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (ingreso_id) REFERENCES ingresos(id),
    FOREIGN KEY (reportado_por) REFERENCES users(id)
);

-- ==========================================
-- Índices
-- ==========================================

-- Buscar alertas pendientes de una persona
CREATE INDEX IF NOT EXISTS idx_alertas_pendientes ON alertas_gafetes(cedula, resuelto);

-- Listar por fecha de reporte
CREATE INDEX IF NOT EXISTS idx_alertas_fecha ON alertas_gafetes(fecha_reporte DESC);

-- Buscar por gafete
CREATE INDEX IF NOT EXISTS idx_alertas_gafete ON alertas_gafetes(gafete_numero);

-- ==========================================
-- Datos de prueba (opcional - descomentar si necesitas)
-- ==========================================

-- INSERT INTO alertas_gafetes (
--     id, persona_id, cedula, nombre_completo, gafete_numero, ingreso_id,
--     fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
--     created_at, updated_at
-- ) VALUES (
--     '660e8400-e29b-41d4-a716-446655440000',
--     'contratista-uuid-aqui',
--     '123456789',
--     'Juan Pérez',
--     '027',
--     'ingreso-uuid-aqui',
--     datetime('now'),
--     0,
--     NULL,
--     'No devolvió gafete al momento de la salida',
--     'usuario-uuid-aqui',
--     datetime('now'),
--     datetime('now')
-- );