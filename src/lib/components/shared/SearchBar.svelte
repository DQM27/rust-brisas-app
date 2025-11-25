<!-- ==========================================
// src/lib/components/shared/SearchBar.svelte
// ========================================== -->

<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { searchStore, hasResults } from '$lib/stores/searchStore';
  import { performSearch } from '$lib/logic/search/performSearch';
  import type { SearchResult } from '$lib/types/search.types';

  // Props
  export let placeholder: string = 'Buscar por nombre, cédula o empresa...';
  export let disabled: boolean = false;
  export let limit: number = 10;
  export let autofocus: boolean = false;

  // Estado local
  let inputRef: HTMLInputElement;
  let query = '';
  let showDropdown = false;
  let highlightedIndex = -1;
  let debounceTimer: ReturnType<typeof setTimeout>;

  const dispatch = createEventDispatcher<{
    select: SearchResult;
    clear: void;
  }>();

  // Subscripciones al store
  $: results = $searchStore.results;
  $: isLoading = $searchStore.isLoading;
  $: error = $searchStore.error;

  // Debounced search
  function handleInput() {
    clearTimeout(debounceTimer);
    
    if (query.trim().length < 2) {
      searchStore.clearResults();
      showDropdown = false;
      return;
    }

    debounceTimer = setTimeout(async () => {
      await performSearch(query, limit);
      showDropdown = true;
      highlightedIndex = -1;
    }, 300);
  }

  function handleSelect(result: SearchResult) {
    dispatch('select', result);
    clear();
  }

  function clear() {
    query = '';
    showDropdown = false;
    highlightedIndex = -1;
    searchStore.clear();
    dispatch('clear');
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (!showDropdown || results.length === 0) return;

    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        highlightedIndex = Math.min(highlightedIndex + 1, results.length - 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        highlightedIndex = Math.max(highlightedIndex - 1, -1);
        break;
      case 'Enter':
        event.preventDefault();
        if (highlightedIndex >= 0 && highlightedIndex < results.length) {
          handleSelect(results[highlightedIndex]);
        }
        break;
      case 'Escape':
        event.preventDefault();
        showDropdown = false;
        highlightedIndex = -1;
        break;
    }
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as Node;
    if (inputRef && !inputRef.parentElement?.contains(target)) {
      showDropdown = false;
    }
  }

  onMount(() => {
    if (autofocus && inputRef) {
      inputRef.focus();
    }
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
      clearTimeout(debounceTimer);
    };
  });
</script>

<div class="relative w-full">
  <!-- Input Container -->
  <div class="relative">
    <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3.5">
      <svg
        class="h-5 w-5 text-gray-400"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
    </div>

    <input
      bind:this={inputRef}
      bind:value={query}
      on:input={handleInput}
      on:focus={() => query.trim().length >= 2 && results.length > 0 && (showDropdown = true)}
      on:keydown={handleKeyDown}
      type="text"
      {placeholder}
      {disabled}
      class="w-full rounded-xl border border-white/10 bg-[#2d2d2d] pl-11 pr-20 py-3.5 text-sm text-white 
             placeholder:text-gray-500 
             focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/20 focus:outline-none 
             disabled:opacity-50 disabled:cursor-not-allowed
             transition-all duration-200"
    />

    <div class="absolute inset-y-0 right-0 flex items-center gap-1 pr-3">
      {#if isLoading}
        <svg
          class="h-5 w-5 animate-spin text-blue-500"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="4"
          />
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
      {:else if query}
        <button
          type="button"
          on:click={clear}
          class="rounded-lg p-1.5 text-gray-400 hover:bg-white/5 hover:text-gray-300 transition-colors"
          title="Limpiar búsqueda"
        >
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      {/if}

      {#if $hasResults && !isLoading}
        <span class="text-xs text-gray-500 font-medium">
          {results.length}
        </span>
      {/if}
    </div>
  </div>

  <!-- Dropdown Results -->
  {#if showDropdown && query.trim().length >= 2}
    <div
      transition:slide={{ duration: 200, easing: cubicOut }}
      class="absolute z-50 mt-2 w-full rounded-xl border border-white/10 bg-[#252526] shadow-2xl overflow-hidden"
    >
      {#if isLoading}
        <div class="p-8 flex flex-col items-center justify-center gap-3">
          <svg
            class="h-8 w-8 animate-spin text-blue-500"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          <p class="text-sm text-gray-400">Buscando...</p>
        </div>
      {:else if error}
        <div class="p-6 flex items-start gap-3">
          <svg class="h-5 w-5 text-red-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <div>
            <p class="text-sm font-medium text-red-400">Error al buscar</p>
            <p class="text-xs text-gray-500 mt-1">{error}</p>
          </div>
        </div>
      {:else if results.length === 0}
        <div class="p-8 flex flex-col items-center justify-center gap-2">
          <svg class="h-12 w-12 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="1.5"
              d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <p class="text-sm font-medium text-gray-400">No se encontraron resultados</p>
          <p class="text-xs text-gray-500">Intenta con otros términos de búsqueda</p>
        </div>
      {:else}
        <div class="max-h-80 overflow-y-auto">
          {#each results as result, index (result.id)}
            <button
              type="button"
              on:click={() => handleSelect(result)}
              on:mouseenter={() => (highlightedIndex = index)}
              class="w-full px-4 py-3.5 text-left border-b border-white/5 last:border-b-0 transition-all duration-150
                     {highlightedIndex === index 
                       ? 'bg-blue-500/10 border-l-2 border-l-blue-500' 
                       : 'hover:bg-white/5'}"
            >
              <div class="flex items-center justify-between gap-3">
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="text-sm font-medium text-white truncate">
                      ID: {result.id}
                    </span>
                    <span class="inline-flex items-center rounded-full bg-blue-500/10 px-2 py-0.5 text-xs font-medium text-blue-400">
                      {result.tipo}
                    </span>
                  </div>
                  <p class="text-xs text-gray-500 mt-0.5">
                    Relevancia: {(result.score * 100).toFixed(0)}%
                  </p>
                </div>
                <svg
                  class="h-5 w-5 text-gray-500 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 5l7 7-7 7"
                  />
                </svg>
              </div>
            </button>
          {/each}
        </div>

        <div class="border-t border-white/10 px-4 py-2.5 bg-[#1e1e1e]">
          <p class="text-xs text-gray-500 text-center">
            Usa ↑↓ para navegar, Enter para seleccionar, Esc para cerrar
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>