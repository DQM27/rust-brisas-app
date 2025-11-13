// src/lib/components/layout/sidebar/types.ts
export interface SidebarItem {
  id: string;
  label: string;
  icon: any; // Componente Svelte
  panelComponent?: any; // Componente del panel (opcional)
  action?: () => void; // Acción directa (si no tiene panel)
}

export type ComponentKey = 
  | 'welcome'
  | 'user-list'
  | 'user-editor'
  | 'user-register'
  | 'dashboard';