<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fade, scale, slide } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import {
    X,
    ChevronDown,
    ChevronRight,
    Car,
    FileText,
    AlertTriangle,
    Building2,
    Briefcase,
  } from "lucide-svelte";

  // Components
  // Podemos reusar GafeteInput
  import GafeteInput from "./shared/gafete/GafeteInput.svelte";

  // Logic
  import { ingresoProveedorService } from "$lib/services/ingresoProveedorService";
  import { currentUser } from "$lib/stores/auth";
  import { invoke } from "@tauri-apps/api/core";

  // Props
  interface Props {
    show: boolean;
    initialPerson?: any;
  }

  let { show = $bindable(false), initialPerson = null }: Props = $props();

  // State
  let loading = $state(false);
  let selectedPerson = $state<any>(null);
  let validationResult = $state<any>(null); // ValidacionIngresoProveedorResponse
  let gafete = $state("");
  // Campos extra requeridos para proveedores según CreateIngresoProveedorInput:
  // areaVisitada, motivo, tipoAutorizacion, modoIngreso
  let vehiculoId = $state<string | null>(null);
  let areaVisitada = $state("");
  let motivo = $state("");
  let tipoAutorizacion = $state<"correo">("correo"); // Proveedores usualmente solo correo o pase
  let observaciones = $state("");
  let showObservaciones = $state(false);
  let submitted = $state(false);

  const dispatch = createEventDispatcher();

  // Computed
  let vehiculosDisponibles = $derived(selectedPerson?.vehiculos || []);
  let tieneVehiculos = $derived(vehiculosDisponibles.length > 0);
  let modoIngreso = $derived(vehiculoId ? "vehiculo" : "caminando");

  // Auto-seleccionar vehículo único
  $effect(() => {
    if (vehiculosDisponibles.length === 1 && !vehiculoId) {
      vehiculoId = vehiculosDisponibles[0].id;
    }
  });

  // Reset y Validar al abrir
  $effect(() => {
    if (show) {
      if (initialPerson) {
        handlePersonSelect(initialPerson);
      }
    }
  });

  async function handlePersonSelect(person: any) {
    selectedPerson = person;
    loading = true;
    try {
      // Validar ingreso proveedor
      // Necesitamos el ID del proveedor. Si PersonaFinder devuelve un objeto mixto, asegurarnos de tener proveedorId.
      // Si el buscador devuelve { id: "proveedor:xyz", ... } está bien.
      validationResult = await ingresoProveedorService.validarIngreso(
        person.id,
      );

      if (!validationResult.puedeIngresar) {
        invoke("play_alert_sound");
        toast.error(
          validationResult.motivoRechazo || "Proveedor no autorizado",
        );
      } else {
        // Si el backend devuelve data actualizada del proveedor, usarla
        if (validationResult.proveedor) {
          selectedPerson = { ...selectedPerson, ...validationResult.proveedor };
        }
      }
    } catch (e: any) {
      toast.error("Error al validar proveedor: " + e.message);
      validationResult = null;
    } finally {
      loading = false;
    }
  }

  async function handleSubmit() {
    if (!selectedPerson || !validationResult?.puedeIngresar) return;

    // Validar campos requeridos
    if (!areaVisitada.trim() || !motivo.trim()) {
      submitted = true;
      toast.error("Por favor complete los campos requeridos");
      return;
    }

    submitted = true;
    loading = true;
    try {
      const finalGafete = gafete.trim() || undefined; // Opcional? Normalmente requerido si está entrando
      // Backend service types says gafete? is optional.

      await ingresoProveedorService.createIngreso({
        cedula: selectedPerson.cedula,
        nombre: selectedPerson.nombre,
        apellido: selectedPerson.apellido,
        empresaId: selectedPerson.empresaId || selectedPerson.empresa, // Ensure ID
        areaVisitada: areaVisitada,
        motivo: motivo,
        gafete: finalGafete,
        tipoAutorizacion: tipoAutorizacion,
        modoIngreso: modoIngreso,
        tipoVehiculo: vehiculoId ? "vehiculo" : "peatonal", // Check backend expectations for vehicle logic
        // Si hay vehículo, pasamos datos. Si es por ID, el backend debería buscarlos,
        // pero el input pide 'placaVehiculo', 'marcaVehiculo', etc.
        // Asi que los extraemos del vehiculo seleccionado.
        placaVehiculo: vehiculoId
          ? vehiculosDisponibles.find((v: any) => v.id === vehiculoId)?.placa
          : undefined,
        marcaVehiculo: vehiculoId
          ? vehiculosDisponibles.find((v: any) => v.id === vehiculoId)?.marca
          : undefined,
        modeloVehiculo: vehiculoId
          ? vehiculosDisponibles.find((v: any) => v.id === vehiculoId)?.modelo
          : undefined,
        colorVehiculo: vehiculoId
          ? vehiculosDisponibles.find((v: any) => v.id === vehiculoId)?.color
          : undefined,

        observaciones: observaciones,
        usuarioIngresoId: $currentUser?.id || "",
      });

      toast.success("Ingreso de proveedor registrado");
      dispatch("complete");
      handleClose();
    } catch (e: any) {
      console.error(e);
      toast.error("Error al registrar: " + e.message);
    } finally {
      loading = false;
    }
  }

  function handleClose() {
    show = false;
    reset();
  }

  function reset() {
    selectedPerson = null;
    validationResult = null;
    gafete = "";
    vehiculoId = null;
    areaVisitada = "";
    motivo = "";
    observaciones = "";
    showObservaciones = false;
    submitted = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") handleClose();
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      handleSubmit();
    }
  }

  function getSeverityClasses(severity?: string) {
    const base =
      "flex items-center gap-2 text-sm px-3 py-2 rounded-md border transition-colors";
    return `${base} text-red-700 bg-red-50 border-red-200 dark:text-red-300 dark:bg-red-900/30 dark:border-red-800`;
  }

  // --- UI PATTERNS ---
  const inputClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 transition-all outline-none disabled:opacity-50";
  const labelClass = "block text-xs font-medium text-secondary mb-1";

  function getFieldStateClass(value: any, isRequired = false) {
    const hasValue = value && String(value).trim() !== "";

    if (isRequired && !hasValue && submitted) {
      return "!border-red-500/50 !ring-1 !ring-red-500/20";
    }

    if (isRequired && hasValue) {
      return "!border-green-500/50 !ring-1 !ring-green-500/20";
    }

    return "border-white/10";
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    transition:fade
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={(e) => e.target === e.currentTarget && handleClose()}
    onkeydown={(e) => e.key === "Escape" && handleClose()}
  >
    <div
      class="bg-surface-2 rounded-lg shadow-surface-xl border border-surface max-w-md w-full max-h-[95vh] flex flex-col overflow-hidden"
      transition:scale={{ start: 0.95 }}
    >
      <!-- Header (Static) -->
      <div
        class="flex-none flex items-center justify-between px-6 py-4 border-b border-surface bg-surface-2"
      >
        <div>
          <h2 class="text-xl font-semibold text-primary">Ingreso Proveedor</h2>
          <p class="text-sm text-secondary mt-1">
            Registrar entrada de proveedor
          </p>
        </div>
        <button
          onclick={handleClose}
          type="button"
          class="p-2 hover:bg-surface-hover rounded-md transition-colors"
        >
          <X size={20} class="text-secondary" />
        </button>
      </div>

      <!-- Form (Scrollable) -->
      <form
        id="ingresoProveedorForm"
        onsubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
        class="flex-1 overflow-y-auto"
      >
        <div class="p-6 space-y-6">
          <!-- Persona Info -->
          {#if selectedPerson}
            <div class="p-4 bg-surface-1 rounded-lg border border-surface">
              <div class="space-y-2 mb-4">
                <div class="flex items-center">
                  <span
                    class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
                    >Nombre</span
                  >
                  <span class="text-primary font-semibold text-sm"
                    >{selectedPerson.nombre} {selectedPerson.apellido}</span
                  >
                </div>
                <div class="flex items-center">
                  <span
                    class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
                    >Cédula</span
                  >
                  <span class="text-primary font-mono text-sm"
                    >{selectedPerson.cedula}</span
                  >
                </div>
                <div class="flex items-center">
                  <span
                    class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
                    >Empresa</span
                  >
                  <span class="text-primary text-sm"
                    >{selectedPerson.empresaNombre || "Sin empresa"}</span
                  >
                </div>
              </div>

              <!-- Validation Status -->
              {#if validationResult}
                {#if validationResult.puedeIngresar}
                  <div
                    class="flex items-center gap-2 text-sm text-success bg-success bg-opacity-10 px-3 py-2 rounded-md mb-4"
                  >
                    <span class="font-medium">✓ Acceso Autorizado</span>
                  </div>
                  {#if validationResult.tieneIngresoAbierto}
                    <div
                      class="flex items-center gap-2 text-sm text-warning bg-warning bg-opacity-10 px-3 py-2 rounded-md mb-4"
                    >
                      <AlertTriangle size={16} />
                      <span class="font-medium"
                        >Ya tiene un ingreso abierto</span
                      >
                    </div>
                  {/if}
                {:else}
                  <div class={getSeverityClasses()}>
                    <span class="font-medium"
                      >✗ {validationResult.motivoRechazo ||
                        "No autorizado"}</span
                    >
                  </div>
                {/if}
              {/if}

              <!-- Form Fields -->
              {#if validationResult?.puedeIngresar}
                <div class="mt-4 space-y-4">
                  <GafeteInput
                    bind:value={gafete}
                    autofocus
                    disabled={loading}
                  />

                  <!-- Área y Motivo -->
                  <div class="grid grid-cols-1 gap-4">
                    <div>
                      <label for="areaVisitada" class={labelClass}
                        >Área Visitada <span class="text-red-500">*</span
                        ></label
                      >
                      <input
                        id="areaVisitada"
                        type="text"
                        bind:value={areaVisitada}
                        class="{inputClass} {getFieldStateClass(
                          areaVisitada,
                          true,
                        )}"
                        placeholder="Ej. Almacén, Mantenimiento"
                      />
                    </div>
                    <div>
                      <label for="motivoVisita" class={labelClass}
                        >Motivo <span class="text-red-500">*</span></label
                      >
                      <input
                        id="motivoVisita"
                        type="text"
                        bind:value={motivo}
                        class="{inputClass} {getFieldStateClass(motivo, true)}"
                        placeholder="Ej. Entrega de material"
                      />
                    </div>
                  </div>

                  <!-- Vehiculo -->
                  {#if tieneVehiculos}
                    <div>
                      <label for="vehiculoSelect" class={labelClass}>
                        Vehículo <span class="text-xs text-tertiary ml-1"
                          >(opcional)</span
                        >
                      </label>
                      <select
                        id="vehiculoSelect"
                        class="{inputClass} cursor-pointer appearance-none bg-no-repeat bg-right pr-8"
                        style="background-image: url('data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%239CA3AF%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E'); background-size: 10px; background-position: calc(100% - 12px) 50%;"
                        bind:value={vehiculoId}
                      >
                        <option value={null}>Sin vehículo (caminando)</option>
                        {#each vehiculosDisponibles as v}
                          <option value={v.id}
                            >{v.placa} - {v.marca} {v.modelo}</option
                          >
                        {/each}
                      </select>
                    </div>
                  {/if}

                  <!-- Observaciones -->
                  <div class="border-t border-surface pt-2">
                    <button
                      type="button"
                      onclick={() => (showObservaciones = !showObservaciones)}
                      class="flex items-center gap-1.5 text-secondary hover:text-primary transition-colors text-sm"
                    >
                      {#if showObservaciones}
                        <ChevronDown size={14} />
                      {:else}
                        <ChevronRight size={14} />
                      {/if}
                      <span>Observaciones</span>
                    </button>
                    {#if showObservaciones}
                      <div class="mt-2" transition:slide>
                        <textarea
                          bind:value={observaciones}
                          class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-2 text-sm text-white resize-none focus:outline-none"
                          rows="2"
                          placeholder="Notas adicionales..."
                        ></textarea>
                      </div>
                    {/if}
                  </div>
                </div>
              {/if}
            </div>
          {/if}
        </div>
      </form>

      <!-- Sticky Footer (Static) -->
      <div
        class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1 sticky bottom-0 z-10"
      >
        <button
          onclick={handleClose}
          type="button"
          disabled={loading}
          class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
        >
          Cancelar
        </button>
        {#if validationResult?.puedeIngresar}
          <button
            type="submit"
            form="ingresoProveedorForm"
            disabled={loading}
            class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 flex items-center gap-2"
          >
            {#if loading}
              <span
                class="w-4 h-4 rounded-full border-2 border-current border-t-transparent animate-spin"
              ></span>
            {/if}
            Registrar Ingreso
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  /* Standardized input focus style */
  input:focus,
  textarea:focus,
  select:focus {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
    outline: none !important;
  }

  /* Autofill Fix for Dark Theme */
  input:-webkit-autofill {
    -webkit-text-fill-color: white !important;
    -webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
    transition: background-color 5000s ease-in-out 0s;
  }
</style>
