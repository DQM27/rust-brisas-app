<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import '../app.css';
  import { isAuthenticated } from '$lib/stores/auth';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import StatusBar from '$lib/components/layout/StatusBar.svelte';
  import { get } from 'svelte/store';

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

  // Manejar eventos de la StatusBar - CORREGIDO
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
  <!-- Top Panel (VS Code style) -->
  <div class="top-panel">
    <nav class="top-menu">
      <ul>
        <li>Archivo</li>
        <li>Editar</li>
        <li>Selección</li>
        <li>Ver</li>
        <li>Ir</li>
        <li>Ejecutar</li>
        <li>Terminal</li>
        <li>Ayuda</li>
      </ul>
    </nav>
    <div class="search-bar">
      <input placeholder="Buscar en el sistema..." />
    </div>
    <div class="window-controls">
      <button class="minimize">−</button>
      <button class="maximize">□</button>
      <button class="close">×</button>
    </div>
  </div>

  <!-- Main Area -->
  <div class="main">
    {#if get(isAuthenticated)}
      <Sidebar />
    {/if}
    <div class="editor-area">
      <slot />
    </div>
  </div>

  <!-- Nueva StatusBar con lucide-svelte -->
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

  /* TOP PANEL */
  .top-panel { 
    background: #3c3c3c; 
    padding: 0 8px; 
    display: flex; 
    align-items: center; 
    height: 35px; 
    border-bottom: 1px solid #2d2d2d;
    -webkit-app-region: drag;
  }
  
  .top-menu ul { 
    display: flex; 
    list-style: none; 
    margin-right: 16px;
  }
  
  .top-menu li { 
    padding: 0 12px; 
    font-size: 13px; 
    cursor: pointer; 
    height: 35px;
    display: flex;
    align-items: center;
    -webkit-app-region: no-drag;
    transition: background-color 0.15s ease;
  }
  
  .top-menu li:hover { 
    background: #2a2d2e; 
  }
  
  .search-bar { 
    flex: 1; 
    max-width: 400px; 
    margin: 0 16px; 
    position: relative; 
    background: #252526; 
    border-radius: 4px; 
    border: 1px solid #3c3c3c;
  }
  
  .search-bar input { 
    width: 100%; 
    background: transparent; 
    border: none; 
    padding: 6px 12px 6px 32px; 
    color: #fff; 
    font-size: 13px; 
    outline: none; 
    -webkit-app-region: no-drag;
  }
  
  .search-bar input::placeholder { 
    color: #858585; 
  }
  
  .search-bar::before { 
    content: '🔍';
    position: absolute;
    left: 10px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 12px;
    color: #858585;
  }
  
  .window-controls { 
    display: flex; 
    -webkit-app-region: no-drag;
  }
  
  .window-controls button { 
    background: none; 
    border: none; 
    color: #ccc; 
    font-size: 16px; 
    width: 46px; 
    height: 100%; 
    cursor: pointer; 
    transition: background-color 0.15s ease;
  }
  
  .window-controls button:hover { 
    background: #2a2d2e; 
  }
  
  .close:hover { 
    background: #e81123; 
    color: white;
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

  .editor-area {
    flex: 1;
    background: #1e1e1e;
    overflow: auto;
    position: relative;
  }

  /* Responsive para móviles */
  @media (max-width: 768px) {
    .top-panel {
      padding: 0 4px;
    }
    
    .top-menu ul {
      margin-right: 8px;
    }
    
    .top-menu li {
      padding: 0 8px;
      font-size: 12px;
    }
    
    .search-bar {
      max-width: 200px;
      margin: 0 8px;
    }
    
    .search-bar input {
      padding: 6px 8px 6px 28px;
      font-size: 12px;
    }
    
    .window-controls button {
      width: 36px;
      font-size: 14px;
    }
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
</style>