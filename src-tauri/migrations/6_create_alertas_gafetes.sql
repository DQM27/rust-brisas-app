-- ==========================================
-- Migración: Tabla de Alertas de Gafetes (REFACTORIZADA)
-- ==========================================
-- Registro de gafetes no devueltos con soporte Polimórfico (Contratistas, Proveedores y Visitas)

CREATE TABLE IF NOT EXISTS alertas_gafetes (
    id TEXT PRIMARY KEY,
    
    -- Persona responsable
    persona_id TEXT,             -- ID opcional del contratista (si aplica)
    cedula TEXT NOT NULL,
    nombre_completo TEXT NOT NULL,
    
    -- Gafete no devuelto
    gafete_numero TEXT NOT NULL,
    
    -- VINCULACIÓN POLIMÓRFICA (uno de los tres debe estar presente)
    ingreso_contratista_id TEXT,  -- FK a la tabla 'ingresos_contratistas'
    ingreso_proveedor_id TEXT,    -- FK a la tabla 'ingresos_proveedores'
    ingreso_visita_id TEXT,       -- FK a la tabla 'ingresos_visitas' (NEW)
    
    -- Estado de la alerta
    fecha_reporte TEXT NOT NULL,
    resuelto INTEGER DEFAULT 0,  -- 0 = pendiente, 1 = resuelto
    fecha_resolucion TEXT,
    resuelto_por TEXT,           -- ID del usuario que resolvió

    -- Observaciones
    notas TEXT,
    
    -- Auditoría
    reportado_por TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    -- Restricción: Debe venir de algún tipo de ingreso
    CHECK (ingreso_contratista_id IS NOT NULL OR ingreso_proveedor_id IS NOT NULL OR ingreso_visita_id IS NOT NULL),

    -- Foreign Keys
    FOREIGN KEY (ingreso_contratista_id) REFERENCES ingresos_contratistas(id),
    FOREIGN KEY (ingreso_proveedor_id) REFERENCES ingresos_proveedores(id),
    FOREIGN KEY (ingreso_visita_id) REFERENCES ingresos_visitas(id),
    FOREIGN KEY (reportado_por) REFERENCES users(id),
    FOREIGN KEY (resuelto_por) REFERENCES users(id)
);

-- Indices
CREATE INDEX IF NOT EXISTS idx_alertas_cedula ON alertas_gafetes(cedula, resuelto);
CREATE INDEX IF NOT EXISTS idx_alertas_gafete ON alertas_gafetes(gafete_numero);
CREATE INDEX IF NOT EXISTS idx_alertas_ingreso_c ON alertas_gafetes(ingreso_contratista_id);
CREATE INDEX IF NOT EXISTS idx_alertas_ingreso_p ON alertas_gafetes(ingreso_proveedor_id);
CREATE INDEX IF NOT EXISTS idx_alertas_ingreso_v ON alertas_gafetes(ingreso_visita_id);