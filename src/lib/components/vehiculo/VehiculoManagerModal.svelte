<!-- src/lib/components/vehiculo/VehiculoManagerModal.svelte -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, Plus, Trash2, Edit2, Car, Bike } from "lucide-svelte";
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
    contratistaId: string;
    contratistaNombre: string;
    onClose: () => void;
  }

  let { show, contratistaId, contratistaNombre, onClose }: Props = $props();

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
  const { form, errors, constraints, enhance, reset, validate } =
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
    if (!contratistaId) return;
    loading = true;
    globalError = null;
    try {
      vehiculosList = await vehiculosApi.getByPropietario(contratistaId);
    } catch (e) {
      console.error("Error loading vehicles:", e);
      globalError = "Error al cargar los vehículos.";
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    if (show && contratistaId) {
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
        propietarioId: contratistaId,
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

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent focus:outline-none transition-all placeholder-gray-400 dark:placeholder-gray-500 disabled:opacity-50";
  const errorInputClass = "border-red-500 focus:ring-red-500";
  const labelClass = "text-xs font-medium text-gray-700 dark:text-gray-300";
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
      class="relative z-10 w-full max-w-2xl max-h-[90vh] flex flex-col rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22] rounded-t-lg"
      >
        <div>
          <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
            Vehículos
          </h2>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
            Gestión de vehículos para {contratistaNombre}
          </p>
        </div>
        <button
          onclick={onClose}
          class="p-1.5 rounded-md text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          <X class="w-5 h-5" />
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
              <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">
                Vehículos Registrados ({vehiculosList.length})
              </h3>
              <button
                onclick={handleAddNew}
                class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-md bg-[#2da44e] text-white hover:bg-[#2c974b] transition-colors"
              >
                <Plus class="w-3.5 h-3.5" /> Nuevo Vehículo
              </button>
            </div>

            {#if loading}
              <div class="text-center py-8 text-gray-500 text-sm">
                Cargando vehículos...
              </div>
            {:else if vehiculosList.length === 0}
              <div
                class="text-center py-8 text-gray-500 dark:text-gray-400 text-sm border-2 border-dashed border-gray-200 dark:border-gray-700 rounded-lg"
              >
                No hay vehículos registrados.
              </div>
            {:else}
              <div class="grid gap-3">
                {#each vehiculosList as v}
                  <div
                    class="flex items-center justify-between p-3 rounded-md border border-gray-200 dark:border-gray-700 bg-gray-50/50 dark:bg-[#161b22]/50 hover:bg-gray-100 dark:hover:bg-[#1c2128] transition-colors"
                  >
                    <div class="flex items-center gap-3">
                      <div
                        class="p-2 rounded-full bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400"
                      >
                        {#if v.tipoVehiculo === "motocicleta"}
                          <Bike class="w-5 h-5" />
                        {:else}
                          <Car class="w-5 h-5" />
                        {/if}
                      </div>
                      <div>
                        <div
                          class="font-semibold text-gray-900 dark:text-gray-100 text-sm"
                        >
                          {v.placa}
                        </div>
                        <div class="text-xs text-gray-500 dark:text-gray-400">
                          {v.marca || "Sin marca"} • {v.modelo || "Sin modelo"} •
                          {v.color || "Sin color"}
                        </div>
                      </div>
                    </div>
                    <div class="flex items-center gap-2">
                      <button
                        onclick={() => handleEdit(v)}
                        class="p-1.5 text-gray-500 hover:text-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded transition-colors"
                        title="Editar"
                      >
                        <Edit2 class="w-4 h-4" />
                      </button>
                      <button
                        onclick={() => handleDelete(v.id)}
                        class="p-1.5 text-gray-500 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded transition-colors"
                        title="Eliminar"
                      >
                        <Trash2 class="w-4 h-4" />
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
            class="bg-gray-50 dark:bg-[#161b22] p-4 rounded-lg border border-gray-200 dark:border-gray-700 space-y-4 shadow-inner"
            transition:fly={{ y: 10, duration: 200 }}
          >
            <h3
              class="text-sm font-semibold text-gray-900 dark:text-gray-100 border-b border-gray-200 dark:border-gray-700 pb-2"
            >
              {editingId ? "Editar Vehículo" : "Nuevo Vehículo"}
            </h3>

            <form method="POST" use:enhance class="space-y-4">
              <!-- Tipo Vehículo -->
              <div class="space-y-1">
                <label for="tipoVehiculo" class={labelClass}
                  >Tipo de Vehículo <span class="text-red-500">*</span></label
                >
                <select
                  id="tipoVehiculo"
                  name="tipoVehiculo"
                  bind:value={$form.tipoVehiculo}
                  disabled={submitting}
                  class="{inputClass} {$errors.tipoVehiculo
                    ? errorInputClass
                    : ''}"
                  {...$constraints.tipoVehiculo}
                >
                  <option value="" disabled>Seleccione un tipo</option>
                  <option value="motocicleta">Motocicleta</option>
                  <option value="automovil">Automóvil</option>
                  <option value="camioneta">Camioneta</option>
                  <option value="camion">Camión</option>
                  <option value="otro">Otro</option>
                </select>
                {#if $errors.tipoVehiculo}
                  <p class="text-xs text-red-500 mt-1">
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
                    class="{inputClass} uppercase {$errors.placa ||
                    duplicateError
                      ? errorInputClass
                      : ''}"
                    disabled={submitting}
                    {...$constraints.placa}
                  />
                  {#if $errors.placa}
                    <p class="text-xs text-red-500 mt-1">{$errors.placa}</p>
                  {:else if duplicateError}
                    <p class="text-xs text-red-500 mt-1">{duplicateError}</p>
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
                    class="{inputClass} {$errors.marca ? errorInputClass : ''}"
                    disabled={submitting}
                    {...$constraints.marca}
                  />
                  {#if $errors.marca}
                    <p class="text-xs text-red-500 mt-1">{$errors.marca}</p>
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
                    class="{inputClass} {$errors.modelo ? errorInputClass : ''}"
                    disabled={submitting}
                    {...$constraints.modelo}
                  />
                  {#if $errors.modelo}
                    <p class="text-xs text-red-500 mt-1">{$errors.modelo}</p>
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
                    class="{inputClass} {$errors.color ? errorInputClass : ''}"
                    disabled={submitting}
                    {...$constraints.color}
                  />
                  {#if $errors.color}
                    <p class="text-xs text-red-500 mt-1">{$errors.color}</p>
                  {/if}
                </div>
              </div>

              <div class="flex justify-end gap-2 pt-2">
                <button
                  type="button"
                  onclick={cancelForm}
                  disabled={submitting}
                  class="px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#21262d]"
                  >Cancelar</button
                >
                <button
                  type="submit"
                  disabled={submitting || !!duplicateError}
                  class="px-3 py-1.5 text-sm font-medium rounded-md bg-[#2da44e] text-white hover:bg-[#2c974b] disabled:opacity-50 flex items-center gap-1.5"
                >
                  {#if submitting}
                    <span class="loading loading-spinner loading-xs"></span>
                    Guardando...
                  {:else}
                    Guardar
                  {/if}
                </button>
              </div>
            </form>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
