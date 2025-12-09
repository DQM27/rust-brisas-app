// ============================================
// src/lib/components/components.ts 
// // ============================================
import type { Component } from 'svelte';
import type { ComponentKey } from '$lib/types/component';

// Importar todos los componentes de tabs
import WelcomePanel from '$lib/components/WelcomePanel.svelte';
import RegisterUserView from '$lib/components/RegisterUserView.svelte';
import UserProfileView from '$lib/components/UserProfileView.svelte';
import UserListView from '$lib/components/user/UserListView.svelte';

import ContratistaView from '$lib/components/contratista/ContratistaView.svelte';
import ContratistaListView from '$lib/components/contratista/ContratistaListView.svelte';
import ListaNegraView from '$lib/components/listaNegra/ListaNegraView.svelte';
import ListaNegraListView from '$lib/components/listaNegra/ListaNegraListView.svelte';
import BlacklistImportView from '$lib/components/BlacklistImport/BlacklistImportView.svelte';
import GeneralSettingsPanel from '$lib/components/settings/GeneralSettingsPanel.svelte';
import UpdateSettingsPanel from '$lib/components/settings/UpdateSettingsPanel.svelte';
import BackupSettingsPanel from '$lib/components/settings/BackupSettingsPanel.svelte';
import DeviceSettingsPanel from '$lib/components/settings/DeviceSettingsPanel.svelte';

import GafeteListView from '$lib/components/gafete/GafeteListView.svelte';
import IngresoListView from '$lib/components/ingreso/IngresoListView.svelte';

/**
 * Registry central de componentes
 * Permite serializar tabs y reconstruirlos después
 */
export const COMPONENT_REGISTRY: Record<ComponentKey, Component<any, any>> = {
  'welcome': WelcomePanel,
  'user-list': UserListView,
  'user-editor': WelcomePanel,
  'user-profile': UserProfileView,
  'dashboard': WelcomePanel,

  "user-register": RegisterUserView,
  'contratista': ContratistaView,
  'contratista-list': ContratistaListView,
  'lista-negra': ListaNegraView,
  'lista-negra-list': ListaNegraListView,
  'blacklist-import': BlacklistImportView,
  'general-settings': GeneralSettingsPanel,
  'update-settings': UpdateSettingsPanel,
  'backup-settings': BackupSettingsPanel,
  'device-settings': DeviceSettingsPanel,

  'gafete-list': GafeteListView,
  'ingreso-list': IngresoListView,
};

/**
 * Obtiene un componente del registry
 * 
 * @param key - Clave del componente
 * @returns Componente de Svelte
 * @throws Error si el componente no existe y no hay fallback disponible
 */
export function getComponent(key: ComponentKey): Component<any, any> {
  const component = COMPONENT_REGISTRY[key];

  if (!component) {
    console.error(`Componente no encontrado: ${key}`);
    console.error('Componentes disponibles:', Object.keys(COMPONENT_REGISTRY));

    // Fallback a welcome si existe
    if (COMPONENT_REGISTRY['welcome']) {
      console.warn(`Usando fallback: welcome para ${key}`);
      return COMPONENT_REGISTRY['welcome'];
    }

    throw new Error(`Componente "${key}" no encontrado y no hay fallback disponible`);
  }

  return component;
}

/**
 * Valida si una key existe en el registry
 * 
 * @param key - Clave a validar
 * @returns true si la clave existe en el registro
 */
export function isValidComponentKey(key: string): key is ComponentKey {
  return key in COMPONENT_REGISTRY;
}

/**
 * Obtiene todas las claves registradas
 * 
 * @returns Array de claves de componentes
 */
export function getRegisteredKeys(): ComponentKey[] {
  return Object.keys(COMPONENT_REGISTRY) as ComponentKey[];
}

/**
 * Verifica si un componente está disponible (no es placeholder)
 * 
 * @param key - Clave del componente
 * @returns true si el componente tiene su propia implementación
 */
export function isComponentAvailable(key: ComponentKey): boolean {
  // Un componente es placeholder si apunta a WelcomePanel
  const component = COMPONENT_REGISTRY[key];
  return component !== undefined && component !== WelcomePanel;
}