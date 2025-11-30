<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { toast } from "svelte-5-french-toast";
  import type { SearchResult } from "$lib/types/search.types";
  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";

  const dispatch = createEventDispatcher();

  export let contratistaId: string = "";
  export let contratistaNombre: string = "";
  export let contratistaData: any = null;
  export let puedeIngresar: boolean = false;
  export let mensajeValidacion: string = "";

  async function handleContratistaSelect(event: CustomEvent<SearchResult>) {
    const selected = event.detail;

    if (selected.tipo !== "contratista") {
      toast.error("Por favor selecciona un contratista");
      return;
    }

    contratistaId = selected.id;
    contratistaNombre = selected.nombreCompleto || selected.id;

    // Validar si puede ingresar
    const result = await ingresoService.validarIngreso(selected.id);

    if (result.ok) {
      const data = result.data;
      puedeIngresar = data.puedeIngresar;
      contratistaData = data.contratista;

      if (!puedeIngresar) {
        mensajeValidacion = data.motivoRechazo || "No autorizado para ingresar";
        toast.error(mensajeValidacion);
      } else {
        mensajeValidacion = "";

        if (data.alertas && data.alertas.length > 0) {
          toast("⚠️ Tiene alertas pendientes", { icon: "⚠️" });
        }

        // Dispatch con datos del contratista para auto-selección
        dispatch("validated", { contratistaData: data.contratista });
      }
    } else {
      toast.error(result.error);
      puedeIngresar = false;
      contratistaData = null;
    }
  }

  function handleSearchClear() {
    contratistaId = "";
    contratistaNombre = "";
    puedeIngresar = false;
    mensajeValidacion = "";
    contratistaData = null;
    dispatch("clear");
  }
</script>

<div>
  <label
    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
  >
    Buscar Contratista
  </label>
  <SearchBar
    placeholder="Buscar por nombre o cédula..."
    limit={10}
    on:select={handleContratistaSelect}
    on:clear={handleSearchClear}
  />

  <!-- Tarjeta de información del contratista -->
  {#if contratistaNombre}
    <div
      class="mt-3 p-4 rounded-lg border-2 {puedeIngresar
        ? 'bg-green-50 border-green-300 dark:bg-green-900/20 dark:border-green-700'
        : 'bg-red-50 border-red-300 dark:bg-red-900/20 dark:border-red-700'}"
    >
      <div class="flex items-start justify-between">
        <div class="flex-1">
          <p class="font-bold text-lg text-gray-900 dark:text-white">
            {contratistaNombre}
          </p>
          {#if contratistaData?.empresaNombre}
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {contratistaData.empresaNombre}
            </p>
          {/if}
          {#if contratistaData?.cedula}
            <p class="text-xs text-gray-500 dark:text-gray-500 mt-1">
              Cédula: {contratistaData.cedula}
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

      {#if !puedeIngresar}
        <p class="text-sm mt-2 text-red-700 dark:text-red-300 font-medium">
          {mensajeValidacion}
        </p>
      {/if}
    </div>
  {/if}
</div>
