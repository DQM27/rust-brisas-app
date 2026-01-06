// ============================================
// src/lib/components/components.ts 
// // ============================================
import type { Component } from 'svelte';
import type { ComponentKey } from '$lib/types/component';

// Importar todos los componentes de tabs
import WelcomePanel from '$lib/components/WelcomePanel.svelte';
import UserListView from '$lib/components/user/UserListView.svelte';

import ContratistaListView from '$lib/components/contratista/ContratistaListView.svelte';
import ProveedorListView from '$lib/components/proveedor/ProveedorListView.svelte';
import ListaNegraListView from '$lib/components/listaNegra/ListaNegraListView.svelte';

import GeneralSettingsPanel from '$lib/components/settings/GeneralSettingsPanel.svelte';
import VisualSettingsPanel from '$lib/components/settings/VisualSettingsPanel.svelte';
import UpdateSettingsPanel from '$lib/components/settings/UpdateSettingsPanel.svelte';
import BackupSettingsPanel from '$lib/components/settings/BackupSettingsPanel.svelte';
import DeviceSettingsPanel from '$lib/components/settings/DeviceSettingsPanel.svelte';
import SecuritySettingsPanel from '$lib/components/settings/SecuritySettingsPanel.svelte';
import SessionSettingsPanel from '$lib/components/settings/SessionSettingsPanel.svelte';
import ExportSettingsPanel from '$lib/components/settings/ExportSettingsPanel.svelte';
import TrashSettingsPanel from '$lib/components/settings/TrashSettingsPanel.svelte';
import RolesSettingsPanel from '$lib/components/settings/RolesSettingsPanel.svelte';
import ReportesListView from '$lib/components/settings/ReportesListView.svelte';
import AboutPanel from '$lib/components/settings/AboutPanel.svelte';



import GafeteListView from '$lib/components/gafete/GafeteListView.svelte';
import IngresoModule from '$lib/components/ingreso/IngresoModule.svelte';
import VisitaListView from '$lib/components/visita/VisitaListView.svelte';
import VisitanteListView from '$lib/components/visitante/VisitanteListView.svelte';
import UnderConstruction from '$lib/components/common/UnderConstruction.svelte';
import DevSettingsPanel from '$lib/components/settings/DevSettingsPanel.svelte';

/**
 * Registry central de componentes
 * Permite serializar tabs y reconstruirlos después
 */
export const COMPONENT_REGISTRY: Record<ComponentKey, Component<any, any>> = {
  'welcome': WelcomePanel,
  'user-list': UserListView,
  'user-editor': WelcomePanel,
  'dashboard': WelcomePanel,
  'citas-view': VisitaListView,
  'visitas-list': VisitaListView,
  'visitante-list': VisitanteListView,

  'contratista': ContratistaListView,
  'contratista-list': ContratistaListView,
  'proveedor': ProveedorListView,
  'proveedor-list': ProveedorListView,
  'lista-negra': ListaNegraListView, // Ahora usa el ListView nuevo directamente
  'lista-negra-list': ListaNegraListView, // También usa ListView


  'general-settings': GeneralSettingsPanel as any,
  'visual-settings': VisualSettingsPanel as any,
  'update-settings': UpdateSettingsPanel,
  'backup-settings': BackupSettingsPanel,
  'device-settings': DeviceSettingsPanel,
  'security-settings': SecuritySettingsPanel,
  'session-settings': SessionSettingsPanel,
  'export-settings': ExportSettingsPanel,
  'trash-settings': TrashSettingsPanel,
  'roles-settings': RolesSettingsPanel,
  'shortcut-settings': WelcomePanel, // Placeholder hasta que se cree el panel real
  'reportes-list': ReportesListView,
  'about': AboutPanel,

  'gafete-list': GafeteListView,
  'ingreso-list': IngresoModule, // Point legacy/main link to new Module
  'ingreso-module': IngresoModule,
  'under-construction': UnderConstruction,
  'dev-settings': DevSettingsPanel,
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