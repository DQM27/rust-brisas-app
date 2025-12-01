<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { toast } from "svelte-5-french-toast";
  import { currentUser } from "$lib/stores/auth";
  import type { CreateIngresoContratistaInput } from "$lib/types/ingreso";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import type { GafeteResponse } from "$lib/types/gafete";

  // Componentes hijos
  import ContratistaSearchSection from "./ContratistaSearchSection.svelte";
  import ModoIngresoSelector from "./ModoIngresoSelector.svelte";
  import VehiculoSelector from "./VehiculoSelector.svelte";
  import GafeteInput from "./GafeteInput.svelte";
  import IngresoFormFields from "./IngresoFormFields.svelte";

  export let loading = false;

  const dispatch = createEventDispatcher();

  // ==========================================
  // ESTADO DEL CONTRATISTA
  // ==========================================
  let contratistaId = "";
  let contratistaNombre = "";
  let contratistaData: any = null;
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
  let gafeteInputRef: any;

  // Cargar gafetes disponibles tipo "contratista"
  onMount(async () => {
    const res = await gafeteService.fetchDisponibles();
    if (res.ok) {
      // Filtrar solo gafetes de tipo contratista
      gafetesDisponibles = res.data.filter((g) => g.tipo === "contratista");
    }
  });

  // ==========================================
  // HANDLERS
  // ==========================================
  function handleContratistaValidated(event: CustomEvent) {
    const { contratistaData: data } = event.detail;

    // Auto-selección inteligente
    if (data?.vehiculos?.length === 1) {
      // Si tiene 1 solo vehículo, pre-seleccionarlo
      vehiculoId = data.vehiculos[0].id;
      modoIngreso = "vehiculo";
    } else if (data?.vehiculos?.length === 0) {
      // Si no tiene vehículos, forzar caminando
      modoIngreso = "caminando";
      vehiculoId = null;
    }
  }

  function handleContratistaCleared() {
    resetForm();
  }

  function handleModoChange(event: CustomEvent) {
    const modo = event.detail;
    if (modo === "caminando") {
      vehiculoId = null;
    }
  }

  function resetForm() {
    modoIngreso = "caminando";
    vehiculoId = null;
    gafeteNumero = "";
    observaciones = "";
  }

  // ==========================================
  // SUBMIT
  // ==========================================
  function handleSubmit() {
    if (!contratistaId || !puedeIngresar) return;

    // Validar gafete si se proporcionó
    if (gafeteInputRef && !gafeteInputRef.isValid()) {
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
      usuarioIngresoId:
        $currentUser?.id || "00000000-0000-0000-0000-000000000000",
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
    <!-- BÚSQUEDA DE CONTRATISTA -->
    <ContratistaSearchSection
      bind:contratistaId
      bind:contratistaNombre
      bind:contratistaData
      bind:puedeIngresar
      bind:mensajeValidacion
      on:validated={handleContratistaValidated}
      on:clear={handleContratistaCleared}
    />

    <!-- DETALLES DEL INGRESO (Solo si puede ingresar) -->
    {#if puedeIngresar}
      <!-- MODO DE INGRESO -->
      <ModoIngresoSelector
        bind:modoIngreso
        {tieneVehiculos}
        on:change={handleModoChange}
      />

      <!-- SELECTOR DE VEHÍCULO (Solo si modo = vehiculo) -->
      {#if modoIngreso === "vehiculo" && tieneVehiculos}
        <VehiculoSelector
          vehiculos={contratistaData.vehiculos}
          bind:vehiculoId
        />
      {/if}

      <!-- INPUT DE GAFETE -->
      <GafeteInput
        bind:this={gafeteInputRef}
        bind:gafeteNumero
        {gafetesDisponibles}
      />

      <!-- CAMPOS ADICIONALES (Autorización y Observaciones) -->
      <IngresoFormFields bind:tipoAutorizacion bind:observaciones />

      <!-- BOTÓN SUBMIT -->
      <div class="pt-2">
        <button
          type="submit"
          disabled={loading ||
            !puedeIngresar ||
            (modoIngreso === "vehiculo" && !vehiculoId)}
          class="w-full flex justify-center items-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
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
