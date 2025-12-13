// src/lib/types/agGrid.ts

import type { ComponentType } from 'svelte';
import type { GridApi, ColDef } from '@ag-grid-community/core';

// ============================================
// Temas y Fuentes
// ============================================

export type AGGridTheme =
  | 'ag-theme-quartz'
  | 'ag-theme-quartz-dark'
  | 'ag-theme-alpine'
  | 'ag-theme-alpine-dark'
  | 'ag-theme-balham';

export type AGGridFont = 'system' | 'inter' | 'roboto' | 'source-sans';

// ============================================
// Grid IDs
// ============================================

export type GridId =
  | 'contratista-list'
  | 'proveedor-list'
  | 'lista-negra-list'
  | 'vehicles-list'
  | 'badges-list'
  | 'entries-list'
  | 'companies-list'
  | 'users-list'
  | 'visitas-activas-grid'
  | 'proveedores-activos-grid'
  | 'proveedores-grid';

// ============================================
// Toolbar
// ============================================

export type ToolbarContext = 'default' | 'singleSelect' | 'multiSelect';

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

export type ButtonVariant = 'default' | 'primary' | 'danger' | 'success';
export type ButtonState = 'normal' | 'hover' | 'disabled' | 'loading' | 'success' | 'error';

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

export interface ToolbarButtonsConfig {
  order: string[];
  hidden: string[];
}

export interface ButtonLimits {
  default: number;
  singleSelect: number;
  multiSelect: number;
}

export interface ToolbarButtonDefinition {
  id: string;
  label: string;
  icon?: ComponentType;
  variant?: ButtonVariant;
  tooltip?: string;
  category?: 'columns' | 'export' | 'selection' | 'data' | 'ui' | 'custom';
}

// ============================================
// Columnas
// ============================================

export type ColumnPinPosition = 'left' | 'right' | null;

export interface AGGridColumnConfig {
  id: string;
  visible: boolean;
  order: number;
  width?: number;
  pinned?: ColumnPinPosition;
}

// ============================================
// Apariencia
// ============================================

export type RowHeight = 'compact' | 'normal' | 'comfortable';
export type GridDensity = 'compact' | 'normal' | 'comfortable';

export interface AppearanceConfig {
  theme: AGGridTheme;
  font: AGGridFont;
  rowHeight: RowHeight;
  headerHeight: number;
  animateRows: boolean;
  enableCellTextSelection: boolean;
}

// ============================================
// Datos y Filtros
// ============================================

export interface DataConfig {
  paginationSize: number;
  showFloatingFilters: boolean;
  enableQuickFilter: boolean;
  quickFilterText: string;
  enableUndoRedo: boolean;
  undoRedoCellEditing: boolean;
}

// ============================================
// Performance
// ============================================

export interface PerformanceConfig {
  suppressColumnVirtualisation: boolean;
  rowBuffer: number;
  debounceVerticalScrollbar: boolean;
  suppressAnimationFrame: boolean;
  suppressRowHoverHighlight: boolean;
}

// ============================================
// Confirmaciones
// ============================================

export interface ConfirmationsConfig {
  deleteRecords: boolean;
  bulkOperations: boolean;
  dontAskAgain: boolean;
}

// ============================================
// Configuración Completa de Grid
// ============================================

export interface GridConfiguration {
  gridId: GridId;

  // Apariencia
  theme: AGGridTheme;
  font: AGGridFont;
  rowHeight: RowHeight;
  headerHeight: number;
  animateRows: boolean;
  enableCellTextSelection: boolean;

  // Columnas
  columns: AGGridColumnConfig[];

  // Botones por contexto
  buttons: {
    default: ToolbarButtonsConfig;
    singleSelect: ToolbarButtonsConfig;
    multiSelect: ToolbarButtonsConfig;
  };

  // Datos
  paginationSize: number;
  showFloatingFilters: boolean;
  enableQuickFilter: boolean;

  // Features
  enableGrouping: boolean;
  enableFilters: boolean;
  enableSidebar: boolean;
  enableUndoRedo: boolean;

  // Performance
  rowBuffer: number;
  debounceVerticalScrollbar: boolean;

  // Confirmaciones
  confirmations: ConfirmationsConfig;
}

// ============================================
// Toolbar Config
// ============================================

export interface AGGridToolbarConfig {
  gridId: GridId;
  availableButtons: {
    default: ToolbarButtonDefinition[];
    singleSelect: ToolbarButtonDefinition[];
    multiSelect: ToolbarButtonDefinition[];
  };
  customButtons?: {
    default?: CustomToolbarButton[];
    singleSelect?: CustomToolbarButton[];
    multiSelect?: CustomToolbarButton[];
  };
  showColumnSelector?: boolean;
  showThemeSelector?: boolean;
  enableGrouping?: boolean;
}

// ============================================
// Props de Componentes
// ============================================

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

export interface OrganizingMode {
  active: boolean;
  context: ToolbarContext;
  tempOrder: string[];
}

// ============================================
// Settings Modal Tab
// ============================================

export type SettingsTab = 'appearance' | 'columns' | 'toolbar' | 'data' | 'advanced';

export interface SettingsTabDefinition {
  id: SettingsTab;
  label: string;
  icon: string;
}
