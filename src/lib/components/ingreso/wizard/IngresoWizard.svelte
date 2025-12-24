<script lang="ts">
  import { fade, slide } from "svelte/transition";
  import { createEventDispatcher } from "svelte";

  // Shared Components
  import PersonaFinder from "../shared/persona/PersonaFinder.svelte";
  import AutorizacionStatus from "../shared/status/AutorizacionStatus.svelte";
  import GafeteInput from "../shared/gafete/GafeteInput.svelte";
  import VehiculoFormSection from "../shared/vehiculo/VehiculoFormSection.svelte";

  // Logic & Types
  import { ingresoService } from "$lib/logic/ingreso/ingresoService";
  import {
    FinalizarIngresoSchema,
    type FinalizarIngresoForm,
  } from "$lib/logic/ingreso/schemas";
  import type {
    ValidacionIngresoResult,
    TipoIngreso,
  } from "$lib/logic/ingreso/types";

  // State
  let step: "SEARCH" | "VALIDATION" | "DETAILS" | "CONFIRM" = "SEARCH";
  let loading = false;
  let validationError: string | null = null;

  // Data
  let tipoIngreso: TipoIngreso | null = null;
  let candidateId: string | null = null;
  let candidateData: any = null;
  let validationResult: ValidacionIngresoResult | null = null;

  // Form Input
  let form: Partial<FinalizarIngresoForm> = {
    gafete: "",
    esExcepcional: false,
    autorizadoPor: "",
    motivoExcepcional: "",
    observaciones: "",
    vehiculoId: undefined,
  };
  let tieneVehiculo = false;

  // Helper computed
  $: isBlocked = validationResult && !validationResult.puedeIngresar;
  $: canProceedFromValidation =
    validationResult && (validationResult.puedeIngresar || form.esExcepcional);

  // Handlers
  async function handleSelection(event: CustomEvent) {
    const { id, type, data } = event.detail;
    console.log("Selected:", id, type, data);

    tipoIngreso = type as TipoIngreso;
    // Ojo: PersonaFinder devuelve 'contratista', 'proveedor', etc. Asegurar que matchee `TipoIngreso`.
    if (!["contratista", "proveedor", "visita"].includes(type)) {
      alert("Tipo de persona no soportado para ingreso: " + type);
      return;
    }

    candidateId = id;
    candidateData = data;
    step = "VALIDATION";

    // Auto-trigger validation
    await runValidation();
  }

  async function runValidation() {
    if (!tipoIngreso || !candidateId) return;
    loading = true;
    validationError = null;
    try {
      validationResult = await ingresoService.validarIngreso(
        tipoIngreso,
        candidateId,
      );
      // If auto-pass? Maybe stop to show status anyway.
    } catch (e: any) {
      console.error("Validation error", e);
      validationError = e.toString();
    } finally {
      loading = false;
    }
  }

  function handleAuthorizeExcepcional() {
    // User wants to override block
    form.esExcepcional = true;
    // We stay in Validation/Details but allow proceeding.
    // Usually we move to next step or show form for exception.
    proceedToDetails();
  }

  function proceedToDetails() {
    step = "DETAILS";
  }

  async function handleSubmit() {
    // Validate Form with Zod
    // Build object
    const payload = {
      gafete: form.gafete,
      vehiculoId: tieneVehiculo ? form.vehiculoId : null,
      observaciones: form.observaciones,
      esExcepcional: form.esExcepcional,
      autorizadoPor: form.autorizadoPor,
      motivoExcepcional: form.motivoExcepcional,
    };

    try {
      // Parse checks
      FinalizarIngresoSchema.parse(payload);

      loading = true;
      await ingresoService.crearIngreso(
        tipoIngreso!,
        candidateId!,
        payload as FinalizarIngresoForm,
        candidateData,
      );

      // Success
      step = "CONFIRM";
      // Reset after generic delay or user action? Or dispatch "complete"
      dispatch("complete", {
        tipo: tipoIngreso,
        persona: validationResult?.persona,
      });
    } catch (e: any) {
      if (e.issues) {
        // Zod Error
        alert(
          "Error de validación: " +
            e.issues.map((i: any) => i.message).join("\n"),
        );
      } else {
        alert("Error al procesar ingreso: " + e.message);
      }
    } finally {
      loading = false;
    }
  }

  function reset() {
    step = "SEARCH";
    tipoIngreso = null;
    candidateId = null;
    candidateData = null;
    validationResult = null;
    form = { gafete: "", esExcepcional: false };
    tieneVehiculo = false;
  }

  const dispatch = createEventDispatcher();
</script>

<div class="h-full flex flex-col max-w-4xl mx-auto py-6">
  <!-- Steps Indicator (Optional) -->
  <ul class="steps mb-8 w-full">
    <li class="step {step !== 'SEARCH' ? 'step-primary' : ''}">Buscar</li>
    <li
      class="step {['VALIDATION', 'DETAILS', 'CONFIRM'].includes(step)
        ? 'step-primary'
        : ''}"
    >
      Validar
    </li>
    <li
      class="step {['DETAILS', 'CONFIRM'].includes(step) ? 'step-primary' : ''}"
    >
      Detalles
    </li>
    <li class="step {step === 'CONFIRM' ? 'step-primary' : ''}">Confirmar</li>
  </ul>

  <div
    class="flex-1 bg-base-100 rounded-box shadow-xl border border-base-200 p-8 min-h-[400px]"
  >
    <!-- STEP 1: SEARCH -->
    {#if step === "SEARCH"}
      <div
        in:fade
        class="flex flex-col items-center justify-center h-full gap-6"
      >
        <h2 class="text-2xl font-bold opacity-80">Iniciar Nuevo Ingreso</h2>
        <PersonaFinder on:select={handleSelection} />
      </div>
    {/if}

    <!-- STEP 2: VALIDATION -->
    {#if step === "VALIDATION" && validationResult}
      <div in:fade class="max-w-xl mx-auto w-full">
        <h2 class="text-2xl font-bold mb-6">Resultado de Validación</h2>

        <AutorizacionStatus
          validation={validationResult}
          onAuthorize={handleAuthorizeExcepcional}
        />

        <!-- Exceptions Inputs -->
        {#if form.esExcepcional}
          <div
            transition:slide
            class="mt-4 p-4 bg-error/10 rounded-lg border border-error/20"
          >
            <h4 class="font-bold text-error mb-2">Autorización Excepcional</h4>
            <div class="form-control mb-2">
              <label class="label" for="auth-exceptional">Autorizado Por:</label
              >
              <input
                id="auth-exceptional"
                type="text"
                bind:value={form.autorizadoPor}
                class="input input-bordered input-sm block w-full"
              />
            </div>
            <div class="form-control">
              <label class="label" for="reason-exceptional">Motivo:</label>
              <input
                id="reason-exceptional"
                type="text"
                bind:value={form.motivoExcepcional}
                class="input input-bordered input-sm block w-full"
              />
            </div>
          </div>
        {/if}

        <div class="flex justify-between mt-8">
          <button class="btn btn-ghost" on:click={reset}
            >Cancelar / Volver</button
          >

          {#if canProceedFromValidation}
            <button class="btn btn-primary" on:click={proceedToDetails}>
              Continuar
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5 ml-2"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fill-rule="evenodd"
                  d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z"
                  clip-rule="evenodd"
                />
              </svg>
            </button>
          {:else}
            <button class="btn btn-disabled">Bloqueado</button>
          {/if}
        </div>
      </div>
    {/if}

    <!-- STEP 3: DETAILS -->
    {#if step === "DETAILS"}
      <div in:fade class="max-w-xl mx-auto w-full flex flex-col gap-6">
        <h2 class="text-2xl font-bold">Detalles del Ingreso</h2>

        <!-- Readonly Persona Info -->
        {#if validationResult?.persona}
          <div class="flex items-center gap-4 p-4 bg-base-200 rounded-lg">
            <div class="avatar placeholder">
              <div class="bg-neutral text-neutral-content rounded-full w-12">
                <span class="text-xl">👤</span>
              </div>
            </div>
            <div>
              <div class="font-bold">
                {validationResult.persona.nombreCompleto}
              </div>
              <div class="text-sm opacity-70">
                {validationResult.persona.empresa || "Sin Empresa"}
              </div>
            </div>
          </div>
        {/if}

        <!-- Gafete -->
        <GafeteInput
          bind:value={form.gafete}
          autofocus
          on:submit={() => document.getElementById("submit-btn")?.click()}
        />

        <!-- Vehiculo -->
        <VehiculoFormSection
          bind:tieneVehiculo
          bind:vehiculoId={form.vehiculoId}
          vehiculosRegistrados={validationResult?.persona?.vehiculos || []}
        />

        <!-- Observaciones -->
        <div class="form-control">
          <label class="label" for="ingreso-obs">Observaciones</label>
          <textarea
            id="ingreso-obs"
            class="textarea textarea-bordered h-24"
            bind:value={form.observaciones}
          ></textarea>
        </div>

        <div class="flex justify-between mt-4">
          <button class="btn btn-ghost" on:click={() => (step = "VALIDATION")}
            >Atrás</button
          >
          <button
            id="submit-btn"
            class="btn btn-primary btn-wide"
            on:click={handleSubmit}
            disabled={loading}
          >
            {#if loading}
              <span class="loading loading-spinner"></span>
            {/if}
            Registrar Ingreso
          </button>
        </div>
      </div>
    {/if}

    {#if step === "CONFIRM"}
      <div in:fade class="text-center py-12">
        <div class="text-6xl mb-4">✅</div>
        <h2 class="text-3xl font-bold text-success mb-2">Ingreso Registrado</h2>
        <p class="text-base-content/60 mb-8">
          El ingreso se ha procesado correctamente.
        </p>
        <button class="btn btn-outline" on:click={reset}>Nuevo Ingreso</button>
      </div>
    {/if}

    {#if loading && step !== "DETAILS"}
      <!-- Global loading overlay if needed (mostly local loading used) -->
    {/if}
  </div>
</div>
