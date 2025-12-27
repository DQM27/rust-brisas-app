-- ==========================================
-- SurrealDB Schema for Brisas (Simplified)
-- ==========================================
-- Esquema flexible sin SCHEMAFULL para desarrollo

-- ==========================================
-- TABLA: roles
-- ==========================================
DEFINE TABLE IF NOT EXISTS roles;

-- Índice único por nombre
DEFINE INDEX IF NOT EXISTS idx_roles_name ON roles COLUMNS name UNIQUE;

-- ==========================================
-- TABLA: usuarios
-- ==========================================
DEFINE TABLE IF NOT EXISTS usuarios;

-- Índices
DEFINE INDEX IF NOT EXISTS idx_usuarios_email ON usuarios COLUMNS email UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_usuarios_cedula ON usuarios COLUMNS cedula UNIQUE;
