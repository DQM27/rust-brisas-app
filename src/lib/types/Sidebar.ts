import type { Permission } from '$lib/logic/permissions';

export interface SidebarItem {
	id: string;
	label: string;
	icon: any; // Componente Svelte
	panelComponent?: any; // Componente del panel (opcional)
	action?: () => void; // Acción directa (si no tiene panel)
	permission?: Permission; // Permiso en formato "module:action"
	roleId?: string[]; // Role IDs permitidos
}

export type { ComponentKey } from './component';
