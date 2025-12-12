<script lang="ts">
  import { Shield, Mail, Key, Database, ChevronRight, ChevronLeft, Check, RefreshCw, AlertCircle, Eye, EyeOff } from "lucide-svelte";
  import { fade, fly } from "svelte/transition";
  import {
    setupCredentials,
    generateRandomSecret,
    testSmtpConnectionWithCreds,
    type SmtpCredentials,
    type Argon2Params
  } from "$lib/services/keyringService";

  // Props
  interface Props {
    onComplete?: () => void;
  }
  let { onComplete }: Props = $props();

  // Estado del wizard
  let currentStep = $state(1);
  let isSubmitting = $state(false);
  let error = $state("");
  let testingSmtp = $state(false);
  let smtpTestResult = $state<{ success: boolean; message: string } | null>(null);

  // Visibility toggles
  let showSmtpPassword = $state(false);
  let showArgon2Secret = $state(false);
  let showSqlitePassword = $state(false);

  // Datos del formulario
  let smtpCredentials = $state<SmtpCredentials>({
    host: "",
    port: 587,
    user: "",
    password: "",
    feedback_email: ""
  });

  let argon2Params = $state<Argon2Params>({
    memory: 19456,
    iterations: 2,
    parallelism: 1,
    secret: ""
  });

  let sqlitePassword = $state("");

  // Validaciones
  let step1Valid = $derived(
    smtpCredentials.host.trim() !== "" &&
    smtpCredentials.user.trim() !== "" &&
    smtpCredentials.password.trim() !== "" &&
    smtpCredentials.feedback_email.trim() !== ""
  );

  let step2Valid = $derived(
    argon2Params.secret.trim() !== "" &&
    argon2Params.memory >= 1024 &&
    argon2Params.iterations >= 1 &&
    argon2Params.parallelism >= 1
  );

  // Funciones
  async function generateSecret() {
    try {
      argon2Params.secret = await generateRandomSecret();
    } catch (e) {
      error = `Error generando secret: ${e}`;
    }
  }

  async function testSmtp() {
    testingSmtp = true;
    smtpTestResult = null;
    try {
      // Usar las credenciales del formulario directamente
      const result = await testSmtpConnectionWithCreds(smtpCredentials);
      smtpTestResult = { success: true, message: result };
    } catch (e) {
      smtpTestResult = { success: false, message: `${e}` };
    } finally {
      testingSmtp = false;
    }
  }

  function nextStep() {
    if (currentStep < 3) {
      currentStep++;
      error = "";
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
        smtp: smtpCredentials,
        argon2: argon2Params,
        sqlite_password: sqlitePassword || undefined
      });
      onComplete?.();
    } catch (e) {
      error = `Error guardando configuracion: ${e}`;
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" transition:fade>
  <div
    class="bg-white dark:bg-[#0d1117] rounded-lg border border-gray-200 dark:border-gray-700 shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col"
    transition:fly={{ y: 20, duration: 300 }}
  >
    <!-- Header -->
    <div class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-b border-gray-200 dark:border-gray-700">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-[#2da44e]/10 rounded-lg">
          <Shield class="w-6 h-6 text-[#2da44e]" />
        </div>
        <div>
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
        {#each [1, 2, 3] as step}
          <div class="flex items-center gap-2 flex-1">
            <div
              class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors {
                step < currentStep
                  ? 'bg-[#2da44e] text-white'
                  : step === currentStep
                  ? 'bg-[#2da44e] text-white'
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-500'
              }"
            >
              {#if step < currentStep}
                <Check class="w-4 h-4" />
              {:else}
                {step}
              {/if}
            </div>
            {#if step < 3}
              <div class="flex-1 h-1 rounded-full {step < currentStep ? 'bg-[#2da44e]' : 'bg-gray-200 dark:bg-gray-700'}"></div>
            {/if}
          </div>
        {/each}
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-6">
      {#if error}
        <div class="mb-4 p-3 rounded-md bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/30 flex items-start gap-2">
          <AlertCircle class="w-4 h-4 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
          <span class="text-sm text-red-700 dark:text-red-300">{error}</span>
        </div>
      {/if}

      <!-- Step 1: SMTP -->
      {#if currentStep === 1}
        <div transition:fade={{ duration: 200 }}>
          <div class="flex items-center gap-2 mb-4">
            <Mail class="w-5 h-5 text-[#2da44e]" />
            <h3 class="font-semibold text-gray-900 dark:text-gray-100">Configuracion SMTP</h3>
          </div>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
            Configura el servidor de correo para recibir notificaciones de errores y sugerencias.
          </p>

          <div class="space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Host SMTP
                </label>
                <input
                  type="text"
                  bind:value={smtpCredentials.host}
                  placeholder="smtp.gmail.com"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Puerto
                </label>
                <input
                  type="number"
                  bind:value={smtpCredentials.port}
                  placeholder="587"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Usuario / Email
              </label>
              <input
                type="email"
                bind:value={smtpCredentials.user}
                placeholder="tu@email.com"
                class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Contrasena de Aplicacion
              </label>
              <div class="relative">
                <input
                  type={showSmtpPassword ? "text" : "password"}
                  bind:value={smtpCredentials.password}
                  placeholder="••••••••••••"
                  class="w-full px-3 py-2 pr-10 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
                <button
                  type="button"
                  onclick={() => showSmtpPassword = !showSmtpPassword}
                  class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-gray-400 hover:text-gray-600"
                >
                  {#if showSmtpPassword}
                    <EyeOff class="w-4 h-4" />
                  {:else}
                    <Eye class="w-4 h-4" />
                  {/if}
                </button>
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Email de Destino (Feedback)
              </label>
              <input
                type="email"
                bind:value={smtpCredentials.feedback_email}
                placeholder="soporte@tuempresa.com"
                class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
              />
            </div>

            <!-- Test Connection -->
            {#if step1Valid}
              <div class="pt-2">
                <button
                  type="button"
                  onclick={testSmtp}
                  disabled={testingSmtp}
                  class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] disabled:opacity-50"
                >
                  {#if testingSmtp}
                    <RefreshCw class="w-4 h-4 animate-spin" />
                    <span>Probando...</span>
                  {:else}
                    <Mail class="w-4 h-4" />
                    <span>Probar conexion</span>
                  {/if}
                </button>

                {#if smtpTestResult}
                  <span class="ml-3 text-sm {smtpTestResult.success ? 'text-green-600' : 'text-red-600'}">
                    {smtpTestResult.message}
                  </span>
                {/if}
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- Step 2: Argon2 -->
      {#if currentStep === 2}
        <div transition:fade={{ duration: 200 }}>
          <div class="flex items-center gap-2 mb-4">
            <Key class="w-5 h-5 text-[#2da44e]" />
            <h3 class="font-semibold text-gray-900 dark:text-gray-100">Parametros de Seguridad</h3>
          </div>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
            Configura los parametros de Argon2 para el hasheo seguro de contrasenas.
          </p>

          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Secret (Pepper)
              </label>
              <div class="flex gap-2">
                <div class="relative flex-1">
                  <input
                    type={showArgon2Secret ? "text" : "password"}
                    bind:value={argon2Params.secret}
                    placeholder="Secret para hasheo de contraseñas"
                    class="w-full px-3 py-2 pr-10 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent font-mono"
                  />
                  <button
                    type="button"
                    onclick={() => showArgon2Secret = !showArgon2Secret}
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
                Este secret se usa como "pepper" adicional al salt. Guardalo de forma segura.
              </p>
            </div>

            <div class="grid grid-cols-3 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Memoria (KB)
                </label>
                <input
                  type="number"
                  bind:value={argon2Params.memory}
                  min="1024"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
                <p class="mt-1 text-xs text-gray-500">~19MB recomendado</p>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Iteraciones
                </label>
                <input
                  type="number"
                  bind:value={argon2Params.iterations}
                  min="1"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
                <p class="mt-1 text-xs text-gray-500">2-4 recomendado</p>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Paralelismo
                </label>
                <input
                  type="number"
                  bind:value={argon2Params.parallelism}
                  min="1"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
                <p class="mt-1 text-xs text-gray-500">1-2 recomendado</p>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Step 3: SQLite (opcional) -->
      {#if currentStep === 3}
        <div transition:fade={{ duration: 200 }}>
          <div class="flex items-center gap-2 mb-4">
            <Database class="w-5 h-5 text-[#2da44e]" />
            <h3 class="font-semibold text-gray-900 dark:text-gray-100">Base de Datos (Opcional)</h3>
          </div>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
            Configura una contrasena para encriptar la base de datos SQLite. Este paso es opcional.
          </p>

          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Contrasena de SQLite
              </label>
              <div class="relative">
                <input
                  type={showSqlitePassword ? "text" : "password"}
                  bind:value={sqlitePassword}
                  placeholder="Dejar vacio para no encriptar"
                  class="w-full px-3 py-2 pr-10 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                />
                <button
                  type="button"
                  onclick={() => showSqlitePassword = !showSqlitePassword}
                  class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-gray-400 hover:text-gray-600"
                >
                  {#if showSqlitePassword}
                    <EyeOff class="w-4 h-4" />
                  {:else}
                    <Eye class="w-4 h-4" />
                  {/if}
                </button>
              </div>
              <p class="mt-1 text-xs text-gray-500">
                Si dejas vacio este campo, la base de datos no sera encriptada.
              </p>
            </div>

            <!-- Resumen -->
            <div class="mt-6 p-4 rounded-md bg-gray-50 dark:bg-[#161b22] border border-gray-200 dark:border-gray-700">
              <h4 class="font-medium text-gray-900 dark:text-gray-100 mb-3">Resumen de Configuracion</h4>
              <ul class="space-y-2 text-sm">
                <li class="flex items-center gap-2">
                  <Check class="w-4 h-4 text-[#2da44e]" />
                  <span class="text-gray-600 dark:text-gray-300">SMTP: {smtpCredentials.host}:{smtpCredentials.port}</span>
                </li>
                <li class="flex items-center gap-2">
                  <Check class="w-4 h-4 text-[#2da44e]" />
                  <span class="text-gray-600 dark:text-gray-300">Argon2: {argon2Params.memory}KB, {argon2Params.iterations} iter, {argon2Params.parallelism} threads</span>
                </li>
                <li class="flex items-center gap-2">
                  {#if sqlitePassword}
                    <Check class="w-4 h-4 text-[#2da44e]" />
                    <span class="text-gray-600 dark:text-gray-300">SQLite: Encriptado</span>
                  {:else}
                    <AlertCircle class="w-4 h-4 text-yellow-500" />
                    <span class="text-gray-600 dark:text-gray-300">SQLite: Sin encriptar</span>
                  {/if}
                </li>
              </ul>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex items-center justify-between">
      <button
        type="button"
        onclick={prevStep}
        disabled={currentStep === 1}
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <ChevronLeft class="w-4 h-4" />
        <span>Anterior</span>
      </button>

      {#if currentStep < 3}
        <button
          type="button"
          onclick={nextStep}
          disabled={currentStep === 1 && !step1Valid || currentStep === 2 && !step2Valid}
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
</div>
