-- ==========================================
-- MANUAL FIX: Insert System User
-- ==========================================
-- Run this SQL manually if migrations aren't running automatically
-- This creates the system user needed for foreign key constraints

-- First, check if the user already exists
SELECT id, email, nombre FROM users WHERE id = '00000000-0000-0000-0000-000000000000';

-- If not found, insert it:
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
    '$2b$12$placeholder',
    'guardia',
    1,
    datetime('now'),
    datetime('now')
);

-- Verify it was created:
SELECT id, email, nombre, apellido, role FROM users WHERE id = '00000000-0000-0000-0000-000000000000';

-- Also check your current logged-in user exists:
-- Replace 'YOUR_USER_ID' with your actual user ID from the auth store
-- SELECT id, email, nombre FROM users WHERE id = 'YOUR_USER_ID';
