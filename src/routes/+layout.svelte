<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import '../app.css';
  import { isAuthenticated } from '$lib/stores/auth';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import StatusBar from '$lib/components/layout/StatusBar.svelte';
  import { get } from 'svelte/store';
  import { onMount } from 'svelte';

  // Estado para la StatusBar
  let statusState = {
    online: navigator.onLine,
    loading: false,
    syncStatus: 'synced' as const,
    lastUpdate: new Date(),
    notifications: 0,
    alerts: 0,
    usersOnline: 0,
    batteryLevel: 100,
    showBattery: false // Ocultar batería por defecto en web
  };

  // Verificar autenticación al montar
  let authenticated = false;
  
  onMount(() => {
    // Suscribirse al store de autenticación
    const unsubscribe = isAuthenticated.subscribe(value => {
      authenticated = value;
    });
    
    return () => unsubscribe();
  });

  // Manejar eventos de la StatusBar
  function handleStatusEvent(event: CustomEvent) {
    const type = event.type;
    console.log('Evento de barra de estado:', type);
    
    switch(type) {
      case 'sync':
        console.log('Iniciando sincronización manual...');
        break;
      case 'notifications':
        console.log('Abriendo panel de notificaciones...');
        break;
      case 'alerts':
        console.log('Abriendo panel de alertas...');
        break;
      case 'users':
        console.log('Mostrando información de usuarios...');
        break;
    }
  }

  // Actualizar estado online/offline
  if (typeof window !== 'undefined') {
    window.addEventListener('online', () => {
      statusState.online = true;
      statusState = statusState; // Trigger update
    });
    
    window.addEventListener('offline', () => {
      statusState.online = false;
      statusState = statusState; // Trigger update
    });
  }
</script>

<div class="layout">
  <!-- Main Area - Sin Top Panel -->
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
    {...statusState}
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