<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { isAuthenticated } from '$lib/stores/auth';
  import Sidebar from '$lib/components/layout/sidebar/Sidebar.svelte';
  import StatusBar from '$lib/components/layout/StatusBar.svelte';
  import { inspectionPanel } from '$lib/stores/ui';
  import { initNetworkMonitor } from '$lib/stores/network';

  // Estado de autenticación reactivo
  $: authenticated = $isAuthenticated;

  // Toggle del panel de inspección
  function toggleInspectionPanel(): void {
    $inspectionPanel.visible = !$inspectionPanel.visible;
  }

  // Inicializar monitor de red
  onMount(() => {
    const cleanup = initNetworkMonitor();
    return cleanup;
  });
</script>

<div class="layout">
  <!-- Main Area -->
  <div class="main">
    {#if authenticated}
      <Sidebar />
    {/if}
    <div class="content-area">
      <slot />
    </div>
  </div>

  <!-- StatusBar -->
  <StatusBar 
    inspectionPanelVisible={$inspectionPanel.visible}
    on:inspectionToggle={toggleInspectionPanel}
  />
</div>

<style>
  * { 
    margin: 0; 
    padding: 0; 
    box-sizing: border-box; 
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
  }
  
  :global(html), :global(body) { 
    height: 100%; 
    overflow: hidden; 
    background: #252526; 
    color: #ccc; 
  }
  
  .layout { 
    display: flex; 
    flex-direction: column; 
    height: 100vh; 
    background: #1e1e1e;
  }

  /* MAIN LAYOUT */
  .main {
    display: flex;
    flex-direction: row;
    flex: 1;
    width: 100%;
    overflow: hidden;
    background: #1e1e1e;
  }

  .content-area {
    flex: 1;
    background: #1e1e1e;
    overflow: auto;
    position: relative;
    display: flex;
  }

  /* Asegurar que el slot ocupe todo el espacio */
  .content-area > :global(*) {
    flex: 1;
    width: 100%;
  }

  /* Scrollbar personalizada */
  :global(::-webkit-scrollbar) {
    width: 10px;
  }

  :global(::-webkit-scrollbar-track) {
    background: #1e1e1e;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: #424242;
    border-radius: 5px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: #4f4f4f;
  }

  /* Responsive para móviles */
  @media (max-width: 768px) {
    .main {
      flex-direction: column;
    }
    
    .content-area {
      min-height: calc(100vh - 24px);
    }
  }
</style>