<script lang="ts">
  import { onMount } from "svelte";
  import { ingresoFormStore } from "$lib/stores/ingresoFormStore";
  import { currentUser } from "$lib/stores/auth";
  import * as controller from "$lib/logic/ingreso/ingresoFormController";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import type { GafeteResponse } from "$lib/types/gafete";

  // Componentes hijos (presentacionales)
  // Componentes hijos (presentacionales)
  import ContratistaSearchSection from "./ContratistaSearchSection.svelte";
  import VehiculoSelector from "./VehiculoSelector.svelte";

  // Common components
  import ModoIngresoSelector from "../common/ModoIngresoSelector.svelte";
  import GafeteInput from "../common/GafeteInput.svelte";
  import IngresoFormFields from "../common/IngresoFormFields.svelte";
  import IngresoObservaciones from "../common/IngresoObservaciones.svelte";

  import { X } from "lucide-svelte";

  export let onSuccess: () => void = () => {};
  export let onClose: () => void = () => {};

  // ==========================================
  // ESTADO LOCAL
  // ==========================================

  let loading = false;
  let gafetesDisponibles: GafeteResponse[] = [];
  let contratistaSearchRef: any;

  // ==========================================
  // SUBSCRIPCIONES A STORES
  // ==========================================

  $: formState = $ingresoFormStore;
  $: tieneVehiculos = formState.contratistaData?.vehiculos?.length > 0;
  $: puedeSubmit =
    formState.puedeIngresar &&
    (formState.modoIngreso === "caminando" ||
      (formState.modoIngreso === "vehiculo" && formState.vehiculoId));

  // ==========================================
  // LIFECYCLE
  // ==========================================

  onMount(() => {
    // 1. Carga de datos asíncrona (Gafetes)
    (async () => {
      const res = await gafeteService.fetchDisponibles();
      if (res.ok) {
        gafetesDisponibles = res.data.filter((g) => g.tipo === "contratista");
      }
    })();

    // 2. Auto-focus (dar tiempo para renderizado)
    setTimeout(() => {
      contratistaSearchRef?.focus();
    }, 100);
  });

  // ==========================================
  // HANDLERS - CONTRATISTA
  // ==========================================

  async function handleContratistaSelect(event: CustomEvent) {
    const { id } = event.detail;
    await controller.buscarYValidarContratista(id);
  }

  function handleContratistaCleared() {
    controller.limpiarContratista();
  }

  // ==========================================
  // HANDLERS - MODO DE INGRESO
  // ==========================================

  function handleModoChange(event: CustomEvent) {
    const modo = event.detail;
    controller.cambiarModoIngreso(modo, formState.contratistaData);
  }

  function handleVehiculoChange(event: CustomEvent) {
    const vehiculoId = event.detail;
    controller.seleccionarVehiculo(vehiculoId);
  }

  // ==========================================
  // HANDLERS - GAFETE
  // ==========================================

  function handleGafeteChange(event: CustomEvent) {
    const gafeteNumero = event.detail;
    controller.establecerGafete(gafeteNumero);
  }

  // ==========================================
  // HANDLERS - CAMPOS ADICIONALES
  // ==========================================

  function handleTipoAutorizacionChange(event: CustomEvent) {
    const tipo = event.detail;
    controller.establecerTipoAutorizacion(tipo);
  }

  function handleObservacionesChange(event: CustomEvent) {
    const observaciones = event.detail;
    controller.establecerObservaciones(observaciones);
  }

  // ==========================================
  // HANDLER - SUBMIT
  // ==========================================

  async function handleSubmit() {
    if (loading) return; // Guard para evitar doble envío
    if (!$currentUser?.id) {
      console.error("No hay usuario autenticado");
      return;
    }

    loading = true;

    const success = await controller.registrarEntrada(
      $currentUser.id,
      gafetesDisponibles,
    );

    loading = false;

    // Si fue exitoso, limpiar búsqueda de contratista y notificar
    if (success) {
      if (contratistaSearchRef) {
        contratistaSearchRef.reset();
      }
      onSuccess();
    }
  }

  function handleCloseForm() {
    controller.resetearFormulario();
    if (contratistaSearchRef) {
      contratistaSearchRef.reset();
    }
    onClose();
  }
</script>

<!-- 
  Contenedor inteligente del formulario de ingreso
  
  Responsabilidades:
  - Coordinar controller y stores
  - Manejar eventos de componentes hijos
  - Orquestar flujo de validación y submit
  - Pasar datos a componentes presentacionales
-->

<div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6 relative">
  <div class="flex justify-between items-center mb-6">
    <h2 class="text-xl font-bold text-gray-900 dark:text-white">
      Registrar Ingreso
    </h2>
    <button
      on:click={handleCloseForm}
      class="text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-400 p-1 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
      type="button"
      aria-label="Cerrar formulario"
    >
      <X size={20} />
    </button>
  </div>

  <form on:submit|preventDefault={handleSubmit} class="space-y-6">
    <!-- BÚSQUEDA DE CONTRATISTA -->
    <ContratistaSearchSection
      bind:this={contratistaSearchRef}
      contratistaNombre={formState.contratistaNombre}
      contratistaData={formState.contratistaData}
      puedeIngresar={formState.puedeIngresar}
      mensajeValidacion={formState.mensajeValidacion}
      on:select={handleContratistaSelect}
      on:clear={handleContratistaCleared}
    />

    <!-- DETALLES DEL INGRESO (Solo si puede ingresar) -->
    {#if formState.puedeIngresar}
      <!-- MODO DE INGRESO -->
      <ModoIngresoSelector
        modoIngreso={formState.modoIngreso}
        {tieneVehiculos}
        on:change={handleModoChange}
      />

      <!-- SELECTOR DE VEHÍCULO (Solo si modo = vehiculo) -->
      {#if formState.modoIngreso === "vehiculo" && tieneVehiculos}
        <VehiculoSelector
          vehiculos={formState.contratistaData.vehiculos}
          vehiculoId={formState.vehiculoId}
          on:change={handleVehiculoChange}
        />
      {/if}

      <!-- CONTROLES COMPACTOS (Gafete + Autorización) -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-start">
        <GafeteInput
          gafeteNumero={formState.gafeteNumero}
          {gafetesDisponibles}
          on:change={handleGafeteChange}
        />
        <IngresoFormFields
          tipoAutorizacion={formState.tipoAutorizacion}
          on:tipoChange={handleTipoAutorizacionChange}
        />
      </div>

      <IngresoObservaciones
        observaciones={formState.observaciones}
        on:change={handleObservacionesChange}
      />

      <!-- BOTÓN SUBMIT -->
      <div class="pt-2 flex gap-3">
        <button
          type="button"
          on:click={handleCloseForm}
          class="flex-1 py-3 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 transition-colors"
        >
          Cancelar
        </button>

        <button
          type="submit"
          disabled={loading || !puedeSubmit}
          class="flex-1 sm:flex-[2] flex justify-center items-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {#if loading}
            <svg
              class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
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
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
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
