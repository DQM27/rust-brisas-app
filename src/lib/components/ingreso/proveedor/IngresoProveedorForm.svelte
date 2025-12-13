<script lang="ts">
  import { onMount } from "svelte";
  import {
    proveedorFormData,
    resetForm,
    updateField,
    setProveedorValidado,
    setVehiculoCatalogo,
    toggleModoIngreso,
  } from "$lib/stores/proveedorFormStore";
  import { currentUser } from "$lib/stores/auth";
  import { ingresoProveedorService } from "$lib/services/ingresoProveedorService";
  import { validarProveedor } from "$lib/logic/ingreso/proveedorService";
  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import type { GafeteResponse } from "$lib/types/gafete";

  // Common components
  import ProveedorSearchSection from "./ProveedorSearchSection.svelte";
  import VehiculoSelector from "../contratista/VehiculoSelector.svelte";
  import ModoIngresoSelector from "../common/ModoIngresoSelector.svelte";
  import GafeteInput from "../common/GafeteInput.svelte";
  import IngresoFormFields from "../common/IngresoFormFields.svelte";
  import IngresoObservaciones from "../common/IngresoObservaciones.svelte";
  import { shortcutService } from "$lib/services/shortcutService";

  import { X } from "lucide-svelte";
  import { toast } from "svelte-5-french-toast";

  export let onSuccess: () => void = () => {};
  export let onClose: () => void = () => {};

  // ==========================================
  // ESTADO LOCAL
  // ==========================================

  let loading = false;
  let gafetesDisponibles: GafeteResponse[] = [];
  let proveedorSearchRef: any;

  // ==========================================
  // SUBSCRIPCIONES A STORES
  // ==========================================

  $: formState = $proveedorFormData;
  $: tieneVehiculos = formState.proveedorData?.vehiculos?.length > 0;
  $: puedeSubmit =
    formState.puedeIngresar &&
    formState.areaVisitada?.trim() &&
    formState.motivo?.trim() &&
    (formState.modoIngreso === "caminando" ||
      (formState.modoIngreso === "vehiculo" && formState.vehiculoId));

  // ==========================================
  // LIFECYCLE
  // ==========================================

  onMount(() => {
    // 1. Carga de datos asíncrona (Gafetes)
    (async () => {
      const res = await gafeteService.fetchDisponibles("proveedor");
      if (res.ok) {
        gafetesDisponibles = res.data.filter((g) => g.tipo === "proveedor");
      }
    })();

    // 2. Atajos
    const unregSave = shortcutService.registerHandler(
      "ingreso-proveedor-form",
      "save",
      () => handleSubmit(),
    );
    const unregCancel = shortcutService.registerHandler(
      "ingreso-proveedor-form",
      "cancel",
      handleCloseForm,
    );

    // 3. Auto-focus
    setTimeout(() => {
      proveedorSearchRef?.focus();
    }, 100);

    return () => {
      unregSave();
      unregCancel();
    };
  });

  // ==========================================
  // HANDLERS - PROVEEDOR
  // ==========================================

  async function handleProveedorSelect(event: CustomEvent) {
    const p = event.detail;
    loading = true;
    try {
      const { validacion, autoSeleccion } = await validarProveedor(p.id);

      if (!validacion.puedeIngresar) {
        toast.error(
          validacion.motivoRechazo || "El proveedor no puede ingresar",
        );
        return;
      }

      // 2. Actualizar store con resultados (incluyendo alertas)
      setProveedorValidado({
        proveedorId: validacion.proveedor.id,
        proveedorNombre: `${validacion.proveedor.nombre} ${validacion.proveedor.apellido}`,
        proveedorData: validacion.proveedor,
        puedeIngresar: validacion.puedeIngresar,
        mensajeValidacion: validacion.motivoRechazo || "",
        alertas: validacion.alertas || [],
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

      toast.success("Proveedor validado");
    } catch (err: any) {
      console.error(err);
      toast.error("Error al validar proveedor: " + err.message);
    } finally {
      loading = false;
    }
  }

  function handleProveedorCleared() {
    resetForm();
  }

  // ==========================================
  // HANDLERS - MODO DE INGRESO
  // ==========================================

  function handleModoChange(event: CustomEvent) {
    toggleModoIngreso(event.detail);
  }

  function handleVehiculoChange(event: CustomEvent) {
    setVehiculoCatalogo(event.detail);
  }

  // ==========================================
  // HANDLERS - GAFETE
  // ==========================================

  function handleGafeteChange(event: CustomEvent) {
    updateField("gafeteNumero", event.detail);
  }

  // ==========================================
  // HANDLERS - CAMPOS ADICIONALES
  // ==========================================

  function handleTipoAutorizacionChange(event: CustomEvent) {
    updateField("tipoAutorizacion", event.detail);
  }

  function handleObservacionesChange(event: CustomEvent) {
    updateField("observaciones", event.detail);
  }

  // ==========================================
  // HANDLER - SUBMIT
  // ==========================================

  async function handleSubmit() {
    console.log("🚀 handleSubmit called");
    if (loading) {
      console.log("⚠️ Already loading, returning");
      return;
    }
    if (!$currentUser?.id) {
      console.error("❌ No hay usuario autenticado");
      return;
    }

    loading = true;
    console.log("📦 Form state:", JSON.stringify(formState, null, 2));

    const payload = {
      cedula: formState.cedula,
      nombre: formState.nombre,
      apellido: formState.apellido,
      empresaId: formState.empresaId,
      areaVisitada: formState.areaVisitada,
      motivo: formState.motivo,
      gafete: formState.gafeteNumero || undefined,
      tipoAutorizacion: formState.tipoAutorizacion,
      modoIngreso: formState.modoIngreso,
      placaVehiculo: formState.vehiculoPlaca || undefined,
      marcaVehiculo: formState.vehiculoMarca,
      modeloVehiculo: formState.vehiculoModelo,
      colorVehiculo: formState.vehiculoColor,
      tipoVehiculo: formState.vehiculoTipo,
      observaciones: formState.observaciones || undefined,
      usuarioIngresoId: $currentUser.id,
    };
    console.log("📤 Payload to send:", JSON.stringify(payload, null, 2));

    try {
      const result = await ingresoProveedorService.createIngreso(payload);
      console.log("✅ Ingreso created:", result);

      toast.success("Ingreso de proveedor registrado");
      resetForm();
      if (proveedorSearchRef) {
        proveedorSearchRef.reset();
      }
      onSuccess();
    } catch (error: any) {
      console.error("❌ Error creating ingreso:", error);
      toast.error(error.message || error || "Error al registrar ingreso");
    } finally {
      loading = false;
    }
  }

  function handleCloseForm() {
    resetForm();
    if (proveedorSearchRef) {
      proveedorSearchRef.reset();
    }
    onClose();
  }
</script>

<!-- 
  Formulario de ingreso de proveedor
  Estructura idéntica al formulario de contratista
-->

<div
  class="bg-white dark:bg-gray-800 rounded-lg shadow p-6 relative"
  use:shortcutService.useScope={"ingreso-proveedor-form"}
>
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
    <!-- BÚSQUEDA DE PROVEEDOR -->
    <ProveedorSearchSection
      bind:this={proveedorSearchRef}
      proveedorData={formState.proveedorData}
      puedeIngresar={formState.puedeIngresar}
      mensajeValidacion={formState.mensajeValidacion}
      alertas={formState.alertas}
      on:select={handleProveedorSelect}
      on:clear={handleProveedorCleared}
    />

    <!-- DETALLES DEL INGRESO (Solo si puede ingresar) -->
    {#if formState.puedeIngresar}
      <!-- ÁREA Y MOTIVO -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="space-y-1">
          <label
            for="area"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            Área a Visitar <span class="text-red-500">*</span>
          </label>
          <input
            id="area"
            type="text"
            value={formState.areaVisitada}
            on:input={(e) => updateField("areaVisitada", e.currentTarget.value)}
            placeholder="Ej: Almacén, Producción"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        <div class="space-y-1">
          <label
            for="motivo"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            Motivo <span class="text-red-500">*</span>
          </label>
          <input
            id="motivo"
            type="text"
            value={formState.motivo}
            on:input={(e) => updateField("motivo", e.currentTarget.value)}
            placeholder="Ej: Entrega de materiales"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
      </div>

      <!-- MODO DE INGRESO -->
      <ModoIngresoSelector
        modoIngreso={formState.modoIngreso}
        {tieneVehiculos}
        on:change={handleModoChange}
      />

      <!-- SELECTOR DE VEHÍCULO (Solo si modo = vehiculo) -->
      {#if formState.modoIngreso === "vehiculo" && tieneVehiculos}
        <VehiculoSelector
          vehiculos={formState.proveedorData.vehiculos}
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
