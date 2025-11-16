<!-- src/lib/components/inspection/InspectionPanel.svelte -->
<script lang="ts">
  import { ChevronDown } from 'lucide-svelte';
  import { inspection } from '$lib/stores/inspection';
  import { getInspector, hasInspector } from '../../config/InspectionPanel';
  import EmptyState from './EmptyState.svelte';

  // Computed
  $: inspectorComponent = $inspection.type ? getInspector($inspection.type) : null;
  $: hasData = $inspection.data !== null;

  function handleClose() {
    inspection.close();
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleClose();
    }
  }
</script>

<div class="inspection-panel">
  <!-- Header -->
  <div class="inspection-header">
    <h4>{$inspection.title}</h4>
    <button 
      class="close-btn" 
      on:click={handleClose}
      on:keydown={handleKeyPress}
      type="button"
      title="Cerrar panel de inspección"
    >
      <ChevronDown size={16} />
    </button>
  </div>

  <!-- Content -->
  <div class="inspection-content">
    {#if !hasData}
      <EmptyState />
    {:else if inspectorComponent}
      <svelte:component this={inspectorComponent} data={$inspection.data} />
    {:else}
      <div class="no-inspector">
        <p>No hay inspector disponible para este tipo de elemento.</p>
        <p class="hint">Tipo: {$inspection.type}</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .inspection-panel {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: #252526;
  }

  .inspection-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: #2d2d2d;
    border-bottom: 1px solid #3c3c3c;
  }

  .inspection-header h4 {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    color: #ccc;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .close-btn {
    background: none;
    border: none;
    color: #ccc;
    cursor: pointer;
    padding: 4px;
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    background: #3c3c3c;
  }

  .close-btn:focus {
    outline: 2px solid #007acc;
    outline-offset: 1px;
  }

  .inspection-content {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .no-inspector {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 2rem;
    text-align: center;
    color: #888;
  }

  .no-inspector p {
    margin: 0.5rem 0;
    font-size: 13px;
  }

  .hint {
    font-size: 11px;
    color: #666;
    font-family: monospace;
  }
</style>