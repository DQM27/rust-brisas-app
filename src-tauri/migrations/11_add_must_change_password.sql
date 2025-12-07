-- Add must_change_password column (boolean, default true for new migrations but false for existing to avoid blocking)
-- We use 0/1 for boolean in SQLite
ALTER TABLE users ADD COLUMN must_change_password INTEGER NOT NULL DEFAULT 0;
