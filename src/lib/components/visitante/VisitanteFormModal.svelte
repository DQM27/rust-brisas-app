<!-- src/lib/components/visitante/VisitanteFormModal.svelte -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X } from "lucide-svelte";
  import { onMount } from "svelte";
  import type {
    VisitanteResponse,
    CreateVisitanteInput,
    UpdateVisitanteInput,
  } from "$lib/types/visitante";
  import { submitFetchActiveEmpresas } from "$lib/logic/empresa/empresaService";

  interface Props {
    show: boolean;
    visitante?: VisitanteResponse | null;
    loading?: boolean;
    onSave: (
      data: CreateVisitanteInput | UpdateVisitanteInput,
    ) => Promise<boolean | void>;
    onClose: () => void;
  }

  let {
    show,
    visitante = null,
    loading = false,
    onSave,
    onClose,
  }: Props = $props();

  const isEditMode = $derived(!!visitante);
  const modalTitle = $derived(
    isEditMode
      ? `Editar: ${visitante?.nombre} ${visitante?.apellido}`
      : "Nuevo Visitante",
  );

  let formData = $state({
    cedula: "",
    nombre: "",
    segundoNombre: "",
    apellido: "",
    segundoApellido: "",
    empresaId: "",
    hasVehicle: false,
  });

  let empresas = $state<{ id: string; nombre: string }[]>([]);
  let loadingEmpresas = $state(false);

  const isFormValid = $derived(
    formData.cedula.trim() &&
      formData.nombre.trim() &&
      formData.apellido.trim(),
  );

  onMount(async () => {
    await loadEmpresas();
  });

  async function loadEmpresas() {
    loadingEmpresas = true;
    const res = await submitFetchActiveEmpresas();
    if (res.ok) {
      empresas = res.empresas;
    }
    loadingEmpresas = false;
  }

  $effect(() => {
    if (show && visitante) {
      formData = {
        cedula: visitante.cedula,
        nombre: visitante.nombre,
        segundoNombre: visitante.segundoNombre || "",
        apellido: visitante.apellido,
        segundoApellido: visitante.segundoApellido || "",
        empresaId: visitante.empresaId || "",
        hasVehicle: visitante.hasVehicle,
      };
    } else if (show && !visitante) {
      resetForm();
    }
  });

  function resetForm() {
    formData = {
      cedula: "",
      nombre: "",
      segundoNombre: "",
      apellido: "",
      segundoApellido: "",
      empresaId: "",
      hasVehicle: false,
    };
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!isFormValid) return;

    const data = { ...formData };
    if (!data.empresaId) delete (data as any).empresaId;

    const success = await onSave(data as CreateVisitanteInput);
    if (success) {
      onClose();
    }
  }

  function handleClose() {
    if (!loading) onClose();
  }

  const labelClass = "text-xs font-medium text-gray-700 dark:text-gray-300";
  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent focus:outline-none transition-all placeholder-gray-400";
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 text-gray-100"
    transition:fade={{ duration: 200 }}
  >
    <button
      type="button"
      class="absolute inset-0 bg-black/60 w-full h-full cursor-default"
      onclick={handleClose}
      aria-label="Cerrar modal"
    ></button>

    <div
      class="relative z-10 w-full max-w-md overflow-hidden rounded-lg bg-[#0d1117] shadow-2xl border border-gray-700"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-gray-700 bg-[#161b22]"
      >
        <h2 class="text-lg font-semibold">{modalTitle}</h2>
        <button onclick={handleClose} class="text-gray-400 hover:text-white">
          <X class="w-5 h-5" />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="p-6 space-y-4">
        <div class="space-y-1">
          <label class={labelClass} for="cedula">Cédula *</label>
          <input
            id="cedula"
            bind:value={formData.cedula}
            class={inputClass}
            disabled={isEditMode}
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-1">
            <label class={labelClass} for="nombre">Nombre *</label>
            <input
              id="nombre"
              bind:value={formData.nombre}
              class={inputClass}
            />
          </div>
          <div class="space-y-1">
            <label class={labelClass} for="apellido">Apellido *</label>
            <input
              id="apellido"
              bind:value={formData.apellido}
              class={inputClass}
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-1">
            <label class={labelClass} for="segundoNombre">Segundo Nombre</label>
            <input
              id="segundoNombre"
              bind:value={formData.segundoNombre}
              class={inputClass}
            />
          </div>
          <div class="space-y-1">
            <label class={labelClass} for="segundoApellido"
              >Segundo Apellido</label
            >
            <input
              id="segundoApellido"
              bind:value={formData.segundoApellido}
              class={inputClass}
            />
          </div>
        </div>

        <div class="space-y-1">
          <label class={labelClass} for="empresaId">Empresa (Opcional)</label>
          <select
            id="empresaId"
            bind:value={formData.empresaId}
            class={inputClass}
          >
            <option value="">Ninguna / Independiente</option>
            {#each empresas as emp}
              <option value={emp.id}>{emp.nombre}</option>
            {/each}
          </select>
        </div>

        <div class="flex items-center gap-2 pt-2">
          <input
            type="checkbox"
            id="hasVehicle"
            bind:checked={formData.hasVehicle}
            class="checkbox checkbox-primary checkbox-sm"
          />
          <label for="hasVehicle" class="text-sm cursor-pointer"
            >¿Tiene vehículo?</label
          >
        </div>

        <div class="flex justify-end gap-3 pt-4">
          <button
            type="button"
            onclick={handleClose}
            class="px-4 py-2 rounded border border-gray-600 hover:bg-gray-800"
            >Cancelar</button
          >
          <button
            type="submit"
            disabled={!isFormValid || loading}
            class="px-4 py-2 rounded bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50"
          >
            {loading ? "Guardando..." : isEditMode ? "Actualizar" : "Crear"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
