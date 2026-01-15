import { writable, derived } from 'svelte/store';
import type { IngresoResponse, IngresoConEstadoResponse } from '$lib/types/ingreso';
import * as ingresoService from '$lib/logic/ingreso/ingresoService';

// ==========================================
// TIPOS
// ==========================================

interface IngresoStoreState {
	data: (IngresoResponse | IngresoConEstadoResponse)[];
	loading: boolean;
	error: string | null;
}

// ==========================================
// STORE FACTORY
// ==========================================

/**
 * Store para la lista de ingresos activos (personas adentro)
 *
 * Responsabilidades:
 * - Mantener lista de ingresos activos
 * - Cargar datos desde el backend
 * - Operaciones CRUD locales (optimistic updates)
 *
 * Este store SÍ puede hacer fetching porque es una lista de datos,
 * no un formulario con lógica de negocio compleja.
 */
function createIngresoStore() {
	const { subscribe, set, update } = writable<IngresoStoreState>({
		data: [],
		loading: false,
		error: null
	});

	return {
		subscribe,

		/**
		 * Cargar ingresos activos desde el backend
		 */
		load: async () => {
			update((state) => ({ ...state, loading: true, error: null }));

			const result = await ingresoService.fetchAbiertos();

			if (result.ok) {
				update((state) => ({
					...state,
					data: result.data,
					loading: false,
					error: null
				}));
			} else {
				console.error('Error al cargar ingresos activos:', result.error);
				update((state) => ({
					...state,
					loading: false,
					error: result.error
				}));
			}
		},

		/**
		 * Agregar un nuevo ingreso a la lista (optimistic update)
		 */
		add: (ingreso: IngresoResponse) => {
			update((state) => ({
				...state,
				data: [ingreso, ...state.data]
			}));
		},

		/**
		 * Remover un ingreso de la lista (cuando sale)
		 */
		remove: (id: string) => {
			update((state) => ({
				...state,
				data: state.data.filter((i) => i.id !== id)
			}));
		},

		/**
		 * Actualizar un ingreso existente en la lista
		 */
		updateItem: (ingreso: IngresoResponse) => {
			update((state) => ({
				...state,
				data: state.data.map((i) => (i.id === ingreso.id ? ingreso : i))
			}));
		},

		/**
		 * Limpiar error
		 */
		clearError: () => {
			update((state) => ({ ...state, error: null }));
		},

		/**
		 * Reset completo del store
		 */
		reset: () => {
			set({
				data: [],
				loading: false,
				error: null
			});
		}
	};
}

// ==========================================
// STORES
// ==========================================

export const ingresoStore = createIngresoStore();

// ==========================================
// DERIVED STORES
// ==========================================

/**
 * Total de personas adentro
 */
export const totalPersonasAdentro = derived(ingresoStore, ($store) => $store.data.length);

/**
 * Total de contratistas adentro
 */
export const contratistasAdentro = derived(
	ingresoStore,
	($store) => $store.data.filter((i) => i.tipoIngreso === 'contratista').length
);

/**
 * Indicador de carga
 */
export const isLoading = derived(ingresoStore, ($store) => $store.loading);

/**
 * Error actual si existe
 */
export const error = derived(ingresoStore, ($store) => $store.error);
