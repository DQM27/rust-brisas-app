<script lang="ts">
  import { X } from "lucide-svelte";

  export let title: string;
  export let loading: boolean = false;
  export let disabled: boolean = false;
  export let submitLabel: string = "Registrar Entrada";
  export let submitIcon: any = null; // Componente de icono opcional

  export let onSubmit: () => void;
  export let onClose: () => void;
</script>

<div
  class="bg-white dark:bg-gray-800 rounded-lg shadow p-6 relative h-full flex flex-col"
>
  <!-- Header -->
  <div class="flex justify-between items-center mb-6 shrink-0">
    <h2 class="text-xl font-bold text-gray-900 dark:text-white">
      {title}
    </h2>
    <button
      on:click={onClose}
      class="text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-400 p-1 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
      type="button"
      aria-label="Cerrar formulario"
    >
      <X size={20} />
    </button>
  </div>

  <!-- Body (Scrollable) -->
  <form
    on:submit|preventDefault={onSubmit}
    class="flex-1 flex flex-col min-h-0"
  >
    <div class="flex-1 overflow-y-auto pr-2 space-y-6">
      <slot />
    </div>

    <!-- Footer buttons -->
    <div
      class="pt-6 shrink-0 flex gap-3 mt-auto border-t border-gray-100 dark:border-gray-700"
    >
      <button
        type="button"
        on:click={onClose}
        class="flex-1 py-3 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 transition-colors"
      >
        Cancelar
      </button>

      <button
        type="submit"
        disabled={loading || disabled}
        class="flex-1 sm:flex-[2] flex justify-center items-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {#if loading}
          <svg
            class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
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
          Registrando...
        {:else}
          {#if submitIcon}
            <svelte:component this={submitIcon} size={18} class="mr-2" />
          {/if}
          {submitLabel}
        {/if}
      </button>
    </div>
  </form>
</div>
