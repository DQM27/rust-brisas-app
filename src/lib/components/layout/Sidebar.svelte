<!-- src/lib/components/layout/Sidebar.svelte -->
<script lang="ts">
  import { activeView } from '$lib/stores/ui';
  import { get } from 'svelte/store';
  import { isAuthenticated } from '$lib/stores/auth';
  import { resetTabs, openTab } from '$lib/stores/tabs';
  import { goto } from '$app/navigation';

  import {
    User,
    Lock,
    FileText,
    Settings,
    LogOut,
    UserPlus,
  } from 'lucide-svelte';

  // Estado para controlar qué panel está activo
  let activePanel: string | null = null;
  
  // Función para alternar paneles
  function togglePanel(panelId: string) {
    if (activePanel === panelId) {
      activePanel = null;
    } else {
      activePanel = panelId;
    }
  }

  // Función para manejar teclado en items del panel
  function handlePanelItemKeydown(event: KeyboardEvent, action: () => void) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      action();
    }
  }

  const items = [
    { 
      id: 'users', 
      icon: User, 
      label: 'Usuarios'
    },
    { 
      id: 'access', 
      icon: Lock, 
      label: 'Accesos'
    },
    { 
      id: 'logs', 
      icon: FileText, 
      label: 'Logs'
    },
    { 
      id: 'settings', 
      icon: Settings, 
      label: 'Configuración'
    },
  ];

  function select(view: string) {
    activeView.set(view);
    togglePanel(view);
  }

  function openUserRegistration() {
    openTab({
      componentKey: 'user-register',
      title: 'Registrar Usuario',
      focusOnOpen: true
    });
  }

  function openView(componentKey: ComponentKey, title: string) {
    openTab({
      componentKey: componentKey,
      title: title,
      focusOnOpen: true
    });
  }

  function handleLogout() {
    isAuthenticated.set(false);
    resetTabs();
    activeView.set('');
    goto('/login');
  }

  const user = {
    name: 'Daniel',
    initials: 'DQ'
  };

  type ComponentKey = 
    | 'welcome'
    | 'user-list'
    | 'user-editor'
    | 'user-register'
    | 'dashboard';
</script>

<div class="sidebar-container">
  <!-- Barra lateral de iconos -->
  <div class="sidebar">
    <div class="top">
      {#each items as item}
        <button
          class:selected={activePanel === item.id}
          on:click={() => select(item.id)}
          title={item.label}
        >
          <svelte:component this={item.icon} size={22} />
          <span class="tooltip">{item.label}</span>
        </button>
      {/each}

      <button on:click={openUserRegistration} title="Registrar nuevo usuario">
        <UserPlus size={22} />
        <span class="tooltip">Registrar Usuario</span>
      </button>
    </div>

    <div class="bottom">
      <button class="avatar" title={user.name}>
        {user.initials}
      </button>

      <button on:click={handleLogout} class="logout" title="Cerrar sesión">
        <LogOut size={20} />
        <span class="tooltip">Cerrar sesión</span>
      </button>
    </div>
  </div>

  <!-- Panel lateral desplegable (estilo VS Code) -->
  {#if activePanel}
    <div class="side-panel">
      <div class="panel-header">
        <span>{items.find(item => item.id === activePanel)?.label}</span>
        <button class="close-btn" on:click={() => activePanel = null} title="Cerrar panel">×</button>
      </div>
      <div class="panel-content">
        {#if activePanel === 'users'}
          <div class="panel-section">
            <div class="panel-section-title">GESTIÓN DE USUARIOS</div>
            <button 
              class="panel-item" 
              on:click={() => openView('user-list', 'Lista de Usuarios')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('user-list', 'Lista de Usuarios'))}
            >
              <i>👥</i> Lista de usuarios
            </button>
            <button 
              class="panel-item" 
              on:click={openUserRegistration}
              on:keydown={(e) => handlePanelItemKeydown(e, openUserRegistration)}
            >
              <i>➕</i> Crear nuevo usuario
            </button>
            <button 
              class="panel-item" 
              on:click={() => openView('user-editor', 'Editor de Usuarios')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('user-editor', 'Editor de Usuarios'))}
            >
              <i>👤</i> Editor de usuarios
            </button>
          </div>
          <div class="panel-section">
            <div class="panel-section-title">VISTAS</div>
            <button 
              class="panel-item" 
              on:click={() => openView('dashboard', 'Dashboard')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('dashboard', 'Dashboard'))}
            >
              <i>📊</i> Dashboard
            </button>
            <button 
              class="panel-item" 
              on:click={() => openView('welcome', 'Bienvenida')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('welcome', 'Bienvenida'))}
            >
              <i>🚪</i> Página de bienvenida
            </button>
          </div>
        {:else if activePanel === 'access'}
          <div class="panel-section">
            <div class="panel-section-title">ACCESOS Y PERMISOS</div>
            <button 
              class="panel-item" 
              on:click={() => openView('user-list', 'Gestión de Accesos')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('user-list', 'Gestión de Accesos'))}
            >
              <i>🔐</i> Gestión de accesos
            </button>
            <button 
              class="panel-item" 
              on:click={() => openView('dashboard', 'Panel de Control')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('dashboard', 'Panel de Control'))}
            >
              <i>📈</i> Panel de control
            </button>
            <div class="panel-item non-clickable">
              <i>🛡️</i> Configuración de seguridad
            </div>
          </div>
          <div class="panel-section">
            <div class="panel-section-title">HERRAMIENTAS</div>
            <button 
              class="panel-item" 
              on:click={() => openView('welcome', 'Documentación')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('welcome', 'Documentación'))}
            >
              <i>📚</i> Documentación
            </button>
          </div>
        {:else if activePanel === 'logs'}
          <div class="panel-section">
            <div class="panel-section-title">REGISTROS DEL SISTEMA</div>
            <button 
              class="panel-item" 
              on:click={() => openView('dashboard', 'Registros del Sistema')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('dashboard', 'Registros del Sistema'))}
            >
              <i>📋</i> Ver registros
            </button>
            <div class="panel-item non-clickable">
              <i>🔍</i> Auditoría del sistema
            </div>
            <div class="panel-item non-clickable">
              <i>⚙️</i> Configuración de logs
            </div>
          </div>
          <div class="panel-section">
            <div class="panel-section-title">HERRAMIENTAS</div>
            <button 
              class="panel-item" 
              on:click={() => openView('user-list', 'Actividad de Usuarios')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('user-list', 'Actividad de Usuarios'))}
            >
              <i>📊</i> Actividad de usuarios
            </button>
            <div class="panel-item non-clickable">
              <i>💾</i> Exportar registros
            </div>
          </div>
        {:else if activePanel === 'settings'}
          <div class="panel-section">
            <div class="panel-section-title">CONFIGURACIÓN GENERAL</div>
            <button 
              class="panel-item" 
              on:click={() => openView('dashboard', 'Ajustes')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('dashboard', 'Ajustes'))}
            >
              <i>🎨</i> Apariencia
            </button>
            <div class="panel-item non-clickable">
              <i>🔔</i> Notificaciones
            </div>
            <div class="panel-item non-clickable">
              <i>💾</i> Backup
            </div>
          </div>
          <div class="panel-section">
            <div class="panel-section-title">SISTEMA</div>
            <div class="panel-item non-clickable">
              <i>🔄</i> Actualizaciones
            </div>
            <button 
              class="panel-item" 
              on:click={() => openView('welcome', 'Acerca del Sistema')}
              on:keydown={(e) => handlePanelItemKeydown(e, () => openView('welcome', 'Acerca del Sistema'))}
            >
              <i>📡</i> Acerca del sistema
            </button>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .sidebar-container {
    display: flex;
    height: 100%;
  }

  .sidebar {
    width: 52px;
    background: #2d2d2d;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 6px 0;
    border-right: 1px solid #1f1f1f;
  }

  .top {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  button {
    background: none;
    border: none;
    color: #bbb;
    width: 100%;
    height: 42px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    cursor: pointer;
  }

  button.selected {
    color: #fff;
    background: #3c3c3c;
  }

  button:hover {
    background: #3a3a3a;
    color: #fff;
  }

  .tooltip {
    position: absolute;
    left: 52px;
    background: #3a3a3a;
    white-space: nowrap;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 11.5px;
    display: none;
    z-index: 1000;
  }

  button:hover .tooltip {
    display: block;
  }

  .bottom {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding-bottom: 8px;
  }

  .avatar {
    width: 32px;
    height: 32px;
    background: #764ba2;
    border-radius: 50%;
    font-size: 13px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    color: white;
    cursor: pointer;
  }

  .avatar:hover {
    background: #8c5fc3;
  }

  .logout {
    width: 80%;
    border-radius: 6px;
    height: 28px;
  }

  /* Estilos para el panel lateral (estilo VS Code) */
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
  }

  .panel-item.non-clickable {
    cursor: default;
    color: #858585;
  }

  .panel-item.non-clickable:hover {
    background-color: transparent;
  }

  .panel-item i {
    margin-right: 8px;
    font-size: 14px;
    width: 16px;
    text-align: center;
  }

  .panel-item:hover:not(.non-clickable) {
    background-color: #2a2d2e;
  }
</style>