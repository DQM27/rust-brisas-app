// src/lib/components/inspection/registry.ts
import type { Component } from 'svelte';
import type { InspectionType } from '$lib/stores/inspection';

// Importar inspectors
import ContratistaInspector from '../components/inspection/ContratistaInspector.svelte';
// TODO: Importar cuando existan
// import IngresoInspector from './inspectors/IngresoInspector.svelte';
// import VehiculoInspector from './inspectors/VehiculoInspector.svelte';

/**
 * Registry de componentes de inspección por tipo
 * Para agregar nuevo tipo:
 * 1. Crear componente en /inspectors/
 * 2. Importarlo arriba
 * 3. Agregarlo al registry
 */
export const INSPECTION_REGISTRY: Record<InspectionType, Component<any, any> | null> = {
  'contratista': ContratistaInspector,
  'ingreso': null,      // TODO: Implementar
  'vehiculo': null,     // TODO: Implementar
  'empresa': null,      // TODO: Implementar
  'gafete': null,       // TODO: Implementar
};

/**
 * Obtiene el componente inspector para un tipo
 */
export function getInspector(type: InspectionType): Component<any, any> | null {
  return INSPECTION_REGISTRY[type] || null;
}

/**
 * Verifica si existe inspector para un tipo
 */
export function hasInspector(type: InspectionType): boolean {
  return INSPECTION_REGISTRY[type] !== null;
}