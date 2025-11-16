<!-- src/lib/components/layout/sidebar/panels/UsersPanel.svelte -->
<script lang="ts">
  import { Users, UserPlus, Edit3, BarChart3, Home } from 'lucide-svelte';
  import { openView, openUserRegistration } from '../../../../stores/sidebar';
  
  // Función para manejar teclado
  function handleKeydown(e: KeyboardEvent, action: () => void) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      action();
    }
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">GESTIÓN DE USUARIOS</div>
  <button 
    class="panel-item" 
    on:click={() => openView('user-list', 'Lista de Usuarios')}
    on:keydown={(e) => handleKeydown(e, () => openView('user-list', 'Lista de Usuarios'))}
  >
    <svelte:component this={Users} size={16} />
    <span>Lista de usuarios</span>
  </button>
  <button 
    class="panel-item" 
    on:click={openUserRegistration}
    on:keydown={(e) => handleKeydown(e, openUserRegistration)}
  >
    <svelte:component this={UserPlus} size={16} />
    <span>Registrar usuario</span>
  </button>
  <button 
    class="panel-item" 
    on:click={() => openView('user-editor', 'Editor de Usuarios')}
    on:keydown={(e) => handleKeydown(e, () => openView('user-editor', 'Editor de Usuarios'))}
  >
    <svelte:component this={Edit3} size={16} />
    <span>Editor de usuarios</span>
  </button>
</div>
<div class="panel-section">
  <div class="panel-section-title">VISTAS</div>
  <button 
    class="panel-item" 
    on:click={() => openView('dashboard', 'Dashboard')}
    on:keydown={(e) => handleKeydown(e, () => openView('dashboard', 'Dashboard'))}
  >
    <svelte:component this={BarChart3} size={16} />
    <span>Dashboard</span>
  </button>
  <button 
    class="panel-item" 
    on:click={() => openView('welcome', 'Bienvenida')}
    on:keydown={(e) => handleKeydown(e, () => openView('welcome', 'Bienvenida'))}
  >
    <svelte:component this={Home} size={16} />
    <span>Página de bienvenida</span>
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

  .panel-item:hover {
    background-color: #2a2d2e;
  }
</style>