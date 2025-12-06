// src/lib/types/agGrid.ts

import type { ComponentType } from 'svelte';
import type { GridApi, ColDef } from '@ag-grid-community/core';

/**
 * Temas disponibles de AG Grid Community
 */
export type AGGridTheme =
  | 'ag-theme-quartz'
  | 'ag-theme-alpine'
  | 'ag-theme-alpine-dark'
  | 'ag-theme-quartz-dark'
  | 'ag-theme-balham';

/**
 * Fuentes disponibles para las grids
 */
export type AGGridFont =
  | 'system'
  | 'inter'
  | 'roboto'
  | 'source-sans';

/**
 * IDs de las grids basados en ComponentKey
 * Solo los componentes que usan AG Grid
 */
export type GridId =
  | 'contratista-list'
  | 'lista-negra-list'
  | 'vehicles-list'
  | 'badges-list'
  | 'entries-list'
  | 'companies-list'
  | 'users-list';

/**
 * Contextos de la toolbar
 */
export type ToolbarContext = 'default' | 'singleSelect' | 'multiSelect';

/**
 * Botones comunes de toolbar (operaciones genéricas de grid)
 */
export type CommonToolbarButton =
  | 'autosize-all'
  | 'autosize-selected'
  | 'reset-columns'
  | 'export-csv'
  | 'export-excel'
  | 'export-json'
  | 'select-all'
  | 'deselect-all'
  | 'copy-selected'
  | 'toggle-filters'
  | 'toggle-sidebar'
  | 'refresh'
  | 'clear-filters'
  | 'clear-sort'
  | 'expand-groups'
  | 'collapse-groups';

/**
 * Variantes de estilo para botones
 */
export type ButtonVariant = 'default' | 'primary' | 'danger' | 'success';

/**
 * Estados visuales de un botón
 */
export type ButtonState = 'normal' | 'hover' | 'disabled' | 'loading' | 'success' | 'error';

/**
 * Configuración de un botón personalizado (específico de cada grid)
 */
export interface CustomToolbarButton {
  id: string;
  label: string;
  icon?: ComponentType;
  onClick?: () => void | Promise<void>;
  disabled?: boolean;
  variant?: ButtonVariant;
  tooltip?: string;
  state?: ButtonState;
  useCommonHandler?: boolean;
}

/**
 * Configuración de botones por contexto
 */
export interface ToolbarButtonsConfig {
  order: string[];      // IDs en orden personalizado
  hidden: string[];     // IDs de botones ocultos
}

/**
 * Límites de botones por contexto
 */
export interface ButtonLimits {
  default: number;
  singleSelect: number;
  multiSelect: number;
}

/**
 * Configuración de una columna
 */
export interface AGGridColumnConfig {
  id: string;
  visible: boolean;
  order: number;
  width?: number;
}

/**
 * Altura de fila
 */
export type RowHeight = 'compact' | 'normal' | 'comfortable';

/**
 * Configuración de confirmaciones
 */
export interface ConfirmationsConfig {
  deleteRecords: boolean;
  dontAskAgain: boolean;
}

/**
 * Configuración completa de una grid
 */
export interface GridConfiguration {
  gridId: GridId;

  // Visual
  theme: AGGridTheme;
  font: AGGridFont;

  // Columnas
  columns: AGGridColumnConfig[];

  // Botones por contexto
  buttons: {
    default: ToolbarButtonsConfig;
    singleSelect: ToolbarButtonsConfig;
    multiSelect: ToolbarButtonsConfig;
  };

  // Avanzado
  rowHeight: RowHeight;
  paginationSize: number;
  enableGrouping: boolean;
  enableFilters: boolean;
  enableSidebar: boolean;

  // Confirmaciones
  confirmations: ConfirmationsConfig;
}

/**
 * Definición de un botón disponible
 */
export interface ToolbarButtonDefinition {
  id: string;
  label: string;
  icon?: ComponentType;
  variant?: ButtonVariant;
  tooltip?: string;
  category?: 'columns' | 'export' | 'selection' | 'data' | 'ui' | 'custom';
}

/**
 * Configuración de toolbar para una grid
 */
export interface AGGridToolbarConfig {
  gridId: GridId;

  // Botones comunes disponibles por contexto
  availableButtons: {
    default: ToolbarButtonDefinition[];
    singleSelect: ToolbarButtonDefinition[];
    multiSelect: ToolbarButtonDefinition[];
  };

  // Botones personalizados (pasados desde el componente)
  customButtons?: {
    default?: CustomToolbarButton[];
    singleSelect?: CustomToolbarButton[];
    multiSelect?: CustomToolbarButton[];
  };

  // Features opcionales
  showColumnSelector?: boolean;
  showThemeSelector?: boolean;
  enableGrouping?: boolean;
}

/**
 * Props para AGGridWrapper
 */
export interface AGGridWrapperProps<T = any> {
  gridId: GridId;
  columnDefs: ColDef[];
  rowData: T[];
  customButtons?: {
    default?: CustomToolbarButton[];
    singleSelect?: CustomToolbarButton[];
    multiSelect?: CustomToolbarButton[];
  };
  onGridReady?: (api: GridApi) => void;
  onSelectionChanged?: (selectedRows: T[]) => void;
  onRowClicked?: (row: T) => void;
  onRowDoubleClicked?: (row: T) => void;
  enableGrouping?: boolean;
  getRowId?: (params: any) => string;
  persistenceKey?: string;
}

/**
 * Props para AGGridToolbar
 */
export interface AGGridToolbarProps {
  gridId: GridId;
  context: ToolbarContext;
  selectedRows: any[];
  gridApi: GridApi | null;
  onOpenSettings: () => void;
  customButtons?: {
    default?: CustomToolbarButton[];
    singleSelect?: CustomToolbarButton[];
    multiSelect?: CustomToolbarButton[];
  };
}

/**
 * Modo de organización de botones
 */
export interface OrganizingMode {
  active: boolean;
  context: ToolbarContext;
  tempOrder: string[];
}