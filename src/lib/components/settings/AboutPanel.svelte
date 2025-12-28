<script lang="ts">
  import { Info } from "lucide-svelte";
</script>

<div class="h-full overflow-auto bg-surface-1">
  <div
    class="p-6 max-w-5xl mx-auto space-y-8 font-sans text-gray-900 dark:text-gray-100"
  >
    <!-- Header Section -->
    <div class="text-center space-y-4 mb-10">
      <h1 class="text-4xl font-extrabold tracking-tight">
        Acerca de Mega Brisas
      </h1>
      <p class="text-gray-500 dark:text-gray-400 text-lg max-w-2xl mx-auto">
        Gestión integral de accesos y control de contratistas.
      </p>
    </div>

    <div class="max-w-2xl mx-auto">
      <!-- Info Card (GitHub Box Style) -->
      <div
        class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] shadow-sm overflow-hidden"
      >
        <!-- Header -->
        <div
          class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2"
        >
          <Info class="w-4 h-4 text-gray-500" />
          <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
            Información del Sistema
          </h3>
        </div>

        <!-- Body -->
        <div class="p-6 space-y-6 text-sm">
          <p class="text-gray-600 dark:text-gray-300 leading-relaxed">
            Mega Brisas es una plataforma robusta construida con tecnologías de
            alto rendimiento para garantizar la eficiencia operativa.
          </p>

          <div
            class="border rounded-md border-gray-200 dark:border-gray-700 divide-y divide-gray-200 dark:divide-gray-700"
          >
            <div class="flex justify-between items-center px-4 py-3">
              <span class="text-gray-500 dark:text-gray-400 font-medium"
                >Versión</span
              >
              <div class="flex items-center gap-2">
                <span
                  class="bg-gray-100 dark:bg-[#21262d] text-gray-700 dark:text-gray-300 px-2 py-0.5 rounded-full text-xs font-mono border border-gray-200 dark:border-gray-600"
                  >v1.2.0</span
                >
              </div>
            </div>
            <div class="flex justify-between items-center px-4 py-3">
              <span class="text-gray-500 dark:text-gray-400 font-medium"
                >Desarrollador</span
              >
              <span class="text-gray-900 dark:text-gray-100 font-semibold"
                >Fempro Brisas</span
              >
            </div>
            <div class="flex justify-between items-center px-4 py-3">
              <span class="text-gray-500 dark:text-gray-400 font-medium"
                >Tecnologías</span
              >
              <span class="text-[#0969da] dark:text-[#58a6ff]"
                >Rust - Tauri - Svelte</span
              >
            </div>
          </div>

          <div
            class="bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-300 p-4 rounded-md text-sm border border-blue-200 dark:border-blue-800/30"
          >
            <strong>¿Necesitas ayuda?</strong>
            <p class="mt-1">
              Para soporte técnico o reportar problemas, por favor contacta
              directamente al administrar del sistema.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
