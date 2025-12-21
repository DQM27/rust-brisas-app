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
    Link2,
    Unlink,
    Lock,
    Unlock,
  } from "lucide-svelte";
  import type { ExportOptions } from "$lib/logic/export";
  import { currentUser } from "$lib/stores/auth";
  import { fade, fly } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";
  import PdfViewer from "./PdfViewer.svelte";

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
  let marginTop = $state(20); // En mm
  let marginBottom = $state(20);
  let marginLeft = $state(15);
  let marginRight = $state(15);
  let marginUnit = $state<"mm" | "cm" | "in" | "pt">("mm"); // Unidad de medida
  let paperSize = $state<"us-letter" | "a4" | "legal">("us-letter");
  let showPreview = $state(true);
  let bannerColor = $state("#059669"); // Color del banner

  // Unidades de medida disponibles
  const marginUnits = [
    { id: "mm", label: "mm" },
    { id: "cm", label: "cm" },
    { id: "in", label: "in" },
    { id: "pt", label: "pt" },
  ];

  // Colores predefinidos para el banner
  const bannerColors = [
    { id: "#059669", label: "Verde" },
    { id: "#2563eb", label: "Azul" },
    { id: "#7c3aed", label: "Violeta" },
    { id: "#dc2626", label: "Rojo" },
    { id: "#ea580c", label: "Naranja" },
    { id: "#0891b2", label: "Cyan" },
    { id: "#374151", label: "Gris" },
    { id: "#000000", label: "Negro" },
  ];

  // Convertir márgenes a cm para el backend
  function marginToCm(value: number): number {
    switch (marginUnit) {
      case "mm":
        return value / 10;
      case "in":
        return value * 2.54;
      case "pt":
        return value / 28.35; // 1cm = 28.35pt
      default:
        return value; // cm
    }
  }

  // Estado de vinculación de márgenes
  let linkVertical = $state(false); // Top <-> Bottom
  let linkHorizontal = $state(false); // Left <-> Right
  let linkAll = $state(false); // All 4

  // Actualizar márgenes con lógica de vinculación
  function updateMargin(
    type: "top" | "bottom" | "left" | "right",
    value: number,
  ) {
    // 1. Si "Link All" está activo, actualizar todos
    if (linkAll) {
      marginTop = value;
      marginBottom = value;
      marginLeft = value;
      marginRight = value;
      return;
    }

    // 2. Si es vertical y está vinculado
    if (linkVertical && (type === "top" || type === "bottom")) {
      marginTop = value;
      marginBottom = value;
      return;
    }

    // 3. Si es horizontal y está vinculado
    if (linkHorizontal && (type === "left" || type === "right")) {
      marginLeft = value;
      marginRight = value;
      return;
    }

    // 4. Individual
    if (type === "top") marginTop = value;
    if (type === "bottom") marginBottom = value;
    if (type === "left") marginLeft = value;
    if (type === "right") marginRight = value;
  }

  // Step y max según unidad
  const marginStep = $derived(
    marginUnit === "mm" ? 1 : marginUnit === "pt" ? 5 : 0.1,
  );
  const marginMax = $derived(
    marginUnit === "mm"
      ? 50
      : marginUnit === "pt"
        ? 150
        : marginUnit === "in"
          ? 2
          : 5,
  );

  let isExporting = $state(false);
  let isGeneratingPreview = $state(false);
  let previewData = $state<Uint8Array | null>(null);
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
    "Times New Roman": ["Times New Roman", "Times New Roman Bold"],
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
            marginTop: marginToCm(marginTop),
            marginBottom: marginToCm(marginBottom),
            marginLeft: marginToCm(marginLeft),
            marginRight: marginToCm(marginRight),
            bannerColor,
            generatedBy: $currentUser?.nombreCompleto || "",
            showPreview: true,
          },
        });

        if (result.bytes && result.bytes.length > 0) {
          // Guardar bytes directamente para pdf.js
          previewData = new Uint8Array(result.bytes);
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
    marginLeft;
    marginRight;
    marginBottom;
    bannerColor;
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
        showPreview: false,
        columnIds: columnSelection.filter((c) => c.selected).map((c) => c.id),
        marginTop: marginToCm(marginTop),
        marginBottom: marginToCm(marginBottom),
        marginLeft: marginToCm(marginLeft),
        marginRight: marginToCm(marginRight),
        bannerColor,
        generatedBy: $currentUser?.nombreCompleto || "",
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
    <div
      class="w-[320px] flex flex-col border-r border-[#30363d] bg-[#161b22] z-10 relative"
    >
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

        <!-- Color del Banner -->
        <div>
          <label
            for="adv-banner-color"
            class="block text-xs font-medium text-[#8b949e] mb-1"
          >
            Color del banner
          </label>
          <div class="flex items-center gap-2">
            <input
              type="color"
              id="adv-banner-color"
              bind:value={bannerColor}
              disabled={isExporting}
              class="w-8 h-8 rounded border border-[#30363d] cursor-pointer"
            />
            <select
              bind:value={bannerColor}
              disabled={isExporting}
              class="flex-1 px-2 py-1.5 text-sm rounded-md border border-[#30363d] bg-[#0d1117] text-[#e6edf3]"
            >
              {#each bannerColors as color}
                <option value={color.id}>{color.label}</option>
              {/each}
            </select>
          </div>
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
        <!-- Márgenes -->
        <div>
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-medium text-[#8b949e]"> Márgenes </span>
            <div class="flex items-center gap-2">
              <span class="text-xs text-[#8b949e]">Unidad:</span>
              <select
                bind:value={marginUnit}
                class="px-2 py-0.5 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3]"
              >
                {#each marginUnits as unit}
                  <option value={unit.id}>{unit.id}</option>
                {/each}
              </select>
            </div>
          </div>

          <div class="flex flex-col gap-2">
            <!-- Row 1: Vertical (Top - Link - Bottom) -->
            <div class="flex items-center justify-between gap-2">
              <!-- Top -->
              <div class="flex items-center gap-1 flex-1">
                <span class="text-xs text-[#8b949e] w-8">Arr:</span>
                <input
                  type="number"
                  step={marginStep}
                  min="0"
                  max={marginMax}
                  bind:value={marginTop}
                  oninput={(e) => updateMargin("top", +e.currentTarget.value)}
                  class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] min-w-0"
                />
              </div>

              <!-- Link Vertical Btn -->
              <button
                class="p-1 rounded text-[#8b949e] hover:bg-[#21262d] transition-colors"
                class:text-[#2563eb]={linkVertical && !linkAll}
                class:opacity-30={linkAll}
                disabled={linkAll}
                onclick={() => (linkVertical = !linkVertical)}
                title="Vincular Vertical"
              >
                {#if linkVertical && !linkAll}
                  <Link2 size={14} />
                {:else}
                  <Unlink size={14} />
                {/if}
              </button>

              <!-- Bottom -->
              <div class="flex items-center gap-1 flex-1">
                <span class="text-xs text-[#8b949e] w-8">Aba:</span>
                <input
                  type="number"
                  step={marginStep}
                  min="0"
                  max={marginMax}
                  bind:value={marginBottom}
                  oninput={(e) =>
                    updateMargin("bottom", +e.currentTarget.value)}
                  class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] min-w-0"
                />
              </div>
            </div>

            <!-- Row 2: Link All (Center) -->
            <div class="flex justify-center -my-1 relative z-10">
              <button
                class="p-1 rounded-full bg-[#161b22] border border-[#30363d] text-[#8b949e] hover:text-[#e6edf3] hover:border-[#8b949e] transition-colors"
                class:text-[#2563eb]={linkAll}
                class:border-[#2563eb]={linkAll}
                onclick={() => {
                  linkAll = !linkAll;
                  if (linkAll) {
                    // Sync all to Top
                    updateMargin("top", marginTop);
                  }
                }}
                title="Vincular Todos"
              >
                {#if linkAll}
                  <Lock size={12} />
                {:else}
                  <Unlock size={12} />
                {/if}
              </button>
            </div>

            <!-- Row 3: Horizontal (Left - Link - Right) -->
            <div class="flex items-center justify-between gap-2">
              <!-- Left -->
              <div class="flex items-center gap-1 flex-1">
                <span class="text-xs text-[#8b949e] w-8">Izq:</span>
                <input
                  type="number"
                  step={marginStep}
                  min="0"
                  max={marginMax}
                  bind:value={marginLeft}
                  oninput={(e) => updateMargin("left", +e.currentTarget.value)}
                  class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] min-w-0"
                />
              </div>

              <!-- Link Horizontal Btn -->
              <button
                class="p-1 rounded text-[#8b949e] hover:bg-[#21262d] transition-colors"
                class:text-[#2563eb]={linkHorizontal && !linkAll}
                class:opacity-30={linkAll}
                disabled={linkAll}
                onclick={() => (linkHorizontal = !linkHorizontal)}
                title="Vincular Horizontal"
              >
                {#if linkHorizontal && !linkAll}
                  <Link2 size={14} />
                {:else}
                  <Unlink size={14} />
                {/if}
              </button>

              <!-- Right -->
              <div class="flex items-center gap-1 flex-1">
                <span class="text-xs text-[#8b949e] w-8">Der:</span>
                <input
                  type="number"
                  step={marginStep}
                  min="0"
                  max={marginMax}
                  bind:value={marginRight}
                  oninput={(e) => updateMargin("right", +e.currentTarget.value)}
                  class="flex-1 px-2 py-1 text-xs rounded border border-[#30363d] bg-[#0d1117] text-[#e6edf3] min-w-0"
                />
              </div>
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

    <!-- Panel Derecho: Vista Previa con pdf.js -->
    <div
      class="flex-1 min-w-0 flex flex-col bg-[#0d1117] overflow-hidden isolate"
    >
      <!-- Header Preview -->
      <div
        class="px-4 py-2 border-b border-[#30363d] flex items-center justify-between"
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

      <!-- PdfViewer o estados alternativos -->
      <div class="flex-1 relative overflow-hidden">
        {#if previewData}
          <PdfViewer
            pdfData={previewData}
            onError={(err) => (previewError = err)}
          />
        {:else if isGeneratingPreview}
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div
                class="w-8 h-8 border-2 border-[#30363d] border-t-[#2563eb] rounded-full animate-spin mx-auto mb-2"
              ></div>
              <p class="text-xs text-[#8b949e]">Generando preview...</p>
            </div>
          </div>
        {:else if previewError}
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <p class="text-sm text-[#f85149]">{previewError}</p>
              <button
                onclick={generatePreview}
                class="mt-2 text-xs text-[#58a6ff] hover:underline"
              >
                Reintentar
              </button>
            </div>
          </div>
        {:else}
          <div
            class="absolute inset-0 flex items-center justify-center text-center text-[#8b949e]"
          >
            <div>
              <FileText size={48} class="mx-auto mb-2 opacity-30" />
              <p class="text-sm">Vista previa del documento</p>
              <p class="text-xs mt-1 opacity-60">
                Scroll: zoom · Espacio+drag: mover
              </p>
            </div>
          </div>
        {/if}

        <!-- Overlay de actualización -->
        {#if isGeneratingPreview && previewData}
          <div
            class="absolute top-2 right-2 px-2 py-1 bg-[#161b22] rounded text-xs text-[#8b949e] border border-[#30363d]"
          >
            <RefreshCw size={12} class="inline animate-spin mr-1" />
            Actualizando...
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
