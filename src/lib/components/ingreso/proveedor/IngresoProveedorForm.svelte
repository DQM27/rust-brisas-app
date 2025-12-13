<script lang="ts">
  import { onMount } from "svelte";
  import IngresoFormLayout from "../common/IngresoFormLayout.svelte";
  import ProveedorFormFields from "./ProveedorFormFields.svelte";
  import ProveedorSearchSection from "./ProveedorSearchSection.svelte";
  import ModoIngresoSelector from "../common/ModoIngresoSelector.svelte";
  import GafeteInput from "../common/GafeteInput.svelte";
  import IngresoFormFields from "../common/IngresoFormFields.svelte"; // Autorización
  import IngresoObservaciones from "../common/IngresoObservaciones.svelte";

  import {
    proveedorFormData,
    formErrors,
    isFormValid,
    shouldShowPlaca,
    updateField,
    toggleModoIngreso,
    resetForm,
  } from "$lib/stores/proveedorFormStore";
  import { ingresoProveedorService } from "$lib/services/ingresoProveedorService";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";
  import type { GafeteResponse } from "$lib/types/gafete";

  export let onClose: () => void;
  export let onSuccess: () => void;

  let loading = false;
  let gafetesDisponibles: GafeteResponse[] = [];

  onMount(async () => {
    // Cargar gafetes de proveedor
    const res = await gafeteService.fetchDisponibles();
    if (res.ok) {
      gafetesDisponibles = res.data.filter((g) => g.tipo === "proveedor");
    }
    resetForm();
  });

  async function handleSubmit() {
    if (!$isFormValid || !$currentUser) return;
    loading = true;
    try {
      const result = await ingresoProveedorService.createIngreso({
        cedula: $proveedorFormData.cedula,
        nombre: $proveedorFormData.nombre,
        apellido: $proveedorFormData.apellido,
        empresa_id: $proveedorFormData.empresaId,
        area_visitada: $proveedorFormData.areaVisitada,
        motivo: $proveedorFormData.motivo,
        gafete: $proveedorFormData.gafeteNumero || undefined,
        tipo_autorizacion: $proveedorFormData.tipoAutorizacion,
        modo_ingreso: $proveedorFormData.modoIngreso,
        placa_vehiculo: $proveedorFormData.vehiculoPlaca || undefined,
        observaciones: $proveedorFormData.observaciones || undefined,
        usuario_ingreso_id: $currentUser.id,
      });

      toast.success("Proveedor registrado correctamente");
      resetForm();
      onSuccess();
    } catch (error: any) {
      toast.error(error.message || "Error al registrar proveedor");
    } finally {
      loading = false;
    }
  }

  function handleModoChange(e: CustomEvent) {
    toggleModoIngreso(e.detail);
  }
</script>

<IngresoFormLayout
  title="Registrar Proveedor"
  {loading}
  disabled={!$isFormValid}
  {onClose}
  onSubmit={handleSubmit}
  submitLabel="Registrar Proveedor"
>
  <ProveedorSearchSection
    on:select={(e) => {
      const p = e.detail;
      updateField("cedula", p.cedula);
      updateField("nombre", p.nombre);
      updateField("apellido", p.apellido);
      updateField("empresaId", p.empresaId);
      toast.success("Datos de proveedor cargados");
    }}
    on:clear={() => {
      updateField("cedula", "");
      updateField("nombre", "");
      updateField("apellido", "");
      updateField("empresaId", "");
    }}
  />

  <ProveedorFormFields
    formData={$proveedorFormData}
    errors={$formErrors}
    onChange={updateField}
  />

  <ModoIngresoSelector
    modoIngreso={$proveedorFormData.modoIngreso}
    tieneVehiculos={true}
    on:change={handleModoChange}
  />

  {#if $shouldShowPlaca}
    <div class="flex flex-col gap-1">
      <label
        for="ve-placa-prov"
        class="text-sm font-medium text-gray-700 dark:text-gray-300"
        >Placa del Vehículo</label
      >
      <input
        id="ve-placa-prov"
        type="text"
        class="w-full p-2 border rounded-md dark:bg-gray-700 dark:border-gray-600"
        placeholder="Ej: ABC-1234"
        value={$proveedorFormData.vehiculoPlaca || ""}
        on:input={(e) => updateField("vehiculoPlaca", e.currentTarget.value)}
      />
      {#if $formErrors.vehiculoPlaca}
        <span class="text-xs text-red-500">{$formErrors.vehiculoPlaca}</span>
      {/if}
    </div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <GafeteInput
      gafeteNumero={$proveedorFormData.gafeteNumero || ""}
      {gafetesDisponibles}
      on:change={(e) => updateField("gafeteNumero", e.detail)}
    />

    <IngresoFormFields
      tipoAutorizacion={$proveedorFormData.tipoAutorizacion}
      on:tipoChange={(e) => updateField("tipoAutorizacion", e.detail)}
    />
  </div>

  <IngresoObservaciones
    observaciones={$proveedorFormData.observaciones || ""}
    on:change={(e) => updateField("observaciones", e.detail)}
  />
</IngresoFormLayout>
