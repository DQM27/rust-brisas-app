<script lang="ts">
  import { Database, ArchiveRestore, Download, Upload, AlertTriangle, RefreshCw } from "lucide-svelte";
  import { scale } from "svelte/transition";
  import { backupDatabase, restoreDatabase } from "$lib/services/backupService";

  let isBackingUp = $state(false);
  let isRestoring = $state(false);

  async function handleBackup() {
    isBackingUp = true;
    try {
      await backupDatabase();
    } finally {
      isBackingUp = false;
    }
  }

  async function handleRestore() {
    isRestoring = true;
    try {
      await restoreDatabase();
    } finally {
      isRestoring = false;
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
      <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">Copia de Seguridad</h2>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
        Protege tus datos realizando copias periodicas.
      </p>
    </div>

    <!-- Backup Card -->
    <div class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden">
      <div class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2">
        <Download class="w-4 h-4 text-gray-500" />
        <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
          Crear Backup
        </h3>
      </div>

      <div class="p-4">
        <p class="text-sm text-gray-600 dark:text-gray-300 mb-4">
          Exporta toda la base de datos a un archivo <code class="bg-gray-100 dark:bg-[#21262d] px-1.5 py-0.5 rounded text-xs">.db</code> que contiene todos los registros actuales.
        </p>

        <button
          class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md border transition-colors
            {isBackingUp
              ? 'bg-gray-100 dark:bg-[#21262d] text-gray-500 dark:text-gray-400 border-gray-300 dark:border-gray-600 cursor-not-allowed'
              : 'bg-[#2da44e] hover:bg-[#2c974b] text-white border-[#2da44e] hover:border-[#2c974b]'}"
          onclick={handleBackup}
          disabled={isBackingUp}
        >
          {#if isBackingUp}
            <RefreshCw class="w-4 h-4 animate-spin" />
            <span>Generando...</span>
          {:else}
            <Database class="w-4 h-4" />
            <span>Generar copia de seguridad</span>
          {/if}
        </button>
      </div>
    </div>

    <!-- Restore Card -->
    <div class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden">
      <div class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2">
        <Upload class="w-4 h-4 text-gray-500" />
        <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
          Restaurar Datos
        </h3>
      </div>

      <div class="p-4">
        <!-- Warning -->
        <div class="flex gap-3 p-3 mb-4 rounded-md bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800/30">
          <AlertTriangle class="w-4 h-4 text-yellow-600 dark:text-yellow-500 flex-shrink-0 mt-0.5" />
          <div class="text-sm text-yellow-800 dark:text-yellow-200">
            <strong>Advertencia:</strong> Esta accion reemplazara todos los datos actuales. La aplicacion se reiniciara automaticamente.
          </div>
        </div>

        <button
          class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md border transition-colors
            {isRestoring
              ? 'bg-gray-100 dark:bg-[#21262d] text-gray-500 dark:text-gray-400 border-gray-300 dark:border-gray-600 cursor-not-allowed'
              : 'bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-[#30363d]'}"
          onclick={handleRestore}
          disabled={isRestoring}
        >
          {#if isRestoring}
            <RefreshCw class="w-4 h-4 animate-spin" />
            <span>Procesando...</span>
          {:else}
            <ArchiveRestore class="w-4 h-4" />
            <span>Seleccionar archivo para restaurar</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
</div>
