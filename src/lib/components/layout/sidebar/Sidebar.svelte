<!-- src/lib/components/layout/sidebar/Sidebar.svelte -->
<script lang="ts">
  import { activeView } from '$lib/stores/ui';
  import { isAuthenticated, logout } from '$lib/stores/auth';
  import { resetTabs } from '$lib/stores/tabs';   
  
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
      // Cerrar el panel después de ejecutar la acción
      activePanel.set(null);
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
      logout();
  }

  const user = {
    name: 'Daniel',
    initials: 'DQ'
  };
</script>

<div class="flex h-full">
  <!-- Barra lateral de iconos -->
  <div class="flex w-[52px] flex-col justify-between border-r border-[#1f1f1f] 
              bg-[#2d2d2d] py-1.5">
    <div class="flex flex-col gap-1.5">
      {#each sidebarItems as item}
        <SidebarIcon
          {item}
          isActive={currentActivePanel === item.id}
          onSelect={handleItemSelect}
        />
      {/each}
    </div>

    <div class="flex flex-col items-center gap-2 pb-2">
      <button 
        class="flex h-8 w-8 items-center justify-center rounded-full border-none 
               bg-[#764ba2] text-[13px] font-semibold text-white cursor-pointer
               transition-all duration-200
               hover:bg-[#8c5fc3] hover:scale-105
               focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 focus-visible:ring-offset-[#2d2d2d]"
        title={user.name}
      >
        {user.initials}
      </button>

      <button 
        on:click={handleLogout} 
        class="group relative flex h-7 w-4/5 items-center justify-center rounded-md 
               border-none bg-transparent text-[#bbb] cursor-pointer
               transition-colors duration-150
               hover:bg-[#3a3a3a] hover:text-white
               focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 focus-visible:ring-offset-[#2d2d2d]"
        title="Cerrar sesión"
      >
        <LogOut size={20} />
        <span class="absolute left-[52px] z-[1000] hidden whitespace-nowrap rounded 
                     bg-[#3a3a3a] px-2 py-1 text-[11.5px] shadow-lg
                     animate-in fade-in slide-in-from-left-1 duration-150
                     group-hover:block">
          Cerrar sesión
        </span>
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