<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { ProveedorCatalogItem } from "$lib/types/ingreso-nuevos";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import { ingresoProveedorService } from "$lib/services/ingresoProveedorService";
  import type { SearchResult } from "$lib/types/search.types";

  const dispatch = createEventDispatcher();

  export let selectedProveedor: ProveedorCatalogItem | null = null;

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

  {#if selectedProveedor}
    <div
      class="mt-3 p-4 rounded-lg border border-blue-200 bg-blue-50 dark:bg-blue-900/20 dark:border-blue-700"
    >
      <div class="flex justify-between items-start">
        <div>
          <p class="font-bold text-gray-900 dark:text-white">
            {selectedProveedor.nombre}
            {selectedProveedor.apellido}
          </p>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {selectedProveedor.cedula}
          </p>
          {#if selectedProveedor.empresaNombre}
            <p class="text-xs text-blue-600 dark:text-blue-400 mt-1">
              Empresa: {selectedProveedor.empresaNombre}
            </p>
          {/if}
        </div>
        <span
          class="text-xs font-semibold bg-blue-100 text-blue-800 px-2 py-1 rounded"
        >
          Información Cargada
        </span>
      </div>
    </div>
  {/if}
</div>
