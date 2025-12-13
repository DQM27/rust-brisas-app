<!-- src/lib/components/export/ExportDialogSimple.svelte -->
<script lang="ts">
  import { X, Download, Settings2, ChevronRight } from "lucide-svelte";
  import { onMount } from "svelte";
  import { exportProfileStore } from "$lib/stores/exportProfileStore";
  import type { ExportProfile } from "$lib/types/exportProfile";
  import { fade, fly } from "svelte/transition";

  interface Props {
    onExport: (profileId: string, columnIds: string[]) => Promise<void>;
    onClose: () => void;
    columns?: { id: string; name: string; selected: boolean }[];
  }

  let { onExport, onClose, columns = [] }: Props = $props();

  let selectedProfile = $state<ExportProfile | null>(null);
  let columnSelection = $state(columns.map((c) => ({ ...c })));
  let isExporting = $state(false);
  let showColumnSelector = $state(false);

  onMount(async () => {
    await exportProfileStore.load();
    // Seleccionar perfil por defecto
    selectedProfile =
      $exportProfileStore.profiles.find((p) => p.is_default) ||
      $exportProfileStore.profiles[0] ||
      null;
  });

  async function handleExport() {
    if (!selectedProfile) return;
    isExporting = true;
    try {
      const selectedColumnIds = columnSelection
        .filter((c) => c.selected)
        .map((c) => c.id);
      await onExport(selectedProfile.id, selectedColumnIds);
      onClose();
    } catch (error) {
      console.error("Error exportando:", error);
      alert("Error al exportar: " + (error as Error).message);
    } finally {
      isExporting = false;
    }
  }

  function toggleAllColumns() {
    const allSelected = columnSelection.every((c) => c.selected);
    columnSelection = columnSelection.map((c) => ({
      ...c,
      selected: !allSelected,
    }));
  }

  let selectedCount = $derived(
    columnSelection.filter((c) => c.selected).length,
  );
</script>

<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
  transition:fade
  onclick={(e) => e.target === e.currentTarget && !isExporting && onClose()}
  role="presentation"
>
  <div
    class="bg-white dark:bg-[#0d1117] rounded-lg border border-gray-200 dark:border-gray-700 shadow-xl flex flex-row overflow-hidden max-h-[90vh]"
    transition:fly={{ y: 20, duration: 300 }}
  >
    <!-- Left Panel: Main Config -->
    <div class="w-full max-w-md flex flex-col min-w-[400px]">
      <!-- Header -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0"
      >
        <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
          Exportar Datos
        </h2>
        <button
          onclick={onClose}
          disabled={isExporting}
          class="p-1.5 rounded-md text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors disabled:opacity-50"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Content -->
      <div class="p-6 space-y-4 overflow-y-auto flex-1">
        <!-- Selector de perfil -->
        <div>
          <label
            for="profile-select"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            Perfil de exportación
          </label>
          <select
            id="profile-select"
            bind:value={selectedProfile}
            disabled={isExporting || $exportProfileStore.loading}
            class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent disabled:opacity-50"
          >
            {#if $exportProfileStore.loading}
              <option>Cargando perfiles...</option>
            {:else if $exportProfileStore.profiles.length === 0}
              <option>No hay perfiles disponibles</option>
            {:else}
              {#each $exportProfileStore.profiles as profile}
                <option value={profile}>
                  {profile.name}
                  {profile.is_default ? "(Predeterminado)" : ""}
                  - {profile.format.toUpperCase()}
                </option>
              {/each}
            {/if}
          </select>
        </div>

        <!-- Info del perfil seleccionado -->
        {#if selectedProfile}
          <div
            class="p-3 rounded-md bg-gray-50 dark:bg-[#161b22] border border-gray-200 dark:border-gray-700"
          >
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600 dark:text-gray-400">Formato:</span>
              <span class="font-medium text-gray-900 dark:text-gray-100">
                {selectedProfile.format.toUpperCase()}
              </span>
            </div>
            {#if selectedProfile.format === "pdf" && selectedProfile.pdf_design}
              <div class="flex items-center justify-between text-sm mt-1">
                <span class="text-gray-600 dark:text-gray-400"
                  >Orientación:</span
                >
                <span class="font-medium text-gray-900 dark:text-gray-100">
                  {selectedProfile.pdf_design.orientation === "landscape"
                    ? "Horizontal"
                    : "Vertical"}
                </span>
              </div>
            {/if}
          </div>
        {/if}

        <!-- Toggle Columnas -->
        {#if columns.length > 0}
          <div
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-[#161b22] border border-gray-200 dark:border-gray-700 rounded-md"
          >
            <div class="flex items-center gap-2">
              <label class="flex items-center cursor-pointer">
                <div class="relative">
                  <input
                    type="checkbox"
                    bind:checked={showColumnSelector}
                    class="sr-only"
                  />
                  <div
                    class="block w-10 h-6 rounded-full transition-colors {showColumnSelector
                      ? 'bg-[#2da44e]'
                      : 'bg-gray-300 dark:bg-gray-600'}"
                  ></div>
                  <div
                    class="dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform {showColumnSelector
                      ? 'translate-x-4'
                      : 'translate-x-0'}"
                  ></div>
                </div>
                <span
                  class="ml-3 text-sm font-medium text-gray-700 dark:text-gray-300"
                >
                  Seleccionar columnas
                </span>
              </label>
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400">
              {selectedCount}/{columns.length}
            </div>
          </div>
        {/if}

        <!-- Link a configuración -->
        <div class="flex items-center justify-center pt-2">
          <button
            onclick={() => {
              /* TODO: Abrir settings */
            }}
            class="text-sm text-[#2da44e] hover:underline flex items-center gap-1"
          >
            <Settings2 size={14} />
            Administrar perfiles en Configuración
          </button>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex items-center justify-end gap-3 flex-shrink-0"
      >
        <button
          onclick={onClose}
          disabled={isExporting}
          class="px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] transition-colors disabled:opacity-50"
        >
          Cancelar
        </button>
        <button
          onclick={handleExport}
          disabled={isExporting || !selectedProfile || selectedCount === 0}
          class="px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
        >
          {#if isExporting}
            <div
              class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
            ></div>
            Exportando...
          {:else}
            <Download size={16} />
            Exportar
          {/if}
        </button>
      </div>
    </div>

    <!-- Right Panel: Columns - Expands Horizontally -->
    {#if showColumnSelector}
      <div
        class="w-80 border-l border-gray-200 dark:border-gray-700 flex flex-col bg-gray-50/50 dark:bg-[#0d1117]"
        transition:fly={{ x: -20, duration: 200 }}
      >
        <!-- Header del drawer -->
        <div
          class="bg-gray-50 dark:bg-[#161b22] px-4 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0"
          style="min-height: 73px;"
        >
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
            Columnas disponibles
          </span>
          <button
            onclick={toggleAllColumns}
            class="text-xs font-medium text-[#2da44e] hover:text-[#2c974b] transition-colors"
          >
            {columnSelection.every((c) => c.selected) ? "Ninguna" : "Todas"}
          </button>
        </div>

        <!-- Lista de columnas -->
        <div class="flex-1 overflow-y-auto p-2">
          <div class="space-y-1">
            {#each columnSelection as col}
              <button
                onclick={() => (col.selected = !col.selected)}
                class="group w-full px-3 py-2 text-left text-sm transition-all flex items-center gap-2 rounded-md {col.selected
                  ? 'bg-[#2da44e]/10 dark:bg-[#2da44e]/20 hover:bg-[#2da44e]/15 dark:hover:bg-[#2da44e]/25'
                  : 'hover:bg-gray-100 dark:hover:bg-[#161b22]'}"
              >
                <div
                  class="flex-shrink-0 w-4 h-4 rounded border-2 transition-all {col.selected
                    ? 'bg-[#2da44e] border-[#2da44e]'
                    : 'border-gray-300 dark:border-gray-600 group-hover:border-[#2da44e]'}"
                >
                  {#if col.selected}
                    <svg
                      class="w-full h-full text-white"
                      viewBox="0 0 16 16"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="2.5"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    >
                      <polyline points="3,8 6,11 13,4" />
                    </svg>
                  {/if}
                </div>
                <span
                  class="flex-1 {col.selected
                    ? 'text-gray-900 dark:text-gray-100 font-medium'
                    : 'text-gray-600 dark:text-gray-400'}"
                >
                  {col.name}
                </span>
              </button>
            {/each}
          </div>
        </div>

        <!-- Footer Warning -->
        {#if selectedCount === 0}
          <div
            class="bg-yellow-50 dark:bg-yellow-900/20 px-4 py-3 border-t border-yellow-200 dark:border-yellow-700 flex-shrink-0"
          >
            <p class="text-xs text-yellow-800 dark:text-yellow-200">
              ⚠️ Selecciona al menos una columna
            </p>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
