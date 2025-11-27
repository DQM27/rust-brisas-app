<script lang="ts">
  import * as Icons from 'lucide-svelte';

  export let buttons: {
    id: string;
    icon: keyof typeof Icons;
    label: string;
    action: () => void;
  }[] = [];

  const handleClick = (btn) => {
    btn.action?.();
  };
</script>

<div class="flex gap-2 p-2 bg-white border rounded-xl shadow-sm">
  {#each buttons as btn}
    <button
      class="
        flex items-center justify-center w-10 h-10
        rounded-lg border border-slate-300
        bg-white hover:bg-slate-100
        transition-all duration-150
        hover:scale-105 active:scale-95
      "
      on:click={() => handleClick(btn)}
      title={btn.label}
    >
      <!-- Icono dinámico -->
      {#if Icons[btn.icon]}
        <svelte:component this={Icons[btn.icon]} size="20" />
      {/if}
    </button>
  {/each}
</div>
