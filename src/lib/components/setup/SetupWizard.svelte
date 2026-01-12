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
  import { fade, fly, scale } from "svelte/transition";
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
    setWindowSize,
    generateRecoveryFragments,
    recoverFromFragments,
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
  let recoveryFragments = $state<string[]>([]);
  let isGeneratingFragments = $state(false);

  // Security Mode
  let securityMode = $state<"new" | "join" | "recovery">("new");
  let isProcessing = $state(false);
  let generatedPassword = $state("");
  let setupPassword = $state("");
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
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-xs text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all";
  const labelClass = "block text-[11px] font-medium text-secondary mb-1 ml-1";
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

  function setSecurityMode(mode: "new" | "join" | "recovery") {
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
      const installPass = setupPassword || generateInstallPassword();
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

  async function handleGenerateFragments() {
    isGeneratingFragments = true;
    try {
      const fragments = await generateRecoveryFragments();
      recoveryFragments = fragments;

      await message(
        "Se han generado 5 fragmentos. Procede a guardarlos en lugares seguros y diferentes.",
        { title: "Fragmentos Generados", kind: "info" },
      );

      for (let i = 0; i < fragments.length; i++) {
        const filePath = await save({
          title: `Guardar Fragmento de Recuperación ${i + 1}/5`,
          defaultPath: `brisas_recovery_fragment_${i + 1}.key`,
          filters: [{ name: "Key Files", extensions: ["key"] }],
        });

        if (filePath) {
          const { writeTextFile } = await import("@tauri-apps/plugin-fs");
          await writeTextFile(filePath, fragments[i]);
        }
      }
    } catch (e: any) {
      error = String(e);
      await message(`Error generando fragmentos: ${e}`, {
        title: "Error de Seguridad",
        kind: "error",
      });
    } finally {
      isGeneratingFragments = false;
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

  async function handleRecoverFromFragments() {
    isProcessing = true;
    error = "";
    try {
      const paths = await open({
        title: "Seleccionar al menos 3 fragmentos (.key)",
        filters: [{ name: "Key Files", extensions: ["key"] }],
        multiple: true,
      });

      if (!paths || !Array.isArray(paths) || paths.length < 3) {
        if (paths) {
          await message(
            "Se requieren al menos 3 fragmentos para la recuperación.",
            {
              title: "Fragmentos Insuficientes",
              kind: "warning",
            },
          );
        }
        isProcessing = false;
        return;
      }

      const { readTextFile } = await import("@tauri-apps/plugin-fs");
      const fragments: string[] = [];

      for (const path of paths) {
        const content = await readTextFile(path);
        fragments.push(content);
      }

      await recoverFromFragments(fragments);
      keyFoundInSystem = true;
      keyImported = true;
      const status = await getCredentialStatus();
      if (status.argon2_configured) {
        argon2Params.secret = "********";
      }

      await message(
        "Sistema recuperado exitosamente usando fragmentos de Shamir.",
        {
          title: "Recuperación Exitosa",
          kind: "info",
        },
      );
    } catch (e: any) {
      error = String(e);
      await message(`Error en recuperación: ${e}`, {
        title: "Error de Recuperación",
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
  class="h-screen w-screen bg-surface-1 flex flex-col items-center justify-center p-0 overflow-hidden relative"
  transition:fade
>
  <!-- No backdrop required as we occupy the whole window -->

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
    <!-- Container filling the exact window size - Stable h-full -->
    <div
      class="relative z-10 w-full h-full flex flex-col bg-surface-1 overflow-hidden"
    >
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

      <!-- Content Area (Stable) -->
      <div
        class="px-6 py-4 flex-1 flex flex-col justify-between overflow-hidden"
      >
        {#if error}
          <div
            class="mb-4 p-3 rounded-lg bg-error/10 border border-error/20 flex items-start gap-3"
          >
            <AlertCircle class="w-5 h-5 text-error flex-shrink-0 mt-0.5" />
            <span class="text-sm text-error/90 font-medium">{error}</span>
          </div>
        {/if}

        <!-- Main Card -->
        <div
          class="bg-black/20 border border-white/5 rounded-xl p-4 flex flex-col gap-4"
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
          <div class="flex flex-col min-h-0">
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
            <div class="grid grid-cols-3 gap-2 mb-4 flex-none">
              <button
                onclick={() => setSecurityMode("new")}
                class="px-2 py-1.5 rounded-lg border transition-all flex items-center justify-center {securityMode ===
                'new'
                  ? 'bg-white/10 border-white/20 text-white'
                  : 'bg-surface-3 border-surface text-secondary hover:bg-surface-2 hover:text-white'}"
              >
                <span class="font-bold text-[10px] uppercase tracking-wider"
                  >Nueva Key</span
                >
              </button>
              <button
                onclick={() => setSecurityMode("join")}
                class="px-2 py-1.5 rounded-lg border transition-all flex items-center justify-center {securityMode ===
                'join'
                  ? 'bg-white/10 border-white/20 text-white'
                  : 'bg-surface-3 border-surface text-secondary hover:bg-surface-2 hover:text-white'}"
              >
                <span class="font-bold text-[10px] uppercase tracking-wider"
                  >Key Existente</span
                >
              </button>
              <button
                onclick={() => setSecurityMode("recovery")}
                class="px-2 py-1.5 rounded-lg border transition-all flex items-center justify-center {securityMode ===
                'recovery'
                  ? 'bg-white/10 border-white/20 text-white'
                  : 'bg-surface-3 border-surface text-secondary hover:bg-surface-2 hover:text-white'}"
              >
                <span class="font-bold text-[10px] uppercase tracking-wider"
                  >Recuperar</span
                >
              </button>
            </div>

            <!-- Action Area -->
            <div class="flex-none">
              {#if securityMode === "new"}
                {#if generatedPassword}
                  <div
                    class="space-y-3 animate-in fade-in slide-in-from-bottom-2"
                  >
                    {#if !setupPassword}
                      <div
                        class="bg-warning/10 border border-warning/20 px-3 py-2 rounded-lg flex items-center justify-between gap-2"
                      >
                        <div class="flex items-center gap-2">
                          <Key size={14} class="text-warning" />
                          <code
                            class="text-base font-mono text-warning font-bold tracking-widest selection:bg-warning/20"
                          >
                            {generatedPassword}
                          </code>
                        </div>
                        <span
                          class="text-[9px] text-warning/70 font-medium uppercase border-l border-warning/20 pl-2"
                        >
                          Guarda esta llave
                        </span>
                      </div>
                    {:else}
                      <div
                        class="bg-primary/10 border border-primary/20 p-3 rounded-xl flex items-center gap-3"
                      >
                        <div class="p-2 bg-primary/20 rounded-full">
                          <Check class="w-4 h-4 text-primary" />
                        </div>
                        <div>
                          <p class="text-sm font-bold text-white">
                            Llave Maestra Lista
                          </p>
                          <p class="text-xs text-secondary">
                            Se ha generado y guardado el archivo de respaldo.
                          </p>
                        </div>
                      </div>
                    {/if}

                    <div class="flex items-center justify-between gap-4 pt-1">
                      <p
                        class="text-[10px] text-secondary flex-1 italic leading-tight"
                      >
                        Se recomienda generar fragmentos por si pierdes la
                        llave.
                      </p>
                      <button
                        onclick={handleGenerateFragments}
                        disabled={isGeneratingFragments}
                        class="shrink-0 px-2.5 py-1.5 rounded-lg border border-accent/40 text-accent text-[10px] font-bold hover:bg-accent/10 transition-all flex items-center gap-2 bg-accent/5"
                      >
                        {#if isGeneratingFragments}
                          <RefreshCw class="w-3 h-3 animate-spin" />
                          <span>Generando...</span>
                        {:else}
                          <Shield class="w-3 h-3" />
                          <span>Generar Fragmentos</span>
                        {/if}
                      </button>
                    </div>
                  </div>
                {:else}
                  <div class="space-y-4 mb-4">
                    <div>
                      <label for="setup-pass" class={labelClass}
                        >Contraseña de Respaldo (Opcional)</label
                      >
                      <div class="flex gap-2">
                        <input
                          id="setup-pass"
                          type="text"
                          bind:value={setupPassword}
                          placeholder="Ingresa una contraseña o genera una..."
                          class={inputClass}
                        />
                        <button
                          type="button"
                          onclick={() =>
                            (setupPassword = generateInstallPassword())}
                          class="p-1.5 bg-surface-3 border border-surface rounded-lg hover:bg-surface-2 transition-colors"
                          title="Sugerir Contraseña"
                        >
                          <RefreshCw class="w-3.5 h-3.5 text-secondary" />
                        </button>
                      </div>
                      <p class="text-[10px] text-gray-400 mt-1 ml-1">
                        Esta contraseña protege tu archivo .key de respaldo.
                      </p>
                    </div>
                  </div>
                  <button
                    onclick={handleGenerateAndExport}
                    disabled={isProcessing}
                    class="w-fit mx-auto px-4 py-1.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-primary hover:text-primary text-[11px] disabled:opacity-50 flex items-center justify-center gap-2"
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
              {:else if securityMode === "join"}
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
                    class="w-fit mx-auto px-4 py-1.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-accent hover:text-accent text-[11px] disabled:opacity-50 flex items-center justify-center gap-2"
                  >
                    {#if isProcessing}
                      <RefreshCw class="w-3.5 h-3.5 animate-spin" />
                      <span>Verificando...</span>
                    {:else}
                      <Key class="w-3.5 h-3.5" />
                      <span>Importar Archivo de Llave</span>
                    {/if}
                  </button>
                </div>
              {:else if securityMode === "recovery"}
                <div
                  class="space-y-4 animate-in fade-in slide-in-from-bottom-2 text-center"
                >
                  <p class="text-xs text-secondary px-6">
                    Selecciona al menos 3 de los 5 fragmentos generados
                    anteriormente para reconstruir la llave maestra.
                  </p>
                  <button
                    onclick={handleRecoverFromFragments}
                    disabled={isProcessing}
                    class="w-fit mx-auto px-4 py-1.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-[11px] disabled:opacity-50 flex items-center justify-center gap-2"
                  >
                    {#if isProcessing}
                      <RefreshCw class="w-4 h-4 animate-spin" />
                      <span>Recuperando...</span>
                    {:else}
                      <Shield class="w-4 h-4" />
                      <span>Seleccionar Fragmentos</span>
                    {/if}
                  </button>
                </div>
              {/if}
            </div>

            <!-- Danger Zone: Factory Reset -->
            <div class="pt-4 mt-2">
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

      <!-- Footer (Fitted) -->
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
