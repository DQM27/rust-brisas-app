-- ==========================================
-- SurrealDB Schema for Brisas
-- ==========================================
-- Esquema flexible sin SCHEMAFULL para desarrollo

-- ==========================================
-- TABLA: role (roles del sistema)
-- ==========================================
DEFINE TABLE IF NOT EXISTS role;
DEFINE INDEX IF NOT EXISTS idx_role_name ON role COLUMNS name UNIQUE;

-- ==========================================
-- TABLA: user (usuarios)
-- ==========================================
DEFINE TABLE IF NOT EXISTS user;
DEFINE INDEX IF NOT EXISTS idx_user_email ON user COLUMNS email UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_user_cedula ON user COLUMNS cedula UNIQUE;

-- ==========================================
-- TABLA: empresa (empresas/compañías)
-- ==========================================
DEFINE TABLE IF NOT EXISTS empresa;
DEFINE INDEX IF NOT EXISTS idx_empresa_nombre ON empresa COLUMNS nombre UNIQUE;

-- ==========================================
-- TABLA: contratista (contratistas)
-- ==========================================
DEFINE TABLE IF NOT EXISTS contratista;
DEFINE INDEX IF NOT EXISTS idx_contratista_cedula ON contratista COLUMNS cedula UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_contratista_empresa ON contratista COLUMNS empresa_id;

-- ==========================================
-- TABLA: proveedor (proveedores)
-- ==========================================
DEFINE TABLE IF NOT EXISTS proveedor;
DEFINE INDEX IF NOT EXISTS idx_proveedor_cedula ON proveedor COLUMNS cedula UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_proveedor_empresa ON proveedor COLUMNS empresa_id;

-- ==========================================
-- TABLA: visitante (visitantes)
-- ==========================================
DEFINE TABLE IF NOT EXISTS visitante;
DEFINE INDEX IF NOT EXISTS idx_visitante_cedula ON visitante COLUMNS cedula UNIQUE;

-- ==========================================
-- TABLA: vehiculo (vehículos)
-- ==========================================
DEFINE TABLE IF NOT EXISTS vehiculo;
DEFINE INDEX IF NOT EXISTS idx_vehiculo_placa ON vehiculo COLUMNS placa UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_vehiculo_contratista ON vehiculo COLUMNS contratista_id;
DEFINE INDEX IF NOT EXISTS idx_vehiculo_proveedor ON vehiculo COLUMNS proveedor_id;

-- ==========================================
-- TABLA: gafete (gafetes/credenciales)
-- ==========================================
DEFINE TABLE IF NOT EXISTS gafete;
DEFINE INDEX IF NOT EXISTS idx_gafete_numero_tipo ON gafete COLUMNS numero, tipo UNIQUE;

-- ==========================================
-- TABLA: ingreso (ingresos unificados)
-- ==========================================
DEFINE TABLE IF NOT EXISTS ingreso;
DEFINE INDEX IF NOT EXISTS idx_ingreso_tipo ON ingreso COLUMNS tipo_ingreso;
DEFINE INDEX IF NOT EXISTS idx_ingreso_cedula ON ingreso COLUMNS cedula;
DEFINE INDEX IF NOT EXISTS idx_ingreso_fecha ON ingreso COLUMNS fecha_hora_ingreso;
DEFINE INDEX IF NOT EXISTS idx_ingreso_contratista ON ingreso COLUMNS contratista_id;
DEFINE INDEX IF NOT EXISTS idx_ingreso_proveedor ON ingreso COLUMNS proveedor_id;
DEFINE INDEX IF NOT EXISTS idx_ingreso_visitante ON ingreso COLUMNS visitante_id;

-- ==========================================
-- TABLA: cita (citas programadas)
-- ==========================================
DEFINE TABLE IF NOT EXISTS cita;
DEFINE INDEX IF NOT EXISTS idx_cita_visitante ON cita COLUMNS visitante_id;
DEFINE INDEX IF NOT EXISTS idx_cita_fecha ON cita COLUMNS fecha_cita;

-- ==========================================
-- TABLA: lista_negra (personas bloqueadas)
-- ==========================================
DEFINE TABLE IF NOT EXISTS lista_negra;
DEFINE INDEX IF NOT EXISTS idx_lista_negra_cedula ON lista_negra COLUMNS cedula UNIQUE;

-- ==========================================
-- TABLA: alerta_gafete (alertas por gafetes no devueltos)
-- ==========================================
DEFINE TABLE IF NOT EXISTS alerta_gafete;
DEFINE INDEX IF NOT EXISTS idx_alerta_gafete_cedula ON alerta_gafete COLUMNS cedula;
DEFINE INDEX IF NOT EXISTS idx_alerta_gafete_numero ON alerta_gafete COLUMNS gafete_numero;

-- ==========================================
-- TABLA: audit_praind (auditoría de cambios PRAIND)
-- ==========================================
DEFINE TABLE IF NOT EXISTS audit_praind;
DEFINE INDEX IF NOT EXISTS idx_audit_praind_contratista ON audit_praind COLUMNS contratista_id;

-- ==========================================
-- TABLA: audit_estado (auditoría de cambios de estado)
-- ==========================================
DEFINE TABLE IF NOT EXISTS audit_estado;
DEFINE INDEX IF NOT EXISTS idx_audit_estado_contratista ON audit_estado COLUMNS contratista_id;
