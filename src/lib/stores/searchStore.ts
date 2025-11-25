// ==========================================
// src/lib/stores/searchStore.ts
// ==========================================

import { writable, derived } from 'svelte/store';
import type { SearchState, SearchResult } from '$lib/types/search.types';

// Estado inicial
const initialState: SearchState = {
  query: '',
  results: [],
  isLoading: false,
  error: null,
};

// NUEVO: Store para el resultado seleccionado
interface SelectedSearchState {
  result: SearchResult | null;
}

const initialSelectedState: SelectedSearchState = {
  result: null,
};

// Store principal
function createSearchStore() {
  const { subscribe, set, update } = writable<SearchState>(initialState);

  return {
    subscribe,
    
    // Actualizar query
    setQuery: (query: string) => update(state => ({ ...state, query })),
    
    // Actualizar resultados
    setResults: (results: SearchResult[]) => 
      update(state => ({ ...state, results, isLoading: false, error: null })),
    
    // Setear loading
    setLoading: (isLoading: boolean) => 
      update(state => ({ ...state, isLoading })),
    
    // Setear error
    setError: (error: string) => 
      update(state => ({ ...state, error, isLoading: false })),
    
    // Limpiar búsqueda
    clear: () => set(initialState),
    
    // Reset solo resultados
    clearResults: () => 
      update(state => ({ ...state, results: [], error: null })),
  };
}

// NUEVO: Store para el resultado seleccionado
function createSelectedSearchStore() {
  const { subscribe, set, update } = writable<SelectedSearchState>(initialSelectedState);

  return {
    subscribe,
    
    // Seleccionar un resultado
    select: (result: SearchResult) => set({ result }),
    
    // Limpiar selección
    clear: () => set(initialSelectedState),
  };
}

export const searchStore = createSearchStore();
export const selectedSearchStore = createSelectedSearchStore();

// Derived stores útiles
export const hasResults = derived(
  searchStore,
  $store => $store.results.length > 0
);

export const isSearching = derived(
  searchStore,
  $store => $store.query.length > 0
);

export const hasSelectedResult = derived(
  selectedSearchStore,
  $store => $store.result !== null
);