-- ==========================================
-- Migration: Ingresos
-- Fecha: 2025-11-15
-- Descripción: Tabla para registro de ingresos y salidas
-- ==========================================

CREATE TABLE IF NOT EXISTS ingresos (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Persona (siempre obligatorios)
    contratista_id TEXT,
    cedula TEXT NOT NULL,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    empresa_nombre TEXT NOT NULL,
    
    -- Tipo
    tipo_ingreso TEXT NOT NULL CHECK(tipo_ingreso IN ('contratista', 'temporal')),
    tipo_autorizacion TEXT NOT NULL CHECK(tipo_autorizacion IN ('praind', 'correo')),
    
    -- Transporte
    modo_ingreso TEXT NOT NULL CHECK(modo_ingreso IN ('caminando', 'vehiculo', 'vehiculo_temporal')),
    vehiculo_id TEXT,
    placa_temporal TEXT,
    
    -- Gafete
    gafete_id TEXT NOT NULL,
    gafete_numero TEXT NOT NULL,
    
    -- Timestamps CRÍTICOS (formato: "YYYY-MM-DD HH:MM:SS")
    fecha_hora_ingreso TEXT NOT NULL,
    fecha_hora_salida TEXT,
    tiempo_permanencia_minutos INTEGER,
    
    -- Usuarios
    usuario_ingreso_id TEXT NOT NULL,
    usuario_salida_id TEXT,
    
    -- Snapshots (histórico - solo para contratistas)
    praind_vigente_al_ingreso INTEGER,
    estado_contratista_al_ingreso TEXT,
    
    -- Observaciones
    observaciones TEXT,
    
    -- Metadata
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id) ON DELETE SET NULL,
    FOREIGN KEY (vehiculo_id) REFERENCES vehiculos(id) ON DELETE SET NULL,
    FOREIGN KEY (gafete_id) REFERENCES gafetes(id),
    FOREIGN KEY (usuario_ingreso_id) REFERENCES users(id),
    FOREIGN KEY (usuario_salida_id) REFERENCES users(id)
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_ingresos_cedula ON ingresos(cedula);
CREATE INDEX IF NOT EXISTS idx_ingresos_contratista ON ingresos(contratista_id);
CREATE INDEX IF NOT EXISTS idx_ingresos_fecha_ingreso ON ingresos(fecha_hora_ingreso);
CREATE INDEX IF NOT EXISTS idx_ingresos_fecha_salida ON ingresos(fecha_hora_salida);
CREATE INDEX IF NOT EXISTS idx_ingresos_gafete ON ingresos(gafete_id);
CREATE INDEX IF NOT EXISTS idx_ingresos_estado ON ingresos(fecha_hora_salida);

-- Constraint crítico: Solo 1 ingreso abierto por cédula
CREATE UNIQUE INDEX idx_ingreso_unico_abierto 
ON ingresos(cedula) 
WHERE fecha_hora_salida IS NULL;

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER IF NOT EXISTS update_ingresos_timestamp 
AFTER UPDATE ON ingresos
BEGIN
    UPDATE ingresos SET updated_at = datetime('now') WHERE id = NEW.id;
END;