<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, Info, Code2, Globe } from "lucide-svelte";
  import { scale } from "svelte/transition";

  interface Props {
    show: boolean;
    onClose: () => void;
  }

  let { show, onClose }: Props = $props();
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
    transition:fade={{ duration: 150 }}
    onclick={onClose}
    role="dialog"
    aria-modal="true"
  >
    <!-- Modal Content -->
    <div
      class="relative z-10 w-full max-w-sm overflow-hidden rounded-xl bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:scale={{ duration: 200, start: 0.95 }}
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="pt-8 pb-4 flex justify-center relative">
        <button
          onclick={onClose}
          class="absolute top-2 right-2 p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          aria-label="Cerrar"
        >
          <X size={20} />
        </button>
        <div
          class="p-3 bg-white dark:bg-[#161b22] rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700"
        >
          <img
            src="/icono-brisas.png"
            alt="Brisas"
            class="w-12 h-12 object-contain"
          />
        </div>
      </div>

      <!-- Body -->
      <div class="pb-8 px-6 text-center">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-1">
          Brisas App
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-6 font-medium">
          Sistema ERP
        </p>

        <div class="space-y-4">
          <div
            class="text-xs text-gray-500 dark:text-gray-400 flex flex-wrap justify-center gap-2"
          >
            <span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]"
              >Rust</span
            >
            <span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]"
              >Tauri</span
            >
            <span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]"
              >SvelteKit</span
            >
            <span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]"
              >SQLite</span
            >
          </div>

          <div
            class="flex flex-col gap-2 p-3 rounded-lg bg-gray-50 dark:bg-[#161b22] border border-gray-100 dark:border-gray-700/50"
          >
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-500 dark:text-gray-400">Versión</span>
              <span
                class="font-mono font-medium text-gray-900 dark:text-gray-200"
                >v1.2.0</span
              >
            </div>
          </div>

          <div class="pt-4 border-t border-gray-100 dark:border-gray-700/50">
            <p
              class="text-xs text-gray-400 dark:text-gray-500 uppercase tracking-wider font-semibold mb-3"
            >
              Desarrollado por
            </p>
            <div
              class="flex items-center justify-center gap-2 text-gray-800 dark:text-gray-200 font-medium"
            >
              <Code2 class="w-4 h-4 text-blue-500" />
              <span>27Design</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-3 border-t border-gray-200 dark:border-gray-700 text-center"
      >
        <p class="text-xs text-gray-400 dark:text-gray-500">
          © {new Date().getFullYear()} Todos los derechos reservados
        </p>
      </div>
    </div>
  </div>
{/if}
