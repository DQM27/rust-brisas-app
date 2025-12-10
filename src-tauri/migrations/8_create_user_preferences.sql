-- Tabla para guardar preferencias del usuario (ej: atajos de teclado)
CREATE TABLE IF NOT EXISTS user_preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    category TEXT NOT NULL, -- ej: 'shortcuts', 'theme', 'grid_layout'
    key TEXT NOT NULL,      -- ej: 'ingreso.save', 'dark_mode'
    value TEXT NOT NULL,    -- ej: 'Control+s', 'true', JSON
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, category, key)
);

-- Índices para búsqueda rápida
CREATE INDEX IF NOT EXISTS idx_preferences_user_category ON user_preferences(user_id, category);
