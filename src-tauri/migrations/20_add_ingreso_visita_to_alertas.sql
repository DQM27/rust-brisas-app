-- Add ingreso_visita_id column to alertas_gafetes table
ALTER TABLE alertas_gafetes ADD COLUMN ingreso_visita_id TEXT REFERENCES ingresos_visitas(id);

-- Create index for the new column
CREATE INDEX IF NOT EXISTS idx_alertas_ingreso_v ON alertas_gafetes(ingreso_visita_id);
