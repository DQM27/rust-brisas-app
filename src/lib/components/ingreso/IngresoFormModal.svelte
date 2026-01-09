<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fade, scale, slide } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { X, ChevronDown, ChevronUp, Car, FileText } from "lucide-svelte";

  // Components
  import PersonaFinder from "./shared/persona/PersonaFinder.svelte";
  import GafeteInput from "./shared/gafete/GafeteInput.svelte";

  // Logic
  import { ingresoService } from "$lib/logic/ingreso/ingresoService";
  import type { ValidacionIngresoResult } from "$lib/logic/ingreso/types";
  import { currentUser } from "$lib/stores/auth";
  import { invoke } from "@tauri-apps/api/core";

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
  let vehiculoId = $state<string | null>(null);
  let tipoAutorizacion = $state<"praind" | "correo">("praind");
  let observaciones = $state("");
  let showObservaciones = $state(false);

  const dispatch = createEventDispatcher();

  // Computed: tiene PRAIND vigente?
  let tienePraind = $derived(
    validationResult?.persona?.praindVigente === true ||
      validationResult?.contratista?.praind_vigente === true ||
      validationResult?.contratista?.praindVigente === true,
  );

  // Computed: vehículos disponibles
  let vehiculosDisponibles = $derived(selectedPerson?.vehiculos || []);
  let tieneVehiculos = $derived(vehiculosDisponibles.length > 0);

  // Auto-seleccionar vehículo único
  $effect(() => {
    if (vehiculosDisponibles.length === 1 && !vehiculoId) {
      vehiculoId = vehiculosDisponibles[0].id;
    }
  });

  // Modo de ingreso derivado
  let modoIngreso = $derived(vehiculoId ? "vehiculo" : "caminando");

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

      // DEBUG: Ver estructura de datos para PRAIND
      console.log(
        "[IngresoFormModal] validationResult:",
        JSON.stringify(validationResult, null, 2),
      );
      console.log(
        "[IngresoFormModal] contratista praind_vigente:",
        validationResult?.contratista?.praind_vigente,
      );
      console.log(
        "[IngresoFormModal] persona praindVigente:",
        validationResult?.persona?.praindVigente,
      );

      if (validationResult.persona) {
        selectedPerson = { ...selectedPerson, ...validationResult.persona };
      }

      if (!validationResult.puedeIngresar) {
        // Trigger native alert sound
        invoke("play_alert_sound");
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

      // Convert RecordId object to string if necessary
      const contratistaIdStr =
        typeof selectedPerson.id === "object"
          ? `${(selectedPerson.id as any).tb}:${(selectedPerson.id as any).id?.String || (selectedPerson.id as any).id}`
          : String(selectedPerson.id);

      const usuarioIdStr =
        typeof $currentUser?.id === "object"
          ? `${($currentUser.id as any).tb}:${($currentUser.id as any).id?.String || ($currentUser.id as any).id}`
          : String($currentUser?.id || "");

      await ingresoService.crearIngreso(
        "contratista",
        contratistaIdStr,
        {
          gafete: finalGafete,
          vehiculoId: vehiculoId,
          observaciones: observaciones.trim() || "",
          esExcepcional: !tienePraind,
          tipoAutorizacion: tienePraind ? "praind" : tipoAutorizacion,
          modoIngreso: modoIngreso,
        },
        selectedPerson,
        usuarioIdStr,
      );

      toast.success("¡Ingreso registrado exitosamente!");
      dispatch("complete");
      handleClose();
    } catch (e: any) {
      console.error("[IngresoFormModal] Error completo:", e);
      // Parsear mensaje de error del backend (formato thiserror)
      let errorMsg = "Error al registrar ingreso";

      if (typeof e === "string") {
        errorMsg = e;
      } else if (e?.type) {
        // Mapear tipos de error a mensajes amigables
        const errorMessages: Record<string, string> = {
          GafeteNotAvailable: "El gafete especificado no está disponible",
          AlreadyInside: "El contratista ya tiene un ingreso activo",
          ContratistaNotFound: "Contratista no encontrado",
          Blacklisted: e.message
            ? `En lista negra: ${e.message}`
            : "Persona en lista negra",
          PraindExpired: e.message
            ? `PRAIND vencido: ${e.message}`
            : "PRAIND vencido",
          ContratistaInactive: "El contratista no está activo",
          Validation: e.message || "Error de validación",
          Database: "Error de base de datos",
          Gafete: e.message || "Error con el gafete",
        };
        errorMsg = errorMessages[e.type] || e.message || e.type;
      } else if (e?.message) {
        errorMsg = e.message;
      }

      toast.error(errorMsg);
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
    vehiculoId = null;
    tipoAutorizacion = "praind";
    observaciones = "";
    showObservaciones = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      handleClose();
    }
    // Ctrl+S para guardar
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      if (
        show &&
        !loading &&
        selectedPerson &&
        validationResult?.puedeIngresar
      ) {
        handleSubmit();
      }
    }
  }

  function getSeverityClasses(severity?: string) {
    const base =
      "flex items-center gap-2 text-sm px-3 py-2 rounded-md border transition-colors";
    const upperSeverity = severity?.toUpperCase();

    if (upperSeverity === "ALTO") {
      return `${base} bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800`;
    }
    if (upperSeverity === "MEDIO") {
      return `${base} bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/30 dark:text-yellow-300 dark:border-yellow-800`;
    }
    if (upperSeverity === "BAJO") {
      return `${base} bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-900/30 dark:text-gray-300 dark:border-gray-800`;
    }

    // Default error style
    return `${base} text-red-700 bg-red-50 border-red-200 dark:text-red-300 dark:bg-red-900/30 dark:border-red-800`;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- Overlay -->
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
    role="dialog"
    aria-modal="true"
  >
    <!-- Modal -->
    <div
      class="bg-surface-2 rounded-lg shadow-surface-xl border border-surface max-w-xl w-full max-h-[90vh] overflow-visible flex flex-col"
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

              <!-- Alertas/Warnings (amarillo) -->
              {#if validationResult.alertas && validationResult.alertas.length > 0}
                <div class="mt-2 space-y-1">
                  {#each validationResult.alertas as alerta}
                    <div
                      class="flex items-center gap-2 text-sm text-yellow-900 bg-yellow-100 px-3 py-2 rounded-md border border-yellow-300"
                    >
                      <span class="font-medium">{alerta}</span>
                    </div>
                  {/each}
                </div>
              {/if}
            {:else}
              <div
                class={getSeverityClasses(validationResult.severidadListaNegra)}
              >
                <span class="font-medium">
                  ✗ {validationResult.motivoRechazo || "No autorizado"}
                </span>
              </div>
            {/if}

            <!-- Form (solo si está autorizado) -->
            {#if validationResult.puedeIngresar}
              <div class="mt-4 space-y-3">
                <!-- Gafete -->
                <GafeteInput bind:value={gafete} autofocus disabled={loading} />

                <!-- Vehículo - Solo si tiene vehículos registrados -->
                {#if tieneVehiculos}
                  <div class="space-y-1.5" transition:slide>
                    <label
                      class="flex items-center gap-2 text-sm font-medium text-secondary"
                    >
                      <Car size={16} />
                      Vehículo
                      <span class="text-xs text-tertiary">(opcional)</span>
                    </label>
                    <select
                      class="w-full bg-surface-1 border border-surface rounded-md px-3 py-2 text-sm text-primary focus:outline-none focus:ring-2 focus:ring-accent"
                      bind:value={vehiculoId}
                      disabled={loading}
                    >
                      <option value={null}>Sin vehículo (caminando)</option>
                      {#each vehiculosDisponibles as v}
                        <option value={v.id}>
                          {v.placa} - {v.marca || ""}
                          {v.modelo || ""}
                        </option>
                      {/each}
                    </select>
                  </div>
                {/if}

                <!-- Tipo de Autorización - Solo si NO tiene PRAIND -->
                {#if !tienePraind}
                  <div
                    class="space-y-1.5 p-3 bg-yellow-500/10 border border-yellow-500/30 rounded-md"
                    transition:slide
                  >
                    <label
                      class="flex items-center gap-2 text-sm font-medium text-yellow-300"
                    >
                      <FileText size={16} />
                      Tipo de Autorización
                    </label>
                    <p class="text-xs text-yellow-400/80 mb-2">
                      El contratista no tiene PRAIND vigente. Seleccione el tipo
                      de autorización.
                    </p>
                    <div class="flex gap-3">
                      <label class="flex items-center gap-2 cursor-pointer">
                        <input
                          type="radio"
                          name="tipoAutorizacion"
                          value="praind"
                          bind:group={tipoAutorizacion}
                          class="radio radio-sm radio-warning"
                          disabled={loading}
                        />
                        <span class="text-sm text-primary">PRAIND</span>
                      </label>
                      <label class="flex items-center gap-2 cursor-pointer">
                        <input
                          type="radio"
                          name="tipoAutorizacion"
                          value="correo"
                          bind:group={tipoAutorizacion}
                          class="radio radio-sm radio-warning"
                          disabled={loading}
                        />
                        <span class="text-sm text-primary">Correo</span>
                      </label>
                    </div>
                  </div>
                {/if}

                <!-- Observaciones - Toggle colapsable -->
                <div class="border-t border-surface pt-2">
                  <button
                    type="button"
                    onclick={() => (showObservaciones = !showObservaciones)}
                    class="flex items-center gap-2 text-sm text-secondary hover:text-primary transition-colors w-full"
                  >
                    {#if showObservaciones}
                      <ChevronUp size={16} />
                    {:else}
                      <ChevronDown size={16} />
                    {/if}
                    <span>Observaciones</span>
                    {#if observaciones.trim()}
                      <span class="text-xs text-accent">(tiene contenido)</span>
                    {/if}
                  </button>

                  {#if showObservaciones}
                    <div class="mt-2" transition:slide>
                      <textarea
                        class="w-full bg-surface-1 border border-surface rounded-md px-3 py-2 text-sm text-primary resize-none focus:outline-none focus:ring-2 focus:ring-accent"
                        rows="2"
                        placeholder="Notas adicionales..."
                        bind:value={observaciones}
                        disabled={loading}
                      ></textarea>
                    </div>
                  {/if}
                </div>

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
