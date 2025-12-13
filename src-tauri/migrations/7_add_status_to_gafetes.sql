-- migrations/7_add_status_to_gafetes.sql

-- Añadir columna estado a la tabla gafetes
-- Valores esperados: 'activo', 'danado', 'extraviado'
-- Default: 'activo' (para los existentes)

ALTER TABLE gafetes ADD COLUMN estado TEXT NOT NULL DEFAULT 'activo';
