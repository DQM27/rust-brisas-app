<script context="module" lang="ts">
  function action(node: HTMLInputElement) {
    // Placeholder
    return {};
  }
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  // import { debounce } from "lodash-es"; // Removed to avoid dependency
  import { fade } from "svelte/transition";

  // Simple debounce implementation
  function debounce<T extends (...args: any[]) => any>(
    func: T,
    wait: number,
  ): (...args: Parameters<T>) => void {
    let timeout: any;
    return (...args: Parameters<T>) => {
      clearTimeout(timeout);
      timeout = setTimeout(() => func(...args), wait);
    };
  }

  // Props
  export let scope: "all" | "contratista" | "proveedor" | "visita" = "all";
  export let autoFocus: boolean = true;

  const dispatch = createEventDispatcher();

  let query = "";
  let results: any[] = [];
  let loading = false;
  let error: string | undefined = undefined;
  let hasSearched = false;
  let focusedIndex = -1;

  // Debounced search
  const doSearch = debounce(async (q: string) => {
    if (!q || q.length < 3) {
      results = [];
      focusedIndex = -1;
      return;
    }

    loading = true;
    error = undefined;
    hasSearched = true;

    try {
      console.log(`Searching for "${q}" in scope ${scope}`);
      // Usamos search_global, el backend filtra o devolvemos todo y filtramos en front?
      // search_global busca en todo.
      // Si queremos especifico, podríamos usar search_contratistas, etc.
      // Por simplicidad del "Finder", usaremos search_global y filtraremos si es necesario.

      const rawResults: any[] = await invoke("search_global", {
        query: q,
        limit: 10,
      });

      // Filter by scope if needed (assuming backend returns 'type' field in SearchResultDto)
      // Backend SearchResultDto: { id, tipo, titulo, subtitulo, ... }
      if (scope !== "all") {
        results = rawResults.filter((r) => r.tipo.toLowerCase() === scope);
      } else {
        results = rawResults;
      }
      focusedIndex = results.length > 0 ? 0 : -1;
      console.log("[PersonaFinder] Search Results:", results);
    } catch (e: any) {
      console.error("Error searching:", e);
      error = "Error al buscar. Intente nuevamente.";
      results = [];
      focusedIndex = -1;
    } finally {
      loading = false;
    }
  }, 300);

  // Focus action
  function focus(node: HTMLInputElement, enabled: boolean) {
    if (enabled) node.focus();
    return {
      update(newEnabled: boolean) {
        if (newEnabled) node.focus();
      },
    };
  }

  function handleInput(e: Event) {
    const val = (e.target as HTMLInputElement).value;
    query = val;
    focusedIndex = -1;
    doSearch(val);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!results.length) return;

    if (e.key === "ArrowDown") {
      e.preventDefault();
      focusedIndex = (focusedIndex + 1) % results.length;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      focusedIndex = (focusedIndex - 1 + results.length) % results.length;
    } else if (e.key === "Enter") {
      if (focusedIndex >= 0) {
        e.preventDefault();
        selectItem(results[focusedIndex]);
      }
    }
  }

  function selectItem(item: any) {
    dispatch("select", {
      id: item.id,
      type: item.tipo.toLowerCase(),
      data: item,
    });
  }

  function getIcon(type: string) {
    switch (type.toLowerCase()) {
      case "contratista":
        return "👷";
      case "proveedor":
        return "🚚";
      case "usuario":
        return "👤";
      case "visita":
        return "👋";
      case "listanegra":
        return "🚫";
      default:
        return "❓";
    }
  }
</script>

<div class="w-full max-w-2xl mx-auto">
  <!-- Search Input -->
  <div class="relative">
    <div
      class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
    >
      {#if loading}
        <span class="loading loading-spinner loading-sm text-primary"></span>
      {:else}
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5 text-base-content/50"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
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
      type="text"
      class="input input-bordered w-full pl-10 text-lg shadow-sm focus:shadow-md transition-shadow"
      placeholder="Buscar por nombre, cédula o empresa..."
      bind:value={query}
      on:input={handleInput}
      on:keydown={handleKeydown}
      use:focus={autoFocus}
    />
  </div>

  <!-- Results Dropdown / List -->
  {#if hasSearched && query.length >= 3}
    <div
      class="mt-2 bg-base-100 rounded-box border border-base-200 shadow-xl overflow-hidden"
      transition:fade={{ duration: 100 }}
    >
      {#if results.length > 0}
        <ul class="menu p-0">
          {#each results as item, i}
            <li>
              <button
                class="flex gap-4 py-3 px-4 w-full text-left transition-colors {focusedIndex ===
                i
                  ? 'bg-primary/10 border-l-4 border-primary'
                  : 'hover:bg-base-200 border-l-4 border-transparent'}"
                on:click={() => selectItem(item)}
              >
                <div class="avatar placeholder">
                  <div
                    class="bg-neutral text-neutral-content rounded-full w-10"
                  >
                    <span class="text-xl">{getIcon(item.tipo)}</span>
                  </div>
                </div>
                <div class="flex-1 text-left">
                  <div class="font-bold text-base">
                    {item.nombreCompleto || item.titulo || "Sin nombre"}
                    {#if item.tipo === "ListaNegra"}
                      <span class="badge badge-error badge-sm ml-2"
                        >BLOQUEADO</span
                      >
                    {/if}
                  </div>
                  <div
                    class="text-xs text-base-content/60 uppercase font-semibold tracking-wide"
                  >
                    {item.tipo} • {item.empresaNombre ||
                      item.subtitulo ||
                      "Sin detalles"}
                  </div>
                  {#if item.cedula}
                    <div
                      class="text-[10px] font-mono opacity-50 bg-base-300 w-fit px-1 rounded mt-1"
                    >
                      ID: {item.cedula}
                    </div>
                  {/if}
                </div>
                <div class="text-right flex items-center">
                  <span class="btn btn-ghost btn-circle btn-sm">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="h-5 w-5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M9 5l7 7-7 7"
                      />
                    </svg>
                  </span>
                </div>
              </button>
            </li>
          {/each}
        </ul>
      {:else if !loading}
        <div class="p-8 text-center text-base-content/60">
          <p class="text-lg font-medium">No se encontraron resultados</p>
          <p class="text-sm">
            Intente con otra búsqueda o registre una nueva persona.
          </p>
          <!-- Opcional: Botón para crear nuevo -->
          <div class="mt-4">
            <button
              class="btn btn-outline btn-sm"
              on:click={() => dispatch("create")}
            >
              Nuevo Registro
            </button>
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#if error}
    <div class="alert alert-error mt-4 shadow-lg">
      <span>{error}</span>
    </div>
  {/if}
</div>
