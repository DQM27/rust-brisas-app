// ============================================
// src/lib/types/tabs.ts (ACTUALIZADO)
// Ahora importa ComponentKey del nuevo archivo
// ============================================
import type { Component } from 'svelte';
import type { ComponentKey } from './component';

/**
 * Tab serializable para persistencia
 * No contiene referencias a componentes de Svelte
 */
export interface SerializableTab {
	/** ID único del tab */
	id: string;

	/** Título mostrado en la UI */
	title: string;

	/** Clave del componente en el registro */
	componentKey: ComponentKey;

	/** Indica si el tab tiene cambios sin guardar */
	isDirty?: boolean;

	/** Datos específicos del componente */
	data?: Record<string, any>;

	/** Orden del tab en la barra */
	order?: number;
}

/**
 * Tab hidratado con componente de Svelte
 * Solo existe en memoria durante la ejecución
 */
export interface HydratedTab extends SerializableTab {
	/** Componente de Svelte 5 */
	component: Component<any, any>;
}

/**
 * Opciones para abrir un nuevo tab
 */
export interface OpenTabOptions {
	/** ID personalizado (opcional, se genera automáticamente si no se provee) */
	id?: string;

	/** Clave del componente a renderizar */
	componentKey: ComponentKey;

	/** Título del tab */
	title: string;

	/** Datos iniciales del componente */
	data?: Record<string, any>;

	/** Si debe enfocarse al abrirse (default: true) */
	focusOnOpen?: boolean;
}

/**
 * Evento de cambio de tab
 */
export interface TabChangeEvent {
	/** ID del tab anterior */
	previousTabId: string | null;

	/** ID del tab actual */
	currentTabId: string | null;
}

/**
 * Props que reciben los componentes montados en tabs
 */
export interface TabComponentProps {
	/** ID del tab que contiene el componente */
	tabId: string;

	/** Datos pasados al abrir el tab */
	data?: Record<string, any>;
}
