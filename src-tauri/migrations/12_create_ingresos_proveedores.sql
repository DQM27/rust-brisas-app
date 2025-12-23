-- ==========================================
-- Migration: Tabla Independiente Ingresos Proveedores
-- Fecha: 2025-12-13
-- Descripción: Tabla exclusiva para el flujo de proveedores.
-- Similar a contratistas pero sin PRAIND, vinculada a Empresas.
-- ==========================================

CREATE TABLE IF NOT EXISTS ingresos_proveedores (
    id TEXT PRIMARY KEY,
    
    -- Datos Personales (Snapshot, ya que no hay tabla 'proveedores' aun)
    cedula TEXT NOT NULL,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    
    -- Relación Empresa
    empresa_id TEXT NOT NULL,
    
    -- Datos del ingreso
    area_visitada TEXT NOT NULL,
    motivo TEXT NOT NULL,
    
    -- Gafete (FK compuesta a gafetes)
    gafete TEXT,
    gafete_tipo TEXT DEFAULT 'proveedor' CHECK (gafete_tipo IN ('contratista', 'proveedor', 'visita', 'otro')),
    
    -- Detalles operativos
    tipo_autorizacion TEXT, -- 'correo', 'pase'
    modo_ingreso TEXT,      -- 'vehiculo', 'caminando'
    placa_vehiculo TEXT,
    
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
    
    FOREIGN KEY (empresa_id) REFERENCES empresas(id),
    FOREIGN KEY (usuario_ingreso_id) REFERENCES users(id),
    FOREIGN KEY (usuario_salida_id) REFERENCES users(id),
    FOREIGN KEY (gafete, gafete_tipo) REFERENCES gafetes(numero, tipo)
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_ingresos_proveedores_empresa ON ingresos_proveedores(empresa_id);
CREATE INDEX IF NOT EXISTS idx_ingresos_proveedores_cedula ON ingresos_proveedores(cedula);
CREATE INDEX IF NOT EXISTS idx_ingresos_proveedores_estado ON ingresos_proveedores(estado);
CREATE INDEX IF NOT EXISTS idx_ingresos_proveedores_fecha ON ingresos_proveedores(fecha_ingreso);
CREATE INDEX IF NOT EXISTS idx_ingresos_proveedores_gafete ON ingresos_proveedores(gafete, gafete_tipo);
