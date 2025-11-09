import type { ComponentType } from 'svelte';

/**
 * Clave de componente registrado
 */
export type ComponentKey = 'welcome' | 'user-list' | 'user-editor' | 'user-register' | 'dashboard' ;

/**
 * Tab serializable para persistencia
 */
export interface SerializableTab {
  id: string;
  title: string;
  componentKey: ComponentKey;
  isDirty?: boolean;
  data?: Record<string, any>;
  order?: number; // Para drag & drop
}

/**
 * Tab con componente Svelte hidratado (solo en memoria)
 */
export interface HydratedTab extends SerializableTab {
  component: ComponentType;
}

/**
 * Opciones para abrir un tab
 */
export interface OpenTabOptions {
  componentKey: ComponentKey;
  title: string;
  data?: Record<string, any>;
  id?: string; // Si no se provee, se genera automáticamente
  focusOnOpen?: boolean; // Default: true
}

/**
 * Evento de cambio en un tab
 */
export interface TabChangeEvent {
  tabId: string;
  isDirty: boolean;
  data?: Record<string, any>;
}