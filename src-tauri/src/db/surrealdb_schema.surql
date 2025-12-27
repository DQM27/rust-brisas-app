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
-- TABLA: permissions
-- ==========================================
DEFINE TABLE IF NOT EXISTS permissions;

-- Índice único por id (module:action)
DEFINE INDEX IF NOT EXISTS idx_permissions_id ON permissions COLUMNS id UNIQUE;

-- ==========================================
-- TABLA: role_permissions (relación N:M)
-- ==========================================
DEFINE TABLE IF NOT EXISTS role_permissions;

-- Índice compuesto para evitar duplicados
DEFINE INDEX IF NOT EXISTS idx_role_permissions ON role_permissions COLUMNS role_id, permission_id UNIQUE;

-- ==========================================
-- TABLA: usuarios
-- ==========================================
DEFINE TABLE IF NOT EXISTS usuarios;

-- Índices
DEFINE INDEX IF NOT EXISTS idx_usuarios_email ON usuarios COLUMNS email UNIQUE;
-- ==========================================
-- TABLA: empresas
-- ==========================================
DEFINE TABLE IF NOT EXISTS empresas;

-- Índices
DEFINE INDEX IF NOT EXISTS idx_empresas_nombre ON empresas COLUMNS nombre UNIQUE;
