-- Add new name fields (optional) and cedula (required)
-- For cedula, we add a DEFAULT '' to ensure migration succeeds on existing rows.
ALTER TABLE users ADD COLUMN segundo_nombre TEXT;
ALTER TABLE users ADD COLUMN segundo_apellido TEXT;
ALTER TABLE users ADD COLUMN cedula TEXT NOT NULL DEFAULT '';
