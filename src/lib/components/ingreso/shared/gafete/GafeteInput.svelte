<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fade } from "svelte/transition";

  export let value: string = "";
  export let error: string | undefined = undefined;
  export let disabled: boolean = false;
  export let autofocus: boolean = false;

  let inputRef: HTMLInputElement;

  const dispatch = createEventDispatcher();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      dispatch("submit");
    }
  }

  function focus(node: HTMLInputElement, enabled: boolean) {
    if (enabled) node.focus();
    return {
      update(newEnabled: boolean) {
        if (newEnabled) node.focus();
      },
    };
  }

  // Simple validation or formatting if needed
  $: value = value.toUpperCase().trim();
</script>

<div class="form-control max-w-[80px] flex flex-col items-start">
  <label class="label pb-1 transition-all px-0" for="gafete-input">
    <span class="text-[12px] font-bold uppercase tracking-wider text-gray-500"
      >Gafete</span
    >
  </label>
  <div
    class="search-container relative w-full flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all outline-none"
  >
    <input
      id="gafete-input"
      type="text"
      bind:this={inputRef}
      bind:value
      {disabled}
      class="w-full h-10 px-3 font-mono text-xl tracking-widest text-center bg-transparent text-white focus:outline-none outline-none border-none placeholder:text-gray-600 appearance-none ring-0"
      placeholder="00"
      autocomplete="off"
      autocorrect="off"
      autocapitalize="off"
      spellcheck="false"
      on:keydown={handleKeydown}
      use:focus={autofocus}
    />
  </div>
  {#if error}
    <div class="label mt-1" transition:fade>
      <span class="text-[10px] font-medium text-error flex items-center gap-1">
        {error}
      </span>
    </div>
  {/if}
</div>

<style>
  /* Asegurar que nada tenga outline cuadrado del navegador */
  .search-container,
  .search-container *:focus {
    outline: none !important;
    box-shadow: none !important;
  }

  /* Re-aplicar el ring redondeado solo vía focus-within */
  .search-container:focus-within {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  }
</style>
