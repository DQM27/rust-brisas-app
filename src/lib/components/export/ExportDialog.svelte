<!-- src/lib/components/export/ExportDialog.svelte -->
<script lang="ts">
  // @ts-nocheck - Svelte 5 runes not recognized by TS
  import {
    X,
    FileText,
    Table2,
    FileSpreadsheet,
    Palette,
    Download,
    Eye,
  } from "lucide-svelte";
  import type { ExportOptions } from "$lib/logic/export";
  import { onMount } from "svelte";
  import { exportProfileStore } from "$lib/stores/exportProfileStore";
  import type { ExportProfile } from "$lib/types/exportProfile";
  import { slide, fade, fly } from "svelte/transition";

  interface Props {
    onExport: (
      format: "pdf" | "excel" | "csv",
      options: ExportOptions,
    ) => Promise<void>;
    onClose: () => void;
    availableFormats?: string[];
    columns?: { id: string; name: string; selected: boolean }[];
  }

  let {
    onExport,
    onClose,
    availableFormats = ["pdf", "excel", "csv"],
    columns = [],
  }: Props = $props();

  // Estado de columnas - inicializar vacío y sincronizar via effect
  let columnSelection = $state<
    { id: string; name: string; selected: boolean }[]
  >([]);

  // Sync when columns prop changes
  $effect(() => {
    columnSelection = columns.map((c) => ({ ...c, selected: c.selected }));
  });
  let showColumnSelector = $state(false);

  // Computed: toggle select all
  let allColumnsSelected = $derived(columnSelection.every((c) => c.selected));
  let someColumnsSelected = $derived(
    columnSelection.some((c) => c.selected) && !allColumnsSelected,
  );

  function toggleAllColumns() {
    const newState = !allColumnsSelected;
    columnSelection = columnSelection.map((c) => ({
      ...c,
      selected: newState,
    }));
  }

  // Estado
  let selectedFormat = $state<"pdf" | "excel" | "csv">("pdf");
  let title = $state("Reporte");
  let orientation = $state<"portrait" | "landscape">("landscape");
  let delimiter = $state<"comma" | "semicolon" | "tab" | "pipe">("comma");
  let includeBom = $state(true);
  let showPreview = $state(false);
  let isExporting = $state(false);

  // Profile State
  let selectedProfile = $state<ExportProfile | null>(null);

  // Cargar profiles al montar
  onMount(async () => {
    await exportProfileStore.load();
    // Seleccionar default
    const defaultProfile =
      $exportProfileStore.profiles.find(
        (p) => p.is_default && p.format === "pdf",
      ) || $exportProfileStore.profiles.find((p) => p.format === "pdf");

    if (defaultProfile) {
      selectedProfile = defaultProfile;
    }
  });

  // Filtrar perfiles PDF disponibles para el selector
  let pdfProfiles = $derived(
    $exportProfileStore.profiles.filter((p) => p.format === "pdf"),
  );

  // Formatos disponibles con metadata
  const formats = $derived([
    {
      id: "pdf" as const,
      label: "PDF",
      icon: FileText,
      description: "Documento profesional con formato",
      available: availableFormats.includes("pdf"),
      color: "text-red-400",
    },
    {
      id: "excel" as const,
      label: "Excel",
      icon: FileSpreadsheet,
      description: "Hoja de cálculo con formato",
      available: availableFormats.includes("excel"),
      color: "text-green-400",
    },
    {
      id: "csv" as const,
      label: "CSV",
      icon: Table2,
      description: "Datos planos universales",
      available: true, // Siempre disponible
      color: "text-blue-400",
    },
  ]);

  async function handleExport() {
    isExporting = true;

    try {
      const options: ExportOptions = {
        title: title.trim() || "Reporte",
        orientation: selectedFormat === "pdf" ? orientation : undefined,
        delimiter: selectedFormat === "csv" ? delimiter : undefined,
        includeBom: selectedFormat === "csv" ? includeBom : undefined,
        showPreview: selectedFormat === "pdf" ? showPreview : undefined,
        templateId:
          selectedFormat === "pdf" && selectedProfile
            ? selectedProfile.id
            : undefined,
        columnIds: columnSelection.filter((c) => c.selected).map((c) => c.id),
      };

      await onExport(selectedFormat, options);
      onClose();
    } catch (error) {
      console.error("Error exportando:", error);
      alert("Error al exportar: " + (error as Error).message);
    } finally {
      isExporting = false;
    }
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget && !isExporting) {
      onClose();
    }
  }
</script>

<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
  transition:fade
  onclick={handleBackdropClick}
  role="presentation"
  onkeydown={(e) => e.key === "Escape" && !isExporting && onClose()}
  tabindex="-1"
>
  <div
    class="bg-white dark:bg-[#0d1117] rounded-lg border border-gray-200 dark:border-gray-700 shadow-xl flex flex-row overflow-hidden max-h-[90vh]"
    transition:fly={{ y: 20, duration: 300 }}
    onclick={(e) => e.stopPropagation()}
    onkeydown={() => {}}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- Left Panel: Main Config -->
    <div class="w-full max-w-md flex flex-col min-w-[420px]">
      <!-- Header -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-b border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
              Exportar Datos
            </h2>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">
              Selecciona formato y opciones de exportación
            </p>
          </div>
          <button
            onclick={onClose}
            disabled={isExporting}
            class="p-1.5 rounded-md text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors disabled:opacity-50"
            aria-label="Cerrar"
          >
            <X size={20} />
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6 space-y-5">
        <!-- Selector de formato -->
        <div>
          <span
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
          >
            Formato de exportación
          </span>
          <div class="grid grid-cols-3 gap-3">
            {#each formats as format}
              {@const Icon = format.icon}
              <button
                onclick={() => (selectedFormat = format.id)}
                disabled={!format.available || isExporting}
                class="relative p-4 rounded-md border-2 transition-all
                  {selectedFormat === format.id
                  ? 'border-[#2da44e] bg-[#2da44e]/5 dark:bg-[#2da44e]/10'
                  : 'border-gray-200 dark:border-gray-700 bg-white dark:bg-[#0d1117] hover:border-gray-300 dark:hover:border-gray-600'}
                  {!format.available
                  ? 'opacity-50 cursor-not-allowed'
                  : 'cursor-pointer'}
                  disabled:opacity-50 disabled:cursor-not-allowed"
                title={format.available ? format.description : "No disponible"}
              >
                <div class="flex flex-col items-center gap-2">
                  <Icon
                    size={28}
                    class="{selectedFormat === format.id
                      ? 'text-[#2da44e]'
                      : format.color} transition-colors"
                  />
                  <span
                    class="text-sm font-medium {selectedFormat === format.id
                      ? 'text-[#2da44e]'
                      : 'text-gray-700 dark:text-gray-300'}"
                    >{format.label}</span
                  >
                </div>
                {#if !format.available}
                  <div
                    class="absolute inset-0 flex items-center justify-center bg-black/10 dark:bg-black/30 rounded-md"
                  >
                    <span class="text-xs text-gray-500">N/A</span>
                  </div>
                {/if}
              </button>
            {/each}
          </div>
        </div>

        <!-- Título -->
        <div>
          <label
            for="export-title"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            Título del documento
          </label>
          <input
            id="export-title"
            type="text"
            bind:value={title}
            disabled={isExporting}
            placeholder="Ej: Reporte Mensual"
            class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent disabled:opacity-50"
          />
        </div>

        <!-- Opciones PDF -->
        {#if selectedFormat === "pdf"}
          <div
            class="space-y-4 p-4 bg-gray-50 dark:bg-[#161b22] border border-gray-200 dark:border-gray-700 rounded-md"
            transition:slide={{ duration: 200 }}
          >
            <h3
              class="text-sm font-semibold text-gray-900 dark:text-gray-100 flex items-center gap-2"
            >
              <FileText size={16} class="text-[#2da44e]" />
              Opciones PDF
            </h3>

            <!-- Selector de Profile (antes Template) -->
            <div>
              <label
                for="profile-select"
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >
                Estilo Visual (Perfil)
              </label>
              <div class="flex gap-2">
                <select
                  id="profile-select"
                  bind:value={selectedProfile}
                  class="flex-1 px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
                >
                  {#each pdfProfiles as p}
                    <option value={p}
                      >{p.name} {p.is_default ? "(Predeterminado)" : ""}</option
                    >
                  {/each}
                </select>
                <!-- Botón de administración removido -->
              </div>
            </div>

            <div>
              <label
                for="orientation"
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >
                Orientación
              </label>
              <select
                id="orientation"
                bind:value={orientation}
                disabled={isExporting}
                class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent disabled:opacity-50"
              >
                <option value="landscape">Horizontal (Landscape)</option>
                <option value="portrait">Vertical (Portrait)</option>
              </select>
            </div>

            <label
              class="flex items-center gap-2 cursor-pointer p-2 rounded-md hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors"
            >
              <input
                type="checkbox"
                bind:checked={showPreview}
                disabled={isExporting}
                class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-[#2da44e] focus:ring-[#2da44e] disabled:opacity-50"
              />
              <Eye size={16} class="text-gray-500 dark:text-gray-400" />
              <span class="text-sm text-gray-700 dark:text-gray-300"
                >Vista previa antes de guardar</span
              >
            </label>
          </div>
        {/if}

        <!-- Opciones CSV -->
        {#if selectedFormat === "csv"}
          <div
            class="space-y-4 p-4 bg-gray-50 dark:bg-[#161b22] border border-gray-200 dark:border-gray-700 rounded-md"
            transition:slide={{ duration: 200 }}
          >
            <h3
              class="text-sm font-semibold text-gray-900 dark:text-gray-100 flex items-center gap-2"
            >
              <Table2 size={16} class="text-[#2da44e]" />
              Opciones CSV
            </h3>

            <div>
              <label
                for="delimiter"
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >
                Delimitador
              </label>
              <select
                id="delimiter"
                bind:value={delimiter}
                disabled={isExporting}
                class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent disabled:opacity-50"
              >
                <option value="comma">Coma (,)</option>
                <option value="semicolon">Punto y coma (;)</option>
                <option value="tab">Tabulación</option>
                <option value="pipe">Barra vertical (|)</option>
              </select>
            </div>

            <label
              class="flex items-center gap-2 cursor-pointer p-2 rounded-md hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors"
            >
              <input
                type="checkbox"
                bind:checked={includeBom}
                disabled={isExporting}
                class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-[#2da44e] focus:ring-[#2da44e] disabled:opacity-50"
              />
              <span class="text-sm text-gray-700 dark:text-gray-300"
                >UTF-8 BOM (recomendado para Excel)</span
              >
            </label>
          </div>
        {/if}

        <!-- Selector de columnas Toggle -->
        {#if columns.length > 0}
          <div class="space-y-2">
            <!-- Toggle para abrir drawer -->
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
                {columnSelection.filter((c) => c.selected)
                  .length}/{columnSelection.length}
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex items-center justify-end gap-3"
      >
        <button
          onclick={onClose}
          disabled={isExporting}
          class="px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Cancelar
        </button>
        <button
          onclick={handleExport}
          disabled={isExporting}
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
          style="min-height: 84px;"
        >
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
            Columnas disponibles
          </span>
          <button
            onclick={toggleAllColumns}
            disabled={isExporting}
            class="text-xs font-medium text-[#2da44e] hover:text-[#2c974b] transition-colors disabled:opacity-50"
          >
            {allColumnsSelected ? "Ninguna" : "Todas"}
          </button>
        </div>

        <!-- Lista de columnas -->
        <div class="flex-1 overflow-y-auto p-2">
          <div class="space-y-1">
            {#each columnSelection as col}
              <button
                onclick={() => (col.selected = !col.selected)}
                disabled={isExporting}
                class="group w-full px-3 py-2 text-left text-sm transition-all disabled:opacity-50 flex items-center gap-2 rounded-md {col.selected
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
        {#if columnSelection.filter((c) => c.selected).length === 0}
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
