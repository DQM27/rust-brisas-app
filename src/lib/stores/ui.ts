// src/lib/stores/ui.ts
import { writable } from 'svelte/store';

// Store para vistas activas (necesario para Sidebar)
export const activeView = writable('users');

// Store para el panel de inspección
export const inspectionPanel = writable({
  visible: false,
  height: 30 // Porcentaje de altura cuando está visible
});

// Store para el sidebar
export const sidebar = writable({
  collapsed: false,
  width: 20 // Porcentaje de ancho
});

// Store para tabs (si no lo tienes en otro archivo)
export const tabs = writable([]);