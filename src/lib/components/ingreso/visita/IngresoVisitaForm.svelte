<script lang="ts">
  import { onMount, createEventDispatcher } from "svelte";
  import IngresoFormLayout from "../common/IngresoFormLayout.svelte";
  import VisitaFormFields from "./VisitaFormFields.svelte";
  import ModoIngresoSelector from "../common/ModoIngresoSelector.svelte";
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
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";

  export let onClose: () => void;
  export let onSuccess: () => void;

  let loading = false;

  onMount(() => {
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
        gafete: undefined,
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
    <div class="flex flex-col gap-1.5">
      <label for="ve-placa-vis" class="text-xs font-medium text-[#8d96a0]"
        >Placa del Vehículo</label
      >
      <input
        id="ve-placa-vis"
        type="text"
        class="w-full px-3 py-2 bg-[#0d1117] border border-[#30363d] rounded-md text-sm text-[#f0f6fc] placeholder-[#484f58] focus:ring-2 focus:ring-[#388bfd] focus:border-transparent outline-none transition-all"
        placeholder="Ej: ABC-1234"
        value={$visitaFormData.vehiculoPlaca || ""}
        on:input={(e) => updateField("vehiculoPlaca", e.currentTarget.value)}
      />
      {#if $formErrors.vehiculoPlaca}
        <span class="text-xs text-red-500">{$formErrors.vehiculoPlaca}</span>
      {/if}
    </div>
  {/if}

  <IngresoObservaciones
    observaciones={$visitaFormData.observaciones || ""}
    on:change={(e) => updateField("observaciones", e.detail)}
  />
</IngresoFormLayout>
