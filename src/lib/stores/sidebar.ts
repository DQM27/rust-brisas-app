// src/lib/components/layout/sidebar/sidebarStore.ts
import { writable } from 'svelte/store';
import { openTab } from '$lib/stores/tabs';
import type { ComponentKey } from '$lib/types/Sidebar';

export const activePanel = writable<string | null>(null);

export function openView(componentKey: ComponentKey, title: string) {
  openTab({
    componentKey: componentKey,
    title: title,
    focusOnOpen: true
  });
}

export function openUserRegistration() {
  openTab({
    componentKey: 'user-register',
    title: 'Registrar Usuario',
    focusOnOpen: true
  });
}

// Función para cerrar el panel activo
export function closeActivePanel() {
  activePanel.set(null);
}