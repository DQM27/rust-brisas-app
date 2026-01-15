// src/lib/stores/searchStore.ts

import { writable, derived, get } from 'svelte/store';
import type { SearchState, SearchResult } from '$lib/types/search.types';

// ============================================
// Tipos internos del store
// ============================================

interface SelectedSearchState {
	result: SearchResult | null;
}

interface SearchCacheEntry {
	results: SearchResult[];
	timestamp: number;
}

interface SearchHistoryEntry {
	query: string;
	resultCount: number;
	timestamp: number;
}

// ============================================
// Configuración
// ============================================

const CONFIG = {
	/** Tiempo de vida del caché en milisegundos (5 minutos) */
	CACHE_TTL: 5 * 60 * 1000,
	/** Máximo de entradas en caché */
	MAX_CACHE_SIZE: 50,
	/** Máximo de entradas en historial */
	MAX_HISTORY_SIZE: 10
} as const;

// ============================================
// Estados iniciales
// ============================================

const initialState: SearchState = {
	query: '',
	results: [],
	isLoading: false,
	error: null
};

const initialSelectedState: SelectedSearchState = {
	result: null
};

// ============================================
// Store principal de búsqueda
// ============================================

function createSearchStore() {
	const { subscribe, set, update } = writable<SearchState>(initialState);

	// Caché interno de resultados
	const cache = new Map<string, SearchCacheEntry>();

	// Historial de búsquedas
	const history: SearchHistoryEntry[] = [];

	/**
	 * Normaliza el query para usar como clave de caché
	 */
	function normalizeQuery(query: string): string {
		return query.trim().toLowerCase();
	}

	/**
	 * Verifica si una entrada de caché es válida
	 */
	function isCacheValid(entry: SearchCacheEntry): boolean {
		return Date.now() - entry.timestamp < CONFIG.CACHE_TTL;
	}

	/**
	 * Limpia entradas antiguas del caché
	 */
	function cleanCache(): void {
		if (cache.size <= CONFIG.MAX_CACHE_SIZE) return;

		// Ordenar por timestamp y eliminar las más antiguas
		const entries = Array.from(cache.entries()).sort((a, b) => a[1].timestamp - b[1].timestamp);

		const toRemove = entries.slice(0, entries.length - CONFIG.MAX_CACHE_SIZE);
		toRemove.forEach(([key]) => cache.delete(key));
	}

	return {
		subscribe,

		/**
		 * Actualiza el query actual
		 */
		setQuery: (query: string) => {
			update((state) => ({ ...state, query }));
		},

		/**
		 * Establece los resultados de búsqueda
		 */
		setResults: (results: SearchResult[]) => {
			update((state) => {
				// Guardar en caché
				const normalizedQuery = normalizeQuery(state.query);
				if (normalizedQuery.length >= 2) {
					cache.set(normalizedQuery, {
						results,
						timestamp: Date.now()
					});
					cleanCache();

					// Agregar al historial
					addToHistory(normalizedQuery, results.length);
				}

				return {
					...state,
					results,
					isLoading: false,
					error: null
				};
			});
		},

		/**
		 * Activa/desactiva el estado de carga
		 */
		setLoading: (isLoading: boolean) => {
			update((state) => ({ ...state, isLoading }));
		},

		/**
		 * Establece un mensaje de error
		 */
		setError: (error: string) => {
			update((state) => ({
				...state,
				error,
				isLoading: false
			}));
		},

		/**
		 * Limpia todo el estado de búsqueda
		 */
		clear: () => {
			set(initialState);
		},

		/**
		 * Limpia solo los resultados (mantiene el query)
		 */
		clearResults: () => {
			update((state) => ({
				...state,
				results: [],
				error: null
			}));
		},

		/**
		 * Obtiene resultados del caché si existen y son válidos
		 */
		getFromCache: (query: string): SearchResult[] | null => {
			const normalizedQuery = normalizeQuery(query);
			const entry = cache.get(normalizedQuery);

			if (entry && isCacheValid(entry)) {
				return entry.results;
			}

			// Limpiar entrada inválida
			if (entry) {
				cache.delete(normalizedQuery);
			}

			return null;
		},

		/**
		 * Invalida todo el caché
		 */
		invalidateCache: () => {
			cache.clear();
		},

		/**
		 * Invalida una entrada específica del caché
		 */
		invalidateCacheEntry: (query: string) => {
			cache.delete(normalizeQuery(query));
		},

		/**
		 * Obtiene el historial de búsquedas recientes
		 */
		getHistory: (): SearchHistoryEntry[] => {
			return [...history];
		},

		/**
		 * Limpia el historial de búsquedas
		 */
		clearHistory: () => {
			history.length = 0;
		}
	};

	/**
	 * Agrega una entrada al historial de búsquedas
	 */
	function addToHistory(query: string, resultCount: number): void {
		// No duplicar entradas recientes
		const existingIndex = history.findIndex((h) => h.query === query);
		if (existingIndex !== -1) {
			history.splice(existingIndex, 1);
		}

		history.unshift({
			query,
			resultCount,
			timestamp: Date.now()
		});

		// Limitar tamaño del historial
		if (history.length > CONFIG.MAX_HISTORY_SIZE) {
			history.pop();
		}
	}
}

// ============================================
// Store de resultado seleccionado
// ============================================

function createSelectedSearchStore() {
	const { subscribe, set } = writable<SelectedSearchState>(initialSelectedState);

	return {
		subscribe,

		/**
		 * Selecciona un resultado
		 */
		select: (result: SearchResult) => {
			set({ result });
		},

		/**
		 * Limpia la selección actual
		 */
		clear: () => {
			set(initialSelectedState);
		},

		/**
		 * Obtiene el resultado seleccionado actual
		 */
		get: (): SearchResult | null => {
			return get({ subscribe }).result;
		}
	};
}

// ============================================
// Exports
// ============================================

export const searchStore = createSearchStore();
export const selectedSearchStore = createSelectedSearchStore();

// ============================================
// Derived stores
// ============================================

/** Indica si hay resultados disponibles */
export const hasResults = derived(searchStore, ($store) => $store.results.length > 0);

/** Indica si hay una búsqueda activa */
export const isSearching = derived(searchStore, ($store) => $store.query.length >= 2);

/** Indica si hay un resultado seleccionado */
export const hasSelectedResult = derived(selectedSearchStore, ($store) => $store.result !== null);

/** Cantidad de resultados actual */
export const resultCount = derived(searchStore, ($store) => $store.results.length);

/** Estado combinado para conveniencia */
export const searchStatus = derived(searchStore, ($store) => ({
	isEmpty: $store.results.length === 0 && !$store.isLoading && !$store.error,
	isLoading: $store.isLoading,
	hasError: $store.error !== null,
	hasResults: $store.results.length > 0
}));
