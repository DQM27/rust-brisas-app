<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { isAuthenticated } from "$lib/stores/auth";
  import Sidebar from "$lib/components/layout/sidebar/Sidebar.svelte";
  import StatusBar from "$lib/components/layout/StatusBar.svelte";
  import { inspectionPanel } from "$lib/stores/ui";
  import { initNetworkMonitor } from "$lib/stores/network";
  import Toast from "$lib/components/Toast.svelte";
  import { themeStore } from "$lib/stores/themeStore"; // Inicializar tema
  import { generalSettings } from "$lib/stores/settingsStore";
  import { shortcutService } from "$lib/services/shortcutService";

  // Estado de autenticación reactivo
  $: authenticated = $isAuthenticated;

  // Toggle del panel de inspección
  function toggleInspectionPanel(): void {
    $inspectionPanel.visible = !$inspectionPanel.visible;
  }

  // Inicializar monitor de red y atajos
  onMount(() => {
    // shortcutService se auto-inicializa, pero podemos forzar init explicito si queremos
    shortcutService.init();
    const cleanup = initNetworkMonitor();
    return cleanup;
  });
</script>

<div
  class="flex flex-col h-screen bg-surface-1 text-primary overflow-hidden font-sans"
>
  <!-- Main Area -->
  <div
    class="flex flex-1 w-full overflow-hidden bg-surface-1 md:flex-row flex-col"
  >
    {#if authenticated && !$generalSettings.isKioskMode}
      <Sidebar />
    {/if}
    <div class="flex-1 bg-surface-1 overflow-auto relative flex">
      <Toast />
      <div class="flex-1 w-full">
        <slot />
      </div>
    </div>
  </div>

  <!-- StatusBar -->
  {#if !$generalSettings.isKioskMode}
    <StatusBar
      inspectionPanelVisible={$inspectionPanel.visible}
      on:inspectionToggle={toggleInspectionPanel}
    />
  {/if}
</div>
