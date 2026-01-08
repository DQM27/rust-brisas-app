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
  import KeyboardShortcuts from "$lib/components/layout/KeyboardShortcuts.svelte";
  import {
    needsSetup,
    setWindowDecorations,
    setWindowSize,
  } from "$lib/services/keyringService";
  import {
    isScreensaverActive,
    awaitingScreensaverPassword,
    attemptExitScreensaver,
  } from "$lib/stores/sessionStore";
  import ScreensaverPasswordModal from "$lib/components/ScreensaverPasswordModal.svelte";
  import { modulesStore } from "$lib/stores/modules"; // Import modulesStore

  // Estado de autenticación reactivo
  let authenticated = $derived($isAuthenticated);
  let { children } = $props();

  // Estado del wizard de setup
  let showSetupWizard = $derived($setupWizardVisible);
  let checkingSetup = $state(true);

  // Estado del screensaver
  let screensaverActive = $derived($isScreensaverActive);
  let showPasswordModal = $derived($awaitingScreensaverPassword);

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
        await modulesStore.load(); // Cargar configuración de módulos
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
          if (showSetupWizard) {
            // Modo Launcher Setup (Grande)
            const { getCurrentWindow } = await import("@tauri-apps/api/window");
            const appWindow = getCurrentWindow();
            if (await appWindow.isMaximized()) await appWindow.unmaximize();

            await setWindowDecorations(false);
            await setWindowSize(700, 550);
            await appWindow.center();
          } else if (!authenticated) {
            // Modo Launcher Login (Compacto)
            const { getCurrentWindow } = await import("@tauri-apps/api/window");
            const appWindow = getCurrentWindow();
            if (await appWindow.isMaximized()) await appWindow.unmaximize();

            await setWindowDecorations(false);
            await setWindowSize(450, 500);
            await appWindow.center();
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

  // Efecto reactivo para manejar el modo screensaver FULLSCREEN
  $effect(() => {
    (async () => {
      if (screensaverActive) {
        try {
          // Enter fullscreen/kiosk mode when screensaver activates
          const { getCurrentWindow } = await import("@tauri-apps/api/window");
          const appWindow = getCurrentWindow();

          // First, check if minimized and restore
          const isMinimized = await appWindow.isMinimized();
          if (isMinimized) {
            console.log("[Layout] Window is minimized, restoring first...");
            await appWindow.unminimize();
            // Small delay to let window restore
            await new Promise((resolve) => setTimeout(resolve, 100));
          }

          // Then check if maximized and unmaximize
          const isMaximized = await appWindow.isMaximized();
          if (isMaximized) {
            console.log("[Layout] Window is maximized, unmaximizing first...");
            await appWindow.unmaximize();
            // Small delay to let window settle
            await new Promise((resolve) => setTimeout(resolve, 100));
          }

          // Set always on top to appear above all other windows (like native screensaver)
          console.log("[Layout] Setting always on top...");
          await appWindow.setAlwaysOnTop(true);

          // Give focus to the window
          console.log("[Layout] Setting focus...");
          await appWindow.setFocus();

          // Force kiosk mode FIRST to hide UI elements
          generalSettings.update((s) => ({ ...s, isKioskMode: true }));

          // Then set fullscreen
          console.log("[Layout] Setting fullscreen to true...");
          await appWindow.setFullscreen(true);

          console.log("[Layout] Screensaver activated - entering fullscreen");
        } catch (e) {
          console.error("[Layout] Error entering screensaver fullscreen:", e);
          // Fallback to just kiosk mode if Tauri fullscreen fails
          generalSettings.update((s) => ({ ...s, isKioskMode: true }));
        }
      } else if (screensaverActive === false) {
        // Only exit fullscreen if we're deactivating screensaver
        // Don't interfere with manual fullscreen toggles (double-click)
        try {
          const { getCurrentWindow } = await import("@tauri-apps/api/window");
          const appWindow = getCurrentWindow();
          const isCurrentlyFullscreen = await appWindow.isFullscreen();

          // Only exit fullscreen if we're currently in fullscreen
          // This prevents interfering with the user double-clicking
          if (isCurrentlyFullscreen) {
            await appWindow.setFullscreen(false);
            await appWindow.setAlwaysOnTop(false); // Remove always on top
            generalSettings.update((s) => ({ ...s, isKioskMode: false }));
            console.log(
              "[Layout] Screensaver deactivated - exiting fullscreen",
            );
          }
        } catch (e) {
          console.error("[Layout] Error exiting screensaver fullscreen:", e);
          generalSettings.update((s) => ({ ...s, isKioskMode: false }));
        }
      }
    })();
  });

  // Handle interaction when screensaver is active (to trigger password modal or exit)
  onMount(() => {
    let ignoreInteractions = false; // Flag to prevent immediate deactivation

    const handleInteraction = () => {
      // Ignore interactions if flag is set (during screensaver activation)
      if (ignoreInteractions) {
        console.log(
          "[Layout] Ignoring interaction during screensaver activation cooldown",
        );
        return;
      }

      if ($isScreensaverActive && !$awaitingScreensaverPassword) {
        console.log(
          "[Layout] User interaction detected - attempting to exit screensaver",
        );
        attemptExitScreensaver();
      }
    };

    // Listen for any interaction
    const events = ["mousedown", "keydown", "touchstart"];
    events.forEach((event) => {
      window.addEventListener(event, handleInteraction, {
        once: false,
        capture: true,
      });
    });

    // Watch for screensaver activation to set the ignore flag
    const unsubscribe = isScreensaverActive.subscribe((active) => {
      if (active) {
        console.log(
          "[Layout] Screensaver activated - ignoring interactions for 500ms",
        );
        ignoreInteractions = true;

        // After a short delay, start listening for real user interactions
        setTimeout(() => {
          ignoreInteractions = false;
          console.log("[Layout] Now listening for user interactions");
        }, 500);
      }
    });

    return () => {
      events.forEach((event) => {
        window.removeEventListener(event, handleInteraction, { capture: true });
      });
      unsubscribe();
    };
  });
</script>

{#if checkingSetup}
  <div class="h-screen w-screen bg-blue-600 flex items-center justify-center">
    <span class="text-white text-xl">Verificando configuración...</span>
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
        <div class="flex-1 w-full relative">
          {@render children()}
        </div>
      </div>
    </div>

    <!-- StatusBar (Solo si autenticado y no Setup) -->
    {#if authenticated && !$generalSettings.isKioskMode && !showSetupWizard}
      <StatusBar />
    {/if}

    <!-- Keyboard Shortcuts (Solo si autenticado) -->
    {#if authenticated}
      <KeyboardShortcuts />
    {/if}

    <!-- Screensaver Password Modal -->
    {#if showPasswordModal}
      <ScreensaverPasswordModal />
    {/if}
  </div>
{/if}
