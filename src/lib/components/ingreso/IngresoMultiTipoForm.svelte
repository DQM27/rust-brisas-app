<script lang="ts">
  // ==========================================
  // IngresoMultiTipoForm.svelte
  // ==========================================
  // Formulario unificado para los 3 tipos de ingreso

  import { onMount } from "svelte";
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";

  // Servicios
  import * as visitaService from "$lib/logic/ingreso/visitaService";
  import * as proveedorService from "$lib/logic/ingreso/proveedorService";
  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";

  // Stores
  import * as visitaStore from "$lib/stores/visitaFormStore";
  import {
    visitaFormData,
    formErrors as visitaErrors,
    isFormValid as visitaValid,
    shouldShowPlaca as visitaShowPlaca,
  } from "$lib/stores/visitaFormStore";

  import * as proveedorStore from "$lib/stores/proveedorFormStore";
  import {
    proveedorFormData,
    formErrors as proveedorErrors,
    isFormValid as proveedorValid,
    shouldShowPlaca as proveedorShowPlaca,
  } from "$lib/stores/proveedorFormStore";

  import { ingresoFormStore } from "$lib/stores/ingresoFormStore";
  import * as controller from "$lib/logic/ingreso/ingresoFormController";

  // Componentes
  import TipoIngresoSelector from "./TipoIngresoSelector.svelte";
  import VisitaFormFields from "./VisitaFormFields.svelte";
  import ProveedorFormFields from "./ProveedorFormFields.svelte";
  import ContratistaSearchSection from "./ContratistaSearchSection.svelte";
  import ModoIngresoSelector from "./ModoIngresoSelector.svelte";
  import VehiculoSelector from "./VehiculoSelector.svelte";
  import GafeteInput from "./GafeteInput.svelte";
  import IngresoFormFields from "./IngresoFormFields.svelte";

  import { X, Save } from "lucide-svelte";
  import type { GafeteResponse } from "$lib/types/gafete";

  export let onSuccess: () => void = () => {};
  export let onClose: () => void = () => {};

  // ==========================================
  // ESTADO LOCAL
  // ==========================================

  let tipoIngreso: "contratista" | "visita" | "proveedor" = "contratista";
  let loading = false;
  let gafetesDisponibles: GafeteResponse[] = [];

  // Stores reactivos
  $: $visitaFormData;
  $: $visitaErrors;
  $: $visitaValid;
  $: $visitaShowPlaca;

  $: $proveedorFormData;
  $: $proveedorErrors;
  $: $proveedorValid;
  $: $proveedorShowPlaca;

  $: contratistaFormState = $ingresoFormStore;
  $: tieneVehiculos =
    contratistaFormState.contratistaData?.vehiculos?.length > 0;
  $: contratistaValid =
    contratistaFormState.puedeIngresar &&
    (contratistaFormState.modoIngreso === "caminando" ||
      (contratistaFormState.modoIngreso === "vehiculo" &&
        contratistaFormState.vehiculoId));

  // Validación general según tipo
  $: canSubmit =
    (tipoIngreso === "contratista" && contratistaValid) ||
    (tipoIngreso === "visita" && $visitaValid) ||
    (tipoIngreso === "proveedor" && $proveedorValid);

  // ==========================================
  // LIFECYCLE
  // ==========================================

  onMount(async () => {
    // Cargar gafetes disponibles
    const res = await gafeteService.fetchDisponibles();
    if (res.ok) {
      gafetesDisponibles = res.data;
    }
  });

  // ==========================================
  // HANDLERS - TIPO DE INGRESO
  // ==========================================

  function handleTipoChange(tipo: typeof tipoIngreso) {
    tipoIngreso = tipo;
    // Reset forms
    if (tipo === "visita") visitaStore.resetForm();
    if (tipo === "proveedor") proveedorStore.resetForm();
    if (tipo === "contratista") ingresoFormStore.reset();
  }

  // ==========================================
  // HANDLERS - CONTRATISTA
  // ==========================================

  async function handleContratistaSelected(event: CustomEvent) {
    const contratistaId = event.detail;
    await controller.buscarYValidarContratista(contratistaId);
  }

  function handleModoChange(event: CustomEvent) {
    const modo = event.detail as "caminando" | "vehiculo";
    if (tipoIngreso === "contratista") {
      ingresoFormStore.setModoIngreso(modo);
    } else if (tipoIngreso === "visita") {
      visitaStore.toggleModoIngreso(modo);
    } else {
      proveedorStore.toggleModoIngreso(modo);
    }
  }

  // ==========================================
  // HANDLERS - SUBMIT
  // ==========================================

  async function handleSubmit() {
    if (!canSubmit || loading) return;
    if (!$currentUser) {
      toast.error("Usuario no autenticado");
      return;
    }

    loading = true;

    try {
      if (tipoIngreso === "contratista") {
        await handleSubmitContratista();
      } else if (tipoIngreso === "visita") {
        await handleSubmitVisita();
      } else {
        await handleSubmitProveedor();
      }
    } catch (error: any) {
      toast.error(error.message || "Error al registrar ingreso");
    } finally {
      loading = false;
    }
  }

  async function handleSubmitContratista() {
    if (!$currentUser) return;

    const result = await ingresoService.registrarEntrada({
      contratistaId: contratistaFormState.contratistaId,
      vehiculoId: contratistaFormState.vehiculoId,
      gafeteNumero: contratistaFormState.gafeteNumero || null,
      tipoAutorizacion: contratistaFormState.tipoAutorizacion,
      modoIngreso: contratistaFormState.modoIngreso,
      observaciones: contratistaFormState.observaciones || null,
      usuarioIngresoId: $currentUser.id,
    });

    if (result.ok) {
      toast.success("Ingreso de contratista registrado");
      ingresoFormStore.reset();
      onSuccess();
    } else {
      toast.error(result.error);
    }
  }

  async function handleSubmitVisita() {
    if (!$currentUser) return;

    const ingreso = await visitaService.crearIngresoVisita(
      $visitaFormData,
      $currentUser.id,
    );

    toast.success("Ingreso de visita registrado");
    visitaStore.resetForm();
    onSuccess();
  }

  async function handleSubmitProveedor() {
    if (!$currentUser) return;

    const ingreso = await proveedorService.crearIngresoProveedor(
      $proveedorFormData,
      $currentUser.id,
    );

    toast.success("Ingreso de proveedor registrado");
    proveedorStore.resetForm();
    onSuccess();
  }

  // ==========================================
  // HANDLERS - CLOSE
  // ==========================================

  function handleClose() {
    onClose();
  }
</script>

<div class="multi-tipo-form">
  <!-- Header -->
  <div class="form-header">
    <h2>Registrar Ingreso</h2>
    <button type="button" class="close-btn" on:click={handleClose}>
      <X size={20} />
    </button>
  </div>

  <!-- Selector de tipo -->
  <TipoIngresoSelector
    tipoSeleccionado={tipoIngreso}
    onChange={handleTipoChange}
  />

  <!-- Formulario según tipo -->
  <div class="form-body">
    {#if tipoIngreso === "contratista"}
      <!-- Búsqueda de contratista -->
      <ContratistaSearchSection on:select={handleContratistaSelected} />

      {#if contratistaFormState.puedeIngresar}
        <!-- Campos de ingreso -->
        <IngresoFormFields />

        <!-- Modo de ingreso -->
        <ModoIngresoSelector
          modoIngreso={contratistaFormState.modoIngreso}
          {tieneVehiculos}
          on:change={handleModoChange}
        />

        <!-- Selector de vehículo -->
        {#if contratistaFormState.modoIngreso === "vehiculo" && tieneVehiculos}
          <VehiculoSelector
            vehiculos={contratistaFormState.contratistaData?.vehiculos || []}
            vehiculoId={contratistaFormState.vehiculoId}
            on:select={(e) => ingresoFormStore.setVehiculo(e.detail)}
          />
        {/if}

        <!-- Gafete -->
        <GafeteInput
          gafetesDisponibles={gafetesDisponibles.filter(
            (g) => g.tipo === "contratista",
          )}
          gafeteNumero={contratistaFormState.gafeteNumero}
          on:change={(e) => ingresoFormStore.setGafete(e.detail)}
        />
      {/if}
    {:else if tipoIngreso === "visita"}
      <!-- Formulario de visita -->
      <VisitaFormFields
        formData={$visitaFormData}
        errors={$visitaErrors}
        onChange={(field, value) => visitaStore.updateField(field, value)}
      />

      <!-- Modo de ingreso -->
      <ModoIngresoSelector
        modoIngreso={$visitaFormData.modoIngreso}
        tieneVehiculos={true}
        on:change={handleModoChange}
      />

      <!-- Placa si modo vehículo -->
      {#if $visitaShowPlaca}
        <div class="form-group">
          <label for="vehiculoPlaca">Placa del Vehículo *</label>
          <input
            id="vehiculoPlaca"
            type="text"
            value={$visitaFormData.vehiculoPlaca || ""}
            on:input={(e) =>
              visitaStore.updateField("vehiculoPlaca", e.currentTarget.value)}
            class:error={$visitaErrors.vehiculoPlaca}
            placeholder="Ej: ABC-1234"
          />
          {#if $visitaErrors.vehiculoPlaca}
            <span class="error-message">{$visitaErrors.vehiculoPlaca}</span>
          {/if}
        </div>
      {/if}

      <!-- Gafete -->
      <GafeteInput
        gafetesDisponibles={gafetesDisponibles.filter(
          (g) => g.tipo === "visita",
        )}
        gafeteNumero={$visitaFormData.gafeteNumero || ""}
        on:change={(e) => visitaStore.updateField("gafeteNumero", e.detail)}
      />
    {:else if tipoIngreso === "proveedor"}
      <!-- Formulario de proveedor -->
      <ProveedorFormFields
        formData={$proveedorFormData}
        errors={$proveedorErrors}
        onChange={(field, value) => proveedorStore.updateField(field, value)}
      />

      <!-- Modo de ingreso -->
      <ModoIngresoSelector
        modoIngreso={$proveedorFormData.modoIngreso}
        tieneVehiculos={true}
        on:change={handleModoChange}
      />

      <!-- Placa si modo vehículo -->
      {#if $proveedorShowPlaca}
        <div class="form-group">
          <label for="vehiculoPlaca">Placa del Vehículo *</label>
          <input
            id="vehiculoPlaca"
            type="text"
            value={$proveedorFormData.vehiculoPlaca || ""}
            on:input={(e) =>
              proveedorStore.updateField(
                "vehiculoPlaca",
                e.currentTarget.value,
              )}
            class:error={$proveedorErrors.vehiculoPlaca}
            placeholder="Ej: ABC-1234"
          />
          {#if $proveedorErrors.vehiculoPlaca}
            <span class="error-message">{$proveedorErrors.vehiculoPlaca}</span>
          {/if}
        </div>
      {/if}

      <!-- Gafete -->
      <GafeteInput
        gafetesDisponibles={gafetesDisponibles.filter(
          (g) => g.tipo === "proveedor",
        )}
        gafeteNumero={$proveedorFormData.gafeteNumero || ""}
        on:change={(e) => proveedorStore.updateField("gafeteNumero", e.detail)}
      />
    {/if}
  </div>

  <!-- Footer -->
  <div class="form-footer">
    <button type="button" class="btn-secondary" on:click={handleClose}>
      Cancelar
    </button>
    <button
      type="button"
      class="btn-primary"
      on:click={handleSubmit}
      disabled={!canSubmit || loading}
    >
      {#if loading}
        Registrando...
      {:else}
        <Save size={16} />
        Registrar Ingreso
      {/if}
    </button>
  </div>
</div>

<style>
  .multi-tipo-form {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary);
  }

  .form-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .form-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .close-btn:hover {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .form-body {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .form-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1.5rem;
    border-top: 1px solid var(--border);
  }

  .btn-primary,
  .btn-secondary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary {
    background: var(--primary);
    color: white;
    border: none;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--primary-dark);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border);
  }

  .btn-secondary:hover {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  input {
    padding: 0.625rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 0.875rem;
    background: var(--bg-primary);
    color: var(--text-primary);
    transition: all 0.2s ease;
  }

  input:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-alpha-10);
  }

  input.error {
    border-color: var(--error);
  }

  .error-message {
    font-size: 0.75rem;
    color: var(--error);
  }
</style>
