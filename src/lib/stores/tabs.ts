
import { writable, derived, get } from 'svelte/store';
import { persisted } from 'svelte-persisted-store';
import type { SerializableTab, HydratedTab, OpenTabOptions } from '$lib/types/tabs';
import { getComponent } from '$lib/components/registry';
import type { Readable } from 'svelte/store';

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
 * 
 * Transforma SerializableTab[] en HydratedTab[] agregando el componente de Svelte
 */
export const tabsStore: Readable<HydratedTab[]> = derived(
  tabsStorePersisted,
  ($tabs) => {
    return $tabs.map((tab) => ({
      ...tab,
      component: getComponent(tab.componentKey)
    })) as HydratedTab[];
  }
);

/**
 * Genera ID único para un tab
 * 
 * @param componentKey - Identificador del componente
 * @param data - Datos opcionales que pueden contener un id
 * @returns ID único del tab
 */
function generateTabId(componentKey: string, data?: Record<string, any>): string {
  if (data?.id) {
    return `${componentKey}-${data.id}`;
  }
  return `${componentKey}-${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
}

/**
 * Abre un nuevo tab o enfoca uno existente
 * 
 * @param options - Opciones para abrir el tab
 * @returns ID del tab abierto
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
  
  tabsStorePersisted.update(currentTabs => [...currentTabs, newTab]);
  
  if (focusOnOpen) {
    activeTabId.set(id);
  }
  
  return id;
}

/**
 * Cierra un tab con confirmación si tiene cambios sin guardar
 * 
 * @param id - ID del tab a cerrar
 * @param force - Si es true, cierra sin confirmación
 * @returns true si se cerró el tab, false si se canceló
 */
export function closeTab(id: string, force: boolean = false): boolean {
  const tabs = get(tabsStorePersisted);
  const tab = tabs.find(t => t.id === id);
  
  if (!tab) {
    return false;
  }
  
  // Verificar si tiene cambios sin guardar
  if (tab.isDirty && !force) {
    const confirmed = confirm(`"${tab.title}" tiene cambios sin guardar. ¿Cerrar de todos modos?`);
    if (!confirmed) {
      return false;
    }
  }
  
  // Filtrar el tab
  const remainingTabs = tabs.filter(t => t.id !== id);
  tabsStorePersisted.set(remainingTabs);
  
  // Manejar tab activo
  const currentActive = get(activeTabId);
  if (currentActive === id) {
    if (remainingTabs.length > 0) {
      // Activar el tab anterior o el primero
      const closedIndex = tabs.findIndex(t => t.id === id);
      const newIndex = Math.max(0, closedIndex - 1);
      activeTabId.set(remainingTabs[newIndex].id);
    } else {
      activeTabId.set('');
    }
  }
  
  return true;
}

/**
 * Cierra todos los tabs con confirmación
 * 
 * @param force - Si es true, cierra sin confirmación
 * @returns true si se cerraron todos los tabs, false si se canceló
 */
export function closeAllTabs(force: boolean = false): boolean {
  const tabs = get(tabsStorePersisted);
  const dirtyTabs = tabs.filter(t => t.isDirty);
  
  if (dirtyTabs.length > 0 && !force) {
    const tabWord = dirtyTabs.length === 1 ? 'tab tiene' : 'tabs tienen';
    const confirmed = confirm(
      `${dirtyTabs.length} ${tabWord} cambios sin guardar. ¿Cerrar todos de todos modos?`
    );
    if (!confirmed) {
      return false;
    }
  }
  
  tabsStorePersisted.set([]);
  activeTabId.set('');
  return true;
}

/**
 * Marca un tab como modificado (dirty)
 * 
 * @param id - ID del tab
 * @param isDirty - Estado de modificación
 */
export function markTabDirty(id: string, isDirty: boolean = true): void {
  tabsStorePersisted.update(tabs => 
    tabs.map(tab => 
      tab.id === id 
        ? { ...tab, isDirty } 
        : tab
    )
  );
}

/**
 * Actualiza los datos de un tab
 * 
 * @param id - ID del tab
 * @param data - Datos a actualizar (se mezclan con los existentes)
 */
export function updateTabData(id: string, data: Record<string, any>): void {
  tabsStorePersisted.update(tabs => 
    tabs.map(tab => 
      tab.id === id 
        ? { ...tab, data: { ...tab.data, ...data } } 
        : tab
    )
  );
}

/**
 * Actualiza el título de un tab
 * 
 * @param id - ID del tab
 * @param title - Nuevo título
 */
export function updateTabTitle(id: string, title: string): void {
  tabsStorePersisted.update(tabs => 
    tabs.map(tab => 
      tab.id === id 
        ? { ...tab, title } 
        : tab
    )
  );
}

/**
 * Reordena tabs (para drag & drop)
 * 
 * @param newOrder - Array con los IDs en el nuevo orden
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
 * 
 * @param tabId - ID del tab actual
 * @returns Objeto con métodos para manipular el estado del tab
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
 * 
 * @param id - ID del tab
 * @returns Tab encontrado o undefined si no existe
 */
export function getTab(id: string): SerializableTab | undefined {
  return get(tabsStorePersisted).find(t => t.id === id);
}

/**
 * Verifica si existe un tab
 * 
 * @param id - ID del tab
 * @returns true si el tab existe
 */
export function hasTab(id: string): boolean {
  return get(tabsStorePersisted).some(t => t.id === id);
}

/**
 * Obtiene el tab actualmente activo
 * 
 * @returns Tab activo o undefined si no hay ninguno
 */
export function getActiveTab(): SerializableTab | undefined {
  const activeId = get(activeTabId);
  if (!activeId) {
    return undefined;
  }
  return getTab(activeId);
}

/**
 * Obtiene todos los tabs abiertos
 * 
 * @returns Array de tabs serializables
 */
export function getAllTabs(): SerializableTab[] {
  return get(tabsStorePersisted);
}

/**
 * Cuenta cuántos tabs están abiertos
 * 
 * @returns Número de tabs abiertos
 */
export function getTabCount(): number {
  return get(tabsStorePersisted).length;
}

/**
 * Obtiene todos los tabs con cambios sin guardar
 * 
 * @returns Array de tabs con isDirty: true
 */
export function getDirtyTabs(): SerializableTab[] {
  return get(tabsStorePersisted).filter(t => t.isDirty);
}

/**
 * Reset completo (útil para logout)
 */
export function resetTabs(): void {
  tabsStorePersisted.set([]);
  activeTabId.set('');
}