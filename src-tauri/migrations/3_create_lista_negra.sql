-- ==========================================
-- Migration: Lista Negra (REFACTORIZADO)
-- ==========================================
-- Bloqueo universal por CÉDULA: aplica a contratistas, proveedores y visitas
-- Si una cédula está bloqueada, NO puede ingresar NI ser registrada en el sistema

CREATE TABLE IF NOT EXISTS lista_negra (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Identificación de la persona (cédula es el identificador universal)
    cedula TEXT NOT NULL,
    nombre TEXT NOT NULL,
    segundo_nombre TEXT,
    apellido TEXT NOT NULL,
    segundo_apellido TEXT,
    
    -- Empresa (opcional - para personas manuales o vinculadas)
    empresa_id TEXT,                -- FK a empresas (si la empresa está registrada)
    empresa_nombre TEXT,            -- Nombre libre (si la empresa NO está registrada)
    
    -- Nivel de severidad (para feedback visual al guardia)
    -- ALTO = 🔴 Rojo = Peligroso, llamar supervisor
    -- MEDIO = 🟡 Amarillo = Precaución
    -- BAJO = 🟢 Verde/Gris = Trato normal, solo no dejar entrar
    nivel_severidad TEXT NOT NULL DEFAULT 'BAJO' CHECK (nivel_severidad IN ('ALTO', 'MEDIO', 'BAJO')),
    
    -- Motivo (solo visible para Admin/Supervisor, NO para guardias)
    motivo_bloqueo TEXT NOT NULL,
    
    -- Quién bloqueó
    bloqueado_por TEXT NOT NULL,
    
    -- Observaciones internas (solo Admin/Supervisor)
    observaciones TEXT,
    
    -- Estado: 1 = bloqueado activo, 0 = desbloqueado (historial implícito)
    is_active INTEGER NOT NULL DEFAULT 1,
    
    -- Auditoría
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    -- FKs opcionales
    FOREIGN KEY (empresa_id) REFERENCES empresas(id) ON DELETE SET NULL,
    FOREIGN KEY (bloqueado_por) REFERENCES users(id) ON DELETE SET NULL
);

-- ==========================================
-- Índices
-- ==========================================

-- Índice principal: búsqueda rápida por cédula + estado activo
CREATE INDEX IF NOT EXISTS idx_lista_negra_cedula_activo 
ON lista_negra(cedula, is_active);

-- Índice para búsqueda solo por cédula
CREATE INDEX IF NOT EXISTS idx_lista_negra_cedula 
ON lista_negra(cedula);

-- Índice por nivel de severidad (para reportes)
CREATE INDEX IF NOT EXISTS idx_lista_negra_nivel 
ON lista_negra(nivel_severidad, is_active);

-- Índice por empresa (para reportes)
CREATE INDEX IF NOT EXISTS idx_lista_negra_empresa 
ON lista_negra(empresa_id);

-- ==========================================
-- Triggers de Unicidad
-- ==========================================

-- Solo puede haber UN bloqueo activo por cédula
CREATE TRIGGER IF NOT EXISTS unique_active_cedula
BEFORE INSERT ON lista_negra
WHEN NEW.is_active = 1 AND (
    SELECT COUNT(id) FROM lista_negra 
    WHERE cedula = NEW.cedula AND is_active = 1
) > 0
BEGIN
    SELECT RAISE(ABORT, 'UNIQUE constraint failed: Solo puede haber un bloqueo activo por cédula.');
END;

-- Trigger para UPDATE (si se reactiva un bloqueo)
CREATE TRIGGER IF NOT EXISTS unique_active_cedula_update
BEFORE UPDATE ON lista_negra
WHEN NEW.is_active = 1 AND (
    SELECT COUNT(id) FROM lista_negra 
    WHERE cedula = NEW.cedula 
    AND is_active = 1
    AND id != NEW.id 
) > 0
BEGIN
    SELECT RAISE(ABORT, 'UNIQUE constraint failed: Ya existe un bloqueo activo para esta cédula.');
END;

-- Trigger para auto-actualizar updated_at
CREATE TRIGGER IF NOT EXISTS update_lista_negra_timestamp 
AFTER UPDATE ON lista_negra
BEGIN
    UPDATE lista_negra SET updated_at = datetime('now') WHERE id = NEW.id;
END;