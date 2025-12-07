-- Migration to add new user fields
-- We use ALTER TABLE to add columns. SQLite supports ADD COLUMN.

ALTER TABLE users ADD COLUMN fecha_inicio_labores TEXT;
ALTER TABLE users ADD COLUMN numero_gafete TEXT;
ALTER TABLE users ADD COLUMN fecha_nacimiento TEXT;
ALTER TABLE users ADD COLUMN telefono TEXT;
ALTER TABLE users ADD COLUMN direccion TEXT;
ALTER TABLE users ADD COLUMN contacto_emergencia_nombre TEXT;
ALTER TABLE users ADD COLUMN contacto_emergencia_telefono TEXT;
