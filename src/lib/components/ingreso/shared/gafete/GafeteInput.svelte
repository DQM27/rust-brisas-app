<script lang="ts">
  import { createEventDispatcher } from "svelte";

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

<div class="form-control max-w-[100px] flex flex-col items-center">
  <label class="label pb-1" for="gafete-input">
    <span class="label-text font-medium text-secondary">Gafete</span>
  </label>
  <div class="relative w-full text-center">
    <input
      id="gafete-input"
      type="text"
      bind:this={inputRef}
      bind:value
      {disabled}
      class="w-full h-10 px-3 font-mono text-xl tracking-widest text-center rounded-2xl border-2 transition-all bg-transparent focus:outline-none focus:ring-0 focus:shadow-none
        {error
        ? 'border-error bg-error/5 text-error'
        : 'border-blue-500/30 bg-blue-500/5 text-primary placeholder:text-gray-500/50 focus:border-blue-500'}"
      placeholder="00"
      on:keydown={handleKeydown}
      use:focus={autofocus}
    />
  </div>
  {#if error}
    <div class="label">
      <span class="label-text-alt text-error">{error}</span>
    </div>
  {/if}
</div>
