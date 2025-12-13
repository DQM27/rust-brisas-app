<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { isAuthenticated } from "$lib/stores/auth";
  import Sidebar from "$lib/components/layout/sidebar/Sidebar.svelte";
  import StatusBar from "$lib/components/layout/StatusBar.svelte";
  import { inspectionPanel, setupWizardVisible } from "$lib/stores/ui";
  import { initNetworkMonitor } from "$lib/stores/network";
  import Toast from "$lib/components/Toast.svelte";
  import { themeStore } from "$lib/stores/themeStore"; // Inicializar tema
  import { generalSettings } from "$lib/stores/settingsStore";
  import { shortcutService } from "$lib/services/shortcutService";
  import SetupWizard from "$lib/components/setup/SetupWizard.svelte";
  import { needsSetup } from "$lib/services/keyringService";

  // Estado de autenticación reactivo
  let authenticated = $derived($isAuthenticated);

  // Estado del wizard de setup
  let showSetupWizard = $derived($setupWizardVisible);
  let checkingSetup = $state(true);

  // Toggle del panel de inspección
  function toggleInspectionPanel(): void {
    $inspectionPanel.visible = !$inspectionPanel.visible;
  }

  // Handler cuando se completa el setup
  function handleSetupComplete() {
    $setupWizardVisible = false;
  }

  // Inicializar monitor de red y atajos
  onMount(() => {
    // Verificar si necesita configuración inicial
    (async () => {
      try {
        $setupWizardVisible = await needsSetup();
      } catch (e) {
        console.error("Error verificando setup:", e);
        $setupWizardVisible = false;
      } finally {
        checkingSetup = false;
      }
    })();

    // Mostrar ventana cuando el frontend esté listo
    invoke("show_main_window").catch(console.error);

    // shortcutService se auto-inicializa, pero podemos forzar init explicito si queremos
    shortcutService.init();
    const cleanup = initNetworkMonitor();
    return cleanup;
  });
</script>

<!-- Setup Wizard (primera ejecución) -->
{#if showSetupWizard}
  <SetupWizard onComplete={handleSetupComplete} />
{/if}

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
