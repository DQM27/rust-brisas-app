// src/lib/stores/ui.ts
import { writable } from 'svelte/store';

// Store para vistas activas (necesario para Sidebar)
export const activeView = writable('users');

// Store para el sidebar
export const sidebar = writable({
	collapsed: false,
	width: 20 // Porcentaje de ancho
});

// Store para tabs (si no lo tienes en otro archivo)
export const tabs = writable([]);

// Store para visibilidad del wizard de setup (para debug)
export const setupWizardVisible = writable(false);

// Store para información de la barra de estado (StatusBar)
export interface StatusBarInfo {
	count?: number;
	selectedCount?: number;
	label?: string;
	message?: string;
}

export const statusBarInfo = writable<StatusBarInfo>({
	count: 0,
	selectedCount: 0,
	label: 'registros'
});

// Store para visibilidad del buscador global (Spotlight)
export const showSpotlight = writable(false);

// Store para visibilidad del modal de ayuda de atajos
export const showShortcutsHelp = writable(false);

// Store para vista rápida de persona (desde Spotlight)
export const personaQuickView = writable<{ id: string; type: string } | null>(null);

// Store para visibilidad del modal de cambio rápido de usuario
export const showQuickSwitch = writable(false);

// Store para establecer un usuario objetivo para el cambio rápido desde afuera (ej. Spotlight)
export const quickSwitchTarget = writable<any | null>(null);

// PROFILES
export const showUserProfileModal = writable(false);
export const selectedUserProfile = writable<any | null>(null);
