-- ==========================================
-- Migración: Tabla de Ingresos
-- ==========================================
-- Registro de entradas y salidas de contratistas

CREATE TABLE IF NOT EXISTS ingresos (
    id TEXT PRIMARY KEY,
    
    -- Contratista
    contratista_id TEXT NOT NULL,
    cedula TEXT NOT NULL,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    empresa_nombre TEXT NOT NULL,
    
    -- Tipo de ingreso (solo contratistas)
    tipo_ingreso TEXT NOT NULL DEFAULT 'contratista' CHECK (tipo_ingreso = 'contratista'),
    tipo_autorizacion TEXT NOT NULL CHECK (tipo_autorizacion IN ('praind', 'correo')),
    modo_ingreso TEXT NOT NULL CHECK (modo_ingreso IN ('caminando', 'vehiculo')),
    
    -- Vehículo
    vehiculo_id TEXT,
    placa_temporal TEXT,  -- Mantener para compatibilidad futura
    
    -- Gafete (NULL = sin gafete)
    gafete_numero TEXT,
    
    -- Tiempos
    fecha_hora_ingreso TEXT NOT NULL,
    fecha_hora_salida TEXT,
    tiempo_permanencia_minutos INTEGER,
    
    -- Usuarios (guardias)
    usuario_ingreso_id TEXT NOT NULL,
    usuario_salida_id TEXT,
    
    -- Snapshot de validaciones al momento del ingreso
    praind_vigente_al_ingreso INTEGER,  -- 0/1 (bool)
    estado_contratista_al_ingreso TEXT,
    
    -- Observaciones
    observaciones TEXT,
    
    -- Auditoría
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (contratista_id) REFERENCES contratistas(id),
    FOREIGN KEY (vehiculo_id) REFERENCES vehiculos(id),
    FOREIGN KEY (usuario_ingreso_id) REFERENCES users(id),
    FOREIGN KEY (usuario_salida_id) REFERENCES users(id)
);

-- ==========================================
-- Índices para optimizar queries frecuentes
-- ==========================================

-- Buscar por contratista (verificar ingreso abierto)
CREATE INDEX IF NOT EXISTS idx_ingresos_contratista ON ingresos(contratista_id, fecha_hora_salida);

-- Buscar por cédula
CREATE INDEX IF NOT EXISTS idx_ingresos_cedula ON ingresos(cedula);

-- Listar ingresos abiertos (personas adentro)
CREATE INDEX IF NOT EXISTS idx_ingresos_abiertos ON ingresos(fecha_hora_salida) 
WHERE fecha_hora_salida IS NULL;

-- Buscar ingreso por gafete (para registrar salida)
CREATE INDEX IF NOT EXISTS idx_ingresos_gafete ON ingresos(gafete_numero, fecha_hora_salida);

-- Listar por fecha (reportes)
CREATE INDEX IF NOT EXISTS idx_ingresos_fecha ON ingresos(fecha_hora_ingreso DESC);