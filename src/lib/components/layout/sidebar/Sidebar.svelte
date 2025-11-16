<!-- src/lib/components/layout/sidebar/Sidebar.svelte -->
<script lang="ts">
  import { activeView } from '$lib/stores/ui';
  import { isAuthenticated } from '$lib/stores/auth';
  import { resetTabs } from '$lib/stores/tabs';
  import { goto } from '$app/navigation';
  
  // Importar iconos directamente
  import { User, Lock, FileText, Settings, LogOut } from 'lucide-svelte';

  // Componentes
  import SidebarIcon from './SidebarIcon.svelte';
  import SidebarPanel from './SidebarPanel.svelte';
  
  // Paneles
  import UsersPanel from './panels/UsersPanel.svelte';
  import AccessPanel from './panels/AccessPanel.svelte';
  import LogsPanel from './panels/LogsPanel.svelte';
  import SettingsPanel from './panels/SettingsPanel.svelte';

  // Store y tipos
  import { activePanel } from '../../../stores/sidebar';
  import type { SidebarItem } from '../../../types/Sidebar';

  // Items configurables - ahora con importaciones directas
  const sidebarItems: SidebarItem[] = [
    { 
      id: 'users', 
      icon: User,
      label: 'Usuarios',
      panelComponent: UsersPanel
    },
    { 
      id: 'access', 
      icon: Lock,
      label: 'Accesos',
      panelComponent: AccessPanel
    },
    { 
      id: 'logs', 
      icon: FileText,
      label: 'Logs', 
      panelComponent: LogsPanel
    },
    { 
      id: 'settings', 
      icon: Settings,
      label: 'Configuración',
      panelComponent: SettingsPanel
    },
  ];

  $: currentActivePanel = $activePanel;
  $: activeItem = sidebarItems.find(item => item.id === currentActivePanel);

  function handleItemSelect(item: SidebarItem) {
    activeView.set(item.id);
    
    if (item.action) {
      item.action();
    } else if (item.panelComponent) {
      // Toggle panel
      if ($activePanel === item.id) {
        activePanel.set(null);
      } else {
        activePanel.set(item.id);
      }
    }
  }

  function handlePanelClose() {
    activePanel.set(null);
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
</script>

<div class="sidebar-container">
  <!-- Barra lateral de iconos -->
  <div class="sidebar">
    <div class="top">
      {#each sidebarItems as item}
        <SidebarIcon
          {item}
          isActive={currentActivePanel === item.id}
          onSelect={handleItemSelect}
        />
      {/each}
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

  <!-- Panel lateral desplegable -->
  {#if activeItem}
    <SidebarPanel
      item={activeItem} 
      isOpen={true}
      onClose={handlePanelClose}
    />
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
    border: none;
  }

  .avatar:hover {
    background: #8c5fc3;
  }

  .logout {
    width: 80%;
    border-radius: 6px;
    height: 28px;
    background: none;
    border: none;
    color: #bbb;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .logout:hover {
    background: #3a3a3a;
    color: #fff;
  }

  .logout .tooltip {
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

  .logout:hover .tooltip {
    display: block;
  }
</style>