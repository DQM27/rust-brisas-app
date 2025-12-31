<script lang="ts">
  import { fade, fly, scale } from "svelte/transition";
  import { X, Car, Plus } from "lucide-svelte";
  import { onMount, onDestroy } from "svelte";
  import type {
    ContratistaResponse,
    CreateContratistaInput,
    UpdateContratistaInput,
  } from "$lib/types/contratista";
  import { submitCreateEmpresa } from "$lib/logic/empresa/empresaService";
  import { invoke } from "@tauri-apps/api/core";
  import { empresaStore } from "$lib/stores/empresaStore.svelte";
  import VehiculoManagerModal from "$lib/components/vehiculo/VehiculoManagerModal.svelte";

  // Superforms & Zod v4
  import { superForm } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
  import {
    contratistaSchema,
    type ContratistaFormData,
  } from "$lib/schemas/contratistaSchema";

  const defaultValues: ContratistaFormData = {
    cedula: "",
    nombre: "",
    segundoNombre: "",
    apellido: "",
    segundoApellido: "",
    empresaId: "",
    fechaVencimientoPraind: "",
  };

  interface Props {
    show: boolean;
    contratista?: ContratistaResponse | null;
    loading?: boolean;
    onSave: (
      data: CreateContratistaInput | UpdateContratistaInput,
    ) => Promise<boolean | void>;
    onClose: () => void;
    readonly?: boolean;
  }

  let {
    show,
    contratista = null,
    loading = false,
    onSave,
    onClose,
    readonly = false,
  }: Props = $props();

  // Derived Mode
  const isEditMode = $derived(!!contratista);
  const modalTitle = $derived(
    readonly
      ? `Ver Detalle: ${contratista?.nombre} ${contratista?.apellido}`
      : isEditMode
        ? `Editar: ${contratista?.nombre} ${contratista?.apellido}`
        : "Nuevo Contratista",
  );

  // Empresas State
  let showEmpresaModal = $state(false);
  let nuevaEmpresaNombre = $state("");
  let creatingEmpresa = $state(false);
  let empresaError = $state("");

  // Vehicle Modal State
  let showVehiculoModal = $state(false);

  // Validation State for Real-time checks
  let checkTimeout: any;
  let cedulaDuplicateError = $state<string | null>(null);

  // Initial Data Construction
  const initialData = $derived.by(() => {
    return {
      cedula: contratista?.cedula || "",
      nombre: contratista?.nombre || "",
      segundoNombre: contratista?.segundoNombre || "",
      apellido: contratista?.apellido || "",
      segundoApellido: contratista?.segundoApellido || "",
      empresaId: contratista?.empresaId || "",
      fechaVencimientoPraind: contratista?.fechaVencimientoPraind
        ? formatDateForDisplay(contratista.fechaVencimientoPraind)
        : "",
    };
  });

  // Superform Initialization with Zod v4 adapter
  // Pattern: Pass initial values directly (like ProveedorForm), use zod4 for validators only
  const { form, errors, constraints, enhance, reset, validate } =
    superForm<ContratistaFormData>(defaultValues, {
      SPA: true,
      validators: zod4(contratistaSchema),
      resetForm: false, // We control reset manually when modal opens/closes
      onUpdate: async ({ form: f }) => {
        if (f.valid) {
          if (cedulaDuplicateError) return; // Block submit if duplicate

          const data = {
            ...f.data,
            fechaVencimientoPraind: formatDateForBackend(
              f.data.fechaVencimientoPraind,
            ),
          };

          const payload: any = {
            cedula: data.cedula,
            nombre: data.nombre,
            apellido: data.apellido,
            empresaId: data.empresaId,
            fechaVencimientoPraind: data.fechaVencimientoPraind,
          };

          if (data.segundoNombre) payload.segundoNombre = data.segundoNombre;
          if (data.segundoApellido)
            payload.segundoApellido = data.segundoApellido;

          const success = await onSave(payload);
          if (success) {
            handleClose();
          }
        }
      },
    });

  // Sync form with props when modal opens/changes
  $effect(() => {
    if (show) {
      if (contratista) {
        reset({ data: initialData });
      } else {
        reset();
        cedulaDuplicateError = null;
      }
    }
  });

  // Load companies
  onMount(async () => {
    await empresaStore.init();
  });

  onDestroy(() => {
    if (checkTimeout) clearTimeout(checkTimeout);
  });

  // Helpers
  function formatDateForDisplay(isoDate: string): string {
    if (!isoDate) return "";
    const [year, month, day] = isoDate.split("T")[0].split("-");
    return `${day}-${month}-${year}`;
  }

  function formatDateForBackend(displayDate: string): string {
    if (!displayDate || displayDate.length !== 10) return "";
    const [day, month, year] = displayDate.split("-");
    return `${year}-${month}-${day}`;
  }

  function handleClose() {
    if (!loading) {
      onClose();
    }
  }

  // Real-time Validation for Cedula
  function handleCedulaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const value = input.value;

    // Update superform store
    $form.cedula = value;

    if (checkTimeout) clearTimeout(checkTimeout);

    if (value.length < 4) {
      cedulaDuplicateError = null;
      return;
    }

    checkTimeout = setTimeout(async () => {
      try {
        const isUnique = await invoke<boolean>("check_unique", {
          table: "contratista",
          field: "cedula",
          value,
          excludeId: contratista?.id,
        });

        if (!isUnique) {
          cedulaDuplicateError = "Esta cédula ya está registrada.";
        } else {
          cedulaDuplicateError = null;
        }
      } catch (e) {
        console.error("Error checking uniqueness:", e);
      }
    }, 400);
  }

  async function handleCrearEmpresa() {
    if (!nuevaEmpresaNombre.trim()) return;
    creatingEmpresa = true;
    empresaError = "";
    const result = await submitCreateEmpresa(nuevaEmpresaNombre);
    if (result.ok) {
      empresaStore.add(result.empresa);
      $form.empresaId = result.empresa.id;
      nuevaEmpresaNombre = "";
      showEmpresaModal = false;
    } else {
      empresaError = result.error;
    }
    creatingEmpresa = false;
  }

  // GitHub-style styles with blue accent
  const labelClass =
    "block text-sm font-medium text-gray-500 dark:text-gray-400 mb-1.5";
  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-[#30363d] bg-white dark:bg-[#0d1117] px-3 py-2.5 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2563eb] focus:border-[#2563eb] focus:outline-none transition-all placeholder-gray-400 dark:placeholder-gray-500 disabled:opacity-50 disabled:bg-gray-100 dark:disabled:bg-[#161b22]";
  const errorInputClass =
    "border-red-500 dark:border-red-500 focus:ring-red-500 focus:border-red-500";
</script>

{#if show}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="absolute inset-0 bg-black/60"
      role="button"
      tabindex="0"
      onclick={handleClose}
    ></div>

    <!-- Modal -->
    <div
      class="relative z-10 w-full max-w-md max-h-[90vh] overflow-hidden rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]"
      >
        <div>
          <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
            {modalTitle}
          </h2>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
            {readonly
              ? "Información detallada del contratista"
              : isEditMode
                ? "Modifique los datos necesarios"
                : "Ingresa los datos del contratista"}
          </p>
        </div>
        <button
          type="button"
          onclick={handleClose}
          disabled={loading}
          class="p-1.5 rounded-md text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Form -->
      <form method="POST" use:enhance>
        <div class="p-6 space-y-4">
          <!-- Cédula -->
          <div class="space-y-1">
            <label for="cedula" class={labelClass}
              >Cédula <span class="text-red-500">*</span></label
            >
            <input
              id="cedula"
              name="cedula"
              type="text"
              bind:value={$form.cedula}
              oninput={handleCedulaInput}
              placeholder="1-2345-6789"
              disabled={loading || isEditMode || readonly}
              class="{inputClass} {$errors.cedula || cedulaDuplicateError
                ? errorInputClass
                : ''}"
              {...$constraints.cedula}
            />
            {#if $errors.cedula}
              <p class="text-xs text-red-500 mt-1">{$errors.cedula}</p>
            {:else if cedulaDuplicateError}
              <p class="text-xs text-red-500 mt-1">{cedulaDuplicateError}</p>
            {/if}
          </div>

          <!-- Nombre / Apellido -->
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-1">
              <label for="nombre" class={labelClass}
                >Nombre <span class="text-red-500">*</span></label
              >
              <input
                id="nombre"
                name="nombre"
                type="text"
                bind:value={$form.nombre}
                placeholder="Juan"
                disabled={loading || readonly}
                class="{inputClass} {$errors.nombre ? errorInputClass : ''}"
                {...$constraints.nombre}
              />
              {#if $errors.nombre}<p class="text-xs text-red-500 mt-1">
                  {$errors.nombre}
                </p>{/if}
            </div>
            <div class="space-y-1">
              <label for="apellido" class={labelClass}
                >Apellido <span class="text-red-500">*</span></label
              >
              <input
                id="apellido"
                name="apellido"
                type="text"
                bind:value={$form.apellido}
                placeholder="Pérez"
                disabled={loading || readonly}
                class="{inputClass} {$errors.apellido ? errorInputClass : ''}"
                {...$constraints.apellido}
              />
              {#if $errors.apellido}<p class="text-xs text-red-500 mt-1">
                  {$errors.apellido}
                </p>{/if}
            </div>
          </div>

          <!-- Segundo Nombre / Apellido (Opcional) -->
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-1">
              <label for="segundoNombre" class={labelClass}
                >Segundo Nombre</label
              >
              <input
                id="segundoNombre"
                name="segundoNombre"
                type="text"
                bind:value={$form.segundoNombre}
                disabled={loading || readonly}
                class={inputClass}
                {...$constraints.segundoNombre}
              />
            </div>
            <div class="space-y-1">
              <label for="segundoApellido" class={labelClass}
                >Segundo Apellido</label
              >
              <input
                id="segundoApellido"
                name="segundoApellido"
                type="text"
                bind:value={$form.segundoApellido}
                disabled={loading || readonly}
                class={inputClass}
                {...$constraints.segundoApellido}
              />
            </div>
          </div>

          <!-- Empresa -->
          <div class="space-y-1">
            <label for="empresaId" class={labelClass}
              >Empresa <span class="text-red-500">*</span></label
            >
            <div class="flex gap-2">
              <div class="relative flex-1">
                <select
                  id="empresaId"
                  name="empresaId"
                  bind:value={$form.empresaId}
                  disabled={loading || empresaStore.loading || readonly}
                  class="{inputClass} {$errors.empresaId
                    ? errorInputClass
                    : ''}"
                  {...$constraints.empresaId}
                >
                  <option value="" disabled>
                    {empresaStore.loading
                      ? "Cargando..."
                      : "Seleccione empresa"}
                  </option>
                  {#each empresaStore.empresas as empresa}
                    <option value={empresa.id}>{empresa.nombre}</option>
                  {/each}
                </select>
              </div>
              {#if !readonly}
                <button
                  type="button"
                  onclick={() => (showEmpresaModal = true)}
                  disabled={loading}
                  class="px-3 py-2 rounded-md border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-[#21262d] text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#30363d] transition-colors text-sm"
                >
                  <Plus class="w-4 h-4" />
                </button>
              {/if}
            </div>
            {#if $errors.empresaId}<p class="text-xs text-red-500 mt-1">
                {$errors.empresaId}
              </p>{/if}
          </div>

          <!-- Fecha PRAIND -->
          <div class="space-y-1">
            <label for="fechaVencimientoPraind" class={labelClass}
              >Vencimiento PRAIND <span class="text-red-500">*</span></label
            >
            <input
              id="fechaVencimientoPraind"
              name="fechaVencimientoPraind"
              type="text"
              bind:value={$form.fechaVencimientoPraind}
              placeholder="DD-MM-YYYY"
              maxlength="10"
              disabled={loading || readonly}
              class="{inputClass} {$errors.fechaVencimientoPraind
                ? errorInputClass
                : ''}"
              {...$constraints.fechaVencimientoPraind}
              oninput={(e) => {
                const input = e.target as HTMLInputElement;
                let value = input.value.replace(/[^\d-]/g, "");
                if (value.length >= 3 && value[2] !== "-") {
                  value = value.slice(0, 2) + "-" + value.slice(2);
                }
                if (value.length >= 6 && value[5] !== "-") {
                  value = value.slice(0, 5) + "-" + value.slice(5);
                }
                value = value.slice(0, 10);
                $form.fechaVencimientoPraind = value;
                input.value = value;
              }}
            />
            {#if $errors.fechaVencimientoPraind}<p
                class="text-xs text-red-500 mt-1"
              >
                {$errors.fechaVencimientoPraind}
              </p>{/if}
          </div>

          <!-- Sección Vehículos (Solo botón) -->
          {#if isEditMode && contratista?.id}
            <div class="flex items-center justify-between pt-2">
              <span class={labelClass}>Vehículos</span>
              <button
                type="button"
                onclick={() => (showVehiculoModal = true)}
                class="flex items-center gap-2 px-3 py-1.5 text-xs font-medium rounded-md border border-gray-300 dark:border-[#30363d] text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors"
              >
                <Car class="w-3.5 h-3.5" />
                Gestionar
              </button>
            </div>
          {/if}
        </div>

        <!-- Footer -->
        <div
          class="sticky bottom-0 z-20 flex justify-end gap-3 px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]"
        >
          <button
            type="button"
            onclick={handleClose}
            disabled={loading}
            class="px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors"
          >
            Cancelar
          </button>

          {#if !readonly}
            <button
              type="submit"
              disabled={loading || !!cedulaDuplicateError}
              class="px-4 py-2.5 text-sm font-medium rounded-md bg-[#2563eb] text-white hover:bg-[#1d4ed8] disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center gap-2"
            >
              {#if loading}
                <span class="loading loading-spinner loading-xs"></span>
              {/if}
              {isEditMode ? "Guardar Cambios" : "Crear Contratista"}
            </button>
          {/if}
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Modal para crear nueva empresa -->
{#if showEmpresaModal}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="absolute inset-0 bg-black/50"
      role="button"
      tabindex="0"
      onclick={() => !creatingEmpresa && (showEmpresaModal = false)}
    ></div>
    <div
      class="relative w-full max-w-md rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:scale={{ start: 0.95, duration: 200 }}
    >
      <div
        class="px-5 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]"
      >
        <h3 class="text-base font-semibold text-gray-900 dark:text-gray-100">
          Nueva Empresa
        </h3>
      </div>
      <div class="p-5 space-y-3">
        {#if empresaError}
          <div
            class="rounded bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-2 text-xs text-red-700 dark:text-red-300"
          >
            {empresaError}
          </div>
        {/if}
        <div class="space-y-1">
          <label for="newEmpresa" class={labelClass}>Nombre de la Empresa</label
          >
          <input
            id="newEmpresa"
            type="text"
            bind:value={nuevaEmpresaNombre}
            placeholder="Servicios Generales S.A."
            disabled={creatingEmpresa}
            class={inputClass}
            onkeydown={(e) => e.key === "Enter" && handleCrearEmpresa()}
          />
        </div>
      </div>
      <div
        class="flex justify-end gap-2 px-5 py-3 bg-gray-50 dark:bg-[#161b22] border-t border-gray-200 dark:border-gray-700"
      >
        <button
          type="button"
          disabled={creatingEmpresa}
          onclick={() => (showEmpresaModal = false)}
          class="px-3 py-1.5 text-xs font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#21262d]"
        >
          Cancelar
        </button>
        <button
          type="button"
          disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
          onclick={handleCrearEmpresa}
          class="px-3 py-1.5 text-xs font-medium rounded-md bg-[#2563eb] text-white hover:bg-[#1d4ed8] disabled:opacity-50 transition-colors"
        >
          {creatingEmpresa ? "Guardando..." : "Guardar"}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Vehiculo Modal (Nested) -->
{#if showVehiculoModal && contratista}
  <VehiculoManagerModal
    show={showVehiculoModal}
    contratistaId={contratista.id}
    contratistaNombre={contratista?.nombre + " " + contratista?.apellido}
    onClose={() => (showVehiculoModal = false)}
  />
{/if}
