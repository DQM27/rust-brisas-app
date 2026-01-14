// src/lib/config/agGridConfigs.ts

import type {
  GridId,
  ToolbarButtonDefinition,
  AGGridToolbarConfig
} from '$lib/types/agGrid';
import {
  Download,
  FileJson,
  Maximize2,
  Minimize2,
  RotateCw,
  CheckSquare,
  X,
  Filter,
  Eraser,
  ArrowUpDown,
  Plus,
  Edit,
  Trash2,
  Upload,
  Eye,
  History,
  Car,
  CreditCard,
  UserCheck
} from 'lucide-svelte';

// ============================================
// Botones Comunes Disponibles
// ============================================

/**
 * Botones comunes para contexto DEFAULT
 */
export const COMMON_DEFAULT_BUTTONS: ToolbarButtonDefinition[] = [
  // Columnas (Community)
  {
    id: 'autosize-all',
    label: 'Ajustar Columnas',
    icon: Maximize2,
    tooltip: 'Ajustar todas las columnas al contenido',
    category: 'columns'
  },
  {
    id: 'autosize-selected',
    label: 'Ajustar Selección',
    icon: Minimize2,
    tooltip: 'Ajustar columnas seleccionadas',
    category: 'columns'
  },
  {
    id: 'size-to-fit',
    label: 'Ajustar al Ancho',
    icon: Minimize2,
    tooltip: 'Ajustar columnas al ancho de la ventana',
    category: 'columns'
  },
  {
    id: 'reset-columns',
    label: 'Restaurar Columnas',
    icon: RotateCw,
    tooltip: 'Restaurar columnas a tamaño por defecto',
    category: 'columns'
  },

  // Export (Community)
  {
    id: 'export-csv',
    label: 'Exportar CSV',
    icon: Download,
    tooltip: 'Exportar datos a CSV',
    category: 'export'
  },
  {
    id: 'export-json',
    label: 'Exportar JSON',
    icon: FileJson,
    tooltip: 'Exportar datos a JSON',
    category: 'export'
  },

  // Selección (Community)
  {
    id: 'select-all',
    label: 'Seleccionar Todo',
    icon: CheckSquare,
    tooltip: 'Seleccionar todas las filas visibles',
    category: 'selection'
  },
  {
    id: 'deselect-all',
    label: 'Deseleccionar',
    icon: X,
    tooltip: 'Quitar todas las selecciones',
    category: 'selection'
  },

  // UI (Community)
  {
    id: 'toggle-filters',
    label: 'Filtros',
    icon: Filter,
    tooltip: 'Mostrar/Ocultar filtros',
    category: 'ui'
  },

  // Data (Community)
  {
    id: 'refresh',
    label: 'Refrescar',
    icon: RotateCw,
    tooltip: 'Refrescar datos de la grid',
    category: 'data'
  },
  {
    id: 'clear-filters',
    label: 'Limpiar Filtros',
    icon: Eraser,
    tooltip: 'Eliminar todos los filtros activos',
    category: 'data'
  },
  {
    id: 'clear-sort',
    label: 'Limpiar Orden',
    icon: ArrowUpDown,
    tooltip: 'Eliminar ordenamiento',
    category: 'data'
  }
];

/**
 * Botones comunes para contexto SINGLE SELECT (1 fila seleccionada)
 * NOTA: Los botones de acción (edit, delete, view-info) deben ser customButtons
 * porque requieren handlers específicos de cada entidad
 */
export const COMMON_SINGLE_SELECT_BUTTONS: ToolbarButtonDefinition[] = [
  {
    id: 'deselect',
    label: 'Cancelar',
    icon: X,
    tooltip: 'Cancelar selección',
    category: 'selection'
  }
];

/**
 * Botones comunes para contexto MULTI SELECT (2+ filas seleccionadas)
 * NOTA: Los botones de acción deben ser customButtons por cada vista
 */
export const COMMON_MULTI_SELECT_BUTTONS: ToolbarButtonDefinition[] = [
  {
    id: 'deselect-all',
    label: 'Cancelar',
    icon: X,
    tooltip: 'Cancelar todas las selecciones',
    category: 'selection'
  }
];

// ============================================
// Helpers para Crear Custom Buttons
// ============================================

export const createCustomButton = {
  nuevo: (onClick: () => void, disabled = false, label = 'Nuevo') => ({
    id: 'new-record',
    label,
    icon: Plus,
    onClick,
    disabled,
    variant: 'primary' as const,
    tooltip: 'Crear nuevo registro'
  }),

  editar: (onClick: () => void, disabled = false) => ({
    id: 'edit-record',
    label: 'Editar',
    icon: Edit,
    onClick,
    disabled,
    tooltip: 'Editar seleccionado'
  }),

  eliminar: (onClick: () => void, disabled = false) => ({
    id: 'delete-record',
    label: 'Eliminar',
    icon: Trash2,
    onClick,
    disabled,
    variant: 'danger' as const,
    tooltip: 'Eliminar seleccionado(s)'
  }),

  importar: (onClick: () => void, disabled = false) => ({
    id: 'import-data',
    label: 'Importar',
    icon: Upload,
    onClick,
    disabled,
    tooltip: 'Importar datos'
  }),

  verInformacion: (onClick: () => void, disabled = false) => ({
    id: 'view-info',
    label: 'Ver Información',
    icon: Eye,
    onClick,
    disabled,
    tooltip: 'Ver información detallada'
  }),

  historial: (onClick: () => void, disabled = false) => ({
    id: 'view-history',
    label: 'Historial',
    icon: History,
    onClick,
    disabled,
    tooltip: 'Ver historial'
  }),

  vehiculos: (onClick: () => void, disabled = false) => ({
    id: 'view-vehicles',
    label: 'Vehículos',
    icon: Car,
    onClick,
    disabled,
    tooltip: 'Ver vehículos asociados'
  }),

  badges: (onClick: () => void, disabled = false) => ({
    id: 'view-badges',
    label: 'Carnets',
    icon: CreditCard,
    onClick,
    disabled,
    tooltip: 'Ver carnets asignados'
  }),

  exportar: (onClick: () => void, disabled = false) => ({
    id: 'export-advanced',
    label: 'Exportar',
    icon: Download,
    onClick,
    disabled,
    variant: 'default' as const,
    tooltip: 'Exportación avanzada (PDF, Excel, CSV)'
  }),

  quitarListaNegra: (onClick: () => void, disabled = false) => ({
    id: 'remove-blacklist',
    label: 'Quitar de Lista',
    icon: UserCheck,
    onClick,
    disabled,
    variant: 'success' as const,
    tooltip: 'Quitar de lista negra'
  })
};

// ============================================
// Configuraciones por Grid
// ============================================

/**
 * Configuración estándar para todas las grids principales
 * Todas las grids heredan TODOS los botones disponibles
 */
const STANDARD_GRID_CONFIG = {
  availableButtons: {
    default: COMMON_DEFAULT_BUTTONS,
    singleSelect: COMMON_SINGLE_SELECT_BUTTONS,
    multiSelect: COMMON_MULTI_SELECT_BUTTONS
  },
  showColumnSelector: true,
  showThemeSelector: true,
  enableGrouping: false
};

/**
 * Configuración para grids de ingreso (menos botones)
 */
const INGRESO_GRID_CONFIG = {
  availableButtons: {
    default: COMMON_DEFAULT_BUTTONS,
    singleSelect: [],
    multiSelect: []
  },
  showColumnSelector: true,
  showThemeSelector: false,
  enableGrouping: false
};

/**
 * Registry de configuraciones por grid
 * Todas las grids principales heredan TODOS los botones disponibles
 */
export const GRID_CONFIGS: Record<GridId, Omit<AGGridToolbarConfig, 'customButtons'>> = {
  // Grids principales - heredan todos los botones
  'contratista-list': { gridId: 'contratista-list', ...STANDARD_GRID_CONFIG },
  'proveedor-list': { gridId: 'proveedor-list', ...STANDARD_GRID_CONFIG },
  'lista-negra-list': { gridId: 'lista-negra-list', ...STANDARD_GRID_CONFIG },
  'lista-negra': { gridId: 'lista-negra', ...STANDARD_GRID_CONFIG },
  'vehicles-list': { gridId: 'vehicles-list', ...STANDARD_GRID_CONFIG },
  'badges-list': { gridId: 'badges-list', ...STANDARD_GRID_CONFIG },
  'entries-list': { gridId: 'entries-list', ...STANDARD_GRID_CONFIG },
  'companies-list': { gridId: 'companies-list', ...STANDARD_GRID_CONFIG },
  'users-list': { gridId: 'users-list', ...STANDARD_GRID_CONFIG },
  'visitas-list': { gridId: 'visitas-list', ...STANDARD_GRID_CONFIG },
  'visitas-activas-grid': { gridId: 'visitas-activas-grid', ...STANDARD_GRID_CONFIG },
  'proveedores-activos-grid': { gridId: 'proveedores-activos-grid', ...STANDARD_GRID_CONFIG },
  'proveedores-grid': { gridId: 'proveedores-grid', ...STANDARD_GRID_CONFIG },
  'visitante-list': { gridId: 'visitante-list', ...STANDARD_GRID_CONFIG },

  // Grids de ingreso - todos los botones disponibles
  'contratista-ingreso-list': { gridId: 'contratista-ingreso-list', ...INGRESO_GRID_CONFIG },
  'visita-ingreso-list': { gridId: 'visita-ingreso-list', ...INGRESO_GRID_CONFIG },
  'proveedor-ingreso-list': { gridId: 'proveedor-ingreso-list', ...INGRESO_GRID_CONFIG },
  'ingreso-list': { gridId: 'ingreso-list', ...INGRESO_GRID_CONFIG },

  // Grids de papelera - todos los botones disponibles
  'contratista-trash': { gridId: 'contratista-trash', ...INGRESO_GRID_CONFIG },
  'universal-trash': { gridId: 'universal-trash', ...INGRESO_GRID_CONFIG },
  'trash-contratista': { gridId: 'trash-contratista', ...INGRESO_GRID_CONFIG },
  'trash-proveedor': { gridId: 'trash-proveedor', ...INGRESO_GRID_CONFIG },
  'trash-visitante': { gridId: 'trash-visitante', ...INGRESO_GRID_CONFIG },

  // Grid de backup - configuración estándar
  'backup-list': { gridId: 'backup-list', ...STANDARD_GRID_CONFIG }
};

/**
 * Helper para obtener la configuración de una grid
 */
export function getGridConfig(gridId: GridId): Omit<AGGridToolbarConfig, 'customButtons'> {
  return GRID_CONFIGS[gridId];
}