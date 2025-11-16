<!-- src/lib/components/layout/sidebar/panels/LogsPanel.svelte -->
<script lang="ts">
  import { Search, Activity, Download, Settings } from 'lucide-svelte';
  import { openView } from '../../../../stores/sidebar';
  
  function handleKeydown(e: KeyboardEvent, action: () => void) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      action();
    }
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">REGISTROS DEL SISTEMA</div>
  <button 
    class="panel-item" 
    on:click={() => openView('dashboard', 'Registros del Sistema')}
    on:keydown={(e) => handleKeydown(e, () => openView('dashboard', 'Registros del Sistema'))}
  >
    <svelte:component this={Search} size={16} />
    <span>Buscar en registros</span>
  </button>
  <button 
    class="panel-item" 
    on:click={() => openView('user-list', 'Actividad de Usuarios')}
    on:keydown={(e) => handleKeydown(e, () => openView('user-list', 'Actividad de Usuarios'))}
  >
    <svelte:component this={Activity} size={16} />
    <span>Actividad de usuarios</span>
  </button>
  <div class="panel-item non-clickable">
    <svelte:component this={Download} size={16} />
    <span>Exportar registros</span>
  </div>
</div>
<div class="panel-section">
  <div class="panel-section-title">CONFIGURACIÓN</div>
  <div class="panel-item non-clickable">
    <svelte:component this={Settings} size={16} />
    <span>Configuración de logs</span>
  </div>
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