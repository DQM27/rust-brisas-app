<!-- src/lib/components/layout/sidebar/SidebarPanel.svelte -->
<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import { cubicOut, cubicIn } from 'svelte/easing';
  import type { SidebarItem } from '../../../types/Sidebar';

  export let item: SidebarItem;
  export let isOpen: boolean = false;
  export let onClose: () => void;
</script>

{#if isOpen}
  <div 
    class="flex w-[250px] flex-col border-r border-[#1f1f1f] bg-[#252526] overflow-hidden"
    in:fly={{ x: -250, duration: 300, easing: cubicOut }}
    out:fly={{ x: -250, duration: 250, easing: cubicIn }}
  >
    <div 
      class="flex items-center justify-between border-b border-[#2d2d2d] 
             bg-[#2d2d2d] px-[15px] py-3 text-[13px] font-semibold text-[#cccccc]"
    >
      <span>{item.label}</span>
      <button 
        class="flex h-5 w-5 items-center justify-center rounded border-none 
               bg-transparent p-0 text-base text-[#858585] cursor-pointer
               transition-all duration-150
               hover:bg-[#3c3c3c] hover:text-[#cccccc] hover:rotate-90
               focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500"
        on:click={onClose}
        title="Cerrar panel"
      >
        ×
      </button>
    </div>
    <div class="flex-grow overflow-y-auto py-2">
      <svelte:component this={item.panelComponent} />
    </div>
  </div>
{/if}