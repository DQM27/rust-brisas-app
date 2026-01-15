// src/lib/logic/search/performSearch.ts

import { searchStore } from '$lib/stores/searchStore';
import { searchGlobal } from '$lib/api/searchService';

// ============================================
// Tipos internos
// ============================================

interface SearchOptions {
	limit?: number;
	useCache?: boolean;
}

// ============================================
// Configuración por defecto
// ============================================

const DEFAULT_LIMIT = 10;

// ============================================
// Control de búsquedas concurrentes
// ============================================

/** ID de la búsqueda actual para cancelar obsoletas */
let currentSearchId = 0;

/**
 * Realiza una búsqueda de contratistas con debounce y caché.
 *
 * @param query - Texto a buscar (mínimo 2 caracteres)
 * @param optionsOrLimit - Límite de resultados (número) u opciones de búsqueda
 *
 * @example
 * ```ts
 * // Forma simple (retrocompatible)
 * await performSearch('Juan', 10);
 *
 * // Con opciones
 * await performSearch('Juan', { limit: 20, useCache: false });
 * ```
 */
export async function performSearch(
	query: string,
	optionsOrLimit: number | SearchOptions = {}
): Promise<void> {
	// Normalizar opciones (retrocompatibilidad con limit como número)
	const opts: SearchOptions =
		typeof optionsOrLimit === 'number'
			? { limit: optionsOrLimit, useCache: true }
			: { limit: DEFAULT_LIMIT, useCache: true, ...optionsOrLimit };

	const trimmedQuery = query.trim();

	// Validación mínima
	if (trimmedQuery.length < 2) {
		searchStore.clearResults();
		return;
	}

	// Generar ID único para esta búsqueda
	const searchId = ++currentSearchId;

	// Verificar caché primero
	if (opts.useCache) {
		const cachedResults = searchStore.getFromCache(trimmedQuery);
		if (cachedResults !== null) {
			// Verificar que no haya una búsqueda más reciente
			if (searchId === currentSearchId) {
				searchStore.setQuery(trimmedQuery);
				searchStore.setResults(cachedResults);
			}
			return;
		}
	}

	// Iniciar búsqueda
	searchStore.setQuery(trimmedQuery);
	searchStore.setLoading(true);

	try {
		const results = await searchGlobal(trimmedQuery, opts.limit);

		// Solo actualizar si esta búsqueda sigue siendo la actual
		// (evita race conditions con búsquedas más recientes)
		if (searchId === currentSearchId) {
			searchStore.setResults(results);
		}
	} catch (err: unknown) {
		// Solo mostrar error si esta búsqueda sigue siendo la actual
		if (searchId === currentSearchId) {
			const errorMessage = extractErrorMessage(err);
			searchStore.setError(errorMessage);
		}
	}
}

/**
 * Cancela cualquier búsqueda en progreso.
 * Útil cuando el componente se desmonta o se necesita limpiar el estado.
 */
export function cancelSearch(): void {
	currentSearchId++;
	searchStore.setLoading(false);
}

/**
 * Realiza una búsqueda sin usar caché.
 * Útil cuando se sabe que los datos pueden haber cambiado.
 */
export async function performFreshSearch(query: string, limit?: number): Promise<void> {
	// Invalidar caché para este query específico
	searchStore.invalidateCacheEntry(query);

	return performSearch(query, {
		limit,
		useCache: false
	});
}

/**
 * Invalida todo el caché de búsquedas.
 * Útil después de operaciones que modifican datos (crear, editar, eliminar).
 */
export function invalidateSearchCache(): void {
	searchStore.invalidateCache();
}

// ============================================
// Utilidades internas
// ============================================

/**
 * Extrae un mensaje de error legible de diferentes tipos de error
 */
function extractErrorMessage(err: unknown): string {
	if (err instanceof Error) {
		return err.message;
	}

	if (typeof err === 'string') {
		return err;
	}

	if (err && typeof err === 'object') {
		// Manejar errores de Tauri
		if ('message' in err && typeof err.message === 'string') {
			return err.message;
		}

		// Intentar serializar el objeto
		try {
			return JSON.stringify(err);
		} catch {
			return 'Error desconocido al buscar';
		}
	}

	return 'Error desconocido al buscar';
}
