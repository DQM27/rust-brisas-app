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
  import { ask } from "@tauri-apps/plugin-dialog";
  import {
    setupCredentials,
    generateRandomSecret,
    exitApp,
    resetAllCredentials,
    getCredentialStatus,
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
      keyFoundInSystem = status.argon2_configured;
      if (keyFoundInSystem && argon2Params.secret === "") {
        argon2Params.secret = "********"; // Placeholder visual
      }
    } catch (e) {
      console.error("Error verificando llaves:", e);
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

        <!-- Step 2: Argon2 (Final) -->
        {#if currentStep === 2}
          <div transition:fade={{ duration: 200 }}>
            <div class="flex items-center gap-2 mb-4">
              <Key class="w-5 h-5 text-[#2da44e]" />
              <h3 class="font-semibold text-gray-900 dark:text-gray-100">
                Parametros de Seguridad
              </h3>
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
              Configura los parametros de Argon2 para el hasheo seguro de
              contrasenas.
            </p>

            <div class="space-y-4">
              <div>
                <label
                  for="argon2Secret"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >
                  Secret (Pepper)
                </label>
                {#if keyFoundInSystem}
                  <div
                    class="mb-4 flex items-center gap-2 p-2 rounded-md bg-blue-50 dark:bg-blue-900/20 text-xs text-blue-700 dark:text-blue-300"
                  >
                    <Check class="w-4 h-4" />
                    <span class="font-medium">Llave detectada en Windows.</span>
                    <span class="opacity-80">Se usará la existente.</span>
                  </div>
                {:else}
                  <div class="flex gap-2">
                    <div class="relative flex-1">
                      <input
                        id="argon2Secret"
                        type={showArgon2Secret ? "text" : "password"}
                        bind:value={argon2Params.secret}
                        placeholder="Secret para hasheo de contraseñas"
                        class="w-full px-3 py-2 pr-10 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent font-mono"
                      />
                      <button
                        type="button"
                        onclick={() => (showArgon2Secret = !showArgon2Secret)}
                        class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-gray-400 hover:text-gray-600"
                      >
                        {#if showArgon2Secret}
                          <EyeOff class="w-4 h-4" />
                        {:else}
                          <Eye class="w-4 h-4" />
                        {/if}
                      </button>
                    </div>
                    <button
                      type="button"
                      onclick={generateSecret}
                      class="px-3 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d]"
                    >
                      Generar
                    </button>
                  </div>
                  <p class="mt-1 text-xs text-gray-500">
                    Este secret se usa como "pepper" adicional al salt. Guardalo
                    de forma segura.
                  </p>
                {/if}
              </div>

              <!-- Parameters removed (Hardcoded in backend) -->
              <input type="hidden" bind:value={argon2Params.memory} />
              <input type="hidden" bind:value={argon2Params.iterations} />
              <input type="hidden" bind:value={argon2Params.parallelism} />
            </div>
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
            disabled={isSubmitting}
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50"
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
