<!-- src/lib/components/export/ExportDialog.svelte -->
<script lang="ts">
  // @ts-nocheck - Svelte 5 runes not recognized by TS
  import {
    X,
    FileText,
    Table2,
    FileSpreadsheet,
    Download,
    Settings,
    Columns,
  } from "lucide-svelte";
  import type { ExportOptions } from "$lib/logic/export";
  import { slide, fade, fly } from "svelte/transition";
  import ExportAdvancedDialog from "./ExportAdvancedDialog.svelte";

  interface Props {
    onExport: (
      format: "pdf" | "excel" | "csv",
      options: ExportOptions,
    ) => Promise<void>;
    onClose: () => void;
    availableFormats?: string[];
    columns?: { id: string; name: string; selected: boolean }[];
    rows?: Record<string, any>[];
    headers?: string[];
  }

  let {
    onExport,
    onClose,
    availableFormats = ["pdf", "excel", "csv"],
    columns = [],
    rows = [],
    headers = [],
  }: Props = $props();

  // Estado de columnas
  let columnSelection = $state<
    { id: string; name: string; selected: boolean }[]
  >([]);

  $effect(() => {
    columnSelection = columns.map((c) => ({ ...c, selected: c.selected }));
  });

  let showColumnSelector = $state(false);
  let showAdvancedDialog = $state(false);

  let allColumnsSelected = $derived(columnSelection.every((c) => c.selected));

  function toggleAllColumns() {
    const newState = !allColumnsSelected;
    columnSelection = columnSelection.map((c) => ({
      ...c,
      selected: newState,
    }));
  }

  // Estado principal
  let selectedFormat = $state<"pdf" | "excel" | "csv">("pdf");
  let title = $state("Reporte");
  let orientation = $state<"portrait" | "landscape">("landscape");
  let delimiter = $state<"comma" | "semicolon" | "tab" | "pipe">("comma");
  let includeBom = $state(true);
  let isExporting = $state(false);

  // Formatos disponibles
  const formats = $derived([
    {
      id: "pdf" as const,
      label: "PDF",
      icon: FileText,
      available: availableFormats.includes("pdf"),
    },
    {
      id: "excel" as const,
      label: "Excel",
      icon: FileSpreadsheet,
      available: availableFormats.includes("excel"),
    },
    {
      id: "csv" as const,
      label: "CSV",
      icon: Table2,
      available: true,
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
        showPreview: false,
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

  async function handleAdvancedExport(options: ExportOptions) {
    await onExport("pdf", options);
  }
</script>

{#if showAdvancedDialog}
  <ExportAdvancedDialog
    onExport={handleAdvancedExport}
    {onClose}
    onBack={() => (showAdvancedDialog = false)}
    columns={columnSelection}
    initialOptions={{ title, orientation }}
    {rows}
    {headers}
  />
{:else}
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4"
    transition:fade={{ duration: 150 }}
    role="presentation"
    tabindex="-1"
  >
    <div
      class="bg-[#161b22] rounded-lg border border-[#30363d] shadow-2xl flex flex-row overflow-hidden max-h-[85vh]"
      transition:fly={{ y: 20, duration: 200 }}
      onclick={(e) => e.stopPropagation()}
      onkeydown={() => {}}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <!-- Main Panel -->
      <div class="w-full max-w-sm flex flex-col">
        <!-- Header -->
        <div
          class="px-5 py-4 border-b border-[#30363d] flex items-center justify-between"
        >
          <div>
            <h2 class="text-base font-semibold text-[#e6edf3]">Exportar</h2>
            <p class="text-xs text-[#8b949e] mt-0.5">
              {columnSelection.filter((c) => c.selected).length} columnas
            </p>
          </div>
          <button
            onclick={onClose}
            disabled={isExporting}
            class="p-1.5 rounded-md text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#21262d] transition-colors disabled:opacity-50"
            aria-label="Cerrar"
          >
            <X size={18} />
          </button>
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-y-auto p-5 space-y-4">
          <!-- Formato -->
          <div>
            <span
              class="block text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide"
            >
              Formato
            </span>
            <div class="flex gap-2">
              {#each formats as format}
                {@const Icon = format.icon}
                <button
                  onclick={() => (selectedFormat = format.id)}
                  disabled={!format.available || isExporting}
                  class="flex-1 px-3 py-2.5 rounded-md border transition-all flex items-center justify-center gap-2
                    {selectedFormat === format.id
                    ? 'border-[#2563eb] bg-[#2563eb]/10 text-[#58a6ff]'
                    : 'border-[#30363d] bg-[#0d1117] text-[#8b949e] hover:border-[#484f58] hover:text-[#e6edf3]'}
                    {!format.available
                    ? 'opacity-50 cursor-not-allowed'
                    : 'cursor-pointer'}
                    disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <Icon size={16} />
                  <span class="text-sm font-medium">{format.label}</span>
                </button>
              {/each}
            </div>
          </div>

          <!-- Título -->
          <div>
            <label
              for="export-title"
              class="block text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide"
            >
              Título del documento
            </label>
            <input
              id="export-title"
              type="text"
              bind:value={title}
              disabled={isExporting}
              placeholder="Ej: Reporte Mensual"
              class="w-full px-3 py-2 text-sm rounded-md border border-[#30363d] bg-[#0d1117] text-[#e6edf3] placeholder-[#484f58] focus:ring-1 focus:ring-[#2563eb] focus:border-[#2563eb] disabled:opacity-50 transition-colors"
            />
          </div>

          <!-- Opciones PDF -->
          {#if selectedFormat === "pdf"}
            <div
              class="space-y-3 p-3 bg-[#0d1117] border border-[#30363d] rounded-md"
              transition:slide={{ duration: 150 }}
            >
              <div>
                <label
                  for="orientation"
                  class="block text-xs font-medium text-[#8b949e] mb-1.5"
                >
                  Orientación
                </label>
                <select
                  id="orientation"
                  bind:value={orientation}
                  disabled={isExporting}
                  class="w-full px-3 py-2 text-sm rounded-md border border-[#30363d] bg-[#161b22] text-[#e6edf3] focus:ring-1 focus:ring-[#2563eb] focus:border-[#2563eb] disabled:opacity-50"
                >
                  <option value="landscape">Horizontal</option>
                  <option value="portrait">Vertical</option>
                </select>
              </div>

              <!-- Botón Avanzado -->
              <button
                onclick={() => (showAdvancedDialog = true)}
                class="w-full flex items-center justify-center gap-2 px-3 py-2 text-sm font-medium rounded-md border border-[#30363d] bg-[#21262d] text-[#e6edf3] hover:bg-[#30363d] transition-colors"
              >
                <Settings size={14} />
                Configuración Avanzada
              </button>
            </div>
          {/if}

          <!-- Opciones CSV -->
          {#if selectedFormat === "csv"}
            <div
              class="space-y-3 p-3 bg-[#0d1117] border border-[#30363d] rounded-md"
              transition:slide={{ duration: 150 }}
            >
              <div>
                <label
                  for="delimiter"
                  class="block text-xs font-medium text-[#8b949e] mb-1.5"
                >
                  Delimitador
                </label>
                <select
                  id="delimiter"
                  bind:value={delimiter}
                  disabled={isExporting}
                  class="w-full px-3 py-2 text-sm rounded-md border border-[#30363d] bg-[#161b22] text-[#e6edf3] focus:ring-1 focus:ring-[#2563eb] focus:border-[#2563eb] disabled:opacity-50"
                >
                  <option value="comma">Coma (,)</option>
                  <option value="semicolon">Punto y coma (;)</option>
                  <option value="tab">Tabulación</option>
                  <option value="pipe">Barra (|)</option>
                </select>
              </div>

              <label class="flex items-center gap-2 cursor-pointer py-1.5">
                <input
                  type="checkbox"
                  bind:checked={includeBom}
                  disabled={isExporting}
                  class="w-4 h-4 rounded border-[#30363d] bg-[#0d1117] text-[#2563eb] focus:ring-[#2563eb] focus:ring-offset-0 disabled:opacity-50"
                />
                <span class="text-sm text-[#e6edf3]"
                  >UTF-8 BOM (para Excel)</span
                >
              </label>
            </div>
          {/if}

          <!-- Toggle columnas -->
          {#if columns.length > 0}
            <button
              onclick={() => (showColumnSelector = !showColumnSelector)}
              class="w-full flex items-center justify-between px-3 py-2.5 bg-[#0d1117] border border-[#30363d] rounded-md hover:border-[#484f58] transition-colors"
            >
              <div class="flex items-center gap-2">
                <Columns size={14} class="text-[#8b949e]" />
                <span class="text-sm text-[#e6edf3]">Columnas</span>
              </div>
              <span class="text-xs text-[#8b949e]">
                {columnSelection.filter((c) => c.selected)
                  .length}/{columnSelection.length}
              </span>
            </button>
          {/if}
        </div>

        <!-- Footer -->
        <div
          class="px-5 py-4 border-t border-[#30363d] flex items-center justify-end gap-2"
        >
          <button
            onclick={onClose}
            disabled={isExporting}
            class="px-4 py-2 text-sm font-medium rounded-md border border-[#30363d] bg-[#21262d] text-[#e6edf3] hover:bg-[#30363d] transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Cancelar
          </button>
          <button
            onclick={handleExport}
            disabled={isExporting ||
              columnSelection.filter((c) => c.selected).length === 0}
            class="px-4 py-2 text-sm font-medium rounded-md bg-[#2563eb] hover:bg-[#1d4ed8] text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            {#if isExporting}
              <div
                class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
              ></div>
              Exportando...
            {:else}
              <Download size={14} />
              Exportar
            {/if}
          </button>
        </div>
      </div>

      <!-- Column Selector Panel -->
      {#if showColumnSelector}
        <div
          class="w-64 border-l border-[#30363d] flex flex-col bg-[#0d1117]"
          transition:fly={{ x: -20, duration: 150 }}
        >
          <div
            class="px-4 py-3 border-b border-[#30363d] flex items-center justify-between"
          >
            <span
              class="text-xs font-medium text-[#8b949e] uppercase tracking-wide"
            >
              Columnas
            </span>
            <button
              onclick={toggleAllColumns}
              disabled={isExporting}
              class="text-xs font-medium text-[#58a6ff] hover:text-[#79c0ff] transition-colors disabled:opacity-50"
            >
              {allColumnsSelected ? "Ninguna" : "Todas"}
            </button>
          </div>

          <div class="flex-1 overflow-y-auto p-2">
            <div class="space-y-0.5">
              {#each columnSelection as col}
                <button
                  onclick={() => (col.selected = !col.selected)}
                  disabled={isExporting}
                  class="group w-full px-2.5 py-1.5 text-left text-sm transition-all disabled:opacity-50 flex items-center gap-2 rounded
                    {col.selected
                    ? 'bg-[#2563eb]/10 text-[#e6edf3]'
                    : 'text-[#8b949e] hover:bg-[#161b22] hover:text-[#e6edf3]'}"
                >
                  <div
                    class="flex-shrink-0 w-3.5 h-3.5 rounded border transition-all flex items-center justify-center
                      {col.selected
                      ? 'bg-[#2563eb] border-[#2563eb]'
                      : 'border-[#484f58] group-hover:border-[#58a6ff]'}"
                  >
                    {#if col.selected}
                      <svg
                        class="w-2.5 h-2.5 text-white"
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
                  <span class="truncate">{col.name}</span>
                </button>
              {/each}
            </div>
          </div>

          {#if columnSelection.filter((c) => c.selected).length === 0}
            <div class="px-4 py-2.5 border-t border-[#30363d] bg-[#3b2f00]">
              <p class="text-xs text-[#d29922]">
                Selecciona al menos una columna
              </p>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
{/if}
