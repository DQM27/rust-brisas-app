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
