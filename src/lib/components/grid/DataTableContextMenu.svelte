<script lang="ts">
  import type { DataTableContextMenuItem } from "$lib/types/dataTable";

  interface Props {
    contextMenuItems: DataTableContextMenuItem<any>[];
    row: any;
    x: number;
    y: number;
    onClose: () => void;
    onItemClick: (item: DataTableContextMenuItem<any>) => void;
  }

  let { contextMenuItems, row, x, y, onClose, onItemClick }: Props = $props();

  let visibleItems = $derived(
    contextMenuItems.filter((item) => !item.show || item.show(row)),
  );
</script>

<div
  class="fixed inset-0 z-50"
  role="button"
  tabindex="0"
  onclick={onClose}
  onkeydown={(e) => e.key === "Escape" && onClose()}
>
  <div
    class="fixed bg-[#252526] border border-white/10 rounded-md shadow-xl min-w-[180px] p-1 z-51 animate-in fade-in-0 zoom-in-95"
    style="left: {x}px; top: {y}px;"
    role="menu"
    tabindex="0"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <div class="space-y-0.5">
      {#each visibleItems as item, index}
        <button
          class="w-full flex items-center gap-2 px-3 py-2 bg-transparent border-none rounded-sm text-gray-300 text-sm cursor-pointer transition-colors hover:bg-white/10 hover:text-white {item.variant ===
          'danger'
            ? 'text-red-400 hover:bg-red-500/10 hover:text-red-300'
            : ''}"
          onclick={() => onItemClick(item)}
        >
          {#if item.icon}
            {@const Icon = item.icon}
            <Icon size={14} class="shrink-0" />
          {/if}
          <span class="text-left whitespace-nowrap">{item.label}</span>
        </button>

        {#if item.dividerAfter && index < visibleItems.length - 1}
          <div class="h-px bg-white/10 my-1"></div>
        {/if}
      {/each}
    </div>
  </div>
</div>
