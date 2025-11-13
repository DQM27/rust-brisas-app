<!-- src/lib/components/layout/sidebar/panels/SettingsPanel.svelte -->
<script lang="ts">
  import { Settings, Bell, Download, RefreshCw, Info } from 'lucide-svelte';
  import { openView } from '../sidebarStore';
  
  function handleKeydown(e: KeyboardEvent, action: () => void) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      action();
    }
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">CONFIGURACIÓN GENERAL</div>
  <button 
    class="panel-item" 
    on:click={() => openView('dashboard', 'Configuración')}
    on:keydown={(e) => handleKeydown(e, () => openView('dashboard', 'Configuración'))}
  >
    <svelte:component this={Settings} size={16} />
    <span>Ajustes generales</span>
  </button>
  <div class="panel-item non-clickable">
    <svelte:component this={Bell} size={16} />
    <span>Notificaciones</span>
  </div>
  <div class="panel-item non-clickable">
    <svelte:component this={Download} size={16} />
    <span>Backup y restore</span>
  </div>
</div>
<div class="panel-section">
  <div class="panel-section-title">SISTEMA</div>
  <div class="panel-item non-clickable">
    <svelte:component this={RefreshCw} size={16} />
    <span>Actualizaciones</span>
  </div>
  <button 
    class="panel-item" 
    on:click={() => openView('welcome', 'Acerca del Sistema')}
    on:keydown={(e) => handleKeydown(e, () => openView('welcome', 'Acerca del Sistema'))}
  >
    <svelte:component this={Info} size={16} />
    <span>Acerca del sistema</span>
  </button>
</div>

<style>
  .panel-section {
    margin-bottom: 16px;
  }

  .panel-section-title {
    font-size: 11px;
    text-transform: uppercase;
    padding: 8px 15px 4px;
    color: #858585;
    font-weight: 600;
    letter-spacing: 0.5px;
  }

  .panel-item {
    padding: 6px 15px;
    cursor: pointer;
    font-size: 13px;
    display: flex;
    align-items: center;
    color: #cccccc;
    transition: background-color 0.1s;
    width: 100%;
    text-align: left;
    border-radius: 0;
    gap: 8px;
    background: none;
    border: none;
  }

  .panel-item.non-clickable {
    cursor: default;
    color: #858585;
  }

  .panel-item.non-clickable:hover {
    background-color: transparent;
  }

  .panel-item:hover:not(.non-clickable) {
    background-color: #2a2d2e;
  }
</style>