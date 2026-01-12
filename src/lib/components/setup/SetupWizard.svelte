<script lang="ts">
  import {
    Shield,
    Key,
    Monitor,
    Check,
    RefreshCw,
    AlertCircle,
    X,
    Server,
    Info,
  } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import { ask, save, open, message } from "@tauri-apps/plugin-dialog";
  import {
    setupCredentials,
    generateRandomSecret,
    exitApp,
    resetAllCredentials,
    getCredentialStatus,
    exportMasterKey,
    importMasterKey,
    updateArgon2Params,
    type Argon2Params,
  } from "$lib/services/keyringService";

  // Props
  interface Props {
    onComplete?: () => void;
  }
  let { onComplete }: Props = $props();

  // State
  let isSubmitting = $state(false);
  let isResetting = $state(false);
  let isConfirming = $state(false);
  let error = $state("");
  let keyFoundInSystem = $state(false);
  let keyImported = $state(false);

  // Security Mode
  let securityMode = $state<"new" | "join">("new");
  let isProcessing = $state(false);
  let generatedPassword = $state("");
  let importPassword = $state("");

  let argon2Params = $state<Argon2Params>({
    memory: 19456,
    iterations: 2,
    parallelism: 1,
    secret: "",
  });

  let terminalName = $state("Terminal Principal");
  let terminalLocation = $state("Sin asignar");

  // Style Constants matching UI Patterns
  const inputClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[36px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all";
  const labelClass = "block text-xs font-medium text-secondary mb-1.5 ml-1";
  const sectionClass = "h-full flex flex-col";
  const sectionHeaderClass =
    "flex items-center gap-3 mb-4 pb-3 border-b border-white/5";
  const sectionTitleClass = "font-bold text-base text-primary";

  // Validations
  let formValid = $derived(
    terminalName.trim() !== "" &&
      terminalLocation.trim() !== "" &&
      argon2Params.secret.trim() !== "",
  );

  // Init
  $effect(() => {
    checkSystemKey();
  });

  async function checkSystemKey() {
    try {
      const status = await getCredentialStatus();
      keyFoundInSystem = status.argon2_configured;
      if (keyFoundInSystem && argon2Params.secret === "") {
        argon2Params.secret = "********"; // Placeholder visual
      }
    } catch (e) {
      console.error("Error verificando llaves:", e);
    }
  }

  function setSecurityMode(mode: "new" | "join") {
    securityMode = mode;
    error = "";
  }

  function generateInstallPassword() {
    const chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let pass = "";
    const bytes = new Uint8Array(12);
    window.crypto.getRandomValues(bytes);

    for (let i = 0; i < bytes.length; i++) {
      pass += chars[bytes[i] % chars.length];
      if ((i + 1) % 4 === 0 && i < bytes.length - 1) pass += "-";
    }
    return pass;
  }

  async function handleGenerateAndExport() {
    isProcessing = true;
    error = "";
    try {
      const secret = await generateRandomSecret();
      argon2Params.secret = secret;
      await updateArgon2Params(argon2Params);
      const installPass = generateInstallPassword();
      const filePath = await save({
        title: "Guardar Llave Maestra de Seguridad",
        defaultPath: "megabrisas_master.key",
        filters: [{ name: "Key Files", extensions: ["key"] }],
      });

      if (!filePath) {
        isProcessing = false;
        return;
      }

      await exportMasterKey(filePath, installPass);
      generatedPassword = installPass;
      keyFoundInSystem = true;
    } catch (e: any) {
      error = String(e);
      await message(`Error generando llave: ${e}`, {
        title: "Error de Seguridad",
        kind: "error",
      });
    } finally {
      isProcessing = false;
    }
  }

  async function handleImportKey() {
    isProcessing = true;
    error = "";
    try {
      const filePath = await open({
        title: "Seleccionar Llave Maestra (.megabrisas_master)",
        filters: [{ name: "Key Files", extensions: ["key"] }],
        multiple: false,
      });

      if (!filePath) {
        isProcessing = false;
        return;
      }

      await importMasterKey(filePath as string, importPassword);
      keyFoundInSystem = true;
      keyImported = true;
      const status = await getCredentialStatus();
      if (status.argon2_configured) {
        argon2Params.secret = "********";
      }
    } catch (e: any) {
      error = String(e);
      await message(`Error importando llave: ${e}`, {
        title: "Error de Importación",
        kind: "error",
      });
    } finally {
      isProcessing = false;
    }
  }

  async function handleSubmit() {
    isSubmitting = true;
    error = "";
    try {
      await setupCredentials({
        argon2: argon2Params,
        terminal_name: terminalName,
        terminal_location: terminalLocation,
      });
      onComplete?.();
    } catch (e: any) {
      error = `Error guardando config: ${e.message || String(e)}`;
    } finally {
      isSubmitting = false;
    }
  }

  async function handleFactoryReset() {
    if (isConfirming || isResetting) return;

    isConfirming = true;
    try {
      const confirmed = await ask(
        "¿Estás seguro? Esto borrará TODA la configuración.",
        {
          title: "Confirmar Reset de Fábrica",
          kind: "warning",
          okLabel: "Aceptar",
          cancelLabel: "Cancelar",
        },
      );

      if (confirmed) {
        isResetting = true;
        await resetAllCredentials(true);
        setTimeout(() => {
          window.location.reload();
        }, 800);
      }
    } finally {
      isConfirming = false;
    }
  }
</script>

<div
  class="h-screen w-screen bg-surface-2 flex flex-col items-center justify-center p-0 overflow-hidden"
  transition:fade
>
  {#if isResetting}
    <div class="flex flex-col items-center gap-4 text-center" transition:fade>
      <RefreshCw class="w-12 h-12 text-success animate-spin" />
      <div>
        <h2 class="text-xl font-bold text-primary">Reiniciando Sistema...</h2>
        <p class="text-sm text-secondary">
          Limpiando configuración y llaves de seguridad.
        </p>
      </div>
    </div>
  {:else}
    <!-- Main Fullscreen Container - No Borders/Shadows -->
    <div class="w-full h-full flex flex-col max-w-[1400px]">
      <!-- Header -->
      <div
        class="flex-none px-6 py-4 border-b border-surface flex items-center justify-between"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary/10 rounded-lg">
            <Shield class="w-6 h-6 text-primary" />
          </div>
          <div>
            <h2 class="text-lg font-bold text-primary">
              Configuración Inicial
            </h2>
            <p class="text-xs text-secondary">
              Configura la terminal y la seguridad del sistema
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={exitApp}
          class="p-2 text-secondary hover:text-error hover:bg-surface-3 rounded-lg transition-colors"
          title="Salir"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content Area -->
      <div class="p-6 flex-1 overflow-y-auto">
        {#if error}
          <div
            class="mb-4 p-3 rounded-lg bg-error/10 border border-error/20 flex items-start gap-3"
          >
            <AlertCircle class="w-5 h-5 text-error flex-shrink-0 mt-0.5" />
            <span class="text-sm text-error/90 font-medium">{error}</span>
          </div>
        {/if}

        <!-- Grid with gap-6 -->
        <!-- Single Unified Content Flow -->
        <div
          class="bg-black/20 border border-white/5 rounded-xl p-6 h-full flex flex-col gap-6"
        >
          <!-- Inputs Grid (Side by Side) -->
          <div class="grid grid-cols-2 gap-4 flex-none">
            <div>
              <label for="terminalName" class={labelClass}
                >Nombre de la Terminal</label
              >
              <input
                id="terminalName"
                type="text"
                bind:value={terminalName}
                placeholder="Ej: Porteria Principal"
                class={inputClass}
              />
            </div>
            <div>
              <label for="terminalLocation" class={labelClass}
                >Ubicación Física</label
              >
              <input
                id="terminalLocation"
                type="text"
                bind:value={terminalLocation}
                placeholder="Ej: Acceso Vehicular Norte"
                class={inputClass}
              />
            </div>
          </div>

          <!-- Security Content -->
          <div class="flex-1 flex flex-col min-h-0 overflow-y-auto pr-1">
            {#if keyImported}
              <div
                class="bg-green-500/10 border border-green-500/20 p-3 rounded-lg flex gap-3 items-center mb-4 flex-none"
              >
                <div class="p-1.5 bg-green-500/20 rounded-full">
                  <Check class="w-4 h-4 text-green-500" />
                </div>
                <div>
                  <p class="text-sm font-bold text-white">
                    Llave Importada Correctamente
                  </p>
                  <p class="text-[11px] text-gray-300">
                    El archivo de llave maestra se ha verificado y cargado.
                  </p>
                </div>
              </div>
            {:else if keyFoundInSystem && !generatedPassword}
              <div
                class="bg-blue-500/10 border border-blue-500/20 p-3 rounded-lg flex gap-3 items-center mb-4 flex-none"
              >
                <div class="p-1.5 bg-blue-500/20 rounded-full">
                  <Info class="w-4 h-4 text-blue-500" />
                </div>
                <div>
                  <p class="text-sm font-bold text-white">
                    Llave Maestra Detectada
                  </p>
                  <p class="text-[11px] text-gray-300">
                    Se detectó una configuración previa segura.
                  </p>
                </div>
              </div>
            {/if}

            <!-- Mode Toggles -->
            <div class="flex gap-3 mb-4 flex-none justify-center">
              <button
                onclick={() => setSecurityMode("new")}
                class="px-4 py-2.5 rounded-lg border transition-all flex items-center justify-center {securityMode ===
                'new'
                  ? 'bg-primary/10 border-primary text-primary shadow-lg shadow-primary/10'
                  : 'bg-surface-3 border-surface text-secondary hover:bg-surface-3/80 hover:border-surface/80'}"
              >
                <span class="font-bold text-sm">Nueva Key</span>
              </button>
              <button
                onclick={() => setSecurityMode("join")}
                class="px-4 py-2.5 rounded-lg border transition-all flex items-center justify-center {securityMode ===
                'join'
                  ? 'bg-accent/10 border-accent text-accent shadow-lg shadow-accent/10'
                  : 'bg-surface-3 border-surface text-secondary hover:bg-surface-3/80 hover:border-surface/80'}"
              >
                <span class="font-bold text-sm">Key Existente</span>
              </button>
            </div>

            <!-- Action Area -->
            <div class="flex-none">
              {#if securityMode === "new"}
                {#if generatedPassword}
                  <div
                    class="space-y-4 animate-in fade-in slide-in-from-bottom-2"
                  >
                    <div
                      class="bg-warning/10 border border-warning/20 p-4 rounded-xl"
                    >
                      <div class="flex items-center gap-2 mb-3">
                        <AlertCircle class="w-4 h-4 text-warning" />
                        <p
                          class="text-xs text-white font-bold uppercase tracking-wide"
                        >
                          Contraseña de Instalación
                        </p>
                      </div>
                      <code
                        class="block w-full text-center p-3 bg-black/40 rounded-lg border border-white/5 text-xl font-mono text-warning select-all tracking-wider"
                      >
                        {generatedPassword}
                      </code>
                      <p class="text-[10px] text-gray-300 text-center mt-2">
                        Guarda esta contraseña en un lugar seguro.
                      </p>
                    </div>
                  </div>
                {:else}
                  <p
                    class="text-sm text-secondary mb-4 leading-normal text-center"
                  >
                    Genera una nueva llave maestra cifrada para iniciar una
                    nueva red de seguridad privada.
                  </p>
                  <button
                    onclick={handleGenerateAndExport}
                    disabled={isProcessing}
                    class="w-fit mx-auto px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-primary hover:text-primary text-sm disabled:opacity-50 flex items-center justify-center gap-2"
                  >
                    {#if isProcessing}
                      <RefreshCw class="w-4 h-4 animate-spin" />
                      <span>Procesando...</span>
                    {:else}
                      <RefreshCw class="w-4 h-4" />
                      <span>Generar Llave Maestra</span>
                    {/if}
                  </button>
                {/if}
              {:else}
                <div
                  class="space-y-4 animate-in fade-in slide-in-from-bottom-2"
                >
                  <div>
                    <label for="import-pass" class={labelClass}
                      >Contraseña del Archivo (.key)</label
                    >
                    <input
                      id="import-pass"
                      type="password"
                      bind:value={importPassword}
                      placeholder="Ingrese la contraseña de instalación..."
                      class={inputClass}
                    />
                  </div>
                  <button
                    onclick={handleImportKey}
                    disabled={!importPassword || isProcessing}
                    class="w-fit mx-auto px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-accent hover:text-accent text-sm disabled:opacity-50 flex items-center justify-center gap-2"
                  >
                    {#if isProcessing}
                      <RefreshCw class="w-4 h-4 animate-spin" />
                      <span>Verificando...</span>
                    {:else}
                      <Key class="w-4 h-4" />
                      <span>Importar Archivo de Llave</span>
                    {/if}
                  </button>
                </div>
              {/if}
            </div>

            <!-- Danger Zone: Factory Reset -->
            <div class="mt-auto pt-4">
              <button
                type="button"
                onclick={handleFactoryReset}
                class="w-full flex items-center justify-between p-2 rounded-lg text-xs text-secondary hover:text-error hover:bg-error/5 transition-all group"
              >
                <span class="font-medium">Restablecer configuración</span>
                {#if isConfirming}
                  <RefreshCw class="w-3 h-3 animate-spin text-error" />
                {:else}
                  <RefreshCw
                    class="w-3 h-3 text-secondary group-hover:text-error transition-colors"
                  />
                {/if}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="flex-none flex items-center justify-end px-6 py-4 border-t border-surface bg-surface-1 sticky bottom-0 z-20"
      >
        <!-- Right: Actions -->
        <div class="flex items-center gap-3">
          <button
            type="button"
            onclick={exitApp}
            class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
          >
            Cancelar
          </button>

          <button
            type="button"
            onclick={handleSubmit}
            disabled={isSubmitting || !formValid}
            class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 flex items-center gap-2"
          >
            {#if isSubmitting}
              <RefreshCw class="w-4 h-4 animate-spin" />
              <span>Guardando...</span>
            {:else}
              <Check class="w-4 h-4" />
              <span>Finalizar y Acceder</span>
            {/if}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  :global(input:focus),
  :global(button:focus) {
    outline: none !important;
  }
</style>
