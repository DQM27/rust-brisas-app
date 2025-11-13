<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import '../app.css';
  import { isAuthenticated } from '$lib/stores/auth';
  import Sidebar from '$lib/components/layout/sidebar/Sidebar.svelte';
  import StatusBar from '$lib/components/layout/StatusBar.svelte';
  import { onMount } from 'svelte';

  // Definir el tipo para syncStatus
  type SyncStatus = 'synced' | 'syncing' | 'error';

  // Estado para la StatusBar - TIPOS CORREGIDOS
  let statusState = {
    online: navigator.onLine,
    loading: false,
    syncStatus: 'synced' as SyncStatus, // ✅ Ahora acepta los 3 estados
    lastUpdate: new Date(),
    notifications: 0,
    alerts: 0,
    usersOnline: 0,
    batteryLevel: 100,
    showBattery: false
  };

  // Estado de autenticación reactivo
  $: authenticated = $isAuthenticated;

  // Manejar eventos de la StatusBar con lógica real
  function handleStatusEvent(event: CustomEvent) {
    const type = event.type;
    console.log('Evento de barra de estado:', type);
    
    switch(type) {
      case 'sync':
        handleSync();
        break;
      case 'notifications':
        handleNotifications();
        break;
      case 'alerts':
        handleAlerts();
        break;
      case 'users':
        handleUsers();
        break;
    }
  }

  // Funciones reales para los eventos
  function handleSync(): void {
    statusState = {
      ...statusState,
      loading: true,
      syncStatus: 'syncing' // ✅ Ahora es válido
    };
    
    // Simular sincronización
    setTimeout(() => {
      statusState = {
        ...statusState,
        loading: false,
        syncStatus: 'synced',
        lastUpdate: new Date()
      };
      console.log('Sincronización completada');
    }, 2000);
  }

  function handleNotifications(): void {
    // Aquí iría la lógica para abrir el panel de notificaciones
    console.log('Abriendo panel de notificaciones...');
    // Ejemplo: abrir un modal o cambiar vista
  }

  function handleAlerts(): void {
    // Aquí iría la lógica para abrir el panel de alertas
    console.log('Abriendo panel de alertas...');
    // Ejemplo: abrir un modal o cambiar vista
  }

  function handleUsers(): void {
    // Aquí iría la lógica para mostrar información de usuarios
    console.log('Mostrando información de usuarios...');
    // Ejemplo: cambiar a vista de usuarios
  }

  // Actualizar estado online/offline
  onMount(() => {
    if (typeof window !== 'undefined') {
      const updateOnlineStatus = () => {
        statusState = {
          ...statusState,
          online: navigator.onLine
        };
      };

      window.addEventListener('online', updateOnlineStatus);
      window.addEventListener('offline', updateOnlineStatus);

      // Actualizar cada minuto
      const interval = setInterval(() => {
        statusState = {
          ...statusState,
          lastUpdate: new Date()
        };
      }, 60000);

      return () => {
        window.removeEventListener('online', updateOnlineStatus);
        window.removeEventListener('offline', updateOnlineStatus);
        clearInterval(interval);
      };
    }
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
    online={statusState.online}
    loading={statusState.loading}
    syncStatus={statusState.syncStatus}
    lastUpdate={statusState.lastUpdate}
    notifications={statusState.notifications}
    alerts={statusState.alerts}
    usersOnline={statusState.usersOnline}
    batteryLevel={statusState.batteryLevel}
    showBattery={statusState.showBattery}
    on:sync={handleStatusEvent}
    on:notifications={handleStatusEvent}
    on:alerts={handleStatusEvent}
    on:users={handleStatusEvent}
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