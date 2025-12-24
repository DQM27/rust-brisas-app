<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { isAuthenticated } from "$lib/stores/auth";
  import Sidebar from "$lib/components/layout/sidebar/Sidebar.svelte";
  import StatusBar from "$lib/components/layout/StatusBar.svelte";
  import { setupWizardVisible } from "$lib/stores/ui";
  import { initNetworkMonitor } from "$lib/stores/network";
  import Toast from "$lib/components/Toast.svelte";
  import { themeStore } from "$lib/stores/themeStore"; // Inicializar tema
  import { generalSettings } from "$lib/stores/settingsStore";
  import SetupWizard from "$lib/components/setup/SetupWizard.svelte";
  import {
    needsSetup,
    setWindowDecorations,
    setWindowSize,
  } from "$lib/services/keyringService";

  // Estado de autenticación reactivo
  let authenticated = $derived($isAuthenticated);
  let { children } = $props();

  // Estado del wizard de setup
  let showSetupWizard = $derived($setupWizardVisible);
  let checkingSetup = $state(true);
  // Handler cuando se completa el setup
  async function handleSetupComplete() {
    $setupWizardVisible = false;
    await setWindowDecorations(true);
    await setWindowSize(1200, 800);
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().center();
  }

  // Inicializar monitor de red y atajos
  onMount(() => {
    // Verificar si necesita configuración inicial
    (async () => {
      try {
        $setupWizardVisible = await needsSetup();
        if ($setupWizardVisible) {
          await setWindowDecorations(false);
          await setWindowSize(700, 550);
          const { getCurrentWindow } = await import("@tauri-apps/api/window");
          await getCurrentWindow().center();
        } else {
          await setWindowDecorations(true);
          // Restaurar tamaño si es necesario o dejar que el sistema lo maneje
        }
      } catch (e) {
        console.error("Error verificando setup:", e);
        $setupWizardVisible = false;
      } finally {
        checkingSetup = false;
      }
    })();

    const cleanup = initNetworkMonitor();

    // Mostrar ventana cuando el frontend esté listo
    invoke("show_main_window").catch(console.error);

    return cleanup;
  });

  // Efecto reactivo para gestionar el estado de la ventana (Setup -> Login -> App)
  $effect(() => {
    if (!checkingSetup) {
      (async () => {
        try {
          if (showSetupWizard || !authenticated) {
            // Modo Launcher (Setup o Login)
            await setWindowDecorations(false);
            await setWindowSize(700, 550);
            const { getCurrentWindow } = await import("@tauri-apps/api/window");
            await getCurrentWindow().center();
          } else {
            // Modo App Completa
            await setWindowDecorations(true);
            await setWindowSize(1200, 800);
            const { getCurrentWindow } = await import("@tauri-apps/api/window");
            await getCurrentWindow().center();
          }
        } catch (e) {
          console.error("Error gestionando estado de ventana:", e);
        }
      })();
    }
  });
</script>

{#if checkingSetup}
  <div class="h-screen w-screen bg-surface-1 flex items-center justify-center">
    <!-- Un div limpio mientras carga el estado -->
  </div>
{:else if showSetupWizard}
  <SetupWizard onComplete={handleSetupComplete} />
{:else}
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
          {@render children()}
        </div>
      </div>
    </div>

    <!-- StatusBar -->
    {#if !$generalSettings.isKioskMode}
      <StatusBar />
    {/if}
  </div>
{/if}
