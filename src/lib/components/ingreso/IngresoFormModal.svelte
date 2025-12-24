<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fade, scale } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { X } from "lucide-svelte";

  // Components
  import PersonaFinder from "./shared/persona/PersonaFinder.svelte";
  import GafeteInput from "./shared/gafete/GafeteInput.svelte";
  import VehiculoFormSection from "./shared/vehiculo/VehiculoFormSection.svelte";

  // Logic
  import { ingresoService } from "$lib/logic/ingreso/ingresoService";
  import type { ValidacionIngresoResult } from "$lib/logic/ingreso/types";
  import { currentUser } from "$lib/stores/auth";

  // Props
  interface Props {
    show: boolean;
  }

  let { show = $bindable(false) }: Props = $props();

  // State
  let loading = $state(false);
  let selectedPerson = $state<any>(null);
  let validationResult = $state<ValidacionIngresoResult | null>(null);
  let gafete = $state("");
  let tieneVehiculo = $state(false);
  let vehiculoId = $state<string | undefined>(undefined);

  const dispatch = createEventDispatcher();

  // ==========================================
  // HANDLERS
  // ==========================================
  async function handlePersonSelect(event: CustomEvent) {
    const { id, type, data } = event.detail;

    // Solo contratistas por ahora
    if (type !== "contratista") {
      toast.error("Por ahora solo se permiten contratistas");
      return;
    }

    selectedPerson = data;

    // Validar automáticamente
    try {
      loading = true;
      validationResult = await ingresoService.validarIngreso("contratista", id);

      if (validationResult.persona) {
        selectedPerson = { ...selectedPerson, ...validationResult.persona };
      }

      if (!validationResult.puedeIngresar) {
        toast.error(
          validationResult.motivoRechazo || "Contratista no autorizado",
        );
      }
    } catch (e: any) {
      toast.error("Error al validar: " + e.message);
      validationResult = null;
    } finally {
      loading = false;
    }
  }

  async function handleSubmit() {
    if (!selectedPerson || !validationResult) {
      toast.error("Complete todos los campos requeridos");
      return;
    }

    if (!validationResult.puedeIngresar) {
      toast.error("Esta persona no está autorizada para ingresar");
      return;
    }

    try {
      loading = true;

      const finalGafete = gafete.trim() || "S/G";

      await ingresoService.crearIngreso(
        "contratista",
        selectedPerson.id,
        {
          gafete: finalGafete,
          vehiculoId: tieneVehiculo ? vehiculoId : null,
          observaciones: "",
          esExcepcional: false,
          tipoAutorizacion: "praind",
          modoIngreso: tieneVehiculo ? "vehiculo" : "caminando",
        },
        selectedPerson,
        $currentUser?.id,
      );

      toast.success("¡Ingreso registrado exitosamente!");
      dispatch("complete");
      handleClose();
    } catch (e: any) {
      toast.error("Error al registrar ingreso: " + e.message);
    } finally {
      loading = false;
    }
  }

  function handleClose() {
    if (loading) return;
    show = false;
    reset();
  }

  function reset() {
    selectedPerson = null;
    validationResult = null;
    gafete = "";
    tieneVehiculo = false;
    vehiculoId = undefined;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      handleClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- Overlay -->
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
    onclick={(e) => {
      if (e.target === e.currentTarget) handleClose();
    }}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        if (e.target === e.currentTarget) handleClose();
      }
    }}
    role="button"
    tabindex="-1"
  >
    <!-- Modal -->
    <div
      class="bg-surface-2 rounded-lg shadow-surface-xl border border-surface max-w-2xl w-full max-h-[90vh] overflow-auto"
      transition:scale={{ duration: 200, start: 0.95 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-surface"
      >
        <div>
          <h2 class="text-xl font-semibold text-primary">Nuevo Ingreso</h2>
          <p class="text-sm text-secondary mt-1">
            Busca por cédula o nombre del contratista
          </p>
        </div>
        <button
          onclick={handleClose}
          class="p-2 hover:bg-surface-hover rounded-md transition-colors"
          disabled={loading}
        >
          <X size={20} class="text-secondary" />
        </button>
      </div>

      <!-- Content -->
      <div class="p-6 space-y-6">
        <!-- Buscador -->
        <div>
          <PersonaFinder on:select={handlePersonSelect} />
        </div>

        <!-- Persona seleccionada -->
        {#if selectedPerson && validationResult}
          <div
            class="p-4 bg-surface-1 rounded-lg border border-surface"
            transition:fade
          >
            <div class="flex items-center gap-3 mb-4">
              <div
                class="w-10 h-10 rounded-full bg-accent flex items-center justify-center text-white font-bold"
              >
                {selectedPerson.nombre?.charAt(0) || "?"}
              </div>
              <div class="flex-1">
                <div class="flex items-center justify-between">
                  <div class="font-semibold text-primary">
                    {selectedPerson.nombreCompleto ||
                      `${selectedPerson.nombre} ${selectedPerson.apellido}`}
                  </div>
                  <div
                    class="text-xs font-mono text-secondary px-2 py-1 bg-surface-2 rounded border border-surface"
                  >
                    {selectedPerson.cedula || "N/A"}
                  </div>
                </div>
                <div class="text-sm text-secondary">
                  {selectedPerson.empresaNombre ||
                    selectedPerson.empresa ||
                    "Sin empresa"}
                </div>
              </div>
            </div>

            <!-- Status Badge -->
            {#if validationResult.puedeIngresar}
              <div
                class="flex items-center gap-2 text-sm text-success bg-success bg-opacity-10 px-3 py-2 rounded-md"
              >
                <span class="font-medium">✓ Autorizado para ingresar</span>
              </div>
            {:else}
              <div
                class="flex items-center gap-2 text-sm text-error bg-error bg-opacity-10 px-3 py-2 rounded-md"
              >
                <span class="font-medium">
                  ✗ {validationResult.motivoRechazo || "No autorizado"}
                </span>
              </div>
            {/if}

            <!-- Form (solo si está autorizado) -->
            {#if validationResult.puedeIngresar}
              <div class="mt-4 space-y-4">
                <!-- Gafete -->
                <GafeteInput bind:value={gafete} autofocus disabled={loading} />

                <!-- Vehículo -->
                <VehiculoFormSection
                  bind:tieneVehiculo
                  bind:vehiculoId
                  vehiculosRegistrados={selectedPerson.vehiculos || []}
                />

                <!-- Botón Registrar -->
                <button
                  onclick={handleSubmit}
                  disabled={loading}
                  class="w-full btn-primary btn-base py-3 font-semibold disabled:opacity-50"
                >
                  {#if loading}
                    <span class="inline-block animate-spin mr-2">⏳</span>
                  {/if}
                  Registrar Ingreso
                </button>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  /* Asegurar que el modal esté por encima de todo */
  :global(body:has(.fixed.z-50)) {
    overflow: hidden;
  }
</style>
