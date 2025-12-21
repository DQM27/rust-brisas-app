<!-- src/lib/components/export/ExportAdvancedDialog.svelte -->
<script lang="ts">
  // @ts-nocheck - Svelte 5 runes not recognized by TS
  import {
    X,
    FileText,
    Download,
    Settings,
    Type,
    Maximize,
    Columns,
    RefreshCw,
  } from "lucide-svelte";
  import type { ExportOptions } from "$lib/logic/export";
  import { fade, fly } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    onExport: (options: ExportOptions) => Promise<void>;
    onClose: () => void;
    onBack: () => void;
    columns: { id: string; name: string; selected: boolean }[];
    initialOptions: Partial<ExportOptions>;
    rows: Record<string, any>[];
    headers: string[];
  }

  let {
    onExport,
    onClose,
    onBack,
    columns,
    initialOptions,
    rows,
    headers,
  }: Props = $props();

  // Estado de columnas
  let columnSelection = $state<
    { id: string; name: string; selected: boolean }[]
  >([]);

  $effect(() => {
    columnSelection = columns.map((c) => ({ ...c, selected: c.selected }));
  });

  // Configuración
  let title = $state(initialOptions.title || "Reporte");
  let orientation = $state<"portrait" | "landscape">(
    initialOptions.orientation || "landscape",
  );
  let fontSize = $state(initialOptions.fontSize || 10);
  let fontFamily = $state(initialOptions.fontFamily || "Inter");
  let marginTop = $state(2.0);
  let marginBottom = $state(2.0);
  let marginLeft = $state(1.5);
  let marginRight = $state(1.5);
  let paperSize = $state<"us-letter" | "a4" | "legal">("us-letter");
  let showPreview = $state(true);

  let isExporting = $state(false);
  let isGeneratingPreview = $state(false);
  let previewUrl = $state<string | null>(null);
  let previewError = $state<string | null>(null);

  // Fuentes disponibles con variantes
  const fontVariants = {
    Inter: [
      "Inter",
      "Inter Light",
      "Inter Medium",
      "Inter SemiBold",
      "Inter Bold",
    ],
    Arial: ["Arial", "Arial Bold"],
    "Segoe UI": ["Segoe UI", "Segoe UI Light", "Segoe UI Semibold"],
    Calibri: ["Calibri", "Calibri Light", "Calibri Bold"],
    Times: ["Times", "Times Bold"],
  };

  const paperSizes = [
    { id: "us-letter", label: "Carta (US Letter)" },
    { id: "a4", label: "A4" },
    { id: "legal", label: "Legal" },
  ];

  // Generar preview (debounced)
  let previewTimeout: ReturnType<typeof setTimeout>;

  async function generatePreview() {
    if (previewTimeout) clearTimeout(previewTimeout);

    previewTimeout = setTimeout(async () => {
      isGeneratingPreview = true;
      previewError = null;

      try {
        const selectedHeaders = columnSelection
          .filter((c) => c.selected)
          .map((c) => c.name);

        // Validar que haya datos
        if (!rows || rows.length === 0) {
          previewError = "No hay filas de datos disponibles";
          isGeneratingPreview = false;
          return;
        }

        if (!selectedHeaders || selectedHeaders.length === 0) {
          previewError = "Selecciona al menos una columna";
          isGeneratingPreview = false;
          return;
        }

        // Preparar datos para preview (máximo 5 filas)
        const previewRows = rows.slice(0, 5);

        const result = await invoke<{
          bytes?: number[];
          success: boolean;
          message?: string;
        }>("export_preview", {
          request: {
            format: "pdf",
            headers: selectedHeaders,
            rows: previewRows,
            title,
            orientation,
            fontSize,
            fontFamily,
            showPreview: true,
          },
        });

        if (result.bytes && result.bytes.length > 0) {
          const blob = new Blob([new Uint8Array(result.bytes)], {
            type: "application/pdf",
          });
          if (previewUrl) URL.revokeObjectURL(previewUrl);
          previewUrl = URL.createObjectURL(blob);
        } else {
          previewError = result.message || "Error generando PDF";
        }
      } catch (e: any) {
        previewError = e.message || "Error generando preview";
        console.error("Error en preview:", e);
      } finally {
        isGeneratingPreview = false;
      }
    }, 500);
  }

  // Regenerar preview cuando cambian opciones
  $effect(() => {
    // Dependencies
    title;
    orientation;
    fontSize;
    fontFamily;
    marginTop;
    paperSize;
    columnSelection.filter((c) => c.selected).length;
    generatePreview();
  });

  async function handleExport() {
    isExporting = true;

    try {
      const options: ExportOptions = {
        title: title.trim() || "Reporte",
        orientation,
        fontSize,
        fontFamily,
        showPreview,
        columnIds: columnSelection.filter((c) => c.selected).map((c) => c.id),
        marginTop,
        marginBottom,
        marginLeft,
        marginRight,
      };

      await onExport(options);
      onClose();
    } catch (error) {
      console.error("Error exportando:", error);
      alert("Error al exportar: " + (error as Error).message);
    } finally {
      isExporting = false;
    }
  }
</script>

<div
  class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 p-4"
  transition:fade={{ duration: 150 }}
  role="presentation"
  tabindex="-1"
>
  <div
    class="bg-[#161b22] rounded-lg border border-[#30363d] shadow-2xl flex flex-row w-[1100px] max-w-[98vw] h-[750px] max-h-[95vh] overflow-hidden"
    transition:fly={{ y: 20, duration: 200 }}
    onclick={(e) => e.stopPropagation()}
    onkeydown={() => {}}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- Panel Izquierdo: Configuración -->
    <div class="w-[320px] flex flex-col border-r border-[#30363d]">
      <!-- Header -->
      <div
        class="px-4 py-3 border-b border-[#30363d] flex items-center justify-between"
      >
        <div class="flex items-center gap-2">
          <button
            onclick={onBack}
            disabled={isExporting}
            class="p-1.5 rounded-md text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#21262d] transition-colors"
            aria-label="Volver"
          >
            ←
          </button>
          <h2 class="text-sm font-semibold text-[#e6edf3]">
            Configuración Avanzada
          </h2>
        </div>
        <button
          onclick={onClose}
          disabled={isExporting}
          class="p-1.5 rounded-md text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#21262d] transition-colors"
          aria-label="Cerrar"
        >
          <X size={16} />
        </button>
      </div>

      <!-- Contenido scrolleable -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">
        <!-- Título -->
        <div>
          <label
            for="adv-title"
            class="block text-xs font-medium text-[#8b949e] mb-1"
          >
            Título
          </label>
          <input
            id="adv-title"
            type="text"
            bind:value={title}
            disabled={isExporting}
            class="w-full px-3 py-1.5 text-sm rounded-md border border-[#30363d] bg-[#0d1117] text-[#e6edf3] focus:ring-1 focus:ring-[#2563eb] focus:border-[#2563eb]"
          />
        </div>

        <!-- Papel + Orientación -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label
              for="adv-paper"
              class="block text-xs font-medium text-[#8b949e] mb-1"
            >
              Papel
            </label>
            <select
              id="adv-paper"
              bind:value={paperSize}
              disabled={isExporting}
              class="w-full px-2 py-1.5 text-sm rounded-md border border-[#30363d] bg-[#0d1117] text-[#e6edf3]"
            >
              {#each paperSizes as size}
                <option value={size.id}>{size.label}</option>
              {/each}
            </select>
          </div>
          <div>
            <label
              for="adv-orientation"
              class="block text-xs font-medium text-[#8b949e] mb-1"
            >
              Orientación
            </label>
            <select
              id="adv-orientation"
              bind:value={orientation}
              disabled={isExporting}
              class="w-full px-2 py-1.5 text-sm rounded-md border border-[#30363d] bg-[#0d1117] text-[#e6edf3]"
            >
              <option value="landscape">Horizontal</option>
              <option value="portrait">Vertical</option>
            </select>
          </div>
        </div>

        <!-- Fuente -->
        <div>
          <label
            for="adv-font"
            class="block text-xs font-medium text-[#8b949e] mb-1"
          >
            Fuente
          </label>
          <select
            id="adv-font"
            bind:value={fontFamily}
            disabled={isExporting}
            class="w-full px-2 py-1.5 text-sm rounded-md border border-[#30363d] bg-[#0d1117] text-[#e6edf3]"
          >
            {#each Object.entries(fontVariants) as [family, variants]}
              <optgroup label={family}>
                {#each variants as variant}
                  <option value={variant}>{variant}</option>
                {/each}
              </optgroup>
            {/each}
          </select>
        </div>

        <!-- Tamaño de texto -->
        <div>
          <label
            for="adv-fontsize"
            class="block text-xs font-medium text-[#8b949e] mb-1"
          >
            Tamaño de texto ({fontSize}pt)
          </label>
          <input
            id="adv-fontsize"
            type="range"
            min="8"
            max="20"
            bind:value={fontSize}
            disabled={isExporting}
            class="w-full h-2 bg-[#30363d] rounded-lg appearance-none cursor-pointer accent-[#2563eb]"
          />
        </div>

        <!-- Márgenes -->
        <div>
          <span class="block text-xs font-medium text-[#8b949e] mb-2">
            Márgenes (cm)
          </span>
          <div class="grid grid-cols-2 gap-2">
            <div class="flex items-center gap-1">
              <span class="text-xs text-[#8b949e] w-8">Arr:</span>
              <input
                type="number"
                step="0.1"
                min="0"
                max="5"
                bind:value={marginTop}
                class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] w-16"
              />
            </div>
            <div class="flex items-center gap-1">
              <span class="text-xs text-[#8b949e] w-8">Aba:</span>
              <input
                type="number"
                step="0.1"
                min="0"
                max="5"
                bind:value={marginBottom}
                class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] w-16"
              />
            </div>
            <div class="flex items-center gap-1">
              <span class="text-xs text-[#8b949e] w-8">Izq:</span>
              <input
                type="number"
                step="0.1"
                min="0"
                max="5"
                bind:value={marginLeft}
                class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] w-16"
              />
            </div>
            <div class="flex items-center gap-1">
              <span class="text-xs text-[#8b949e] w-8">Der:</span>
              <input
                type="number"
                step="0.1"
                min="0"
                max="5"
                bind:value={marginRight}
                class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] w-16"
              />
            </div>
          </div>
        </div>

        <!-- Columnas (compacto) -->
        <div>
          <span class="block text-xs font-medium text-[#8b949e] mb-2">
            Columnas ({columnSelection.filter((c) => c.selected)
              .length}/{columnSelection.length})
          </span>
          <div
            class="max-h-32 overflow-y-auto bg-[#0d1117] border border-[#30363d] rounded-md p-2 space-y-1"
          >
            {#each columnSelection as col}
              <label class="flex items-center gap-2 text-xs cursor-pointer">
                <input
                  type="checkbox"
                  bind:checked={col.selected}
                  class="w-3 h-3 rounded border-[#30363d] bg-[#0d1117] text-[#2563eb]"
                />
                <span class="text-[#e6edf3] truncate">{col.name}</span>
              </label>
            {/each}
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="px-4 py-3 border-t border-[#30363d] flex gap-2">
        <button
          onclick={onBack}
          disabled={isExporting}
          class="flex-1 px-3 py-2 text-sm font-medium rounded-md border border-[#30363d] bg-[#21262d] text-[#e6edf3] hover:bg-[#30363d] transition-colors"
        >
          Cancelar
        </button>
        <button
          onclick={handleExport}
          disabled={isExporting ||
            columnSelection.filter((c) => c.selected).length === 0}
          class="flex-1 px-3 py-2 text-sm font-medium rounded-md bg-[#2563eb] hover:bg-[#1d4ed8] text-white transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
        >
          {#if isExporting}
            <div
              class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
            ></div>
          {:else}
            <Download size={14} />
          {/if}
          Exportar
        </button>
      </div>
    </div>

    <!-- Panel Derecho: Vista Previa -->
    <div class="flex-1 flex flex-col bg-[#0d1117]">
      <!-- Header Preview -->
      <div
        class="px-4 py-3 border-b border-[#30363d] flex items-center justify-between"
      >
        <span class="text-sm font-medium text-[#8b949e]">Vista Previa</span>
        <button
          onclick={generatePreview}
          disabled={isGeneratingPreview}
          class="p-1.5 rounded-md text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#21262d] transition-colors"
          aria-label="Regenerar preview"
        >
          <RefreshCw
            size={14}
            class={isGeneratingPreview ? "animate-spin" : ""}
          />
        </button>
      </div>

      <!-- Preview Content -->
      <div class="flex-1 flex items-center justify-center p-4 relative">
        <!-- Iframe siempre visible si hay URL -->
        {#if previewUrl}
          <iframe
            src={previewUrl}
            class="w-full h-full rounded-md border border-[#30363d] bg-white transition-opacity duration-200"
            class:opacity-50={isGeneratingPreview}
            title="Vista previa del PDF"
          ></iframe>
        {/if}

        <!-- Spinner superpuesto mientras genera -->
        {#if isGeneratingPreview}
          <div
            class="absolute inset-0 flex items-center justify-center bg-[#0d1117]/50 rounded-md"
          >
            <div
              class="text-center bg-[#161b22] px-4 py-3 rounded-lg border border-[#30363d] shadow-lg"
            >
              <div
                class="w-6 h-6 border-2 border-[#30363d] border-t-[#2563eb] rounded-full animate-spin mx-auto mb-2"
              ></div>
              <p class="text-xs text-[#8b949e]">Actualizando...</p>
            </div>
          </div>
        {/if}

        <!-- Mensaje de error -->
        {#if previewError && !previewUrl}
          <div class="text-center">
            <p class="text-sm text-[#f85149]">{previewError}</p>
            <button
              onclick={generatePreview}
              class="mt-2 text-xs text-[#58a6ff] hover:underline"
            >
              Reintentar
            </button>
          </div>
        {/if}

        <!-- Estado inicial sin preview -->
        {#if !previewUrl && !isGeneratingPreview && !previewError}
          <div class="text-center text-[#8b949e]">
            <FileText size={48} class="mx-auto mb-2 opacity-30" />
            <p class="text-sm">Vista previa del documento</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
