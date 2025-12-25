<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, RefreshCw, DownloadCloud, CheckCircle2 } from "lucide-svelte";
  import { checkAndInstallUpdate } from "$lib/services/updateService";

  interface Props {
    show: boolean;
    onClose: () => void;
  }

  let { show, onClose }: Props = $props();

  let isChecking = $state(false);

  async function handleCheckUpdate() {
    isChecking = true;
    try {
      await checkAndInstallUpdate();
    } finally {
      isChecking = false;
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
      class="relative z-10 w-full max-w-md overflow-hidden rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 bg-white dark:bg-[#0d1117] border-b border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-2">
          <DownloadCloud class="w-5 h-5 text-blue-500" />
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
            Actualizaciones
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
      <div class="p-6">
        <div class="flex flex-col gap-4">
          <div
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-[#161b22] rounded-md border border-gray-100 dark:border-gray-700"
          >
            <div
              class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-300"
            >
              <CheckCircle2 class="w-4 h-4 text-green-500" />
              <span>Versión actual:</span>
            </div>
            <span
              class="font-mono font-medium text-gray-900 dark:text-gray-100 bg-white dark:bg-[#0d1117] px-2 py-0.5 rounded border border-gray-200 dark:border-gray-600 text-xs"
            >
              v1.2.0
            </span>
          </div>

          <p class="text-sm text-gray-500 dark:text-gray-400">
            Busca nuevas actualizaciones disponibles desde el servidor oficial.
            La aplicación se reiniciará si se instala una nueva versión.
          </p>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 flex justify-end gap-3 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          onclick={onClose}
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md transition-colors"
          disabled={isChecking}
        >
          Cerrar
        </button>
        <button
          onclick={handleCheckUpdate}
          disabled={isChecking}
          class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md text-white transition-colors
               {isChecking
            ? 'bg-blue-400 cursor-not-allowed'
            : 'bg-blue-600 hover:bg-blue-700'}"
        >
          {#if isChecking}
            <RefreshCw class="w-4 h-4 animate-spin" />
            <span>Buscando...</span>
          {:else}
            <RefreshCw class="w-4 h-4" />
            <span>Buscar ahora</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
