-- ==========================================
-- Migration: Insert System User for Testing
-- ==========================================
-- This user is used as a placeholder until proper authentication is implemented
-- The zero-UUID is used in frontend forms for usuario_ingreso_id and usuario_salida_id

INSERT OR IGNORE INTO users (
    id, 
    email, 
    nombre, 
    apellido, 
    password_hash,
    role, 
    is_active,
    created_at, 
    updated_at
)
VALUES (
    '00000000-0000-0000-0000-000000000000',
    'system@brisas.local',
    'Sistema',
    'Temporal',
    '$2b$12$placeholder.hash.for.system.user.only',  -- Placeholder hash (won't be used for login)
    'guardia',
    1,
    datetime('now'),
    datetime('now')
);
