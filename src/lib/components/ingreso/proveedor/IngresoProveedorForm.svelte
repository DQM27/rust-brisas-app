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
    setProveedorValidado,
    setVehiculoCatalogo,
  } from "$lib/stores/proveedorFormStore";
  import { ingresoProveedorService } from "$lib/services/ingresoProveedorService";
  import { validarProveedor } from "$lib/logic/ingreso/proveedorService";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";
  import type { GafeteResponse } from "$lib/types/gafete";
  import VehiculoSelector from "../contratista/VehiculoSelector.svelte";

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
      // NOTE: backend expects specific structure or flat fields?
      // Check createIngresoProveedor logic in logic/ingreso/proveedorService.ts
      // It takes ProveedorFormData and maps it.
      // We need to ensure logic service maps provider fields correctly too.
      // wait, logic/ingreso/proveedorService.ts creates `input` from `datosNormalizados` which uses `vehiculoPlaca`.
      // The store handles putting selected vehicle details into `vehiculoPlaca`.
      // So calling createIngreso should work if store state is correct.

      // Actually, we should call the LOGIC function wrapper `crearIngresoProveedor` instead of service directly?
      // The current code calls `ingresoProveedorService.createIngreso` directly with manual mapping.
      // This duplicates logic. Ideally we should use the wrapper.
      // But let's stick to existing pattern in this file if works, or refactor to use wrapper.
      // `ingresoProveedorService.createIngreso` takes `CreateIngresoProveedorInput`.

      const res = await ingresoProveedorService.createIngreso({
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
        // Add new vehicle fields
        marca_vehiculo: $proveedorFormData.vehiculoMarca,
        modelo_vehiculo: $proveedorFormData.vehiculoModelo,
        color_vehiculo: $proveedorFormData.vehiculoColor,
        tipo_vehiculo: $proveedorFormData.vehiculoTipo,

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

  function handleVehiculoChange(e: CustomEvent) {
    const vehiculoId = e.detail;
    setVehiculoCatalogo(vehiculoId);
  }

  async function handleSelectProveedor(e: CustomEvent) {
    const p = e.detail;
    // Trigger full validation
    loading = true;
    try {
      const { validacion, autoSeleccion } = await validarProveedor(p.id);

      if (!validacion.puedeIngresar) {
        toast.error(
          validacion.motivoRechazo || "El proveedor no puede ingresar",
        );
        if (validacion.tieneIngresoAbierto) {
          // Maybe show alert or something?
        }
        return;
      }

      // Update store with validated data
      setProveedorValidado({
        proveedorId: validacion.proveedor.id,
        proveedorNombre: `${validacion.proveedor.nombre} ${validacion.proveedor.apellido}`,
        proveedorData: validacion.proveedor,
        puedeIngresar: validacion.puedeIngresar,
        mensajeValidacion: "",
      });

      // Handle auto-selection
      if (autoSeleccion.suggestedMode) {
        toggleModoIngreso(autoSeleccion.suggestedMode);
      }
      if (autoSeleccion.suggestedVehicleId) {
        setVehiculoCatalogo(autoSeleccion.suggestedVehicleId);
      } else {
        setVehiculoCatalogo(null);
      }

      toast.success("Proveedor validado correctamente");
    } catch (err: any) {
      console.error(err);
      toast.error("Error al validar proveedor: " + err.message);
    } finally {
      loading = false;
    }
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
    on:select={handleSelectProveedor}
    on:clear={() => {
      resetForm();
    }}
  />

  <ProveedorFormFields
    formData={$proveedorFormData}
    errors={$formErrors}
    onChange={updateField}
  />

  <ModoIngresoSelector
    modoIngreso={$proveedorFormData.modoIngreso}
    tieneVehiculos={$proveedorFormData.proveedorData?.vehiculos?.length > 0}
    on:change={handleModoChange}
  />

  {#if $proveedorFormData.modoIngreso === "vehiculo"}
    {#if $proveedorFormData.proveedorData?.vehiculos?.length > 0}
      <VehiculoSelector
        vehiculos={$proveedorFormData.proveedorData.vehiculos}
        vehiculoId={$proveedorFormData.vehiculoId || null}
        on:change={handleVehiculoChange}
      />
    {/if}

    <div class="flex flex-col gap-1 mt-2">
      <label
        for="ve-placa-prov"
        class="text-sm font-medium text-gray-700 dark:text-gray-300"
        >Placa del Vehículo {$proveedorFormData.vehiculoId
          ? "(Autocompletado)"
          : ""}</label
      >
      <input
        id="ve-placa-prov"
        type="text"
        class="w-full p-2 border rounded-md dark:bg-gray-700 dark:border-gray-600 {$proveedorFormData.vehiculoId
          ? 'bg-gray-100 dark:bg-gray-600'
          : ''}"
        placeholder="Ej: ABC-1234"
        value={$proveedorFormData.vehiculoPlaca || ""}
        readonly={!!$proveedorFormData.vehiculoId}
        on:input={(e) => updateField("vehiculoPlaca", e.currentTarget.value)}
      />
      {#if $formErrors.vehiculoPlaca}
        <span class="text-xs text-red-500">{$formErrors.vehiculoPlaca}</span>
      {/if}
    </div>

    <!-- Campos adicionales de vehículo (Tipo, Marca, Modelo, Color) -->
    <div class="grid grid-cols-2 gap-4 mt-2">
      <div class="flex flex-col gap-1">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
          >Tipo</label
        >
        <select
          class="w-full p-2 border rounded-md dark:bg-gray-700 dark:border-gray-600 {$proveedorFormData.vehiculoId
            ? 'bg-gray-100 dark:bg-gray-600'
            : ''}"
          value={$proveedorFormData.vehiculoTipo || "automovil"}
          disabled={!!$proveedorFormData.vehiculoId}
          on:change={(e) => updateField("vehiculoTipo", e.currentTarget.value)}
        >
          <option value="automovil">Automóvil</option>
          <option value="motocicleta">Motocicleta</option>
          <option value="camion">Camión</option>
          <option value="camioneta">Camioneta</option>
          <option value="otro">Otro</option>
        </select>
      </div>

      <div class="flex flex-col gap-1">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
          >Marca</label
        >
        <input
          type="text"
          class="w-full p-2 border rounded-md dark:bg-gray-700 dark:border-gray-600 {$proveedorFormData.vehiculoId
            ? 'bg-gray-100 dark:bg-gray-600'
            : ''}"
          placeholder="Ej: Toyota"
          value={$proveedorFormData.vehiculoMarca || ""}
          readonly={!!$proveedorFormData.vehiculoId}
          on:input={(e) => updateField("vehiculoMarca", e.currentTarget.value)}
        />
      </div>

      <div class="flex flex-col gap-1">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
          >Modelo</label
        >
        <input
          type="text"
          class="w-full p-2 border rounded-md dark:bg-gray-700 dark:border-gray-600 {$proveedorFormData.vehiculoId
            ? 'bg-gray-100 dark:bg-gray-600'
            : ''}"
          placeholder="Ej: Hilux"
          value={$proveedorFormData.vehiculoModelo || ""}
          readonly={!!$proveedorFormData.vehiculoId}
          on:input={(e) => updateField("vehiculoModelo", e.currentTarget.value)}
        />
      </div>

      <div class="flex flex-col gap-1">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
          >Color</label
        >
        <input
          type="text"
          class="w-full p-2 border rounded-md dark:bg-gray-700 dark:border-gray-600 {$proveedorFormData.vehiculoId
            ? 'bg-gray-100 dark:bg-gray-600'
            : ''}"
          placeholder="Ej: Blanco"
          value={$proveedorFormData.vehiculoColor || ""}
          readonly={!!$proveedorFormData.vehiculoId}
          on:input={(e) => updateField("vehiculoColor", e.currentTarget.value)}
        />
      </div>
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
