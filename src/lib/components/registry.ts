import type { Component } from 'svelte';
import type { ComponentKey } from '$lib/types/tabs';

// Importar todos los componentes de tabs
import WelcomePanel from '$lib/components/WelcomePanel.svelte';
import UserRegisterPanel from '$lib/components/admin/UserRegisterPanel.svelte';

// TODO: Importar cuando existan
// import UserList from '$lib/components/UserList.svelte';
// import UserEditor from '$lib/components/UserEditor.svelte';
// import Dashboard from '$lib/components/Dashboard.svelte';

/**
 * Registry central de componentes
 * Permite serializar tabs y reconstruirlos después
 */
export const COMPONENT_REGISTRY: Record<ComponentKey, Component<any>> = {
  'welcome': WelcomePanel,
  'user-list': WelcomePanel, // Placeholder
  'user-editor': WelcomePanel, // Placeholder
  'dashboard': WelcomePanel, // Placeholder
  'user-register': UserRegisterPanel,
};

/**
 * Obtiene un componente del registry
 */
export function getComponent(key: ComponentKey): Component<any> {
  const component = COMPONENT_REGISTRY[key];
  
  if (!component) {
    console.error(`Componente no encontrado: ${key}`);
    return COMPONENT_REGISTRY['welcome']; // Fallback
  }
  
  return component;
}

/**
 * Valida si una key existe en el registry
 */
export function isValidComponentKey(key: string): key is ComponentKey {
  return key in COMPONENT_REGISTRY;
}