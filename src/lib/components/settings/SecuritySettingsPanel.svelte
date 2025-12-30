<script lang="ts">
  import {
    Shield,
    Key,
    RefreshCw,
    Check,
    AlertCircle,
    Eye,
    EyeOff,
    AlertTriangle,
    Save,
  } from "lucide-svelte";
  import { setupWizardVisible } from "$lib/stores/ui";
  import { scale } from "svelte/transition";
  import { onMount } from "svelte";
  import {
    getCredentialStatus,
    getArgon2Config,
    updateArgon2Params,
    generateRandomSecret,
    type CredentialStatus,
    type Argon2ParamsSafe,
    type Argon2Params,
  } from "$lib/services/keyringService";
  import { can } from "$lib/logic/permissions";
  import { currentUser } from "$lib/stores/auth";

  // Permisos
  const canUpdate = $derived(
    $currentUser && can($currentUser, "UPDATE_SETTINGS_SECURITY"),
  );

  // Estado
  let status = $state<CredentialStatus | null>(null);
  let argon2Config = $state<Argon2ParamsSafe | null>(null);

  let loading = $state(true);
  let error = $state("");
  let successMessage = $state("");

  // Formularios
  let editingArgon2 = $state(false);

  // Argon2 Form
  let argon2Form = $state<Argon2Params>({
    memory: 19456,
    iterations: 2,
    parallelism: 1,
    secret: "",
  });
  let showArgon2Secret = $state(false);
  let savingArgon2 = $state(false);

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    error = "";
    try {
      [status, argon2Config] = await Promise.all([
        getCredentialStatus(),
        getArgon2Config(),
      ]);

      if (argon2Config) {
        argon2Form = {
          memory: argon2Config.memory,
          iterations: argon2Config.iterations,
          parallelism: argon2Config.parallelism,
          secret: "",
        };
      }
    } catch (e) {
      error = `Error cargando configuracion: ${e}`;
    } finally {
      loading = false;
    }
  }

  function showSuccess(msg: string) {
    successMessage = msg;
    setTimeout(() => (successMessage = ""), 3000);
  }

  // Argon2 Functions
  async function handleGenerateSecret() {
    try {
      argon2Form.secret = await generateRandomSecret();
    } catch (e) {
      error = `Error generando secret: ${e}`;
    }
  }

  async function handleSaveArgon2() {
    if (!argon2Form.secret) {
      error = "El secret de Argon2 es obligatorio";
      return;
    }
    savingArgon2 = true;
    error = "";
    try {
      await updateArgon2Params(argon2Form);
      editingArgon2 = false;
      showSuccess("Parametros de Argon2 actualizados");
      await loadData();
    } catch (e) {
      error = `Error guardando Argon2: ${e}`;
    } finally {
      savingArgon2 = false;
    }
  }
</script>

<div
  class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="max-w-2xl space-y-6">
    <!-- Header -->
    <div>
      <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
        Seguridad y Credenciales
      </h2>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
        Gestiona las credenciales almacenadas de forma segura en el sistema.
      </p>
    </div>

    {#if loading}
      <div class="flex items-center justify-center py-12">
        <RefreshCw class="w-6 h-6 animate-spin text-gray-400" />
      </div>
    {:else}
      {#if error}
        <div
          class="p-3 rounded-md bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/30 flex items-start gap-2"
        >
          <AlertCircle
            class="w-4 h-4 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5"
          />
          <span class="text-sm text-red-700 dark:text-red-300">{error}</span>
        </div>
      {/if}

      {#if successMessage}
        <div
          class="p-3 rounded-md bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800/30 flex items-start gap-2"
        >
          <Check
            class="w-4 h-4 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5"
          />
          <span class="text-sm text-green-700 dark:text-green-300"
            >{successMessage}</span
          >
        </div>
      {/if}

      <!-- Status Overview -->
      {#if status}
        <div
          class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden"
        >
          <div
            class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2"
          >
            <Shield class="w-4 h-4 text-gray-500" />
            <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
              Estado de Configuracion
            </h3>
          </div>
          <div class="p-4">
            <div class="grid grid-cols-3 gap-4">
              <div class="flex items-center gap-2">
                {#if status.argon2_configured}
                  <Check class="w-4 h-4 text-green-500" />
                {:else}
                  <AlertCircle class="w-4 h-4 text-yellow-500" />
                {/if}
                <span class="text-sm text-gray-600 dark:text-gray-300"
                  >Argon2</span
                >
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Argon2 Card -->
      <div
        class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden"
      >
        <div
          class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between"
        >
          <div class="flex items-center gap-2">
            <Key class="w-4 h-4 text-gray-500" />
            <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
              Parametros Argon2
            </h3>
          </div>
          {#if !editingArgon2 && canUpdate}
            <button
              onclick={() => (editingArgon2 = true)}
              class="text-sm text-[#2da44e] hover:underline"
            >
              Editar
            </button>
          {/if}
        </div>
        <div class="p-4">
          {#if editingArgon2}
            <!-- Warning -->
            <div
              class="mb-4 p-3 rounded-md bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800/30 flex items-start gap-2"
            >
              <AlertTriangle
                class="w-4 h-4 text-yellow-600 dark:text-yellow-500 flex-shrink-0 mt-0.5"
              />
              <span class="text-sm text-yellow-800 dark:text-yellow-200">
                Cambiar estos parametros puede invalidar las contrasenas
                existentes.
              </span>
            </div>
            <div class="space-y-4">
              <div>
                <label
                  for="argon2EditSecret"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                  >Nuevo Secret</label
                >
                <div class="flex gap-2">
                  <div class="relative flex-1">
                    <input
                      id="argon2EditSecret"
                      type={showArgon2Secret ? "text" : "password"}
                      bind:value={argon2Form.secret}
                      placeholder="Ingresa nuevo secret"
                      class="w-full px-3 py-2 pr-10 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 font-mono"
                    />
                    <button
                      type="button"
                      onclick={() => (showArgon2Secret = !showArgon2Secret)}
                      class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-gray-400 hover:text-gray-600"
                    >
                      {#if showArgon2Secret}<EyeOff
                          class="w-4 h-4"
                        />{:else}<Eye class="w-4 h-4" />{/if}
                    </button>
                  </div>
                  <button
                    onclick={handleGenerateSecret}
                    class="px-3 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d]"
                  >
                    Generar
                  </button>
                </div>
              </div>
              <div class="grid grid-cols-3 gap-4">
                <div>
                  <label
                    for="argon2EditMemory"
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                    >Memoria (KB)</label
                  >
                  <input
                    id="argon2EditMemory"
                    type="number"
                    bind:value={argon2Form.memory}
                    min="1024"
                    class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100"
                  />
                </div>
                <div>
                  <label
                    for="argon2EditIterations"
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                    >Iteraciones</label
                  >
                  <input
                    id="argon2EditIterations"
                    type="number"
                    bind:value={argon2Form.iterations}
                    min="1"
                    class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100"
                  />
                </div>
                <div>
                  <label
                    for="argon2EditParallelism"
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                    >Paralelismo</label
                  >
                  <input
                    id="argon2EditParallelism"
                    type="number"
                    bind:value={argon2Form.parallelism}
                    min="1"
                    class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100"
                  />
                </div>
              </div>
              <div class="flex items-center gap-2 pt-2">
                <button
                  onclick={handleSaveArgon2}
                  disabled={savingArgon2}
                  class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50"
                >
                  {#if savingArgon2}<RefreshCw
                      class="w-4 h-4 animate-spin"
                    />{:else}<Save class="w-4 h-4" />{/if}
                  <span>Guardar</span>
                </button>
                <button
                  onclick={() => {
                    editingArgon2 = false;
                    error = "";
                  }}
                  class="px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d]"
                >
                  Cancelar
                </button>
              </div>
            </div>
          {:else if argon2Config}
            <div class="space-y-2 text-sm">
              <p class="text-gray-600 dark:text-gray-300">
                <span class="font-medium">Memoria:</span>
                {argon2Config.memory} KB
              </p>
              <p class="text-gray-600 dark:text-gray-300">
                <span class="font-medium">Iteraciones:</span>
                {argon2Config.iterations}
              </p>
              <p class="text-gray-600 dark:text-gray-300">
                <span class="font-medium">Paralelismo:</span>
                {argon2Config.parallelism}
              </p>
              <p class="text-gray-600 dark:text-gray-300">
                <span class="font-medium">Secret:</span>
                {#if argon2Config.has_secret}
                  <span class="text-green-600">Configurado</span>
                {:else}
                  <span class="text-yellow-600">No configurado</span>
                {/if}
              </p>
            </div>
          {:else}
            <p class="text-sm text-gray-500">No configurado</p>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>
