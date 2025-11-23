<script lang="ts">
  import { Copy, FileDown, Eye, Trash2 } from "lucide-svelte";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";

  interface Props {
    row: ListaNegraResponse;
    x: number;
    y: number;
    onClose: () => void;
    onCopyRow: (row: ListaNegraResponse) => void;
    onViewDetails: (row: ListaNegraResponse) => void;
    onUnblock?: (row: ListaNegraResponse) => void;
  }

  let { row, x, y, onClose, onCopyRow, onViewDetails, onUnblock }: Props =
    $props();

  function handleAction(action: () => void) {
    action();
    onClose();
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="context-menu-overlay" onclick={onClose}>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="context-menu"
    style="left: {x}px; top: {y}px;"
    onclick={(e) => e.stopPropagation()}
  >
    <button
      class="menu-item"
      onclick={() => handleAction(() => onCopyRow(row))}
    >
      <Copy size={14} />
      <span>Copiar cédula</span>
    </button>

    <button
      class="menu-item"
      onclick={() => handleAction(() => onViewDetails(row))}
    >
      <Eye size={14} />
      <span>Ver detalles</span>
    </button>

    {#if row.isActive && onUnblock}
      <div class="menu-divider"></div>
      <button
        class="menu-item danger"
        onclick={() => handleAction(() => onUnblock?.(row))}
      >
        <Trash2 size={14} />
        <span>Desbloquear</span>
      </button>
    {/if}
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
