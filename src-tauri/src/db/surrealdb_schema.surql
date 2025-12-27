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

-- ==========================================
-- TABLA: contratistas
-- ==========================================
DEFINE TABLE IF NOT EXISTS contratistas;

-- Índices
DEFINE INDEX IF NOT EXISTS idx_contratistas_cedula ON contratistas COLUMNS cedula UNIQUE;
-- ==========================================
-- TABLA: audit_praind
-- ==========================================
DEFINE TABLE IF NOT EXISTS audit_praind;
DEFINE INDEX IF NOT EXISTS idx_audit_praind_contratista ON audit_praind COLUMNS contratista;

-- ==========================================
-- TABLA: audit_estado
-- ==========================================
DEFINE TABLE IF NOT EXISTS audit_estado;
DEFINE INDEX IF NOT EXISTS idx_audit_estado_contratista ON audit_estado COLUMNS contratista;

-- ==========================================
-- TABLA: visitantes
-- ==========================================
DEFINE TABLE IF NOT EXISTS visitantes;

-- Índice único por cédula
DEFINE INDEX IF NOT EXISTS idx_visitantes_cedula ON visitantes COLUMNS cedula UNIQUE;

-- ==========================================
-- TABLA: vehiculos
-- ==========================================
DEFINE TABLE IF NOT EXISTS vehiculos;

-- Índice único por placa
DEFINE INDEX IF NOT EXISTS idx_vehiculos_placa ON vehiculos COLUMNS placa UNIQUE;

-- Índices para relaciones (record links)
DEFINE INDEX IF NOT EXISTS idx_vehiculos_contratista ON vehiculos COLUMNS contratista_id;
DEFINE INDEX IF NOT EXISTS idx_vehiculos_visitante ON vehiculos COLUMNS visitante_id;
