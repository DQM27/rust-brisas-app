<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { ProveedorCatalogItem } from "$lib/types/ingreso-nuevos";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import { ingresoProveedorService } from "$lib/services/ingresoProveedorService";
  import type { SearchResult } from "$lib/types/search.types";

  const dispatch = createEventDispatcher();

  export let selectedProveedor: ProveedorCatalogItem | null = null;
  // Props para validación (rich UI)
  export let puedeIngresar: boolean = false;
  export let mensajeValidacion: string = "";
  export let alertas: any[] = [];
  export let proveedorData: any = null; // Datos completos del proveedor validados

  let searchBar: any = null;

  async function handleSearch(term: string): Promise<SearchResult[]> {
    try {
      if (!term || term.length < 3) return [];
      const results = await ingresoProveedorService.searchProveedores(term);
      return results.map(
        (p) =>
          ({
            id: p.cedula,
            nombreCompleto: `${p.nombre} ${p.apellido}`,
            cedula: p.cedula,
            empresaNombre: p.empresaNombre || "Sin empresa",
            tipo: "proveedor",
            score: 1,
            // Helper metadata attached to object (JS dynamic nature)
            metadata: p,
          }) as any,
      );
    } catch (e) {
      console.error(e);
      return [];
    }
  }

  function handleSelect(event: CustomEvent<SearchResult>) {
    const item = event.detail as any;
    if (item.metadata) {
      dispatch("select", item.metadata);
    }
  }

  function handleClear() {
    dispatch("clear");
  }

  export function reset() {
    searchBar?.clear();
  }

  export function focus() {
    searchBar?.focus();
  }
</script>

<div class="mb-6">
  <label
    for="prov-search"
    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
  >
    Buscar Proveedor (Catálogo)
  </label>

  <SearchBar
    bind:this={searchBar}
    placeholder="Nombre, Apellido o Cédula..."
    searchFunction={handleSearch}
    on:select={handleSelect}
    on:clear={handleClear}
  />

  <!-- Tarjeta de información del proveedor -->
  {#if selectedProveedor || proveedorData}
    {@const data = proveedorData || selectedProveedor}
    {@const nombreCompleto =
      data.nombreCompleto || `${data.nombre} ${data.apellido}`}

    <div
      class="mt-3 p-4 rounded-lg border-2 {puedeIngresar
        ? 'bg-green-50 border-green-300 dark:bg-green-900/20 dark:border-green-700'
        : 'bg-red-50 border-red-300 dark:bg-red-900/20 dark:border-red-700'}"
    >
      <div class="flex items-start justify-between">
        <div class="flex-1">
          <p class="font-bold text-lg text-gray-900 dark:text-white">
            {nombreCompleto}
          </p>
          {#if data.cedula}
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
              Cédula: {data.cedula}
            </p>
          {/if}
          {#if data.empresaNombre || data.empresa_nombre}
            <p class="text-sm text-blue-600 dark:text-blue-400 mt-1">
              Empresa: {data.empresaNombre || data.empresa_nombre}
            </p>
          {/if}
        </div>
        <div>
          {#if puedeIngresar}
            <span
              class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
            >
              ✓ Autorizado
            </span>
          {:else}
            <span
              class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
            >
              ✗ No autorizado
            </span>
          {/if}
        </div>
      </div>

      <!-- Mensaje de validación -->
      {#if !puedeIngresar && mensajeValidacion}
        <div
          class="mt-3 p-2 bg-red-100 dark:bg-red-900/30 rounded text-sm text-red-800 dark:text-red-200"
        >
          {mensajeValidacion}
        </div>
      {/if}

      <!-- Alertas de Gafetes Pendientes -->
      {#if alertas && alertas.length > 0}
        <div class="mt-3 space-y-2">
          {#each alertas as alerta}
            <div
              class="flex items-center gap-2 p-2 rounded bg-yellow-50 border border-yellow-200 text-yellow-800 dark:bg-yellow-900/30 dark:border-yellow-700 dark:text-yellow-200 text-sm"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5 shrink-0"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fill-rule="evenodd"
                  d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                  clip-rule="evenodd"
                />
              </svg>
              <span>
                <strong>Gafete Pendiente:</strong> Debe devolver el gafete
                <span class="font-mono font-bold">#{alerta.gafeteNumero}</span>
              </span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
