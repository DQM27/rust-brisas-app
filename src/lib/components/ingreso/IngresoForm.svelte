<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-5-french-toast";
  import type { CreateIngresoContratistaInput } from "$lib/types/ingreso";
  import type { SearchResult } from "$lib/types/search.types";
  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import type { GafeteResponse } from "$lib/types/gafete";

  import SearchBar from "$lib/components/shared/SearchBar.svelte";

  export let loading = false;

  const dispatch = createEventDispatcher();

  // ==========================================
  // ESTADO DEL CONTRATISTA
  // ==========================================
  let contratistaId = "";
  let contratistaNombre = "";
  let contratistaData: any = null; // Datos completos del contratista
  let puedeIngresar = false;
  let mensajeValidacion = "";

  // ==========================================
  // ESTADO DEL FORMULARIO
  // ==========================================
  let modoIngreso: "caminando" | "vehiculo" = "caminando";
  let vehiculoId: string | null = null;
  let gafeteNumero = "";
  let tipoAutorizacion = "praind";
  let observaciones = "";

  // ==========================================
  // GAFETES DISPONIBLES
  // ==========================================
  let gafetesDisponibles: GafeteResponse[] = [];
  let gafeteValido = false;
  let gafeteSugerencias: string[] = [];

  // Cargar gafetes disponibles tipo "contratista"
  onMount(async () => {
    const res = await gafeteService.fetchDisponibles();
    if (res.ok) {
      // Filtrar solo gafetes de tipo contratista
      gafetesDisponibles = res.data.filter(g => g.tipo === 'contratista');
    }
  });

  // ==========================================
  // VALIDACIÓN DE GAFETE EN TIEMPO REAL
  // ==========================================
  $: {
    if (gafeteNumero.trim()) {
      const normalizado = gafeteNumero.trim().toUpperCase();
      gafeteValido = gafetesDisponibles.some(
        g => g.numero === normalizado && g.estaDisponible
      );
      
      // Generar sugerencias
      if (!gafeteValido) {
        gafeteSugerencias = gafetesDisponibles
          .filter(g => g.numero.includes(normalizado))
          .map(g => g.numero)
          .slice(0, 5);
      } else {
        gafeteSugerencias = [];
      }
    } else {
      gafeteValido = true; // Sin gafete es válido
      gafeteSugerencias = [];
    }
  }

  // ==========================================
  // SELECCIÓN DE CONTRATISTA
  // ==========================================
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
      contratistaData = data.contratista; // Guardar datos completos

      if (!puedeIngresar) {
        mensajeValidacion = data.motivoRechazo || "No autorizado para ingresar";
        toast.error(mensajeValidacion);
      } else {
        mensajeValidacion = "";
        
        // Auto-selección inteligente
        if (contratistaData?.vehiculos?.length === 1) {
          // Si tiene 1 solo vehículo, pre-seleccionarlo
          vehiculoId = contratistaData.vehiculos[0].id;
          modoIngreso = "vehiculo";
        } else if (contratistaData?.vehiculos?.length === 0) {
          // Si no tiene vehículos, forzar caminando
          modoIngreso = "caminando";
          vehiculoId = null;
        }

        if (data.alertas && data.alertas.length > 0) {
          toast("⚠️ Tiene alertas pendientes", { icon: "⚠️" });
        }
      }
    } else {
      toast.error(result.error);
      puedeIngresar = false;
      contratistaData = null;
    }
  }

  function handleSearchClear() {
    resetForm();
  }

  function resetForm() {
    contratistaId = "";
    contratistaNombre = "";
    contratistaData = null;
    puedeIngresar = false;
    mensajeValidacion = "";
    modoIngreso = "caminando";
    vehiculoId = null;
    gafeteNumero = "";
    observaciones = "";
  }

  // ==========================================
  // CAMBIO DE MODO INGRESO
  // ==========================================
  function handleModoChange(modo: "caminando" | "vehiculo") {
    modoIngreso = modo;
    if (modo === "caminando") {
      vehiculoId = null;
    }
  }

  // ==========================================
  // SUBMIT
  // ==========================================
  function handleSubmit() {
    if (!contratistaId || !puedeIngresar) return;

    // Validar gafete si se proporcionó
    if (gafeteNumero.trim() && !gafeteValido) {
      toast.error("Número de gafete inválido o no disponible");
      return;
    }

    // Validar vehículo si modo = vehiculo
    if (modoIngreso === "vehiculo" && !vehiculoId) {
      toast.error("Selecciona un vehículo");
      return;
    }

    const data: CreateIngresoContratistaInput = {
      contratistaId,
      tipoAutorizacion,
      modoIngreso,
      gafeteNumero: gafeteNumero.trim().toUpperCase() || null,
      vehiculoId: modoIngreso === "vehiculo" ? vehiculoId : null,
      observaciones: observaciones.trim() || null,
      usuarioIngresoId: "usuario_actual_id", // TODO: Obtener del store de auth
    };

    dispatch("submit", data);
    
    // Resetear solo los campos del formulario, mantener contratista
    gafeteNumero = "";
    observaciones = "";
  }

  // Computed: tiene vehículos registrados?
  $: tieneVehiculos = contratistaData?.vehiculos?.length > 0;
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
  <h2 class="text-xl font-bold mb-6 text-gray-900 dark:text-white">
    Registrar Ingreso
  </h2>

  <form on:submit|preventDefault={handleSubmit} class="space-y-6">
    <!-- ========================================== -->
    <!-- BÚSQUEDA DE CONTRATISTA -->
    <!-- ========================================== -->
    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
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
                <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
                  ✓ Autorizado
                </span>
              {:else}
                <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200">
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

    <!-- ========================================== -->
    <!-- DETALLES DEL INGRESO (Solo si puede ingresar) -->
    <!-- ========================================== -->
    {#if puedeIngresar}
      <!-- MODO DE INGRESO - Radio Buttons Visuales -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
          Modo de Ingreso
        </label>
        <div class="grid grid-cols-2 gap-3">
          <!-- Caminando -->
          <button
            type="button"
            on:click={() => handleModoChange("caminando")}
            class="relative flex items-center justify-center p-4 border-2 rounded-lg transition-all {modoIngreso === 'caminando'
              ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 dark:border-blue-400'
              : 'border-gray-300 hover:border-gray-400 dark:border-gray-600 dark:hover:border-gray-500'}"
          >
            <div class="text-center">
              <svg class="w-8 h-8 mx-auto mb-2 {modoIngreso === 'caminando' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-500'}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
              <span class="text-sm font-medium {modoIngreso === 'caminando' ? 'text-blue-700 dark:text-blue-300' : 'text-gray-700 dark:text-gray-300'}">
                Caminando
              </span>
            </div>
            {#if modoIngreso === "caminando"}
              <div class="absolute top-2 right-2 w-5 h-5 bg-blue-600 rounded-full flex items-center justify-center">
                <svg class="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                </svg>
              </div>
            {/if}
          </button>

          <!-- Vehículo -->
          <button
            type="button"
            on:click={() => handleModoChange("vehiculo")}
            disabled={!tieneVehiculos}
            class="relative flex items-center justify-center p-4 border-2 rounded-lg transition-all {modoIngreso === 'vehiculo'
              ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 dark:border-blue-400'
              : 'border-gray-300 hover:border-gray-400 dark:border-gray-600 dark:hover:border-gray-500'} {!tieneVehiculos ? 'opacity-50 cursor-not-allowed' : ''}"
          >
            <div class="text-center">
              <svg class="w-8 h-8 mx-auto mb-2 {modoIngreso === 'vehiculo' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-500'}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17a2 2 0 11-4 0 2 2 0 014 0zM19 17a2 2 0 11-4 0 2 2 0 014 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16V6a1 1 0 00-1-1H4a1 1 0 00-1 1v10a1 1 0 001 1h1m8-1a1 1 0 01-1 1H9m4-1V8a1 1 0 011-1h2.586a1 1 0 01.707.293l3.414 3.414a1 1 0 01.293.707V16a1 1 0 01-1 1h-1m-6-1a1 1 0 001 1h1M5 17a2 2 0 104 0m-4 0a2 2 0 114 0m6 0a2 2 0 104 0m-4 0a2 2 0 114 0" />
              </svg>
              <span class="text-sm font-medium {modoIngreso === 'vehiculo' ? 'text-blue-700 dark:text-blue-300' : 'text-gray-700 dark:text-gray-300'}">
                Vehículo
              </span>
            </div>
            {#if modoIngreso === "vehiculo"}
              <div class="absolute top-2 right-2 w-5 h-5 bg-blue-600 rounded-full flex items-center justify-center">
                <svg class="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                </svg>
              </div>
            {/if}
          </button>
        </div>
        
        {#if !tieneVehiculos}
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
            Este contratista no tiene vehículos registrados
          </p>
        {/if}
      </div>

      <!-- SELECTOR DE VEHÍCULO (Solo si modo = vehiculo) -->
      {#if modoIngreso === "vehiculo" && tieneVehiculos}
        <div class="pl-4 border-l-4 border-blue-500">
          <label for="vehiculo" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Seleccionar Vehículo
          </label>
          <select
            id="vehiculo"
            bind:value={vehiculoId}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value={null}>-- Seleccione vehículo --</option>
            {#each contratistaData.vehiculos as vehiculo}
              <option value={vehiculo.id}>
                {vehiculo.placa} - {vehiculo.marca} {vehiculo.modelo} ({vehiculo.tipo})
              </option>
            {/each}
          </select>
        </div>
      {/if}

      <!-- GAFETE - Input de texto -->
      <div>
        <label for="gafete" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Número de Gafete (Opcional)
        </label>
        <div class="relative">
          <input
            type="text"
            id="gafete"
            bind:value={gafeteNumero}
            placeholder="Ej: 027"
            class="w-full px-3 py-2 border {gafeteNumero.trim() && !gafeteValido ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'} rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white uppercase font-mono"
            maxlength="20"
          />
          {#if gafeteNumero.trim()}
            <div class="absolute right-3 top-2.5">
              {#if gafeteValido}
                <svg class="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                </svg>
              {:else}
                <svg class="w-5 h-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
                </svg>
              {/if}
            </div>
          {/if}
        </div>
        
        <!-- Sugerencias -->
        {#if gafeteSugerencias.length > 0}
          <div class="mt-2 p-2 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md">
            <p class="text-xs text-yellow-800 dark:text-yellow-200 mb-1">Sugerencias disponibles:</p>
            <div class="flex flex-wrap gap-1">
              {#each gafeteSugerencias as sugerencia}
                <button
                  type="button"
                  on:click={() => gafeteNumero = sugerencia}
                  class="px-2 py-1 text-xs bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-50 dark:hover:bg-gray-600"
                >
                  {sugerencia}
                </button>
              {/each}
            </div>
          </div>
        {/if}
        
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Disponibles: {gafetesDisponibles.length} gafetes tipo contratista
        </p>
      </div>

      <!-- AUTORIZACIÓN -->
      <div>
        <label for="auth" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Tipo de Autorización
        </label>
        <select
          id="auth"
          bind:value={tipoAutorizacion}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
        >
          <option value="praind">PRAIND</option>
          <option value="correo">Correo/Autorización especial</option>
        </select>
      </div>

      <!-- OBSERVACIONES -->
      <div>
        <label for="obs" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Observaciones
        </label>
        <textarea
          id="obs"
          bind:value={observaciones}
          rows="2"
          maxlength="500"
          placeholder="Comentarios adicionales..."
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white text-sm"
        ></textarea>
      </div>

      <!-- BOTÓN SUBMIT -->
      <div class="pt-2">
        <button
          type="submit"
          disabled={loading || !puedeIngresar || (gafeteNumero.trim() && !gafeteValido) || (modoIngreso === 'vehiculo' && !vehiculoId)}
          class="w-full flex justify-center items-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {#if loading}
            <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Registrando...
          {:else}
            ✓ Registrar Entrada
          {/if}
        </button>
      </div>
    {/if}
  </form>
</div>