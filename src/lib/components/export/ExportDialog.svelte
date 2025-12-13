<!-- src/lib/components/export/ExportDialog.svelte -->
<script lang="ts">
  import {
    X,
    FileText,
    Table2,
    FileSpreadsheet,
    Palette,
    ChevronDown,
    Download,
    Eye,
  } from "lucide-svelte";
  import type { ExportOptions } from "$lib/logic/export";
  import { onMount } from "svelte";
  import { templateStore } from "$lib/stores/templateStore";
  import type { PdfTemplate } from "$lib/types/template";
  import TemplateManager from "./TemplateManager.svelte";
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

  // Estado de columnas
  let showColumns = $state(false);
  let columnSelection = $state(
    columns.map((c) => ({ ...c, selected: c.selected })),
  );

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

  // Template State
  let showTemplateManager = $state(false);
  let selectedTemplate = $state<PdfTemplate | null>(null);

  // Cargar templates al montar
  onMount(async () => {
    await templateStore.load();
    // Seleccionar default si existe
    if ($templateStore.templates.length > 0) {
      selectedTemplate = $templateStore.templates[0];
    }
  });

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
          selectedFormat === "pdf" && selectedTemplate
            ? selectedTemplate.id
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
    class="bg-white dark:bg-[#0d1117] rounded-lg border border-gray-200 dark:border-gray-700 shadow-xl w-full max-w-md max-h-[90vh] overflow-hidden flex flex-col"
    transition:fly={{ y: 20, duration: 300 }}
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
  >
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
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
          Formato de exportación
        </label>
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

          <!-- Selector de Template -->
          <div>
            <label
              for="template-select"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              Estilo Visual
            </label>
            <div class="flex gap-2">
              <select
                id="template-select"
                bind:value={selectedTemplate}
                class="flex-1 px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
              >
                {#each $templateStore.templates as t}
                  <option value={t}
                    >{t.name} {t.is_predefined ? "(Oficial)" : ""}</option
                  >
                {/each}
              </select>
              <button
                onclick={() => (showTemplateManager = true)}
                class="p-2 rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] transition-colors"
                title="Administrar Estilos"
              >
                <Palette size={18} />
              </button>
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

      <!-- Selector de Columnas (Collapsible) -->
      {#if columns.length > 0}
        <div
          class="border border-gray-200 dark:border-gray-700 rounded-md overflow-hidden"
        >
          <button
            class="w-full flex items-center justify-between p-3 text-sm font-medium text-gray-900 dark:text-gray-100 bg-gray-50 dark:bg-[#161b22] hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors"
            onclick={() => (showColumns = !showColumns)}
          >
            <span class="flex items-center gap-2">
              <Table2 size={16} class="text-[#2da44e]" />
              Columnas a exportar
              <span class="text-xs text-gray-500 dark:text-gray-400 font-normal">
                ({columnSelection.filter((c) => c.selected)
                  .length}/{columnSelection.length}
                seleccionadas)
              </span>
            </span>
            <ChevronDown
              size={16}
              class="text-gray-500 transition-transform duration-200 {showColumns
                ? 'rotate-180'
                : ''}"
            />
          </button>

          {#if showColumns}
            <div
              transition:slide={{ duration: 200 }}
              class="p-4 bg-white dark:bg-[#0d1117] border-t border-gray-200 dark:border-gray-700"
            >
              <div class="flex justify-end mb-3">
                <button
                  class="text-xs text-[#2da44e] hover:underline"
                  onclick={toggleAllColumns}
                >
                  {allColumnsSelected
                    ? "Deseleccionar todas"
                    : "Seleccionar todas"}
                </button>
              </div>

              <div
                class="flex flex-wrap gap-2 max-h-40 overflow-y-auto custom-scrollbar"
              >
                {#each columnSelection as col}
                  <label
                    class="flex items-center gap-2 cursor-pointer px-3 py-1.5 rounded-md border transition-all text-sm
                      {col.selected
                      ? 'bg-[#2da44e]/10 border-[#2da44e]/50 text-[#2da44e]'
                      : 'bg-gray-50 dark:bg-[#161b22] border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-400 hover:border-gray-300 dark:hover:border-gray-600'}"
                  >
                    <input
                      type="checkbox"
                      bind:checked={col.selected}
                      class="hidden"
                    />
                    <span class="truncate max-w-[150px]" title={col.name}>
                      {col.name}
                    </span>
                    {#if col.selected}
                      <div
                        class="w-1.5 h-1.5 rounded-full bg-[#2da44e]"
                      ></div>
                    {/if}
                  </label>
                {/each}
              </div>
            </div>
          {/if}
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
</div>

{#if showTemplateManager}
  <TemplateManager
    onClose={() => (showTemplateManager = false)}
    onSelect={(template) => {
      selectedTemplate = template;
      showTemplateManager = false;
    }}
    currentTemplateId={selectedTemplate?.id}
  />
{/if}
