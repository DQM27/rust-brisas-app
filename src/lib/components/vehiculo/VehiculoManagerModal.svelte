<!-- src/lib/components/vehiculo/VehiculoManagerModal.svelte -->
<script lang="ts">
  import { fade, fly, slide } from "svelte/transition";
  import {
    X,
    Plus,
    Trash2,
    Edit2,
    Car,
    Bike,
    ChevronDown,
    Check,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import type {
    VehiculoResponse,
    CreateVehiculoInput,
  } from "$lib/types/vehiculo";
  import { vehiculos as vehiculosApi } from "$lib/api/vehiculos";
  import { invoke } from "@tauri-apps/api/core";

  // Superforms & Zod v4
  import { superForm } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
  import { z } from "zod";
  import {
    vehiculoSchema,
    type VehiculoFormData,
  } from "$lib/schemas/vehiculoSchema";

  interface Props {
    show: boolean;
    propietarioId: string;
    propietarioNombre: string;
    onClose: () => void;
  }

  let { show, propietarioId, propietarioNombre, onClose }: Props = $props();

  let vehiculosList = $state<VehiculoResponse[]>([]);
  let loading = $state(false);
  let globalError = $state<string | null>(null);

  // Validation State
  let checkTimeout: any;
  let duplicateError = $state<string | null>(null);

  // Form State
  let showForm = $state(false);
  let editingId = $state<string | null>(null);

  // Default values for form
  const defaultValues: VehiculoFormData = {
    tipoVehiculo: "",
    placa: "",
    marca: "",
    modelo: "",
    color: "",
  };

  // Initialize Superform (SPA Mode) with zod4 adapter
  const { form, errors, constraints, enhance, reset, validate, tainted } =
    superForm<VehiculoFormData>(defaultValues, {
      SPA: true,
      validators: zod4(vehiculoSchema),
      onUpdate: async ({ form: f }) => {
        if (f.valid) {
          await saveData(f.data);
        }
      },
    });

  let submitting = $state(false);

  async function loadVehiculos() {
    if (!propietarioId) return;
    loading = true;
    globalError = null;
    try {
      vehiculosList = await vehiculosApi.getByPropietario(propietarioId);
    } catch (e) {
      console.error("Error loading vehicles:", e);
      globalError = "Error al cargar los vehículos.";
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    if (show && propietarioId) {
      loadVehiculos();
      cancelForm();
    }
  });

  function cancelForm() {
    showForm = false;
    editingId = null;
    reset();
    globalError = null;
    duplicateError = null;
    if (checkTimeout) clearTimeout(checkTimeout);
  }

  function handleAddNew() {
    cancelForm(); // Reset first
    showForm = true;
  }

  function handleEdit(vehiculo: VehiculoResponse) {
    cancelForm(); // Reset first
    editingId = vehiculo.id;

    // Populate form
    $form.tipoVehiculo = vehiculo.tipoVehiculo;
    $form.placa = vehiculo.placa;
    $form.marca = vehiculo.marca || "";
    $form.modelo = vehiculo.modelo || "";
    $form.color = vehiculo.color || "";

    showForm = true;
  }

  async function handleDelete(id: string) {
    if (!confirm("¿Está seguro de eliminar este vehículo?")) return;

    try {
      await vehiculosApi.delete(id);
      await loadVehiculos();
    } catch (e) {
      console.error("Error deleting vehicle:", e);
      globalError = "Error al eliminar el vehículo.";
    }
  }

  async function saveData(data: any) {
    if (duplicateError) return; // Prevent save if duplicate

    submitting = true;
    globalError = null;

    try {
      const input: CreateVehiculoInput = {
        propietarioId: propietarioId,
        tipoVehiculo: data.tipoVehiculo,
        placa: data.placa,
        marca: data.marca || undefined,
        modelo: data.modelo || undefined,
        color: data.color || undefined,
      };

      if (editingId) {
        await vehiculosApi.update(editingId, input);
      } else {
        await vehiculosApi.create(input);
      }

      await loadVehiculos();
      cancelForm();
    } catch (e) {
      console.error("Error saving vehicle:", e);
      globalError =
        "Error al guardar el vehículo. Verifique si la placa ya existe.";
    } finally {
      submitting = false;
    }
  }

  // Real-time validation
  function handlePlacaInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const value = input.value.trim().toUpperCase();

    // Update form store manually to ensure sync (though bind:value does it)
    $form.placa = value;

    if (checkTimeout) clearTimeout(checkTimeout);

    if (value.length < 3) {
      duplicateError = null;
      return;
    }

    checkTimeout = setTimeout(async () => {
      try {
        // Basic regex check from schema to avoid unnecessary backend calls
        if (!/^[A-Z0-9\s-]+$/.test(value)) return;

        const isUnique = await invoke<boolean>("check_unique", {
          table: "vehiculo",
          field: "placa",
          value,
          excludeId: editingId,
        });

        if (!isUnique) {
          duplicateError = "Esta placa ya está registrada en el sistema";
        } else {
          duplicateError = null;
        }
      } catch (e) {
        console.error("Error checking uniqueness:", e);
      }
    }, 400);
  }

  // --- STANDARD UI PATTERNS ---
  // Input de texto estándar (34px altura)
  const inputClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all";

  // Botón trigger para Selects Custom
  const selectClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white focus:outline-none disabled:opacity-50 transition-all cursor-pointer appearance-none bg-no-repeat bg-right pr-8 flex items-center justify-between";

  // Labels
  const labelClass = "block text-xs font-medium text-secondary mb-1";

  // Mensajes de Error
  const errorClass = "text-xs text-red-500 mt-0.5";

  // Helper to determine field border color based on state
  function getFieldStateClass(field: string, value: any) {
    if (($errors as any)[field] || (field === "placa" && duplicateError))
      return "!border-red-500/50 !ring-1 !ring-red-500/20";

    // Success state CHECK
    if (value && String(value).trim() !== "") {
      return "!border-green-500/50 !ring-1 !ring-green-500/20";
    }

    return "";
  }

  // Custom Dropdown State
  let showTipoDropdown = $state(false);
  const tipoOptions = [
    { value: "motocicleta", label: "Motocicleta" },
    { value: "automovil", label: "Automóvil" },
    { value: "camioneta", label: "Camioneta" },
    { value: "camion", label: "Camión" },
    { value: "otro", label: "Otro" },
  ];
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- Backdrop -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="absolute inset-0 bg-black/60"
      role="button"
      tabindex="0"
      onclick={onClose}
    ></div>

    <!-- Modal -->
    <div
      class="relative z-10 w-full max-w-[450px] min-h-[500px] max-h-[95vh] flex flex-col rounded-xl bg-surface-2 shadow-2xl border border-surface overflow-hidden"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <!-- Header -->
      <div
        class="flex-none flex items-center justify-between px-3 py-3 bg-surface-2 border-b border-surface"
      >
        <div>
          <h2 class="text-xl font-semibold text-primary">Vehículos</h2>
          <p class="text-xs text-secondary mt-0.5">
            Gestión de vehículos para {propietarioNombre}
          </p>
        </div>
        <button
          onclick={onClose}
          class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6 space-y-6">
        {#if globalError}
          <div
            class="p-3 rounded-md bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-300 text-sm border border-red-200 dark:border-red-800"
          >
            {globalError}
          </div>
        {/if}

        <!-- List -->
        {#if !showForm}
          <div class="space-y-3">
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-sm font-medium text-secondary">
                Vehículos Registrados ({vehiculosList.length})
              </h3>
              <button
                onclick={handleAddNew}
                class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-lg text-secondary border border-surface hover:text-white hover:border-white/20 transition-all"
              >
                <Plus size={14} /> Nuevo Vehículo
              </button>
            </div>

            {#if loading}
              <div class="text-center py-8 text-secondary text-sm">
                Cargando vehículos...
              </div>
            {:else if vehiculosList.length === 0}
              <div
                class="text-center py-8 text-secondary text-sm border border-dashed border-surface rounded-lg bg-surface-1/50"
              >
                No hay vehículos registrados.
              </div>
            {:else}
              <div class="grid gap-3">
                {#each vehiculosList as v}
                  <div
                    class="flex items-center justify-between p-3 rounded-lg border border-surface bg-surface-1 hover:border-white/10 transition-colors"
                  >
                    <div class="flex items-center gap-3">
                      <div class="p-2 rounded-lg bg-surface-3 text-primary">
                        {#if v.tipoVehiculo === "motocicleta"}
                          <Bike size={18} />
                        {:else}
                          <Car size={18} />
                        {/if}
                      </div>
                      <div>
                        <div class="font-semibold text-primary text-sm">
                          {v.placa}
                        </div>
                        <div class="text-xs text-secondary">
                          {v.marca || "Sin marca"} • {v.modelo || "Sin modelo"} •
                          {v.color || "Sin color"}
                        </div>
                      </div>
                    </div>
                    <div class="flex items-center gap-2">
                      <button
                        onclick={() => handleEdit(v)}
                        class="p-1.5 text-secondary hover:text-blue-400 hover:bg-blue-500/10 rounded-lg transition-colors"
                        title="Editar"
                      >
                        <Edit2 size={16} />
                      </button>
                      <button
                        onclick={() => handleDelete(v.id)}
                        class="p-1.5 text-secondary hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors"
                        title="Eliminar"
                      >
                        <Trash2 size={16} />
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {:else}
          <!-- Form -->
          <div
            class="bg-surface-1 p-6 rounded-lg border border-surface space-y-4"
            transition:fly={{ y: 10, duration: 200 }}
          >
            <h3
              class="text-sm font-semibold text-primary border-b border-surface pb-2"
            >
              {editingId ? "Editar Vehículo" : "Nuevo Vehículo"}
            </h3>

            <form method="POST" use:enhance class="space-y-4" id="vehiculoForm">
              <!-- Tipo Vehículo Custom Dropdown -->
              <div class="space-y-1 relative">
                <label for="tipoVehiculo" class={labelClass}
                  >Tipo de Vehículo <span class="text-red-500">*</span></label
                >
                <div class="relative">
                  <button
                    type="button"
                    onclick={() => (showTipoDropdown = !showTipoDropdown)}
                    class="{selectClass} {getFieldStateClass(
                      'tipoVehiculo',
                      $form.tipoVehiculo,
                    )}"
                  >
                    <span
                      class={$form.tipoVehiculo
                        ? "text-white"
                        : "text-gray-500"}
                    >
                      {tipoOptions.find((o) => o.value === $form.tipoVehiculo)
                        ?.label || "Seleccione un tipo"}
                    </span>
                    <ChevronDown size={14} class="text-gray-400" />
                  </button>

                  {#if showTipoDropdown}
                    <div
                      class="absolute z-50 left-0 right-0 top-full mt-1 bg-[#1c2128] border border-surface rounded-lg shadow-xl overflow-hidden"
                      transition:slide={{ duration: 150 }}
                    >
                      <div class="max-h-48 overflow-y-auto p-1">
                        {#each tipoOptions as option}
                          <button
                            type="button"
                            onclick={() => {
                              $form.tipoVehiculo = option.value;
                              showTipoDropdown = false;
                            }}
                            class="w-full text-left px-3 py-2 text-sm text-gray-300 hover:bg-white/5 hover:text-white rounded-md transition-colors flex items-center justify-between"
                          >
                            {option.label}
                            {#if $form.tipoVehiculo === option.value}
                              <Check size={14} class="text-blue-400" />
                            {/if}
                          </button>
                        {/each}
                      </div>
                    </div>
                    <!-- Close when clicking outside -->
                    <div
                      class="fixed inset-0 z-40"
                      onclick={() => (showTipoDropdown = false)}
                      role="button"
                      tabindex="-1"
                      onkeydown={(e) =>
                        e.key === "Escape" && (showTipoDropdown = false)}
                    ></div>
                  {/if}
                </div>
                <!-- Hidden input for binding/validation compatibility if needed, though bind:value updates form -->
                <input
                  type="hidden"
                  name="tipoVehiculo"
                  bind:value={$form.tipoVehiculo}
                />

                {#if $errors.tipoVehiculo}
                  <p class={errorClass}>
                    {$errors.tipoVehiculo}
                  </p>
                {/if}
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-1">
                  <label for="placa" class={labelClass}
                    >Placa <span class="text-red-500">*</span></label
                  >
                  <input
                    id="placa"
                    name="placa"
                    type="text"
                    bind:value={$form.placa}
                    oninput={handlePlacaInput}
                    placeholder="ABC-123"
                    class="{inputClass} uppercase {getFieldStateClass(
                      'placa',
                      $form.placa,
                    )}"
                    disabled={submitting}
                    {...$constraints.placa}
                  />
                  {#if $errors.placa}
                    <p class={errorClass}>{$errors.placa}</p>
                  {:else if duplicateError}
                    <p class={errorClass}>{duplicateError}</p>
                  {/if}
                </div>
                <div class="space-y-1">
                  <label for="marca" class={labelClass}>Marca</label>
                  <input
                    id="marca"
                    name="marca"
                    type="text"
                    bind:value={$form.marca}
                    placeholder="Toyota"
                    class="{inputClass} {getFieldStateClass(
                      'marca',
                      $form.marca,
                    )}"
                    disabled={submitting}
                    {...$constraints.marca}
                  />
                  {#if $errors.marca}
                    <p class={errorClass}>{$errors.marca}</p>
                  {/if}
                </div>
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-1">
                  <label for="modelo" class={labelClass}>Modelo</label>
                  <input
                    id="modelo"
                    name="modelo"
                    type="text"
                    bind:value={$form.modelo}
                    placeholder="Corolla"
                    class="{inputClass} {getFieldStateClass(
                      'modelo',
                      $form.modelo,
                    )}"
                    disabled={submitting}
                    {...$constraints.modelo}
                  />
                  {#if $errors.modelo}
                    <p class={errorClass}>{$errors.modelo}</p>
                  {/if}
                </div>
                <div class="space-y-1">
                  <label for="color" class={labelClass}>Color</label>
                  <input
                    id="color"
                    name="color"
                    type="text"
                    bind:value={$form.color}
                    placeholder="Blanco"
                    class="{inputClass} {getFieldStateClass(
                      'color',
                      $form.color,
                    )}"
                    disabled={submitting}
                    {...$constraints.color}
                  />
                  {#if $errors.color}
                    <p class={errorClass}>{$errors.color}</p>
                  {/if}
                </div>
              </div>
            </form>
          </div>
        {/if}
      </div>

      <!-- Footer Actions -->
      {#if showForm}
        <div
          class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
        >
          <button
            type="button"
            onclick={cancelForm}
            disabled={submitting}
            class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
          >
            Cancelar
          </button>

          <button
            type="submit"
            form="vehiculoForm"
            disabled={submitting || !!duplicateError}
            class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm flex items-center gap-1.5"
          >
            {#if submitting}
              <span class="loading loading-spinner loading-xs"></span>
              Guardando...
            {:else}
              Guardar
            {/if}
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  /* Autofill Fix (Evita fondo blanco de Chrome) */
  input:-webkit-autofill {
    -webkit-text-fill-color: white !important;
    -webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
    transition: background-color 5000s ease-in-out 0s;
  }

  /* Focus Override Global */
  input:focus {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
    outline: none !important;
  }
</style>
