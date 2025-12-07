import type { Action } from '$lib/logic/permissions';

export interface SidebarItem {
  id: string;
  label: string;
  icon: any; // Componente Svelte
  panelComponent?: any; // Componente del panel (opcional)
  action?: () => void; // Acción directa (si no tiene panel)
  permission?: Action;
  role?: string[];
}

export type { ComponentKey } from './component';