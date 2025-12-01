import { writable, derived } from 'svelte/store';
import type { IngresoResponse } from '$lib/types/ingreso';
import * as ingresoService from '$lib/logic/ingreso/ingresoService';

// Store principal para los ingresos activos (personas adentro)
function createIngresoStore() {
    const { subscribe, set, update } = writable<IngresoResponse[]>([]);

    return {
        subscribe,
        set,
        // Cargar datos desde el backend
        load: async () => {
            const result = await ingresoService.fetchAbiertos();
            if (result.ok) {
                set(result.data.ingresos);
            } else {
                console.error("Error al cargar ingresos activos:", result.error);
            }
        },
        // Agregar un nuevo ingreso a la lista local
        add: (ingreso: IngresoResponse) => {
            update(items => items ? [ingreso, ...items] : [ingreso]);
        },
        // Remover un ingreso (cuando sale)
        remove: (id: string) => {
            update(items => items.filter(i => i.id !== id));
        },
        // Actualizar un ingreso existente
        updateItem: (ingreso: IngresoResponse) => {
            update(items => items.map(i => i.id === ingreso.id ? ingreso : i));
        }
    };
}

export const ingresoStore = createIngresoStore();

// Stores derivados para contadores
export const totalPersonasAdentro = derived(ingresoStore, $ingresos => $ingresos.length);
export const contratistasAdentro = derived(ingresoStore, $ingresos =>
    $ingresos.filter(i => i.tipoIngreso === 'contratista').length
);
