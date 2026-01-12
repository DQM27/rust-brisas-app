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
      const status = await getCredentialStatus();
      keyFoundInSystem = status.has_argon2_secret;
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
      if (status.has_argon2_secret) {
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
  class="h-screen w-screen bg-[#0d1117] flex items-center justify-center p-0"
  transition:fade
>
  {#if isResetting}
    <div class="flex flex-col items-center gap-4 text-center" transition:fade>
      <RefreshCw class="w-12 h-12 text-[#2da44e] animate-spin" />
      <div>
        <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
          Reiniciando Sistema...
        </h2>
        <p class="text-sm text-gray-500">
          Limpiando configuración y llaves de seguridad.
        </p>
      </div>
    </div>
  {:else}
    <div
      class="bg-white dark:bg-[#0d1117] w-full h-full overflow-hidden flex flex-col relative"
      transition:fade
    >
      <!-- Close Button (X) -->
      <button
        type="button"
        onclick={exitApp}
        class="absolute top-4 right-4 p-1 text-gray-400 hover:text-red-500 transition-colors z-[60]"
        title="Salir"
      >
        <X class="w-5 h-5" />
      </button>
      <!-- Header (Sin drag para efecto 'bloqueado') -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-b border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 bg-[#2da44e]/10 rounded-lg">
            <Shield class="w-6 h-6 text-[#2da44e]" />
          </div>
          <div class="flex-1">
            <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
              Configuracion Inicial
            </h2>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Configura las credenciales seguras de la aplicacion
            </p>
          </div>
        </div>

        <!-- Progress Steps -->
        <div class="flex items-center gap-2 mt-4">
          {#each [1, 2] as step}
            <div class="flex items-center gap-2 flex-1">
              <div
                class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors {step <
                  currentStep || step === currentStep
                  ? 'bg-[#2da44e] text-white'
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-500'}"
              >
                {#if step < currentStep}
                  <Check class="w-4 h-4" />
                {:else}
                  {step}
                {/if}
              </div>
              {#if step < 2}
                <div
                  class="flex-1 h-1 rounded-full {step < currentStep
                    ? 'bg-[#2da44e]'
                    : 'bg-gray-200 dark:bg-gray-700'}"
                ></div>
              {/if}
            </div>
          {/each}
          <div class="flex justify-between w-full mt-1 px-1">
            <span
              class="text-xs text-gray-500 {currentStep >= 1
                ? 'font-medium text-[#2da44e]'
                : ''}">Terminal</span
            >
            <span
              class="text-xs text-gray-500 {currentStep >= 2
                ? 'font-medium text-[#2da44e]'
                : ''}">Seguridad</span
            >
          </div>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6">
        {#if error}
          <div
            class="mb-4 p-3 rounded-md bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/30 flex items-start gap-2"
          >
            <AlertCircle
              class="w-4 h-4 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5"
            />
            <span class="text-sm text-red-700 dark:text-red-300">{error}</span>
          </div>
        {/if}

        <!-- Step 1: Terminal Config -->
        {#if currentStep === 1}
          <div transition:fade={{ duration: 200 }}>
            <div class="flex items-center gap-2 mb-4">
              <Monitor class="w-5 h-5 text-[#2da44e]" />
              <h3 class="font-semibold text-gray-900 dark:text-gray-100">
                Configuracion de Terminal
              </h3>
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
              Identifica esta terminal para los registros de auditoria y control
              de accesos.
            </p>

            <div class="space-y-4">
              <div>
                <label
                  for="terminalName"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >
                  Nombre de la Terminal
                </label>
                <input
                  id="terminalName"
                  type="text"
                  bind:value={terminalName}
                  placeholder="Ej: Porteria Principal - Acceso 1"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
              </div>
              <div>
                <label
                  for="terminalLocation"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >
                  Ubicacion Fisica
                </label>
                <input
                  id="terminalLocation"
                  type="text"
                  bind:value={terminalLocation}
                  placeholder="Ej: Entrada Norte"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
              </div>
            </div>
          </div>
        {/if}

        <!-- Step 2: Security Mode Selection -->
        {#if currentStep === 2}
          <div transition:fade={{ duration: 200 }} class="space-y-6">
            <div class="flex items-center gap-2 mb-4">
              <Shield class="w-5 h-5 text-[#2da44e]" />
              <h3 class="font-semibold text-gray-900 dark:text-gray-100">
                Seguridad de la Red
              </h3>
            </div>

            {#if keyFoundInSystem}
              <div
                class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 p-4 rounded-lg flex flex-col gap-3"
              >
                <div
                  class="flex items-center gap-2 text-green-700 dark:text-green-300"
                >
                  <Check class="w-5 h-5" />
                  <span class="font-medium">Llave Maestra Configurada</span>
                </div>
                <p class="text-sm text-green-600 dark:text-green-400">
                  Este equipo ya tiene una llave de seguridad válida. Puedes
                  continuar para finalizar la configuración.
                </p>
                <!-- Botón opcional para re-importar si fuera necesario, o simplemente mostrar éxito -->
              </div>
            {:else}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <!-- OPCIÓN A: NUEVA INSTALACIÓN -->
                <button
                  class="flex flex-col items-start p-4 rounded-lg border-2 transition-all text-left
                    {securityMode === 'new'
                    ? 'border-[#2da44e] bg-[#2da44e]/5 dark:bg-[#2da44e]/10'
                    : 'border-gray-200 dark:border-gray-700 hover:border-[#2da44e]/50 hover:bg-gray-50 dark:hover:bg-[#161b22]'}"
                  onclick={() => setSecurityMode("new")}
                >
                  <div
                    class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg mb-3"
                  >
                    <RefreshCw
                      class="w-5 h-5 text-blue-600 dark:text-blue-400"
                    />
                  </div>
                  <h4 class="font-medium text-gray-900 dark:text-gray-100 mb-1">
                    Nueva Instalación
                  </h4>
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    Generar una nueva Llave Maestra. Elige esto si es la
                    **primera computadora** del sistema.
                  </p>
                </button>

                <!-- OPCIÓN B: UNIRSE A FLOTA -->
                <button
                  class="flex flex-col items-start p-4 rounded-lg border-2 transition-all text-left
                    {securityMode === 'join'
                    ? 'border-[#2da44e] bg-[#2da44e]/5 dark:bg-[#2da44e]/10'
                    : 'border-gray-200 dark:border-gray-700 hover:border-[#2da44e]/50 hover:bg-gray-50 dark:hover:bg-[#161b22]'}"
                  onclick={() => setSecurityMode("join")}
                >
                  <div
                    class="p-2 bg-purple-100 dark:bg-purple-900/30 rounded-lg mb-3"
                  >
                    <Key class="w-5 h-5 text-purple-600 dark:text-purple-400" />
                  </div>
                  <h4 class="font-medium text-gray-900 dark:text-gray-100 mb-1">
                    Unirse a Flota
                  </h4>
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    Importar una Llave Maestra existente. Elige esto para
                    **computadoras adicionales**.
                  </p>
                </button>
              </div>

              <!-- DETALLES DE LA ACCIÓN -->
              <div
                class="mt-6 p-4 bg-gray-50 dark:bg-[#161b22] rounded-lg border border-gray-200 dark:border-gray-700"
              >
                {#if securityMode === "new"}
                  <h4 class="text-sm font-medium mb-3">
                    Generación de Llave Maestra
                  </h4>

                  {#if generatedPassword}
                    <div
                      class="mb-4 bg-yellow-50 dark:bg-yellow-900/20 p-3 rounded border border-yellow-200 dark:border-yellow-800"
                    >
                      <p
                        class="text-xs font-bold text-yellow-800 dark:text-yellow-200 mb-1"
                      >
                        ⚠️ IMPORTANTE
                      </p>
                      <p class="text-xs text-yellow-700 dark:text-yellow-300">
                        Guarda este archivo (.megabrisas_master) y la contraseña
                        en un lugar seguro.
                        <br />Los necesitarás para conectar otras computadoras.
                      </p>
                    </div>
                    <div class="mb-4">
                      <label
                        class="block text-xs font-medium text-gray-500 mb-1"
                        >Contraseña de Instalación (IMPORTANTE)</label
                      >
                      <div class="flex gap-2">
                        <code
                          class="flex-1 p-2 bg-white dark:bg-black rounded border border-gray-300 dark:border-gray-600 font-mono text-lg text-center select-all"
                        >
                          {generatedPassword}
                        </code>
                      </div>
                    </div>
                    <div
                      class="text-xs text-green-600 dark:text-green-400 flex items-center gap-2"
                    >
                      <Check class="w-4 h-4" />
                      <span>Llave generada y exportada correctamente.</span>
                    </div>
                  {:else}
                    <div class="flex flex-col gap-3">
                      <p class="text-sm text-gray-600 dark:text-gray-400">
                        Se generará una contraseña aleatoria y se te pedirá
                        guardar el archivo de respaldo.
                      </p>
                      <button
                        onclick={handleGenerateAndExport}
                        disabled={isProcessing}
                        class="self-start px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md text-sm font-medium disabled:opacity-50 flex items-center gap-2"
                      >
                        {#if isProcessing}
                          <RefreshCw class="w-4 h-4 animate-spin" />
                          Generando...
                        {:else}
                          <RefreshCw class="w-4 h-4" />
                          Generar y Exportar Llave
                        {/if}
                      </button>
                    </div>
                  {/if}
                {:else if securityMode === "join"}
                  <h4 class="text-sm font-medium mb-3">Importación de Llave</h4>
                  <div class="space-y-3">
                    <div>
                      <label
                        class="block text-xs font-medium text-gray-500 mb-1"
                        for="import-password">Contraseña de Instalación</label
                      >
                      <input
                        id="import-password"
                        type="password"
                        bind:value={importPassword}
                        placeholder="Ingresa la contraseña del archivo..."
                        class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800"
                      />
                    </div>
                    <button
                      onclick={handleImportKey}
                      disabled={!importPassword || isProcessing}
                      class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-md text-sm font-medium disabled:opacity-50 flex items-center gap-2"
                    >
                      {#if isProcessing}
                        <RefreshCw class="w-4 h-4 animate-spin" />
                        Importando...
                      {:else}
                        <Key class="w-4 h-4" />
                        Seleccionar Archivo .megabrisas_master
                      {/if}
                    </button>
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex items-center justify-between"
      >
        <div class="flex gap-2">
          <button
            type="button"
            onclick={handleFactoryReset}
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] transition-colors"
            title="Borrar todo y empezar de cero"
          >
            <RefreshCw class="w-4 h-4 {isConfirming ? 'animate-spin' : ''}" />
            <span>Reset de Fábrica</span>
          </button>

          <button
            type="button"
            onclick={prevStep}
            disabled={currentStep === 1}
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <ChevronLeft class="w-4 h-4" />
            <span>Anterior</span>
          </button>
        </div>

        {#if currentStep < 2}
          <button
            type="button"
            onclick={nextStep}
            disabled={currentStep === 1 && !step1Valid}
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span>Siguiente</span>
            <ChevronRight class="w-4 h-4" />
          </button>
        {:else}
          <button
            type="button"
            onclick={handleSubmit}
            disabled={isSubmitting || !step2Valid}
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {#if isSubmitting}
              <RefreshCw class="w-4 h-4 animate-spin" />
              <span>Guardando...</span>
            {:else}
              <Check class="w-4 h-4" />
              <span>Completar Configuracion</span>
            {/if}
          </button>
        {/if}
      </div>
    </div>
  {/if}
</div>
