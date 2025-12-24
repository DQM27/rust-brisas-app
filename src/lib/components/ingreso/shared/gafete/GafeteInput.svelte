<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let value: string = "";
  export let error: string | undefined = undefined;
  export let disabled: boolean = false;
  export let autofocus: boolean = false;

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

<div class="form-control w-full">
  <label class="label" for="gafete-input">
    <span class="label-text font-medium">Número de Gafete</span>
  </label>
  <div class="relative">
    <input
      id="gafete-input"
      type="text"
      bind:value
      {disabled}
      class="input input-bordered w-full font-mono text-lg tracking-wider {error
        ? 'input-error'
        : ''}"
      placeholder="Escanear o escribir..."
      on:keydown={handleKeydown}
      use:focus={autofocus}
    />
    {#if value}
      <div class="absolute right-3 top-3">
        <span class="badge badge-neutral">Gafete</span>
      </div>
    {/if}
  </div>
  {#if error}
    <div class="label">
      <span class="label-text-alt text-error">{error}</span>
    </div>
  {/if}
</div>
