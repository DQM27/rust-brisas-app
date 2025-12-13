<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { SearchResult } from "$lib/types/search.types";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";

  /**
   * Componente de presentación para búsqueda de contratista
   *
   * Responsabilidades:
   * - Renderizar SearchBar
   * - Mostrar información del contratista validado
   * - Mostrar alertas de gafetes pendientes
   * - Emitir eventos al padre
   *
   * NO hace:
   * - Llamadas al service
   * - Validaciones de negocio
   * - Manejo de toasts
   */

  const dispatch = createEventDispatcher();

  // ==========================================
  // PROPS
  // ==========================================

  export let contratistaNombre: string = "";
  export let contratistaData: any | null = null;
  export let puedeIngresar: boolean = false;
  export let mensajeValidacion: string = "";

  // ==========================================
  // ESTADO LOCAL
  // ==========================================

  let searchBar: any = null;

  // ==========================================
  // HANDLERS
  // ==========================================

  function handleContratistaSelect(event: CustomEvent<SearchResult>) {
    const selected = event.detail;

    // Validación simple de tipo (UI level)
    if (selected.tipo !== "contratista") {
      // Emit evento de error para que el padre maneje
      dispatch("error", { message: "Por favor selecciona un contratista" });
      return;
    }

    // Emit evento con el ID seleccionado
    dispatch("select", {
      id: selected.id,
      nombreCompleto: selected.nombreCompleto || selected.id,
    });
  }

  function handleSearchClear() {
    dispatch("clear");
  }

  // ==========================================
  // MÉTODOS PÚBLICOS
  // ==========================================

  /**
   * Reset del componente (llamado desde el padre)
   */
  export function reset() {
    if (searchBar) {
      searchBar.clear();
    }
    handleSearchClear();
  }

  export function focus() {
    searchBar?.focus();
  }
</script>

<!-- 
  Sección de búsqueda de contratista
  Componente de presentación puro
-->

<div>
  <label
    for="contratista-search"
    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
  >
    Buscar Contratista
  </label>

  <SearchBar
    bind:this={searchBar}
    placeholder="Buscar por nombre o cédula..."
    limit={10}
    on:select={handleContratistaSelect}
    on:clear={handleSearchClear}
  />

  <!-- Tarjeta de información del contratista -->
  {#if contratistaNombre || contratistaData}
    {@const computedName =
      contratistaNombre ||
      contratistaData?.nombreCompleto ||
      [
        contratistaData?.nombre,
        contratistaData?.segundoNombre,
        contratistaData?.apellido,
        contratistaData?.segundoApellido,
      ]
        .filter(Boolean)
        .join(" ") ||
      "Contratista"}
    <div
      class="mt-3 p-4 rounded-lg border-2 {puedeIngresar
        ? 'bg-green-50 border-green-300 dark:bg-green-900/20 dark:border-green-700'
        : 'bg-red-50 border-red-300 dark:bg-red-900/20 dark:border-red-700'}"
    >
      <div class="flex items-start justify-between">
        <div class="flex-1">
          <p class="font-bold text-lg text-gray-900 dark:text-white">
            {computedName}
          </p>
          {#if contratistaData?.cedula}
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
              Cédula: {contratistaData.cedula}
            </p>
          {/if}
          {#if contratistaData?.empresa_nombre}
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Empresa: {contratistaData.empresa_nombre}
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

      <!-- Mensaje de validación (si no puede ingresar) -->
      <!-- OJO: No mostramos el mensaje genérico para evitar redundancia y proteger privacidad -->
      {#if !puedeIngresar && mensajeValidacion && mensajeValidacion !== "Contratista no autorizado para ingresar."}
        <div
          class="mt-3 p-2 bg-red-100 dark:bg-red-900/30 rounded text-sm text-red-800 dark:text-red-200"
        >
          {mensajeValidacion}
        </div>
      {/if}

      <!-- Alertas de Gafetes Pendientes -->
      {#if contratistaData?.alertas && contratistaData.alertas.length > 0}
        <div class="mt-3 space-y-2">
          {#each contratistaData.alertas as alerta}
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
