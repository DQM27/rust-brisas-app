-- ==========================================
-- Migración: Módulo de Pre-registro (Citas)
-- ==========================================

-- 1. Tabla: Visitantes (Catálogo de personas frecuentes o esporádicas)
-- Similar a Contratistas, para evitar re-escribir datos.
CREATE TABLE IF NOT EXISTS visitantes (
    id TEXT PRIMARY KEY,
    cedula TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    segundo_nombre TEXT,
    segundo_apellido TEXT,
    empresa TEXT,              -- String libre, no FK a empresas (amenos que se requiera estricto)
    has_vehicle INTEGER DEFAULT 0, -- Booleano para saber si pre-cargar vehículo
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Índices para búsqueda rápida de visitantes
CREATE INDEX IF NOT EXISTS idx_visitantes_cedula ON visitantes(cedula);
CREATE INDEX IF NOT EXISTS idx_visitantes_nombre ON visitantes(nombre, apellido);

-- 2. Tabla: Citas (Pre-registros)
-- Representa una intención de visita en una fecha específica.
CREATE TABLE IF NOT EXISTS citas (
    id TEXT PRIMARY KEY,
    
    -- Relación con Visitante
    visitante_id TEXT NOT NULL,
    
    -- Detalles de la visita (lo que se sabe de antemano)
    fecha_cita TEXT NOT NULL,  -- Fecha y hora aproximada
    anfitrion TEXT NOT NULL,
    area_visitada TEXT NOT NULL,
    motivo TEXT NOT NULL,
    
    -- Estado del flujo
    estado TEXT NOT NULL DEFAULT 'PENDIENTE' CHECK (estado IN ('PENDIENTE', 'COMPLETADA', 'CANCELADA', 'EXPIRADA')),
    
    -- Auditoría
    registrado_por TEXT NOT NULL, -- Usuario que creó la cita (recepción/admin)
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (visitante_id) REFERENCES visitantes(id),
    FOREIGN KEY (registrado_por) REFERENCES users(id)
);

-- Índices para citas
CREATE INDEX IF NOT EXISTS idx_citas_fecha ON citas(fecha_cita);
CREATE INDEX IF NOT EXISTS idx_citas_pendientes ON citas(estado) WHERE estado = 'PENDIENTE';
CREATE INDEX IF NOT EXISTS idx_citas_visitante ON citas(visitante_id);
