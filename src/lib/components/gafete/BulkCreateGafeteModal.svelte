<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fade } from "svelte/transition";
  import type { CreateGafeteRangeInput } from "$lib/types/gafete";

  const dispatch = createEventDispatcher<{
    submit: CreateGafeteRangeInput;
    cancel: void;
  }>();

  export let loading = false;

  let start = 1;
  let end = 50;
  let prefix = "";
  let padding = 2; // e.g. 01 vs 001
  let tipo: "contratista" | "proveedor" | "visita" | "otro" = "contratista";

  // Calcular vista previa
  const previewStart = $derived(
    `${prefix}${start.toString().padStart(padding, "0")}`,
  );
  const previewEnd = $derived(
    `${prefix}${end.toString().padStart(padding, "0")}`,
  );

  function handleSubmit() {
    if (start > end) {
      return alert("El inicio debe ser menor o igual al fin");
    }
    dispatch("submit", {
      start,
      end,
      prefix: prefix.trim(),
      padding,
      tipo,
    });
  }
</script>

<div
  class="bg-white dark:bg-[#0d1117] rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 p-6 w-full max-w-md mx-auto"
>
  <h2
    class="text-xl font-bold mb-4 text-gray-900 dark:text-gray-100 border-b border-gray-200 dark:border-gray-700 pb-2"
  >
    Generar Lote de Gafetes
  </h2>

  <form
    onsubmit={(e) => {
      e.preventDefault();
      handleSubmit();
    }}
    class="space-y-4"
  >
    <!-- Rango -->
    <div class="grid grid-cols-2 gap-4">
      <div>
        <label
          for="start"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >
          Inicio
        </label>
        <input
          type="number"
          id="start"
          bind:value={start}
          min="1"
          class="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#161b22] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e]"
        />
      </div>
      <div>
        <label
          for="end"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >
          Fin
        </label>
        <input
          type="number"
          id="end"
          bind:value={end}
          min="1"
          class="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#161b22] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e]"
        />
      </div>
    </div>

    <!-- Formato -->
    <div class="grid grid-cols-2 gap-4">
      <div>
        <label
          for="prefix"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >
          Prefijo (Opcional)
        </label>
        <input
          type="text"
          id="prefix"
          bind:value={prefix}
          placeholder="Ej. C-"
          class="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#161b22] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e]"
        />
      </div>
      <div>
        <label
          for="padding"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >
          Min. Dígitos
        </label>
        <input
          type="number"
          id="padding"
          bind:value={padding}
          min="1"
          max="10"
          class="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#161b22] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e]"
        />
      </div>
    </div>

    <!-- Tipo -->
    <div>
      <label
        for="tipo"
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
      >
        Tipo de Gafete
      </label>
      <select
        id="tipo"
        bind:value={tipo}
        class="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#161b22] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e]"
      >
        <option value="contratista">Contratista</option>
        <option value="proveedor">Proveedor</option>
        <option value="visita">Visita</option>
        <option value="otro">Otro</option>
      </select>
    </div>

    <!-- Preview -->
    <div
      class="p-3 bg-gray-50 dark:bg-gray-800/50 rounded-md border border-gray-200 dark:border-gray-700 text-sm"
    >
      <p class="text-gray-500 dark:text-gray-400">
        Se generarán {end - start + 1} gafetes:
      </p>
      <p class="font-mono text-gray-900 dark:text-gray-200 mt-1">
        {previewStart} ... {previewEnd}
      </p>
    </div>

    <!-- Botones -->
    <div
      class="flex justify-end space-x-3 pt-4 border-t border-gray-200 dark:border-gray-700"
    >
      <button
        type="button"
        onclick={() => dispatch("cancel")}
        disabled={loading}
        class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 border border-transparent rounded-md transition-colors"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={loading}
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
          Generando...
        {:else}
          Generar Gafetes
        {/if}
      </button>
    </div>
  </form>
</div>
