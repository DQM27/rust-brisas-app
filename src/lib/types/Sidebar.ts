import type { Action } from '$lib/logic/permissions';

export interface SidebarItem {
	id: string;
	label: string;
	icon: any; // Componente Svelte
	panelComponent?: any; // Componente del panel (opcional)
	action?: () => void; // Acción directa (si no tiene panel)
	permission?: Action;
	roleId?: string[]; // Role IDs permitidos
}

export type { ComponentKey } from './component';
