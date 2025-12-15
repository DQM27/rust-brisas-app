<script lang="ts">
  import { X } from "lucide-svelte";

  export let title: string;
  export let loading: boolean = false;
  export let disabled: boolean = false;
  export let submitLabel: string = "Registrar Entrada";
  export let submitIcon: any = null;

  export let onSubmit: () => void;
  export let onClose: () => void;
</script>

<div
  class="bg-[#161b22] rounded-lg border border-[#30363d] p-5 relative h-full flex flex-col"
>
  <!-- Header -->
  <div class="flex justify-between items-center mb-5 shrink-0">
    <h2 class="text-base font-semibold text-[#f0f6fc]">
      {title}
    </h2>
    <button
      on:click={onClose}
      class="text-[#8d96a0] hover:text-[#f0f6fc] p-1 rounded hover:bg-[#30363d] transition-colors"
      type="button"
      aria-label="Cerrar formulario"
    >
      <X size={18} />
    </button>
  </div>

  <!-- Body (Scrollable) -->
  <form
    on:submit|preventDefault={onSubmit}
    class="flex-1 flex flex-col min-h-0"
  >
    <div class="flex-1 overflow-y-auto pr-1 space-y-4">
      <slot />
    </div>

    <!-- Footer buttons -->
    <div class="pt-4 shrink-0 flex gap-3 mt-auto border-t border-[#21262d]">
      <button
        type="button"
        on:click={onClose}
        class="flex-1 py-2 px-4 border border-[#30363d] rounded-md text-sm font-medium text-[#c9d1d9] bg-[#21262d] hover:bg-[#30363d] hover:border-[#484f58] focus:outline-none focus:ring-2 focus:ring-[#388bfd] transition-colors"
      >
        Cancelar
      </button>

      <button
        type="submit"
        disabled={loading || disabled}
        class="flex-1 sm:flex-[2] flex justify-center items-center py-2 px-4 border border-transparent rounded-md text-sm font-medium text-white bg-[#238636] hover:bg-[#2ea043] focus:outline-none focus:ring-2 focus:ring-[#238636] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
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
            <svelte:component this={submitIcon} size={16} class="mr-2" />
          {/if}
          {submitLabel}
        {/if}
      </button>
    </div>
  </form>
</div>
