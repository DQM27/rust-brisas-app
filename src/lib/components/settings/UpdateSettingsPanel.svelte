<script lang="ts">
  import { DownloadCloud, HardDrive } from "lucide-svelte";
  import { scale } from "svelte/transition";
  import {
    checkAndInstallUpdate,
    installUpdateFromFile,
  } from "$lib/services/updateService";
</script>

<div
  class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-primary">Actualizaciones</h2>
    <p class="text-secondary mt-1">
      Gestiona las actualizaciones y versiones del sistema.
    </p>
  </div>

  <div class="grid gap-4 max-w-3xl pb-8">
    <!-- ================================================================== -->
    <!-- UPDATES CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-3 rounded-lg bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400"
        >
          <DownloadCloud size={22} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">
            Sistema de Actualización
          </h3>
          <p class="text-sm text-secondary">
            Verifica e instala nuevas versiones de Brisas App.
          </p>
          <p class="text-xs text-secondary/60 mt-1 italic">
            Configuración del servidor: tauri.conf.json
          </p>
        </div>
      </div>

      <div class="flex flex-col gap-3">
        <button
          class="flex items-center justify-between p-3 rounded-lg bg-surface-2 hover:bg-surface-3 transition-colors text-left cursor-pointer"
          onclick={() => checkAndInstallUpdate()}
        >
          <div class="flex items-center gap-3">
            <DownloadCloud size={18} class="text-blue-500" />
            <div class="flex flex-col">
              <span class="text-sm font-medium text-primary"
                >Buscar Actualización Online</span
              >
              <span class="text-xs text-secondary mt-0.5"
                >Conecta con el servidor para buscar novedades.</span
              >
            </div>
          </div>
          <span
            class="text-xs text-secondary bg-surface-1 px-2 py-1 rounded border border-white/5"
            >Remoto</span
          >
        </button>

        <button
          class="flex items-center justify-between p-3 rounded-lg bg-surface-2 hover:bg-surface-3 transition-colors text-left cursor-pointer"
          onclick={() => installUpdateFromFile()}
        >
          <div class="flex items-center gap-3">
            <HardDrive size={18} class="text-emerald-500" />
            <div class="flex flex-col">
              <span class="text-sm font-medium text-primary"
                >Instrucciones Offline</span
              >
              <span class="text-xs text-secondary mt-0.5"
                >Guía para instalar manualmente un archivo.</span
              >
            </div>
          </div>
          <span
            class="text-xs text-secondary bg-surface-1 px-2 py-1 rounded border border-white/5"
            >Local</span
          >
        </button>
      </div>
    </div>
  </div>
</div>
