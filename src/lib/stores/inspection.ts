// src/lib/stores/inspection.ts
import { writable, derived } from 'svelte/store';

export type InspectionType = 'contratista' | 'ingreso' | 'vehiculo' | 'empresa' | 'gafete';

export interface InspectionState {
  visible: boolean;
  type: InspectionType | null;
  data: any | null;
  title: string;
}

const initialState: InspectionState = {
  visible: false,
  type: null,
  data: null,
  title: 'Inspección'
};

function createInspectionStore() {
  const store = writable<InspectionState>(initialState);
  const { subscribe, set, update } = store;

  return {
    subscribe,
    
    /**
     * Inspecciona un elemento (abre el panel si está cerrado)
     */
    inspect: (type: InspectionType, data: any, title?: string) => {
      update(state => ({
        visible: true,
        type,
        data,
        title: title || getTitleForType(type, data)
      }));
    },

    /**
     * Actualiza los datos del elemento actual sin cambiar tipo
     */
    updateData: (data: any) => {
      update(state => ({
        ...state,
        data
      }));
    },

    /**
     * Cierra el panel y limpia datos
     */
    close: () => {
      set(initialState);
    },

    /**
     * Toggle visibility (mantiene datos)
     */
    toggle: () => {
      update(state => ({
        ...state,
        visible: !state.visible
      }));
    },

    /**
     * Solo cambia visibilidad
     */
    setVisible: (visible: boolean) => {
      update(state => ({
        ...state,
        visible
      }));
    },

    /**
     * Reset completo
     */
    reset: () => {
      set(initialState);
    }
  };
}

/**
 * Genera título automático basado en tipo y datos
 */
function getTitleForType(type: InspectionType, data: any): string {
  switch (type) {
    case 'contratista':
      return data?.nombre && data?.apellido 
        ? `${data.nombre} ${data.apellido}`
        : 'Contratista';
    case 'ingreso':
      return 'Detalles de Ingreso';
    case 'vehiculo':
      return data?.placa || 'Vehículo';
    case 'empresa':
      return data?.nombre || 'Empresa';
    case 'gafete':
      return data?.numero ? `Gafete #${data.numero}` : 'Gafete';
    default:
      return 'Inspección';
  }
}

export const inspection = createInspectionStore();

// Tipo explícito para TypeScript
export type InspectionStore = ReturnType<typeof createInspectionStore>;