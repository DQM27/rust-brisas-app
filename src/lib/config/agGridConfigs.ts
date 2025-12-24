// src/lib/config/agGridConfigs.ts

import type {
  GridId,
  ToolbarButtonDefinition,
  AGGridToolbarConfig
} from '$lib/types/agGrid';
import {
  Download,
  FileSpreadsheet,
  FileJson,
  Maximize2,
  Minimize2,
  RotateCw,
  CheckSquare,
  X,
  Copy,
  Filter,
  SlidersHorizontal,
  ChevronDown,
  ChevronRight,
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
  // Columnas
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
    id: 'reset-columns',
    label: 'Restaurar Columnas',
    icon: RotateCw,
    tooltip: 'Restaurar columnas a tamaño por defecto',
    category: 'columns'
  },

  // Export
  {
    id: 'export-csv',
    label: 'Exportar CSV',
    icon: Download,
    tooltip: 'Exportar datos a CSV',
    category: 'export'
  },
  {
    id: 'export-excel',
    label: 'Exportar Excel',
    icon: FileSpreadsheet,
    tooltip: 'Exportar datos a Excel',
    category: 'export'
  },
  {
    id: 'export-json',
    label: 'Exportar JSON',
    icon: FileJson,
    tooltip: 'Exportar datos a JSON',
    category: 'export'
  },

  // Selección
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

  // UI
  {
    id: 'toggle-filters',
    label: 'Filtros',
    icon: Filter,
    tooltip: 'Mostrar/Ocultar filtros',
    category: 'ui'
  },
  {
    id: 'toggle-sidebar',
    label: 'Panel Lateral',
    icon: SlidersHorizontal,
    tooltip: 'Mostrar/Ocultar panel lateral',
    category: 'ui'
  },

  // Data
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
  },

  // Grouping
  {
    id: 'expand-groups',
    label: 'Expandir Grupos',
    icon: ChevronDown,
    tooltip: 'Expandir todos los grupos',
    category: 'ui'
  },
  {
    id: 'collapse-groups',
    label: 'Contraer Grupos',
    icon: ChevronRight,
    tooltip: 'Contraer todos los grupos',
    category: 'ui'
  }
];

/**
 * Botones comunes para contexto SINGLE SELECT
 */
export const COMMON_SINGLE_SELECT_BUTTONS: ToolbarButtonDefinition[] = [
  {
    id: 'copy-selected',
    label: 'Copiar',
    icon: Copy,
    tooltip: 'Copiar fila seleccionada',
    category: 'selection'
  },
  {
    id: 'deselect',
    label: 'Cancelar',
    icon: X,
    tooltip: 'Cancelar selección',
    category: 'selection'
  }
];

/**
 * Botones comunes para contexto MULTI SELECT
 */
export const COMMON_MULTI_SELECT_BUTTONS: ToolbarButtonDefinition[] = [
  {
    id: 'copy-selected',
    label: 'Copiar',
    icon: Copy,
    tooltip: 'Copiar filas seleccionadas',
    category: 'selection'
  },
  {
    id: 'deselect-all',
    label: 'Cancelar',
    icon: X,
    tooltip: 'Cancelar todas las selecciones',
    category: 'selection'
  },
  {
    id: 'export-selection',
    label: 'Exportar Selección',
    icon: Download,
    tooltip: 'Exportar solo las filas seleccionadas',
    category: 'export'
  }
];

// ============================================
// Helpers para Crear Custom Buttons
// ============================================

export const createCustomButton = {
  nuevo: (onClick: () => void, disabled = false) => ({
    id: 'new-record',
    label: 'Nuevo',
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
 * Configuración de botones para CONTRATISTA LIST
 */
const CONTRATISTA_LIST_CONFIG: Omit<AGGridToolbarConfig, 'customButtons'> = {
  gridId: 'contratista-list',
  availableButtons: {
    default: [
      ...COMMON_DEFAULT_BUTTONS.filter(b => b.id === 'toggle-filters')
    ], // Los botones comunes se mueven a customButtons en el componente para controlar el orden (Nuevo primero)
    singleSelect: [
      ...COMMON_SINGLE_SELECT_BUTTONS.filter(b => b.id !== 'copy-selected')
      // Los custom buttons se agregan desde el componente
    ],
    multiSelect: [
      ...COMMON_MULTI_SELECT_BUTTONS.filter(b => !['copy-selected', 'export-selection'].includes(b.id))
    ]
  },
  showColumnSelector: true,
  showThemeSelector: true,
  enableGrouping: false
};

/**
 * Configuración de botones para LISTA NEGRA
 */
const LISTA_NEGRA_CONFIG: Omit<AGGridToolbarConfig, 'customButtons'> = {
  gridId: 'lista-negra-list',
  availableButtons: {
    default: [
      ...COMMON_DEFAULT_BUTTONS.filter(b => b.id === 'toggle-filters')
    ],
    singleSelect: [
      ...COMMON_SINGLE_SELECT_BUTTONS.filter(b => b.id !== 'copy-selected')
    ],
    multiSelect: [
      ...COMMON_MULTI_SELECT_BUTTONS.filter(b => !['copy-selected', 'export-selection'].includes(b.id))
    ]
  },
  showColumnSelector: true,
  showThemeSelector: true,
  enableGrouping: false
};

/**
 * Registry de configuraciones por grid
 */
export const GRID_CONFIGS: Record<GridId, Omit<AGGridToolbarConfig, 'customButtons'>> = {
  'contratista-list': CONTRATISTA_LIST_CONFIG,
  'proveedor-list': {
    gridId: 'proveedor-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b => b.id === 'toggle-filters')
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS.filter(b => b.id !== 'copy-selected')
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => !['copy-selected', 'export-selection'].includes(b.id))
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true,
    enableGrouping: false
  },
  'lista-negra-list': LISTA_NEGRA_CONFIG,
  'lista-negra': LISTA_NEGRA_CONFIG,

  // Placeholders para las demás grids
  'vehicles-list': {
    gridId: 'vehicles-list',
    availableButtons: {
      default: COMMON_DEFAULT_BUTTONS,
      singleSelect: COMMON_SINGLE_SELECT_BUTTONS,
      multiSelect: COMMON_MULTI_SELECT_BUTTONS
    },
    showColumnSelector: true,
    showThemeSelector: true
  },
  'badges-list': {
    gridId: 'badges-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b =>
          ['autosize-all', 'autosize-selected', 'reset-columns', 'toggle-filters'].includes(b.id)
        )
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS.filter(b => b.id !== 'copy-selected')
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => !['copy-selected', 'export-selection'].includes(b.id))
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true
  },
  'entries-list': {
    gridId: 'entries-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b =>
          ['autosize-all', 'reset-columns', 'refresh', 'toggle-filters'].includes(b.id)
        )
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => b.id !== 'export-selection')
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true,
    enableGrouping: false
  },
  'companies-list': {
    gridId: 'companies-list',
    availableButtons: {
      default: COMMON_DEFAULT_BUTTONS,
      singleSelect: COMMON_SINGLE_SELECT_BUTTONS,
      multiSelect: COMMON_MULTI_SELECT_BUTTONS
    },
    showColumnSelector: true,
    showThemeSelector: true
  },
  'users-list': {
    gridId: 'users-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b => b.id === 'toggle-filters')
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS.filter(b => b.id !== 'copy-selected')
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => !['copy-selected', 'export-selection'].includes(b.id))
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true,
    enableGrouping: false
  },
  'visitas-list': {
    gridId: 'visitas-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b => b.id === 'toggle-filters')
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS.filter(b => b.id !== 'copy-selected')
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => !['copy-selected', 'export-selection'].includes(b.id))
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true,
    enableGrouping: false
  },
  'visitas-activas-grid': {
    gridId: 'visitas-activas-grid',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b =>
          ['autosize-all', 'reset-columns', 'refresh', 'toggle-filters'].includes(b.id)
        )
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => b.id !== 'export-selection')
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true,
    enableGrouping: false
  },
  'proveedores-activos-grid': {
    gridId: 'proveedores-activos-grid',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b =>
          ['autosize-all', 'reset-columns', 'refresh', 'toggle-filters'].includes(b.id)
        )
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => b.id !== 'export-selection')
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true,
    enableGrouping: false
  },
  'proveedores-grid': {
    gridId: 'proveedores-grid',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b =>
          ['autosize-all', 'reset-columns', 'refresh', 'toggle-filters'].includes(b.id)
        )
      ],
      singleSelect: [
        ...COMMON_SINGLE_SELECT_BUTTONS
      ],
      multiSelect: [
        ...COMMON_MULTI_SELECT_BUTTONS.filter(b => b.id !== 'export-selection')
      ]
    },
    showColumnSelector: true,
    showThemeSelector: true,
    enableGrouping: false
  },
  'contratista-ingreso-list': {
    gridId: 'contratista-ingreso-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b => ['refresh', 'autosize-all'].includes(b.id))
      ],
      singleSelect: [],
      multiSelect: []
    },
    showColumnSelector: true,
    showThemeSelector: false,
    enableGrouping: false
  },
  'visita-ingreso-list': {
    gridId: 'visita-ingreso-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b => ['refresh', 'autosize-all'].includes(b.id))
      ],
      singleSelect: [],
      multiSelect: []
    },
    showColumnSelector: true,
    showThemeSelector: false,
    enableGrouping: false
  },
  'proveedor-ingreso-list': {
    gridId: 'proveedor-ingreso-list',
    availableButtons: {
      default: [
        ...COMMON_DEFAULT_BUTTONS.filter(b => ['refresh', 'autosize-all'].includes(b.id))
      ],
      singleSelect: [],
      multiSelect: []
    },
    showColumnSelector: true,
    showThemeSelector: false,
    enableGrouping: false
  }
};

/**
 * Helper para obtener la configuración de una grid
 */
export function getGridConfig(gridId: GridId): Omit<AGGridToolbarConfig, 'customButtons'> {
  return GRID_CONFIGS[gridId];
}