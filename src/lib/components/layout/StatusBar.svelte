<!-- src/lib/components/layout/StatusBar.svelte -->
<script lang="ts">
  import {
    Wifi,
    WifiOff,
    RefreshCw,
    Check,
    AlertCircle,
    Users,
    Bell,
    Battery,
    BatteryMedium,
    BatteryLow,
    Clock,
    AlertTriangle
  } from 'lucide-svelte';
  import InspectionToggle from './InspectionToggle.svelte';

  // Props del componente
  export let online: boolean = true;
  export let loading: boolean = false;
  export let syncStatus: 'synced' | 'syncing' | 'error' = 'synced';
  export let lastUpdate: Date = new Date();
  export let notifications: number = 0;
  export let alerts: number = 0;
  export let usersOnline: number = 0;
  export let batteryLevel: number = 100;
  export let showBattery: boolean = false;
  export let inspectionPanelVisible: boolean = false;

  // Formatear fecha y hora
  function formatTime(date: Date): string {
    return date.toLocaleTimeString('es-ES', { 
      hour: '2-digit', 
      minute: '2-digit' 
    });
  }

  function formatDate(date: Date): string {
    return date.toLocaleDateString('es-ES');
  }

  // Dispatcher para eventos
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher<{
    sync: {};
    notifications: {};
    alerts: {};
    users: {};
    inspectionToggle: { visible: boolean };
  }>();

  // Handlers para eventos
  function handleSyncClick(): void {
    console.log('Sincronizar manualmente');
    dispatch('sync', {});
  }

  function handleNotificationsClick(): void {
    console.log('Mostrar notificaciones');
    dispatch('notifications', {});
  }

  function handleAlertsClick(): void {
    console.log('Mostrar alertas');
    dispatch('alerts', {});
  }

  function handleUsersClick(): void {
    console.log('Mostrar usuarios online');
    dispatch('users', {});
  }

  function handleInspectionToggle(event: CustomEvent<{ visible: boolean }>): void {
    dispatch('inspectionToggle', event.detail);
  }

  // Handler para teclado
  function handleKeyPress(event: KeyboardEvent, handler: () => void): void {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handler();
    }
  }
</script>

<div class="status-bar">
  <!-- Sección izquierda - Estado del sistema -->
  <div class="status-left">
    <!-- Estado de conexión -->
    <div class="status-item {online ? 'online' : 'offline'}" role="status">
      {#if online}
        <Wifi size={14} />
        <span>En línea</span>
      {:else}
        <WifiOff size={14} />
        <span>Sin conexión</span>
      {/if}
    </div>
    
    <!-- Estado de sincronización -->
    <button 
      class="status-item sync-status {syncStatus}" 
      on:click={handleSyncClick}
      on:keydown={(e) => handleKeyPress(e, handleSyncClick)}
      type="button"
      title="Sincronizar manualmente"
    >
      {#if syncStatus === 'syncing'}
        <RefreshCw size={14} class="spinning" />
        <span>Sincronizando...</span>
      {:else if syncStatus === 'error'}
        <AlertCircle size={14} />
        <span>Error de sync</span>
      {:else}
        <Check size={14} />
        <span>Sincronizado</span>
      {/if}
    </button>
    
    <!-- Usuarios en línea -->
    {#if usersOnline > 0}
      <button 
        class="status-item" 
        on:click={handleUsersClick}
        on:keydown={(e) => handleKeyPress(e, handleUsersClick)}
        type="button"
        title="Ver usuarios en línea"
      >
        <Users size={14} />
        <span>{usersOnline} usuarios</span>
      </button>
    {/if}
  </div>
  
  <!-- Sección central - Información temporal -->
  <div class="status-center">
    {#if loading}
      <div class="status-item loading" role="status">
        <RefreshCw size={14} class="spinning" />
        <span>Procesando...</span>
      </div>
    {:else}
      <div class="status-item timestamp" role="status">
        <Clock size={14} />
        <span>{formatDate(lastUpdate)} {formatTime(lastUpdate)}</span>
      </div>
    {/if}
  </div>
  
  <!-- Sección derecha - Alertas, notificaciones e inspección -->
  <div class="status-right">
    <!-- Componente de inspección -->
    <InspectionToggle 
      visible={inspectionPanelVisible}
      variant="compact"
      on:toggle={handleInspectionToggle}
    />
    
    <!-- Nivel de batería -->
    {#if showBattery && batteryLevel < 100}
      <div class="status-item battery" role="status">
        {#if batteryLevel >= 80}
          <Battery size={16} />
        {:else if batteryLevel >= 30}
          <BatteryMedium size={16} />
        {:else}
          <BatteryLow size={16} />
        {/if}
        <span>{batteryLevel}%</span>
      </div>
    {/if}
    
    <!-- Alertas críticas -->
    {#if alerts > 0}
      <button 
        class="status-item alert" 
        on:click={handleAlertsClick}
        on:keydown={(e) => handleKeyPress(e, handleAlertsClick)}
        type="button"
        title="Ver alertas críticas"
      >
        <AlertTriangle size={14} />
        <span class="alert-count">{alerts}</span>
      </button>
    {/if}
    
    <!-- Notificaciones -->
    <button 
      class="status-item notification" 
      on:click={handleNotificationsClick}
      on:keydown={(e) => handleKeyPress(e, handleNotificationsClick)}
      type="button"
      title="Ver notificaciones"
    >
      <Bell size={14} />
      {#if notifications > 0}
        <span class="notification-count">{notifications}</span>
      {/if}
    </button>
  </div>
</div>

<style>
  .status-bar {
    background: var(--status-bar-background, #2d2d2d);
    color: var(--status-bar-foreground, #cccccc);
    height: 24px;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    padding: 0 12px;
    font-size: 12px;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    border-top: 1px solid var(--status-bar-border, #3c3c3c);
    user-select: none;
    gap: 8px;
  }
  
  .status-left,
  .status-center,
  .status-right {
    display: flex;
    align-items: center;
    gap: 16px;
  }
  
  .status-center {
    justify-self: center;
  }
  
  .status-right {
    justify-self: end;
  }
  
  .status-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 2px 8px;
    border-radius: 3px;
    transition: background-color 0.15s ease;
    white-space: nowrap;
    border: none;
    background: transparent;
    color: inherit;
    font-size: inherit;
  }
  
  button.status-item {
    cursor: pointer;
  }
  
  button.status-item:hover {
    background: var(--status-bar-hover, rgba(255, 255, 255, 0.08));
  }
  
  div.status-item {
    cursor: default;
  }
  
  .status-item.timestamp {
    color: #999;
    font-size: 11px;
  }
  
  /* Estados específicos */
  .status-item.online {
    color: #4caf50;
  }
  
  .status-item.offline {
    color: #f44336;
  }
  
  .status-item.sync-status.syncing {
    color: #2196f3;
  }
  
  .status-item.sync-status.error {
    color: #ff9800;
  }
  
  .status-item.loading {
    color: #2196f3;
  }
  
  .status-item.alert {
    color: #ff5722;
  }
  
  .status-item.battery {
    color: #4caf50;
  }
  
  /* Focus styles para accesibilidad */
  button.status-item:focus {
    outline: 2px solid #007acc;
    outline-offset: 1px;
  }
  
  /* Animaciones */
  .spinning {
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  /* Contadores */
  .notification-count,
  .alert-count {
    background: #007acc;
    color: white;
    border-radius: 10px;
    padding: 1px 5px;
    font-size: 10px;
    min-width: 16px;
    text-align: center;
    font-weight: 600;
    line-height: 1;
  }
  
  .alert-count {
    background: #f44336;
  }
  
  /* Responsive */
  @media (max-width: 768px) {
    .status-bar {
      grid-template-columns: 1fr;
      grid-template-rows: auto auto auto;
      height: auto;
      padding: 8px 12px;
      gap: 4px;
    }
    
    .status-left,
    .status-center,
    .status-right {
      justify-self: start;
      gap: 12px;
    }
    
    .status-center {
      order: 1;
    }
    
    .status-left {
      order: 2;
    }
    
    .status-right {
      order: 3;
    }
  }
</style>