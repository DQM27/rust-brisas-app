<!-- src/lib/components/gafete/GafeteFormModal.svelte -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X } from "lucide-svelte";
  import type {
    GafeteResponse,
    CreateGafeteInput,
    UpdateGafeteInput,
  } from "$lib/types/gafete";
  import { CreateGafeteSchema } from "$lib/types/gafete";

  interface Props {
    show: boolean;
    gafete?: GafeteResponse | null;
    loading?: boolean;
    onSave: (
      data: CreateGafeteInput | UpdateGafeteInput,
    ) => Promise<void> | void;
    onClose: () => void;
  }

  let {
    show,
    gafete = null,
    loading = false,
    onSave,
    onClose,
  }: Props = $props();

  // Modo derivado
  const isEditMode = $derived(!!gafete);
  const modalTitle = $derived(isEditMode ? "Editar Gafete" : "Nuevo Gafete");

  // Estado del formulario
  let numero = $state("");
  let tipo = $state<"contratista" | "proveedor" | "visita" | "otro">(
    "contratista",
  );
  let errors = $state<Record<string, string>>({});

  // Cargar datos iniciales
  $effect(() => {
    if (show && gafete) {
      numero = String(gafete.numero);
      tipo = gafete.tipo;
      errors = {};
    } else if (show) {
      numero = "";
      tipo = "contratista";
      errors = {};
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();

    const data = { numero, tipo };

    // Validación por ahora simple o vía schema si se prefiere
    if (!numero.trim()) {
      errors.numero = "El número es requerido";
      return;
    }

    try {
      await onSave(data);
    } catch (err: any) {
      errors.form = err.message || "Error al guardar";
    }
  }

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-[#2da44e] disabled:opacity-60 transition-all";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade={{ duration: 150 }}
  >
    <!-- Backdrop/Overlay (No cierra al hacer click fuera como pidió el usuario para ingresos, mantenemos consistencia) -->
    <div class="absolute inset-0"></div>

    <!-- Modal Content -->
    <div
      class="relative z-10 w-full max-w-md bg-white dark:bg-[#0d1117] rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between bg-white dark:bg-[#0d1117]"
      >
        <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
          {modalTitle}
        </h2>
        <button
          onclick={onClose}
          class="p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          <X size={20} />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="p-6 space-y-4">
        <!-- Número de Gafete -->
        <div>
          <label for="numero" class={labelClass}> Número de Gafete </label>
          <input
            type="text"
            id="numero"
            bind:value={numero}
            disabled={isEditMode || loading}
            class={inputClass}
            placeholder="Ej: G-101"
            required
          />
          {#if isEditMode}
            <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
              El número no se puede cambiar una vez creado.
            </p>
          {/if}
          {#if errors.numero}
            <p class="mt-1 text-xs text-red-500">{errors.numero}</p>
          {/if}
        </div>

        <!-- Tipo de Gafete -->
        <div>
          <label for="tipo" class={labelClass}> Tipo </label>
          <select
            id="tipo"
            bind:value={tipo}
            disabled={loading}
            class={inputClass}
          >
            <option value="contratista">Contratista</option>
            <option value="proveedor">Proveedor</option>
            <option value="visita">Visita</option>
            <option value="otro">Otro</option>
          </select>
        </div>

        <!-- Botones de Acción -->
        <div
          class="flex justify-end space-x-3 mt-6 pt-4 border-t border-gray-200 dark:border-gray-700"
        >
          <button
            type="button"
            onclick={onClose}
            disabled={loading}
            class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 border border-transparent rounded-md transition-colors"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={loading || !numero.trim()}
            class="inline-flex justify-center px-4 py-2 text-sm font-medium text-white bg-[#2da44e] border border-transparent rounded-md shadow-sm hover:bg-[#2c974b] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#2da44e] disabled:opacity-50 disabled:cursor-not-allowed transition-all"
          >
            {#if loading}
              <svg
                class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
                xmlns="http://www.w3.org/2000/svg"
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
              Guardando...
            {:else}
              {isEditMode ? "Actualizar" : "Crear Gafete"}
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
