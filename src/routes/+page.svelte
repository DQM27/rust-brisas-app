<!-- src/routes/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import Tabs from '$lib/components/layout/Tabs.svelte';
  import { tabsStore } from '$lib/stores/tabs';
  import { isAuthenticated, checkSession } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { inspectionPanel } from '$lib/stores/ui';
  import { ChevronDown } from 'lucide-svelte';

  let inspectionContent = "Sos una loca";
  
  onMount(() => {
    checkSession();
    if (!$isAuthenticated) {
      goto('/login');
    }
  });

  // Función para manejar teclado (solo para el botón de cerrar)
  function handleKeyPress(event: KeyboardEvent, handler: () => void): void {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handler();
    }
  }

  // Función para cerrar el panel desde dentro
  function closeInspectionPanel(): void {
    $inspectionPanel.visible = false;
  }
</script>

{#if $isAuthenticated}
  <div class="main-container">
    <Splitpanes horizontal class="default-theme">
      <!-- Contenido principal -->
      <Pane minSize={30} size={$inspectionPanel.visible ? 70 : 100}>
        <div class="content-area">
          <Tabs tabs={$tabsStore} />
        </div>
      </Pane>

      <!-- Panel de inspección -->
      {#if $inspectionPanel.visible}
        <Pane minSize={20} size={30}>
          <div class="inspection-panel">
            <div class="inspection-header">
              <h4>Inspección</h4>
              <button 
                class="close-btn" 
                on:click={closeInspectionPanel}
                on:keydown={(e) => handleKeyPress(e, closeInspectionPanel)}
                type="button"
                title="Cerrar panel de inspección"
              >
                <ChevronDown size={16} />
              </button>
            </div>
            <div class="inspection-content">
              {inspectionContent}
              
              <div class="inspection-items">
                <div class="inspection-item">
                  <span class="label">Estado:</span>
                  <span class="value success">Conectado</span>
                </div>
                <div class="inspection-item">
                  <span class="label">Última actualización:</span>
                  <span class="value">{new Date().toLocaleTimeString()}</span>
                </div>
                <div class="inspection-item">
                  <span class="label">Registros hoy:</span>
                  <span class="value">1,247</span>
                </div>
              </div>
            </div>
          </div>
        </Pane>
      {/if}
    </Splitpanes>
  </div>
{:else}
  <div style="display: none;"></div>
{/if}

<style>
  .main-container {
    height: 100%;
    background: #1e1e1e;
  }

  .content-area {
    height: 100%;
    background: #1e1e1e;
  }

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
    padding: 12px;
    color: #ccc;
    font-size: 13px;
    overflow-y: auto;
  }

  .inspection-items {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 12px;
  }

  .inspection-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 0;
    border-bottom: 1px solid #3c3c3c;
  }

  .inspection-item .label {
    color: #888;
    font-size: 12px;
  }

  .inspection-item .value {
    font-size: 12px;
    font-weight: 600;
  }

  .inspection-item .value.success {
    color: #4caf50;
  }

  /* Ajustes para el splitpanes */
  :global(.splitpanes__pane) {
    background: transparent;
  }

  :global(.splitpanes__splitter) {
    background: #2d2d2d;
    border: none;
  }

  :global(.splitpanes__splitter:hover) {
    background: #3c3c3c;
  }

  :global(.splitpanes--horizontal .splitpanes__splitter) {
    min-height: 6px;
    border-top: 1px solid #3c3c3c;
    border-bottom: 1px solid #3c3c3c;
  }
</style>