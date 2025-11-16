<!-- src/lib/components/layout/sidebar/panels/AccessPanel.svelte -->
<script lang="ts">
  import { Lock, BarChart3, Shield, FileText } from 'lucide-svelte';
  import { openView } from '../../../../stores/sidebar';
  
  function handleKeydown(e: KeyboardEvent, action: () => void) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      action();
    }
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">CONTROLES DE ACCESO</div>
  <button 
    class="panel-item" 
    on:click={() => openView('user-list', 'Gestión de Permisos')}
    on:keydown={(e) => handleKeydown(e, () => openView('user-list', 'Gestión de Permisos'))}
  >
    <svelte:component this={Lock} size={16} />
    <span>Gestión de permisos</span>
  </button>
  <button 
    class="panel-item" 
    on:click={() => openView('dashboard', 'Panel de Control')}
    on:keydown={(e) => handleKeydown(e, () => openView('dashboard', 'Panel de Control'))}
  >
    <svelte:component this={BarChart3} size={16} />
    <span>Panel de control</span>
  </button>
  <div class="panel-item non-clickable">
    <svelte:component this={Shield} size={16} />
    <span>Políticas de seguridad</span>
  </div>
</div>
<div class="panel-section">
  <div class="panel-section-title">HERRAMIENTAS</div>
  <button 
    class="panel-item" 
    on:click={() => openView('welcome', 'Documentación')}
    on:keydown={(e) => handleKeydown(e, () => openView('welcome', 'Documentación'))}
  >
    <svelte:component this={FileText} size={16} />
    <span>Documentación</span>
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