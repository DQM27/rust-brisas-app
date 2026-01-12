<script lang="ts">
  import {
    Shield,
    Key,
    Monitor,
    ChevronRight,
    ChevronLeft,
    Check,
    RefreshCw,
    AlertCircle,
    Eye,
    EyeOff,
    X,
  } from "lucide-svelte";
  import { fade, fly } from "svelte/transition";
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

  // Estado del wizard
  let currentStep = $state(1);
  let isSubmitting = $state(false);
  let isResetting = $state(false);
  let isConfirming = $state(false); // Evitar doble click en reset
  let error = $state("");
  let keyFoundInSystem = $state(false);

  // New Security States
  let securityMode = $state<"new" | "join">("new");
  let isProcessing = $state(false);
  let generatedPassword = $state("");
  let importPassword = $state("");

  // Visibility toggles
  let showArgon2Secret = $state(false);

  let argon2Params = $state<Argon2Params>({
    memory: 19456,
    iterations: 2,
    parallelism: 1,
    secret: "",
  });

  let terminalName = $state("Terminal Principal");
  let terminalLocation = $state("Sin asignar");

  // Validaciones
  let step1Valid = $derived(
    terminalName.trim() !== "" && terminalLocation.trim() !== "",
  );

  let step2Valid = $derived(
    argon2Params.secret.trim() !== "" && argon2Params.secret.trim() !== "",
  );

  // Funciones
  async function generateSecret() {
    try {
      argon2Params.secret = await generateRandomSecret();
    } catch (e) {
      error = `Error generando secret: ${e}`;
    }
  }

  async function checkSystemKey() {
    try {
      console.log("🔍 Checking system key credentials...");
      const status = await getCredentialStatus();
      console.log("🔍 Credential Status:", status);
      keyFoundInSystem = status.argon2_configured;
      console.log("🔍 keyFoundInSystem set to:", keyFoundInSystem);
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
      // 1. Generar Secret
      const secret = await generateRandomSecret();
      argon2Params.secret = secret;

      // 2. Guardar en Keyring Local
      await updateArgon2Params(argon2Params);

      // 3. Generar Password de Instalación
      const installPass = generateInstallPassword();

      // 4. Solicitar dónde guardar
      const filePath = await save({
        title: "Guardar Llave Maestra de Seguridad",
        defaultPath: "megabrisas_master.key",
        filters: [{ name: "Key Files", extensions: ["key"] }],
      });

      if (!filePath) {
        isProcessing = false;
        return; // Cancelado por usuario
      }

      // 5. Exportar
      await exportMasterKey(filePath, installPass);

      // 6. Éxito
      generatedPassword = installPass;
      keyFoundInSystem = true;
    } catch (e: any) {
      error = String(e);
      console.error(e);
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
      // 1. Seleccionar archivo
      const filePath = await open({
        title: "Seleccionar Llave Maestra (.megabrisas_master)",
        filters: [{ name: "Key Files", extensions: ["key"] }],
        multiple: false,
      });

      if (!filePath) {
        isProcessing = false;
        return;
      }

      // 2. Importar
      await importMasterKey(filePath as string, importPassword);

      // 3. Éxito
      keyFoundInSystem = true;
      // Cargar params actualizados para el estado local si fuera necesario
      const status = await getCredentialStatus();
      if (status.argon2_configured) {
        argon2Params.secret = "********"; // Ya configurado
      }
    } catch (e: any) {
      error = String(e);
      console.error(e);
      await message(`Error importando llave: ${e}`, {
        title: "Error de Importación",
        kind: "error",
      });
    } finally {
      isProcessing = false;
    }
  }

  function nextStep() {
    if (currentStep < 2) {
      currentStep++;
      error = "";
      if (currentStep === 2) {
        checkSystemKey();
      }
    }
  }

  function prevStep() {
    if (currentStep > 1) {
      currentStep--;
      error = "";
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
    } catch (e) {
      console.error("Error detallado:", e);
      let errorDetail = "";
      if (typeof e === "string") {
        errorDetail = e;
      } else if (e instanceof Error) {
        errorDetail = e.message;
      } else {
        try {
          errorDetail = JSON.stringify(e);
        } catch {
          errorDetail = String(e);
        }
      }
      error = `Error guardando config: ${errorDetail}`;
    } finally {
      isSubmitting = false;
    }
  }

  async function handleFactoryReset() {
    if (isConfirming || isResetting) return;

    isConfirming = true;
    try {
      const confirmed = await ask(
        "¿Estás seguro? Esto borrará TODA la configuración, el nombre de la terminal y las llaves de seguridad de Windows.",
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
        // Pequeño delay para feedback visual
        setTimeout(() => {
          window.location.reload();
        }, 800);
      }
    } finally {
      isConfirming = false;
    }
  }

  async function minimizeWindow() {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().minimize();
  }
</script>

<div
  class="h-screen w-screen bg-[#0d1117] flex items-center justify-center p-4"
  transition:fade
>
  {#if isResetting}
    <div class="flex flex-col items-center gap-4 text-center" transition:fade>
      <RefreshCw class="w-12 h-12 text-[#2da44e] animate-spin" />
      <div>
        <h2 class="text-xl font-bold text-gray-100">Reiniciando Sistema...</h2>
        <p class="text-sm text-gray-500">
          Limpiando configuración y llaves de seguridad.
        </p>
      </div>
    </div>
  {:else}
    <!-- Main Card Container matching UserFormModal dimensions/style -->
    <div
      class="w-full max-w-[800px] bg-[#161b22] shadow-2xl border border-gray-800 rounded-xl overflow-hidden flex flex-col max-h-[90vh]"
      transition:fade
    >
      <!-- Header -->
      <div
        class="flex-none bg-[#0d1117] px-6 py-4 border-b border-gray-800 flex items-center justify-between"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 bg-[#2da44e]/10 rounded-lg">
            <Shield class="w-6 h-6 text-[#2da44e]" />
          </div>
          <div>
            <h2 class="text-lg font-semibold text-gray-100">
              Configuracion Inicial
            </h2>
            <p class="text-xs text-gray-400">
              Configura las credenciales seguras de la aplicacion
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={exitApp}
          class="p-1.5 text-gray-400 hover:text-red-400 hover:bg-gray-800 rounded-lg transition-colors"
          title="Salir"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Progress Indicators -->
      <div class="bg-[#0d1117] px-6 pb-4 border-b border-gray-800">
        <div class="flex items-center gap-2">
          {#each [1, 2] as step}
            <div class="flex items-center gap-2 flex-1">
              <div
                class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors {step <
                  currentStep || step === currentStep
                  ? 'bg-[#2da44e] text-white'
                  : 'bg-gray-800 text-gray-500 border border-gray-700'}"
              >
                {#if step < currentStep}
                  <Check class="w-4 h-4" />
                {:else}
                  {step}
                {/if}
              </div>
              <div class="flex flex-col">
                <span
                  class="text-xs font-medium {step === currentStep
                    ? 'text-[#2da44e]'
                    : 'text-gray-500'}"
                >
                  {step === 1 ? "Terminal" : "Seguridad"}
                </span>
              </div>
              {#if step < 2}
                <div
                  class="flex-1 h-0.5 rounded-full {step < currentStep
                    ? 'bg-[#2da44e]'
                    : 'bg-gray-800'}"
                ></div>
              {/if}
            </div>
          {/each}
        </div>
      </div>

      <!-- Content Area (Scrollable) -->
      <div class="flex-1 overflow-y-auto p-6 bg-[#0d1117] space-y-4">
        {#if error}
          <div
            class="mb-4 p-3 rounded-md bg-red-900/20 border border-red-800 flex items-start gap-2"
          >
            <AlertCircle class="w-4 h-4 text-red-500 flex-shrink-0 mt-0.5" />
            <span class="text-sm text-red-200">{error}</span>
          </div>
        {/if}

        <!-- Step 1: Terminal Config -->
        {#if currentStep === 1}
          <div transition:fade={{ duration: 200 }}>
            <div class="flex items-center gap-2 mb-4">
              <Monitor class="w-5 h-5 text-[#2da44e]" />
              <h3 class="font-semibold text-gray-100">
                Configuracion de Terminal
              </h3>
            </div>
            <p class="text-sm text-gray-400 mb-6">
              Identifica esta terminal para los registros de auditoria y control
              de accesos.
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label
                  for="terminalName"
                  class="block text-xs font-medium text-gray-400 mb-1"
                >
                  Nombre de la Terminal
                </label>
                <input
                  id="terminalName"
                  type="text"
                  bind:value={terminalName}
                  placeholder="Ej: Porteria Principal"
                  class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-600 focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 transition-all"
                />
              </div>
              <div>
                <label
                  for="terminalLocation"
                  class="block text-xs font-medium text-gray-400 mb-1"
                >
                  Ubicacion Fisica
                </label>
                <input
                  id="terminalLocation"
                  type="text"
                  bind:value={terminalLocation}
                  placeholder="Ej: Acceso Vehicular"
                  class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-600 focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 transition-all"
                />
              </div>
            </div>
          </div>
        {/if}

        <!-- Step 2: Security Mode Selection -->
        {#if currentStep === 2}
          <div transition:fade={{ duration: 200 }} class="space-y-6">
            {#if keyFoundInSystem && !generatedPassword}
              <!-- SUCCESS BANNER -->
              <div
                class="bg-green-900/10 border border-green-800/50 p-4 rounded-lg flex items-start gap-4 mb-4"
              >
                <div class="p-2 bg-green-500/10 rounded-full flex-shrink-0">
                  <Check class="w-5 h-5 text-green-400" />
                </div>
                <div class="flex-1">
                  <h3 class="text-sm font-medium text-green-400">
                    Sistema Seguro Configurado
                  </h3>
                  <p class="text-xs text-green-300/70 mt-1 mb-2">
                    Este equipo ya tiene una llave maestra válida. Puedes hacer
                    clic en <strong>Finalizar</strong> para usarla, o seleccionar
                    una opción abajo para generar/importar una nueva (sobrescribir).
                  </p>
                </div>
              </div>
            {/if}

            <!-- SELECTION CARDS (Always visible now) -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <!-- OPCIÓN A: NUEVA INSTALACIÓN -->
              <button
                class="group flex flex-row items-center gap-4 p-4 rounded-lg border-2 transition-all text-left h-full
                    {securityMode === 'new'
                  ? 'border-[#2da44e] bg-[#2da44e]/10'
                  : 'border-white/10 hover:border-[#2da44e]/50 hover:bg-white/5'}"
                onclick={() => setSecurityMode("new")}
              >
                <div
                  class="p-3 bg-blue-500/10 rounded-lg group-hover:scale-110 transition-transform"
                >
                  <RefreshCw class="w-6 h-6 text-blue-400" />
                </div>
                <div>
                  <h4 class="font-medium text-gray-100">Nueva Instalación</h4>
                  <p class="text-xs text-gray-400 mt-1">
                    Primera computadora del sistema
                  </p>
                </div>
              </button>

              <!-- OPCIÓN B: UNIRSE A FLOTA -->
              <button
                class="group flex flex-row items-center gap-4 p-4 rounded-lg border-2 transition-all text-left h-full
                    {securityMode === 'join'
                  ? 'border-[#2da44e] bg-[#2da44e]/10'
                  : 'border-white/10 hover:border-[#2da44e]/50 hover:bg-white/5'}"
                onclick={() => setSecurityMode("join")}
              >
                <div
                  class="p-3 bg-purple-500/10 rounded-lg group-hover:scale-110 transition-transform"
                >
                  <Key class="w-6 h-6 text-purple-400" />
                </div>
                <div>
                  <h4 class="font-medium text-gray-100">Unirse a Flota</h4>
                  <p class="text-xs text-gray-400 mt-1">
                    Computadora adicional
                  </p>
                </div>
              </button>
            </div>

            <!-- ACTION AREA -->
            <div class="animate-in slide-in-from-top-4 fade-in duration-300">
              <div
                class="p-5 bg-[#161b22] rounded-lg border border-white/10 shadow-inner"
              >
                {#if securityMode === "new"}
                  <div class="flex items-center justify-between mb-4">
                    <h4 class="text-sm font-medium text-gray-200">
                      Generar Llave Maestra
                    </h4>
                  </div>

                  {#if generatedPassword}
                    <!-- RESULT DISPLAY -->
                    <div class="space-y-4">
                      <div
                        class="bg-yellow-900/20 border border-yellow-700/50 p-3 rounded-lg flex gap-3"
                      >
                        <AlertCircle
                          class="w-5 h-5 text-yellow-500 flex-shrink-0"
                        />
                        <div class="text-xs text-yellow-200/90">
                          <strong class="block mb-1">IMPORTANTE</strong>
                          Guarda el archivo
                          <code class="bg-black/30 px-1 rounded"
                            >.megabrisas_master</code
                          > y la contraseña mostrada abajo. Sin ellos no podrás conectar
                          más equipos.
                        </div>
                      </div>

                      <div>
                        <label
                          class="block text-xs font-medium text-gray-500 mb-1"
                          >Contraseña de Instalación</label
                        >
                        <div class="relative">
                          <code
                            class="block w-full text-center p-3 bg-black/40 border border-white/10 rounded-lg font-mono text-xl tracking-wider text-[#2da44e] select-all"
                          >
                            {generatedPassword}
                          </code>
                        </div>
                      </div>
                    </div>
                  {:else}
                    <!-- INITIAL ACTION -->
                    <p class="text-sm text-gray-400 mb-4">
                      Se generará una contraseña aleatoria segura y se exportará
                      a un archivo encriptado.
                    </p>
                    <button
                      onclick={handleGenerateAndExport}
                      disabled={isProcessing}
                      class="px-5 py-2.5 bg-blue-600 hover:bg-blue-500 text-white rounded-lg text-sm font-medium disabled:opacity-50 flex items-center gap-2 transition-all"
                    >
                      {#if isProcessing}
                        <RefreshCw class="w-4 h-4 animate-spin" />
                        <span>Generando y Cifrando...</span>
                      {:else}
                        <RefreshCw class="w-4 h-4" />
                        <span>Generar y Exportar Llave</span>
                      {/if}
                    </button>
                  {/if}
                {:else if securityMode === "join"}
                  <div class="flex items-center justify-between mb-4">
                    <h4 class="text-sm font-medium text-gray-200">
                      Importar Llave Existente
                    </h4>
                  </div>
                  <div class="space-y-4 max-w-md">
                    <div>
                      <label
                        class="block text-xs font-medium text-gray-400 mb-1"
                        for="import-pass"
                      >
                        Contraseña del Archivo
                      </label>
                      <input
                        id="import-pass"
                        type="password"
                        bind:value={importPassword}
                        placeholder="ABCD-EFGH-..."
                        class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white focus:outline-none focus:border-purple-500/50 focus:ring-1 focus:ring-purple-500/20 transition-all font-mono"
                      />
                    </div>
                    <button
                      onclick={handleImportKey}
                      disabled={!importPassword || isProcessing}
                      class="w-full px-5 py-2.5 bg-purple-600 hover:bg-purple-500 text-white rounded-lg text-sm font-medium disabled:opacity-50 flex items-center justify-center gap-2 transition-all"
                    >
                      {#if isProcessing}
                        <RefreshCw class="w-4 h-4 animate-spin" />
                        <span>Descifrando...</span>
                      {:else}
                        <Key class="w-4 h-4" />
                        <span>Seleccionar Archivo .megabrisas_master</span>
                      {/if}
                    </button>
                  </div>
                {/if}
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="flex-none bg-[#161b22] px-6 py-4 border-t border-gray-800 flex items-center justify-between"
      >
        <div class="flex gap-3">
          <button
            type="button"
            onclick={handleFactoryReset}
            class="px-4 py-2 text-xs font-medium text-gray-500 hover:text-red-400 transition-colors flex items-center gap-2"
            title="Borrar todo"
          >
            {#if isConfirming}
              <RefreshCw class="w-3 h-3 animate-spin" />
            {:else}
              <RefreshCw class="w-3 h-3" />
            {/if}
            <span>Reset</span>
          </button>

          <button
            type="button"
            onclick={prevStep}
            disabled={currentStep === 1}
            class="px-4 py-2 text-sm font-medium rounded-lg border-2 border-white/5 text-gray-400 hover:text-white hover:border-white/20 disabled:opacity-30 disabled:cursor-not-allowed transition-all"
          >
            Anterior
          </button>
        </div>

        <div>
          {#if currentStep < 2}
            <button
              type="button"
              onclick={nextStep}
              disabled={currentStep === 1 && !step1Valid}
              class="px-6 py-2 text-sm font-medium rounded-lg bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-green-900/20 transition-all"
            >
              Siguiente
            </button>
          {:else}
            <button
              type="button"
              onclick={handleSubmit}
              disabled={isSubmitting || !step2Valid}
              class="px-6 py-2 text-sm font-medium rounded-lg bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-green-900/20 flex items-center gap-2 transition-all"
            >
              {#if isSubmitting}
                <RefreshCw class="w-4 h-4 animate-spin" />
                <span>Guardando...</span>
              {:else}
                <Check class="w-4 h-4" />
                <span>Finalizar</span>
              {/if}
            </button>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
