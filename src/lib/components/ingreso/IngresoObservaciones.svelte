<script lang="ts">
  import { MessageSquare, ChevronDown, ChevronUp } from "lucide-svelte";
  import { slide } from "svelte/transition";
  import { createEventDispatcher } from "svelte";

  export let observaciones: string = "";

  const dispatch = createEventDispatcher();
  let showObservaciones = false;

  function handleInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    dispatch("change", target.value);
  }
</script>

<div class="mt-4 border-t border-gray-200 dark:border-gray-700 pt-4 w-full">
  <button
    type="button"
    class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 transition-colors focus:outline-none w-full"
    on:click={() => (showObservaciones = !showObservaciones)}
  >
    <MessageSquare size={16} />
    <span class="flex-1 text-left"
      >{showObservaciones ? "Ocultar" : "Agregar"} observaciones</span
    >
    {#if showObservaciones}
      <ChevronUp size={16} />
    {:else}
      <ChevronDown size={16} />
    {/if}
  </button>

  {#if showObservaciones}
    <div transition:slide={{ duration: 200 }} class="mt-3 w-full">
      <label
        for="observaciones-detalle"
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
      >
        Detalle de observaciones
      </label>
      <textarea
        id="observaciones-detalle"
        value={observaciones}
        on:input={handleInput}
        rows="3"
        class="w-full px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-[#252526] text-gray-900 dark:text-white"
        placeholder="Escriba aquí cualquier observación pertinente..."
      ></textarea>
    </div>
  {/if}
</div>
