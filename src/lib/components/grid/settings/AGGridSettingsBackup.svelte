<script lang="ts">
  import { onMount } from "svelte";
  import { Clock, Calendar, Settings, Save, RefreshCw } from "lucide-svelte";
  import { toast } from "svelte-5-french-toast";
  import {
    getBackupConfig,
    updateBackupConfig,
    cleanupOldBackups,
  } from "$lib/services/backupService";
  import type { BackupConfig } from "$lib/types/backup";

  // Estados
  let isLoading = $state(true);
  let isSaving = $state(false);
  let isCleaning = $state(false);

  // Config form
  let configEnabled = $state(false);
  let configHora = $state("02:00");
  let configDiasRetencion = $state(30);
  let lastBackup = $state<string | null>(null);

  onMount(async () => {
    await loadConfig();
  });

  async function loadConfig() {
    isLoading = true;
    try {
      const config = await getBackupConfig();
      if (config) {
        configEnabled = config.enabled;
        configHora = config.hora;
        configDiasRetencion = config.diasRetencion;
        lastBackup = config.ultimoBackup;
      }
    } catch (err) {
      console.error("Error loading backup config:", err);
    } finally {
      isLoading = false;
    }
  }

  async function handleSave() {
    isSaving = true;
    try {
      await updateBackupConfig(configEnabled, configHora, configDiasRetencion);
      toast.success("Configuración de backup guardada");
    } catch (err) {
      console.error("Error saving backup config:", err);
      toast.error(`Error: ${err}`);
    } finally {
      isSaving = false;
    }
  }

  async function handleCleanup() {
    isCleaning = true;
    try {
      const count = await cleanupOldBackups();
      toast.success(`${count} backup(s) antiguo(s) eliminado(s)`);
    } catch (err) {
      console.error("Error cleaning up:", err);
      toast.error(`Error: ${err}`);
    } finally {
      isCleaning = false;
    }
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div>
    <h3 class="text-sm font-semibold text-[#e6edf3] mb-1">Backup Automático</h3>
    <p class="text-xs text-[#8b949e]">
      Configura las copias de seguridad automáticas del sistema
    </p>
  </div>

  {#if isLoading}
    <div class="flex items-center justify-center py-8">
      <RefreshCw class="w-5 h-5 text-[#8b949e] animate-spin" />
    </div>
  {:else}
    <!-- Toggle Enable -->
    <div
      class="flex items-center justify-between p-4 bg-[#161b22] rounded-lg border border-[#30363d]"
    >
      <div>
        <p class="text-sm font-medium text-[#e6edf3]">
          Activar Backup Automático
        </p>
        <p class="text-xs text-[#8b949e] mt-0.5">
          Se ejecutará diariamente a la hora especificada
        </p>
      </div>
      <label class="relative inline-flex items-center cursor-pointer">
        <input
          type="checkbox"
          bind:checked={configEnabled}
          class="sr-only peer"
        />
        <div
          class="w-11 h-6 bg-[#30363d] rounded-full peer peer-checked:bg-[#238636] peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all"
        ></div>
      </label>
    </div>

    <!-- Schedule -->
    <div class="p-4 bg-[#161b22] rounded-lg border border-[#30363d] space-y-4">
      <div class="flex items-center gap-2">
        <Clock class="w-4 h-4 text-[#8b949e]" />
        <p class="text-sm font-medium text-[#e6edf3]">Programación</p>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <label for="backup-hora" class="block text-xs text-[#8b949e] mb-1.5"
            >Hora del backup</label
          >
          <input
            id="backup-hora"
            type="time"
            bind:value={configHora}
            class="w-full px-3 py-2 text-sm rounded-md bg-[#0d1117] border border-[#30363d] text-[#e6edf3] focus:border-[#238636] focus:outline-none transition-colors"
          />
        </div>

        <div>
          <label
            for="backup-retencion"
            class="block text-xs text-[#8b949e] mb-1.5">Días de retención</label
          >
          <div class="flex items-center gap-2">
            <input
              id="backup-retencion"
              type="number"
              min="1"
              max="365"
              bind:value={configDiasRetencion}
              class="flex-1 px-3 py-2 text-sm rounded-md bg-[#0d1117] border border-[#30363d] text-[#e6edf3] focus:border-[#238636] focus:outline-none transition-colors"
            />
            <span class="text-xs text-[#8b949e]">días</span>
          </div>
        </div>
      </div>

      <p class="text-xs text-[#8b949e]">
        Los backups con más de {configDiasRetencion} días serán eliminados automáticamente
      </p>
    </div>

    <!-- Last Backup Info -->
    {#if lastBackup}
      <div
        class="p-3 bg-[#0d1117] rounded-lg border border-[#30363d] flex items-center gap-3"
      >
        <Calendar class="w-4 h-4 text-[#8b949e]" />
        <div>
          <p class="text-xs text-[#8b949e]">Último backup automático</p>
          <p class="text-sm text-[#e6edf3]">
            {new Date(lastBackup).toLocaleString("es-MX")}
          </p>
        </div>
      </div>
    {/if}

    <!-- Actions -->
    <div class="flex items-center gap-3 pt-2">
      <button
        onclick={handleSave}
        disabled={isSaving}
        class="flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md bg-[#238636] hover:bg-[#2ea043] text-white disabled:opacity-50 transition-colors"
      >
        {#if isSaving}
          <RefreshCw class="w-4 h-4 animate-spin" />
        {:else}
          <Save class="w-4 h-4" />
        {/if}
        Guardar Configuración
      </button>

      <button
        onclick={handleCleanup}
        disabled={isCleaning}
        class="flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md border border-[#f8514966] text-[#f85149] hover:bg-[#f8514922] disabled:opacity-50 transition-colors"
      >
        {#if isCleaning}
          <RefreshCw class="w-4 h-4 animate-spin" />
        {:else}
          <Settings class="w-4 h-4" />
        {/if}
        Limpiar Backups Antiguos
      </button>
    </div>
  {/if}
</div>
