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

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="context-menu-overlay" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="context-menu"
    style="left: {x}px; top: {y}px;"
    onclick={(e) => e.stopPropagation()}
  >
    {#each visibleItems as item}
      <button
        class="menu-item"
        class:danger={item.variant === "danger"}
        onclick={() => onItemClick(item)}
      >
        {#if item.icon}
          {@const Icon = item.icon}
          <Icon size={14} />
        {/if}
        <span>{item.label}</span>
      </button>

      {#if item.dividerAfter}
        <div class="menu-divider"></div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .context-menu-overlay {
    position: fixed;
    inset: 0;
    z-index: 1000;
  }

  .context-menu {
    position: fixed;
    background-color: #252526;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.5);
    min-width: 180px;
    padding: 4px;
    z-index: 1001;
    animation: slideIn 0.15s ease-out;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: #d4d4d4;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
  }

  .menu-item:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }

  .menu-item.danger {
    color: #f87171;
  }

  .menu-item.danger:hover {
    background-color: rgba(248, 113, 113, 0.1);
  }

  .menu-divider {
    height: 1px;
    background-color: rgba(255, 255, 255, 0.1);
    margin: 4px 0;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: scale(0.95) translateY(-5px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }
</style>
