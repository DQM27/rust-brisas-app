-- ==========================================
-- Migration: Modificaciones para visitantes
-- ==========================================
-- Unificar lista negra y alertas de gafete para ambos tipos

-- Agregar tipo_persona a lista_negra si no existe
-- SQLite no soporta ADD COLUMN IF NOT EXISTS, usar try/catch pattern
-- Esta migración asume que tipo_persona no existe

ALTER TABLE lista_negra ADD COLUMN tipo_persona TEXT 
    DEFAULT 'contratista';

-- Agregar ingreso_visita_id a alertas_gafetes
ALTER TABLE alertas_gafetes ADD COLUMN ingreso_visita_id TEXT 
    REFERENCES ingresos_visitas(id);

-- Agregar campos de gafete a ingresos_visitas
ALTER TABLE ingresos_visitas ADD COLUMN gafete_tipo TEXT DEFAULT 'visita';
ALTER TABLE ingresos_visitas ADD COLUMN razon_sin_gafete TEXT;

-- Índices para búsquedas rápidas
CREATE INDEX IF NOT EXISTS idx_lista_negra_tipo_persona 
ON lista_negra(tipo_persona);

CREATE INDEX IF NOT EXISTS idx_alertas_gafetes_visita 
ON alertas_gafetes(ingreso_visita_id);
