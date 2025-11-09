import { writable, derived, get } from 'svelte/store';
import { persisted } from 'svelte-persisted-store';
import type { SerializableTab, HydratedTab, OpenTabOptions, TabChangeEvent } from '$lib/types/tabs';
import { getComponent } from '$lib/components/registry';

/**
 * Store persistido con tabs serializables
 */
const tabsStorePersisted = persisted<SerializableTab[]>('brisas-tabs', []);

/**
 * Store del tab activo
 */
export const activeTabId = persisted<string>('brisas-active-tab', '');

/**
 * Store derivado con componentes hidratados (solo en memoria)
 */
export const tabsStore = derived<typeof tabsStorePersisted, HydratedTab[]>(
  tabsStorePersisted,
  ($tabs) => {
    return $tabs.map(tab => ({
      ...tab,
      component: getComponent(tab.componentKey)
    }));
  }
);

/**
 * Genera ID único para un tab
 */
function generateTabId(componentKey: string, data?: Record<string, any>): string {
  if (data?.id) {
    return `${componentKey}-${data.id}`;
  }
  return `${componentKey}-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Abre un nuevo tab o enfoca uno existente
 */
export function openTab(options: OpenTabOptions): string {
  const { componentKey, title, data, focusOnOpen = true } = options;
  
  const id = options.id || generateTabId(componentKey, data);
  const tabs = get(tabsStorePersisted);
  
  // Si ya existe, solo enfocarlo
  const existingTab = tabs.find(t => t.id === id);
  if (existingTab) {
    if (focusOnOpen) {
      activeTabId.set(id);
    }
    return id;
  }
  
  // Crear nuevo tab
  const newTab: SerializableTab = {
    id,
    title,
    componentKey,
    data,
    isDirty: false,
    order: tabs.length
  };
  
  tabsStorePersisted.update(tabs => [...tabs, newTab]);
  
  if (focusOnOpen) {
    activeTabId.set(id);
  }
  
  return id;
}

/**
 * Cierra un tab con confirmación si tiene cambios sin guardar
 */
export function closeTab(id: string, force: boolean = false): boolean {
  const tabs = get(tabsStorePersisted);
  const tab = tabs.find(t => t.id === id);
  
  if (!tab) return false;
  
  // Verificar si tiene cambios sin guardar
  if (tab.isDirty && !force) {
    const confirmed = confirm(`"${tab.title}" tiene cambios sin guardar. ¿Cerrar de todos modos?`);
    if (!confirmed) return false;
  }
  
  // Filtrar el tab
  const filtered = tabs.filter(t => t.id !== id);
  tabsStorePersisted.set(filtered);
  
  // Manejar tab activo
  const currentActive = get(activeTabId);
  if (currentActive === id) {
    if (filtered.length > 0) {
      // Activar el tab anterior o el primero
      const index = tabs.findIndex(t => t.id === id);
      const newIndex = Math.max(0, index - 1);
      activeTabId.set(filtered[newIndex].id);
    } else {
      activeTabId.set('');
    }
  }
  
  return true;
}

/**
 * Cierra todos los tabs con confirmación
 */
export function closeAllTabs(force: boolean = false): boolean {
  const tabs = get(tabsStorePersisted);
  const dirtyTabs = tabs.filter(t => t.isDirty);
  
  if (dirtyTabs.length > 0 && !force) {
    const confirmed = confirm(
      `${dirtyTabs.length} tab(s) tienen cambios sin guardar. ¿Cerrar todos de todos modos?`
    );
    if (!confirmed) return false;
  }
  
  tabsStorePersisted.set([]);
  activeTabId.set('');
  return true;
}

/**
 * Marca un tab como modificado (dirty)
 */
export function markTabDirty(id: string, isDirty: boolean = true): void {
  tabsStorePersisted.update(tabs => 
    tabs.map(tab => tab.id === id ? { ...tab, isDirty } : tab)
  );
}

/**
 * Actualiza los datos de un tab
 */
export function updateTabData(id: string, data: Record<string, any>): void {
  tabsStorePersisted.update(tabs => 
    tabs.map(tab => tab.id === id ? { ...tab, data: { ...tab.data, ...data } } : tab)
  );
}

/**
 * Actualiza el título de un tab
 */
export function updateTabTitle(id: string, title: string): void {
  tabsStorePersisted.update(tabs => 
    tabs.map(tab => tab.id === id ? { ...tab, title } : tab)
  );
}

/**
 * Reordena tabs (para drag & drop)
 */
export function reorderTabs(newOrder: string[]): void {
  const tabs = get(tabsStorePersisted);
  
  const reordered = newOrder
    .map(id => tabs.find(t => t.id === id))
    .filter((tab): tab is SerializableTab => tab !== undefined)
    .map((tab, index) => ({ ...tab, order: index }));
  
  tabsStorePersisted.set(reordered);
}

/**
 * Hook para que componentes reporten cambios
 */
export function useTabState(tabId: string) {
  return {
    markDirty: () => markTabDirty(tabId, true),
    markClean: () => markTabDirty(tabId, false),
    updateData: (data: Record<string, any>) => updateTabData(tabId, data),
    updateTitle: (title: string) => updateTabTitle(tabId, title),
    close: () => closeTab(tabId)
  };
}

/**
 * Obtiene un tab por ID
 */
export function getTab(id: string): SerializableTab | undefined {
  return get(tabsStorePersisted).find(t => t.id === id);
}

/**
 * Verifica si existe un tab
 */
export function hasTab(id: string): boolean {
  return get(tabsStorePersisted).some(t => t.id === id);
}

/**
 * Reset completo (útil para logout)
 */
export function resetTabs(): void {
  tabsStorePersisted.set([]);
  activeTabId.set('');
}