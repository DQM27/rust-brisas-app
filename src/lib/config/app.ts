// ============================================
// src/lib/config/app.ts
// ============================================
// Configuración centralizada de la aplicación
// Para cambiar nombre o versión, modifica solo aquí

export const APP_CONFIG = {
	name: 'Mega Brisas',
	version: 'v1.0.0-alpha',
	description: 'Sistema ERP de Control de Acceso',
	developer: '27Design',
	identifier: 'com.femprobrisas.mega-brisas'
} as const;

// Alias para compatibilidad
export const APP_NAME = APP_CONFIG.name;
export const APP_VERSION = APP_CONFIG.version;
