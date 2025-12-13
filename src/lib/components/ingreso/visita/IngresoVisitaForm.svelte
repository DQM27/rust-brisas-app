<script lang="ts">
  import { onMount, createEventDispatcher } from "svelte";
  import IngresoFormLayout from "../common/IngresoFormLayout.svelte";
  import VisitaFormFields from "./VisitaFormFields.svelte";
  import ModoIngresoSelector from "../common/ModoIngresoSelector.svelte";
  import GafeteInput from "../common/GafeteInput.svelte";
  import IngresoFormFields from "../common/IngresoFormFields.svelte"; // Autorización
  import IngresoObservaciones from "../common/IngresoObservaciones.svelte";

  import {
    visitaFormData,
    formErrors,
    isFormValid,
    shouldShowPlaca,
    updateField,
    toggleModoIngreso,
    resetForm,
  } from "$lib/stores/visitaFormStore";
  import { ingresoVisitaService } from "$lib/services/ingresoVisitaService";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";
  import type { GafeteResponse } from "$lib/types/gafete";

  export let onClose: () => void;
  export let onSuccess: () => void;

  let loading = false;
  let gafetesDisponibles: GafeteResponse[] = [];

  onMount(async () => {
    // Cargar gafetes de visita
    const res = await gafeteService.fetchDisponibles();
    if (res.ok) {
      gafetesDisponibles = res.data.filter((g) => g.tipo === "visita");
    }
    resetForm();
  });

  async function handleSubmit() {
    if (!$isFormValid || !$currentUser) return;
    loading = true;
    try {
      const result = await ingresoVisitaService.createIngreso({
        cedula: $visitaFormData.cedula,
        nombre: $visitaFormData.nombre,
        apellido: $visitaFormData.apellido,
        empresa: $visitaFormData.empresa || undefined,
        anfitrion: $visitaFormData.anfitrion,
        area_visitada: $visitaFormData.areaVisitada,
        motivo: $visitaFormData.motivoVisita,
        gafete: $visitaFormData.gafeteNumero || undefined,
        observaciones: $visitaFormData.observaciones || undefined,
        usuario_ingreso_id: $currentUser.id,
      });

      toast.success("Visita registrada correctamente");
      resetForm();
      onSuccess();
    } catch (error: any) {
      toast.error(error.message || "Error al registrar visita");
    } finally {
      loading = false;
    }
  }

  function handleModoChange(e: CustomEvent) {
    toggleModoIngreso(e.detail);
  }
</script>

<IngresoFormLayout
  title="Registrar Visita"
  {loading}
  disabled={!$isFormValid}
  {onClose}
  onSubmit={handleSubmit}
  submitLabel="Registrar Visita"
>
  <VisitaFormFields
    formData={$visitaFormData}
    errors={$formErrors}
    onChange={updateField}
  />

  <ModoIngresoSelector
    modoIngreso={$visitaFormData.modoIngreso}
    tieneVehiculos={true}
    on:change={handleModoChange}
  />

  {#if $shouldShowPlaca}
    <div class="flex flex-col gap-1">
      <label
        for="ve-placa-vis"
        class="text-sm font-medium text-gray-700 dark:text-gray-300"
        >Placa del Vehículo</label
      >
      <input
        id="ve-placa-vis"
        type="text"
        class="w-full p-2 border rounded-md dark:bg-gray-700 dark:border-gray-600"
        placeholder="Ej: ABC-1234"
        value={$visitaFormData.vehiculoPlaca || ""}
        on:input={(e) => updateField("vehiculoPlaca", e.currentTarget.value)}
      />
      {#if $formErrors.vehiculoPlaca}
        <span class="text-xs text-red-500">{$formErrors.vehiculoPlaca}</span>
      {/if}
    </div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <GafeteInput
      gafeteNumero={$visitaFormData.gafeteNumero || ""}
      {gafetesDisponibles}
      on:change={(e) => updateField("gafeteNumero", e.detail)}
    />

    <IngresoFormFields
      tipoAutorizacion={$visitaFormData.tipoAutorizacion}
      on:tipoChange={(e) => updateField("tipoAutorizacion", e.detail)}
    />
  </div>

  <IngresoObservaciones
    observaciones={$visitaFormData.observaciones || ""}
    on:change={(e) => updateField("observaciones", e.detail)}
  />
</IngresoFormLayout>
