<!-- src/lib/components/layout/sidebar/SidebarPanel.svelte -->
<script lang="ts">
  import type { SidebarItem } from './types';

  // Asegurarnos de que las props estén correctamente exportadas
  export let item: SidebarItem;
  export let isOpen: boolean = false;
  export let onClose: () => void;
</script>

{#if isOpen}
  <div class="side-panel">
    <div class="panel-header">
      <span>{item.label}</span>
      <button 
        class="close-btn" 
        on:click={onClose}
        title="Cerrar panel"
      >
        ×
      </button>
    </div>
    <div class="panel-content">
      <svelte:component this={item.panelComponent} />
    </div>
  </div>
{/if}

<style>
  .side-panel {
    width: 250px;
    background: #252526;
    border-right: 1px solid #1f1f1f;
    display: flex;
    flex-direction: column;
    animation: slideIn 0.2s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-10px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .panel-header {
    padding: 12px 15px;
    border-bottom: 1px solid #2d2d2d;
    font-weight: 600;
    font-size: 13px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #cccccc;
    background: #2d2d2d;
  }

  .close-btn {
    background: none;
    border: none;
    color: #858585;
    cursor: pointer;
    font-size: 16px;
    padding: 0;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
  }

  .close-btn:hover {
    background: #3c3c3c;
    color: #cccccc;
  }

  .panel-content {
    padding: 8px 0;
    overflow-y: auto;
    flex-grow: 1;
  }
</style>