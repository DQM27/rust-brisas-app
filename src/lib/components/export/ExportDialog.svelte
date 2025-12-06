<!-- src/lib/components/export/ExportDialog.svelte -->
<script lang="ts">
  import {
    X,
    FileText,
    Table2,
    FileSpreadsheet,
    Palette,
    ChevronDown,
  } from "lucide-svelte";
  import type { ExportOptions } from "$lib/logic/export";
  import { onMount } from "svelte";
  import { templateStore } from "$lib/stores/templateStore";
  import type { PdfTemplate } from "$lib/types/template";
  import TemplateManager from "./TemplateManager.svelte";
  import { slide } from "svelte/transition";

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
    },
    {
      id: "excel" as const,
      label: "Excel",
      icon: FileSpreadsheet,
      description: "Hoja de cálculo con formato",
      available: availableFormats.includes("excel"),
    },
    {
      id: "csv" as const,
      label: "CSV",
      icon: Table2,
      description: "Datos planos universales",
      available: true, // Siempre disponible
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
        // ✅ Enviar ID del template seleccionado
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
  class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
  onclick={handleBackdropClick}
  role="presentation"
>
  <div
    class="bg-[#1e1e1e] border border-white/10 rounded-lg shadow-2xl w-full max-w-md"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-white/10">
      <div>
        <h2 class="text-lg font-semibold text-white">Exportar Datos</h2>
        <p class="text-xs text-gray-400 mt-0.5">
          Selecciona formato y opciones
        </p>
      </div>
      <button
        onclick={onClose}
        disabled={isExporting}
        class="p-1.5 rounded-md text-gray-400 hover:text-white hover:bg-white/5 transition-colors disabled:opacity-50"
        aria-label="Cerrar"
      >
        <X size={18} />
      </button>
    </div>

    <!-- Content -->
    <div class="p-5 space-y-5">
      <!-- Selector de formato -->
      <div>
        <label class="block text-sm font-medium text-gray-400 mb-3">
          Formato de exportación
        </label>
        <div class="grid grid-cols-3 gap-2">
          {#each formats as format}
            <button
              onclick={() => (selectedFormat = format.id)}
              disabled={!format.available || isExporting}
              class="relative p-3 rounded-lg border-2 transition-all
                {selectedFormat === format.id
                ? 'border-blue-500 bg-blue-500/10'
                : 'border-white/10 bg-[#252526] hover:border-white/20'}
                {!format.available
                ? 'opacity-50 cursor-not-allowed'
                : 'cursor-pointer'}
                disabled:opacity-50 disabled:cursor-not-allowed"
              title={format.available ? format.description : "No disponible"}
            >
              <div class="flex flex-col items-center gap-2">
                <svelte:component
                  this={format.icon}
                  size={24}
                  class="text-white"
                />
                <span class="text-xs font-medium text-white"
                  >{format.label}</span
                >
              </div>
              {#if !format.available}
                <div
                  class="absolute inset-0 flex items-center justify-center bg-black/50 rounded-lg"
                >
                  <span class="text-xs text-gray-400">N/A</span>
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
          class="block text-sm font-medium text-gray-400 mb-2"
        >
          Título del documento
        </label>
        <input
          id="export-title"
          type="text"
          bind:value={title}
          disabled={isExporting}
          placeholder="Ej: Reporte Mensual"
          class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm
            focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
        />
      </div>

      <!-- Opciones PDF -->
      {#if selectedFormat === "pdf"}
        <div
          class="space-y-3 p-3 bg-[#252526] border border-white/10 rounded-lg"
        >
          <h3
            class="text-sm font-medium text-white flex justify-between items-center"
          >
            Opciones PDF
          </h3>

          <!-- Selector de Template -->
          <div>
            <label class="block text-xs text-gray-400 mb-1">
              Estilo Visual
            </label>
            <div class="flex gap-2">
              <select
                bind:value={selectedTemplate}
                class="flex-1 px-3 py-2 bg-[#1e1e1e] border border-white/10 rounded-lg text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {#each $templateStore.templates as t}
                  <option value={t}
                    >{t.name} {t.is_predefined ? "(Oficial)" : ""}</option
                  >
                {/each}
              </select>
              <button
                onclick={() => (showTemplateManager = true)}
                class="p-2 bg-[#1e1e1e] border border-white/10 rounded-lg text-gray-300 hover:text-white hover:bg-white/5 transition-colors"
                title="Administrar Estilos"
              >
                <Palette size={18} />
              </button>
            </div>
          </div>

          <div>
            <label for="orientation" class="block text-xs text-gray-400 mb-1">
              Orientación
            </label>
            <select
              id="orientation"
              bind:value={orientation}
              disabled={isExporting}
              class="w-full px-3 py-2 bg-[#1e1e1e] border border-white/10 rounded-lg text-white text-sm
                focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
            >
              <option value="landscape">Horizontal (Landscape)</option>
              <option value="portrait">Vertical (Portrait)</option>
            </select>
          </div>

          <label
            class="flex items-center gap-2 cursor-pointer mt-2 pt-2 border-t border-white/5"
          >
            <input
              type="checkbox"
              bind:checked={showPreview}
              disabled={isExporting}
              class="w-4 h-4 text-blue-500 bg-[#1e1e1e] border-white/20 rounded focus:ring-blue-500
                disabled:opacity-50"
            />
            <span class="text-sm text-gray-400"
              >Vista previa antes de guardar</span
            >
          </label>
        </div>
      {/if}

      <!-- Opciones CSV -->
      {#if selectedFormat === "csv"}
        <!-- ... existing CSV options ... -->
        <div
          class="space-y-3 p-3 bg-[#252526] border border-white/10 rounded-lg"
        >
          <h3 class="text-sm font-medium text-white">Opciones CSV</h3>

          <div>
            <label for="delimiter" class="block text-sm text-gray-400 mb-2">
              Delimitador
            </label>
            <select
              id="delimiter"
              bind:value={delimiter}
              disabled={isExporting}
              class="w-full px-3 py-2 bg-[#1e1e1e] border border-white/10 rounded-lg text-white text-sm
                focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
            >
              <option value="comma">Coma (,)</option>
              <option value="semicolon">Punto y coma (;)</option>
              <option value="tab">Tabulación</option>
              <option value="pipe">Barra vertical (|)</option>
            </select>
          </div>

          <label class="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              bind:checked={includeBom}
              disabled={isExporting}
              class="w-4 h-4 text-blue-500 bg-[#1e1e1e] border-white/20 rounded focus:ring-blue-500
                disabled:opacity-50"
            />
            <span class="text-sm text-gray-400">UTF-8 BOM (para Excel)</span>
          </label>
        </div>
      {/if}

      <!-- Selector de Columnas (Collapsible) -->
      {#if columns.length > 0}
        <div
          class="border border-white/10 rounded-lg overflow-hidden bg-[#252526]"
        >
          <button
            class="w-full flex items-center justify-between p-3 text-sm font-medium text-white hover:bg-white/5 transition-colors"
            onclick={() => (showColumns = !showColumns)}
          >
            <span class="flex items-center gap-2">
              <Table2 size={16} class="text-gray-400" />
              Columnas
              <span class="text-xs text-gray-400 font-normal">
                ({columnSelection.filter((c) => c.selected)
                  .length}/{columnSelection.length}
                seleccionadas)
              </span>
            </span>
            <ChevronDown
              size={16}
              class="text-gray-400 transition-transform duration-200 {showColumns
                ? 'rotate-180'
                : ''}"
            />
          </button>

          {#if showColumns}
            <div
              transition:slide={{ duration: 200 }}
              class="p-3 pt-0 border-t border-white/10"
            >
              <div class="flex justify-end mb-3 mt-3">
                <button
                  class="text-xs text-blue-400 hover:text-blue-300"
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
                    class="flex items-center gap-2 cursor-pointer px-3 py-1.5 rounded-full border transition-all text-xs
                      {col.selected
                      ? 'bg-blue-500/20 border-blue-500/50 text-blue-200'
                      : 'bg-[#1e1e1e] border-white/10 text-gray-400 hover:border-white/30'}"
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
                        class="w-1.5 h-1.5 rounded-full bg-blue-400 ml-1"
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
      class="flex items-center justify-end gap-2 p-4 border-t border-white/10"
    >
      <button
        onclick={onClose}
        disabled={isExporting}
        class="px-4 py-2 rounded-md text-sm font-medium text-gray-300 hover:text-white hover:bg-white/5
          transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Cancelar
      </button>
      <button
        onclick={handleExport}
        disabled={isExporting}
        class="px-4 py-2 rounded-md text-sm font-medium bg-blue-500 text-white hover:bg-blue-600
          transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        {#if isExporting}
          <div
            class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
          ></div>
          Exportando...
        {:else}
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
