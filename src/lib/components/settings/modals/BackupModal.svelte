<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import {
    X,
    Database,
    ArchiveRestore,
    RefreshCw,
    AlertTriangle,
    Download,
    Upload,
  } from "lucide-svelte";
  import { backupDatabase, restoreDatabase } from "$lib/services/backupService";

  interface Props {
    show: boolean;
    onClose: () => void;
  }

  let { show, onClose }: Props = $props();

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

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
  >
    <!-- Backdrop -->
    <button
      class="absolute inset-0 bg-black/60 backdrop-blur-sm border-0 cursor-default"
      onclick={onClose}
      aria-label="Cerrar modal"
    ></button>

    <!-- Modal Content -->
    <div
      class="relative z-10 w-full max-w-lg overflow-hidden rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 bg-white dark:bg-[#0d1117] border-b border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-2">
          <Database class="w-5 h-5 text-purple-500" />
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
            Copias de Seguridad
          </h2>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          aria-label="Cerrar"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Body -->
      <div class="p-6 space-y-6">
        <!-- Section Backup -->
        <div
          class="bg-gray-50 dark:bg-[#161b22] p-4 rounded-md border border-gray-200 dark:border-gray-700"
        >
          <div class="flex items-start justify-between mb-3">
            <div>
              <h3
                class="text-sm font-semibold text-gray-900 dark:text-white flex items-center gap-2"
              >
                <Download class="w-4 h-4 text-gray-500" />
                Crear Nuevo Respaldo
              </h3>
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                Guarda una copia completa de la base de datos actual.
              </p>
            </div>
            <button
              class="inline-flex items-center gap-2 px-3 py-1.5 text-xs font-medium rounded-md text-white transition-colors
                     {isBackingUp
                ? 'bg-purple-400 cursor-not-allowed'
                : 'bg-purple-600 hover:bg-purple-700 border border-purple-600'}"
              onclick={handleBackup}
              disabled={isBackingUp}
            >
              {#if isBackingUp}
                <RefreshCw class="w-3.5 h-3.5 animate-spin" />
                Generando...
              {:else}
                Generar Copia
              {/if}
            </button>
          </div>
        </div>

        <!-- Section Restore -->
        <div
          class="bg-white dark:bg-[#0d1117] rounded-md border border-gray-200 dark:border-gray-700"
        >
          <div
            class="p-4 border-b border-gray-100 dark:border-gray-700/50 bg-gray-50/50 dark:bg-[#161b22]/50"
          >
            <h3
              class="text-sm font-semibold text-gray-900 dark:text-white flex items-center gap-2"
            >
              <Upload class="w-4 h-4 text-gray-500" />
              Restaurar desde Archivo
            </h3>
          </div>

          <div class="p-4">
            <div
              class="flex gap-3 p-3 mb-4 rounded-md bg-amber-50 dark:bg-amber-900/10 border border-amber-200 dark:border-amber-800/20"
            >
              <AlertTriangle
                class="w-4 h-4 text-amber-600 dark:text-amber-500 flex-shrink-0 mt-0.5"
              />
              <div class="text-xs text-amber-800 dark:text-amber-200">
                <strong>Advertencia:</strong> Esta acción sobrescribirá todos los
                datos actuales. La aplicación se reiniciará automáticamente.
              </div>
            </div>

            <button
              class="w-full inline-flex items-center justify-center gap-2 px-3 py-2 text-sm font-medium rounded-md border transition-colors
                     {isRestoring
                ? 'bg-gray-100 dark:bg-gray-800 text-gray-500 cursor-not-allowed border-gray-200 dark:border-gray-700'
                : 'bg-white dark:bg-[#161b22] text-gray-700 dark:text-gray-200 border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-[#21262d]'}"
              onclick={handleRestore}
              disabled={isRestoring}
            >
              {#if isRestoring}
                <RefreshCw class="w-4 h-4 animate-spin" />
                <span>Restaurando datos...</span>
              {:else}
                <ArchiveRestore class="w-4 h-4" />
                <span>Seleccionar archivo .db</span>
              {/if}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
