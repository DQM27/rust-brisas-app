<!-- src/lib/components/layout/StatusBar.svelte -->
<script lang="ts">
  import { Wifi, WifiOff } from 'lucide-svelte';
  import InspectionToggle from './InspectionToggle.svelte';
  import { createEventDispatcher } from 'svelte';
  import { online } from '$lib/stores/network';

  // Props
  export let inspectionPanelVisible: boolean = false;

  // Dispatcher
  const dispatch = createEventDispatcher<{
    inspectionToggle: { visible: boolean };
  }>();

  function handleInspectionToggle(event: CustomEvent<{ visible: boolean }>): void {
    dispatch('inspectionToggle', event.detail);
  }
</script>

<div class="status-bar">
  <!-- Sección izquierda - Estado de conexión -->
  <div class="status-left">
    <div class="status-item {$online ? 'online' : 'offline'}" role="status">
      {#if $online}
        <Wifi size={14} />
        <span>En línea</span>
      {:else}
        <WifiOff size={14} />
        <span>Sin conexión</span>
      {/if}
    </div>
  </div>
  
  <!-- Sección derecha - Inspección -->
  <div class="status-right">
    <InspectionToggle 
      visible={inspectionPanelVisible}
      variant="compact"
      on:toggle={handleInspectionToggle}
    />
  </div>
</div>

<style>
  .status-bar {
    background: var(--status-bar-background, #2d2d2d);
    color: var(--status-bar-foreground, #cccccc);
    height: 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 12px;
    font-size: 12px;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    user-select: none;
  }
  
  .status-left,
  .status-right {
    display: flex;
    align-items: center;
    gap: 16px;
  }
  
  .status-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 2px 8px;
    border-radius: 3px;
    white-space: nowrap;
  }
  
  /* Estados específicos */
  .status-item.online {
    color: #4caf50;
  }
  
  .status-item.offline {
    color: #f44336;
  }
  
  /* Responsive */
  @media (max-width: 768px) {
    .status-bar {
      padding: 4px 12px;
    }
  }
</style>