<!-- src/lib/components/gafete/modals/BulkCreateGafeteModal.svelte -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, Plus } from "lucide-svelte";
  import type { CreateGafeteRangeInput } from "$lib/types/gafete";

  interface Props {
    show: boolean;
    loading?: boolean;
    onSave: (data: CreateGafeteRangeInput) => Promise<void> | void;
    onClose: () => void;
  }

  let { show, loading = false, onSave, onClose }: Props = $props();

  let start = $state(1);
  let end = $state(50);
  let prefix = $state("");
  let padding = $state(2); // e.g. 01 vs 001
  let tipo = $state<"contratista" | "proveedor" | "visita" | "otro">(
    "contratista",
  );

  // Calcular vista previa
  const previewStart = $derived(
    `${prefix}${start.toString().padStart(padding, "0")}`,
  );
  const previewEnd = $derived(
    `${prefix}${end.toString().padStart(padding, "0")}`,
  );
  const totalCount = $derived(Math.max(0, end - start + 1));

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (start > end) {
      return alert("El inicio debe ser menor o igual al fin");
    }

    await onSave({
      start,
      end,
      prefix: prefix.trim(),
      padding,
      tipo,
    });
  }

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#161b22] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] outline-none transition-all";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade={{ duration: 150 }}
  >
    <div class="absolute inset-0"></div>

    <div
      class="relative z-10 w-full max-w-md bg-white dark:bg-[#0d1117] rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between bg-white dark:bg-[#0d1117]"
      >
        <h2
          class="text-xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2"
        >
          <Plus size={20} class="text-blue-500" />
          Generar Gafetes en Lote
        </h2>
        <button
          onclick={onClose}
          class="p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          <X size={20} />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="p-6 space-y-4">
        <!-- Rango -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="start" class={labelClass}>Inicio</label>
            <input
              type="number"
              id="start"
              bind:value={start}
              min="1"
              disabled={loading}
              class={inputClass}
            />
          </div>
          <div>
            <label for="end" class={labelClass}>Fin</label>
            <input
              type="number"
              id="end"
              bind:value={end}
              min="1"
              disabled={loading}
              class={inputClass}
            />
          </div>
        </div>

        <!-- Formato -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="prefix" class={labelClass}>Prefijo (Opcional)</label>
            <input
              type="text"
              id="prefix"
              bind:value={prefix}
              placeholder="Ej. C-"
              disabled={loading}
              class={inputClass}
            />
          </div>
          <div>
            <label for="padding" class={labelClass}>Mín. Dígitos</label>
            <input
              type="number"
              id="padding"
              bind:value={padding}
              min="1"
              max="10"
              disabled={loading}
              class={inputClass}
            />
          </div>
        </div>

        <!-- Tipo -->
        <div>
          <label for="tipo" class={labelClass}>Tipo de Gafete</label>
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

        <!-- Preview -->
        <div
          class="p-4 bg-blue-50/50 dark:bg-blue-900/10 rounded-lg border border-blue-100 dark:border-blue-900/30 text-sm"
        >
          <p class="text-blue-700 dark:text-blue-400 font-medium mb-1">
            Resumen de generación:
          </p>
          <div class="flex justify-between items-end">
            <div>
              <p class="text-gray-500 dark:text-gray-500 text-xs">Rango:</p>
              <p class="font-mono text-gray-900 dark:text-gray-200">
                {previewStart} → {previewEnd}
              </p>
            </div>
            <div class="text-right">
              <p class="text-gray-500 dark:text-gray-500 text-xs">Total:</p>
              <p class="font-bold text-blue-600 dark:text-blue-400">
                {totalCount} gafetes
              </p>
            </div>
          </div>
        </div>

        <!-- Botones -->
        <div
          class="flex justify-end space-x-3 pt-4 border-t border-gray-200 dark:border-gray-700"
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
            disabled={loading || totalCount <= 0}
            class="inline-flex justify-center px-6 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
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
              Generando...
            {:else}
              Generar Gafetes
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
