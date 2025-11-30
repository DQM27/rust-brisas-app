<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { toast } from "svelte-5-french-toast";
  import type { CreateIngresoContratistaInput } from "$lib/types/ingreso";
  import type { SearchResult } from "$lib/types/search.types";
  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import type { GafeteResponse } from "$lib/types/gafete";

  import SearchBar from "$lib/components/shared/SearchBar.svelte";

  export let loading = false;

  const dispatch = createEventDispatcher();

  // Estado del formulario
  let contratistaId = "";
  let contratistaNombre = "";
  let gafeteNumero = "";
  let tipoAutorizacion = "praind";
  let modoIngreso = "caminando";
  let observaciones = "";
  let vehiculoId: string | null = null;

  // Estado de validación
  let puedeIngresar = false;
  let mensajeValidacion = "";
  let gafetesDisponibles: GafeteResponse[] = [];

  // Cargar gafetes disponibles al montar
  import { onMount } from "svelte";
  onMount(async () => {
    const res = await gafeteService.fetchDisponibles();
    if (res.ok) {
      gafetesDisponibles = res.data;
    }
  });

  // Handler para cuando se selecciona un contratista del SearchBar
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

      if (!puedeIngresar) {
        mensajeValidacion = data.motivoRechazo || "No autorizado para ingresar";
        toast.error(mensajeValidacion);
      } else {
        mensajeValidacion = "";
        if (data.alertas && data.alertas.length > 0) {
          toast("Tiene alertas pendientes", { icon: "⚠️" });
        }
      }
    } else {
      toast.error(result.error);
      puedeIngresar = false;
    }
  }

  function handleSearchClear() {
    contratistaId = "";
    contratistaNombre = "";
    puedeIngresar = false;
    mensajeValidacion = "";
  }

  function handleSubmit() {
    if (!contratistaId || !puedeIngresar) return;

    const data: CreateIngresoContratistaInput = {
      contratistaId,
      tipoAutorizacion,
      modoIngreso,
      gafeteNumero: gafeteNumero || null,
      vehiculoId,
      observaciones: observaciones.trim() || null,
      usuarioIngresoId: "usuario_actual_id", // TODO: Obtener del store de auth
    };

    dispatch("submit", data);
  }
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
  <h2 class="text-xl font-bold mb-6 text-gray-900 dark:text-white">
    Registrar Ingreso
  </h2>

  <form on:submit|preventDefault={handleSubmit} class="space-y-6">
    <!-- Búsqueda de Contratista -->
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

      {#if contratistaNombre}
        <div
          class="mt-2 p-3 rounded-md {puedeIngresar
            ? 'bg-green-50 text-green-800 border-green-200 dark:bg-green-900/20 dark:text-green-200 dark:border-green-800'
            : 'bg-red-50 text-red-800 border-red-200 dark:bg-red-900/20 dark:text-red-200 dark:border-red-800'} border"
        >
          <p class="font-bold">{contratistaNombre}</p>
          {#if !puedeIngresar}
            <p class="text-sm mt-1">{mensajeValidacion}</p>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Detalles del Ingreso -->
    {#if puedeIngresar}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Gafete -->
        <div>
          <label
            for="gafete"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            Asignar Gafete
          </label>
          <select
            id="gafete"
            bind:value={gafeteNumero}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="">-- Sin Gafete --</option>
            {#each gafetesDisponibles as gafete}
              <option value={gafete.numero}
                >{gafete.numero} ({gafete.tipo})</option
              >
            {/each}
          </select>
        </div>

        <!-- Modo de Ingreso -->
        <div>
          <label
            for="modo"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            Modo de Ingreso
          </label>
          <select
            id="modo"
            bind:value={modoIngreso}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="caminando">Caminando</option>
            <option value="vehiculo">Vehículo</option>
          </select>
        </div>

        <!-- Autorización -->
        <div>
          <label
            for="auth"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            Autorización
          </label>
          <select
            id="auth"
            bind:value={tipoAutorizacion}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="praind">PRAIND</option>
            <option value="correo">Correo</option>
          </select>
        </div>
      </div>

      <!-- Observaciones -->
      <div>
        <label
          for="obs"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >
          Observaciones
        </label>
        <textarea
          id="obs"
          bind:value={observaciones}
          rows="2"
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
        ></textarea>
      </div>

      <!-- Botón Submit -->
      <div class="pt-4">
        <button
          type="submit"
          disabled={loading}
          class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
        >
          {#if loading}
            Registrando...
          {:else}
            Registrar Entrada
          {/if}
        </button>
      </div>
    {/if}
  </form>
</div>
