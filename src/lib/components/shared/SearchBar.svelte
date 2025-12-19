<script lang="ts">
  // @ts-ignore
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  // @ts-ignore
  import { slide } from "svelte/transition";
  // @ts-ignore
  import { cubicOut } from "svelte/easing";
  import {
    searchStore,
    selectedSearchStore,
    hasResults,
  } from "$lib/stores/searchStore";
  import { performSearch } from "$lib/logic/search/performSearch";
  import type { SearchResult, SearchState } from "$lib/types/search.types";

  export let placeholder: string = "Buscar por nombre, cédula o empresa...";
  export let disabled: boolean = false;
  export let limit: number = 10;
  export let autofocus: boolean = false;
  export let searchFunction:
    | ((query: string) => Promise<SearchResult[]>)
    | null = null;

  export function focus() {
    inputRef?.focus();
  }

  let inputRef: HTMLInputElement;
  let query = "";
  let showDropdown = false;
  let highlightedIndex = -1;
  let debounceTimer: ReturnType<typeof setTimeout>;

  const dispatch = createEventDispatcher<{
    select: SearchResult;
    clear: void;
  }>();

  let results: SearchResult[] = [];
  $: results = ($searchStore as SearchState).results;
  $: isLoading = ($searchStore as SearchState).isLoading;
  $: error = ($searchStore as SearchState).error;
  $: selectedResult = ($selectedSearchStore as any).result;

  function handleInput() {
    clearTimeout(debounceTimer);

    if (query.trim().length < 2) {
      searchStore.clearResults();
      showDropdown = false;
      return;
    }

    debounceTimer = setTimeout(async () => {
      if (searchFunction) {
        // Custom local search
        searchStore.setLoading(true);
        try {
          const customResults = await searchFunction(query);
          searchStore.setResults(customResults);
        } catch (e) {
          console.error(e);
          searchStore.setError("Error en la búsqueda");
        } finally {
          searchStore.setLoading(false);
        }
      } else {
        // Default global search
        await performSearch(query, limit);
      }
      showDropdown = true;
      highlightedIndex = -1;
    }, 300);
  }

  function handleSelect(result: SearchResult) {
    selectedSearchStore.select(result);
    dispatch("select", result);
    query = "";
    showDropdown = false;
    highlightedIndex = -1;
    searchStore.clear();
  }

  export function clear() {
    query = "";
    showDropdown = false;
    highlightedIndex = -1;
    searchStore.clear();
    selectedSearchStore.clear();
    dispatch("clear");
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (!showDropdown || results.length === 0) return;

    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        highlightedIndex = Math.min(highlightedIndex + 1, results.length - 1);
        break;
      case "ArrowUp":
        event.preventDefault();
        highlightedIndex = Math.max(highlightedIndex - 1, -1);
        break;
      case "Enter":
        event.preventDefault();
        if (highlightedIndex >= 0 && highlightedIndex < results.length) {
          handleSelect(results[highlightedIndex]);
        }
        break;
      case "Escape":
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
    if (autofocus && inputRef) inputRef.focus();
    document.addEventListener("click", handleClickOutside);
  });

  onDestroy(() => {
    document.removeEventListener("click", handleClickOutside);
    clearTimeout(debounceTimer);
    searchStore.clear();
    selectedSearchStore.clear();
  });
</script>

<div class="relative w-full">
  <!-- Input -->
  <div class="relative">
    <div
      class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3"
    >
      {#if isLoading}
        <svg
          class="h-4 w-4 animate-spin text-gray-400"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="3"
          />
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
      {:else}
        <svg
          class="h-4 w-4 text-gray-500"
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
      {/if}
    </div>

    <input
      bind:this={inputRef}
      bind:value={query}
      on:input={handleInput}
      on:focus={() =>
        query.trim().length >= 2 && results.length > 0 && (showDropdown = true)}
      on:keydown={handleKeyDown}
      type="text"
      autocomplete="off"
      placeholder={selectedResult
        ? `Filtrando: ${selectedResult.nombreCompleto || selectedResult.id}`
        : placeholder}
      {disabled}
      class="w-full rounded-lg border bg-[#1e1e1e] pl-10 pr-16 py-2.5 text-sm text-white
        placeholder:text-gray-500 transition-colors
        focus:outline-none focus:ring-1
        disabled:opacity-50 disabled:cursor-not-allowed
        {selectedResult
        ? 'border-blue-500/50 focus:ring-blue-500/30'
        : 'border-white/10 hover:border-white/20 focus:border-white/30 focus:ring-white/10'}"
    />

    <div class="absolute inset-y-0 right-0 flex items-center gap-2 pr-3">
      {#if selectedResult || query}
        <button
          type="button"
          on:click={clear}
          class="p-1 text-gray-500 hover:text-gray-300 transition-colors"
          title="Limpiar"
        >
          <svg
            class="h-4 w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      {/if}

      {#if selectedResult}
        <span class="text-xs text-blue-400 font-medium">Filtrado</span>
      {:else if $hasResults && !isLoading && query.length >= 2}
        <span class="text-xs text-gray-500">{results.length}</span>
      {/if}
    </div>
  </div>

  <!-- Dropdown -->
  {#if showDropdown && query.trim().length >= 2}
    <div
      transition:slide={{ duration: 150, easing: cubicOut }}
      class="absolute z-50 mt-1 w-full rounded-lg border border-white/10 bg-[#1e1e1e] shadow-lg overflow-hidden"
    >
      {#if isLoading}
        <div class="px-4 py-6 text-center">
          <p class="text-sm text-gray-400">Buscando...</p>
        </div>
      {:else if error}
        <div class="px-4 py-3">
          <p class="text-sm text-red-400">{error}</p>
        </div>
      {:else if results.length === 0}
        <div class="px-4 py-6 text-center">
          <p class="text-sm text-gray-400">Sin resultados para "{query}"</p>
        </div>
      {:else}
        <div class="max-h-64 overflow-y-auto">
          {#each results as result, index}
            {@const typedResult = result as SearchResult}
            <button
              type="button"
              on:click={() => handleSelect(typedResult)}
              on:mouseenter={() => (highlightedIndex = index)}
              class="w-full px-4 py-2.5 text-left transition-colors
                {highlightedIndex === index
                ? 'bg-white/5'
                : 'hover:bg-white/5'}"
            >
              <div class="flex items-center justify-between">
                <div class="min-w-0 flex-1">
                  <p class="text-sm text-white truncate">
                    {typedResult.nombreCompleto || `ID: ${typedResult.id}`}
                  </p>
                  <div class="flex items-center gap-2 mt-0.5">
                    {#if typedResult.cedula}
                      <span class="text-xs text-gray-500"
                        >{typedResult.cedula}</span
                      >
                    {/if}
                    {#if typedResult.empresaNombre}
                      <span class="text-xs text-gray-600"
                        >• {typedResult.empresaNombre}</span
                      >
                    {/if}
                  </div>
                </div>
                <span class="text-[10px] text-gray-600 uppercase ml-2"
                  >{typedResult.tipo}</span
                >
              </div>
            </button>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
