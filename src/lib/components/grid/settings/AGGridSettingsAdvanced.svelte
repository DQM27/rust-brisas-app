<script lang="ts">
  // @ts-nocheck
  import type { GridApi } from "@ag-grid-community/core";
  import type { GridId } from "$lib/types/agGrid";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import {
    Trash2,
    Layers,
    Undo2,
    Zap,
    Download,
    Upload,
    Copy,
    Check,
    FileDown,
    FileUp,
    AlertTriangle,
  } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  // Estado UI directo del store
  let copyWithHeaders = $derived(agGridSettings.getCopyWithHeaders(gridId));
  let suppressContextMenu = $derived(
    agGridSettings.getSuppressContextMenu(gridId),
  );
  let enableUndoRedo = $derived(
    agGridSettings.getEnableUndoRedo?.(gridId) ?? false,
  );
  let rowBuffer = $derived(agGridSettings.getRowBuffer?.(gridId) ?? 10);
  let debounceScroll = $derived(
    agGridSettings.getDebounceScroll?.(gridId) ?? true,
  );

  // Estados locales
  let confirmDelete = $state(agGridSettings.getConfirmDelete?.() ?? true);
  let confirmBulk = $state(agGridSettings.getConfirmBulkOperations?.() ?? true);
  let dontAskAgain = $state(false);

  let exportedConfig = $state("");
  let importJson = $state("");
  let copySuccess = $state(false);
  let importError = $state("");
  let importSuccess = $state(false);

  const rowBufferOptions = [5, 10, 20, 50];

  // Handlers
  function toggleCopyWithHeaders(checked: boolean) {
    agGridSettings.setCopyWithHeaders(gridId, checked);
    gridApi?.setGridOption("copyHeadersToClipboard", checked);
  }

  function toggleContextMenu(checked: boolean) {
    // Note: Logic inverted in original (suppress is true to hide)
    // Here checked means "Hide menu" (Suppress = true)
    agGridSettings.setSuppressContextMenu(gridId, checked);
    gridApi?.setGridOption("suppressContextMenu", checked);
  }

  function handleConfirmDelete(checked: boolean) {
    confirmDelete = checked;
    agGridSettings.setConfirmDelete?.(checked);
    if (checked) dontAskAgain = false;
  }

  function handleConfirmBulk(checked: boolean) {
    confirmBulk = checked;
    agGridSettings.setConfirmBulkOperations?.(checked);
    if (checked) dontAskAgain = false;
  }

  function handleDontAskAgain(checked: boolean) {
    dontAskAgain = checked;
    if (checked) {
      handleConfirmDelete(false);
      handleConfirmBulk(false);
    } else {
      handleConfirmDelete(true);
      handleConfirmBulk(true);
    }
  }

  function toggleUndoRedo(checked: boolean) {
    agGridSettings.setEnableUndoRedo?.(gridId, checked);
    if (gridApi) {
      gridApi.setGridOption("undoRedoCellEditing", checked);
      if (checked) gridApi.setGridOption("undoRedoCellEditingLimit", 20);
    }
  }

  function setRowBuffer(value: number) {
    agGridSettings.setRowBuffer?.(gridId, value);
    gridApi?.setGridOption("rowBuffer", value);
  }

  function toggleDebounceScroll(checked: boolean) {
    agGridSettings.setDebounceScroll?.(gridId, checked);
    gridApi?.setGridOption("debounceVerticalScrollbar", checked);
  }

  // Export/Import Lógica (sin cambios funcionales, solo estilo)
  function exportConfig() {
    const config = agGridSettings.exportSettings?.(gridId);
    if (config) exportedConfig = JSON.stringify(config, null, 2);
  }

  function copyToClipboard() {
    if (exportedConfig) {
      navigator.clipboard.writeText(exportedConfig);
      copySuccess = true;
      setTimeout(() => (copySuccess = false), 2000);
    }
  }

  function downloadConfig() {
    if (exportedConfig) {
      const blob = new Blob([exportedConfig], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `grid-config-${gridId}-${new Date().toISOString().split("T")[0]}.json`;
      a.click();
      URL.revokeObjectURL(url);
    }
  }

  function importConfig() {
    importError = "";
    importSuccess = false;
    if (!importJson.trim()) {
      importError = "Ingresa un JSON válido";
      return;
    }
    try {
      const config = JSON.parse(importJson);
      if (!config || typeof config !== "object")
        throw new Error("Configuración inválida");
      agGridSettings.importSettings?.(gridId, config);
      importSuccess = true;
      importJson = "";
      gridApi?.refreshCells();
      setTimeout(() => (importSuccess = false), 3000);
    } catch (e) {
      importError = e instanceof Error ? e.message : "Error al parsear JSON";
    }
  }

  function handleFileUpload(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => (importJson = (e.target?.result as string) || "");
      reader.readAsText(file);
    }
  }

  const sectionClass = "space-y-4 p-1";
  const labelClass = "block text-xs font-medium text-zinc-400 mb-1.5 ml-0.5";
  const checkboxClass =
    "w-4 h-4 rounded bg-black/20 border-zinc-600 text-blue-600 focus:ring-blue-600 focus:ring-offset-0 transition-all checked:bg-blue-600 checked:border-blue-600 cursor-pointer flex-shrink-0";
  const groupClass =
    "space-y-3 p-3 rounded-lg bg-black/10 border border-white/5";
</script>

<div class={sectionClass}>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- Column 1: Behavior & Confirmations -->
    <div class="space-y-4">
      <h3
        class="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2"
      >
        Comportamiento
      </h3>

      <div class={groupClass}>
        <!-- Undo/Redo -->
        <label class="flex items-center gap-3 cursor-pointer group">
          <input
            type="checkbox"
            checked={enableUndoRedo}
            onchange={(e) => toggleUndoRedo(e.currentTarget.checked)}
            class={checkboxClass}
          />
          <div class="flex flex-col">
            <span
              class="text-xs text-zinc-200 group-hover:text-white flex items-center gap-2"
            >
              <Undo2 size={12} class="text-purple-400" /> Deshacer / Rehacer
            </span>
            <span class="text-[10px] text-zinc-500"
              >Habilitar Ctrl+Z / Ctrl+Y</span
            >
          </div>
        </label>

        <!-- Copy Headers -->
        <label class="flex items-center gap-3 cursor-pointer group">
          <input
            type="checkbox"
            checked={copyWithHeaders}
            onchange={(e) => toggleCopyWithHeaders(e.currentTarget.checked)}
            class={checkboxClass}
          />
          <div class="flex flex-col">
            <span class="text-xs text-zinc-200 group-hover:text-white"
              >Copiar Encabezados</span
            >
            <span class="text-[10px] text-zinc-500"
              >Incluir al copiar filas</span
            >
          </div>
        </label>

        <!-- Suppress Context Menu -->
        <label class="flex items-center gap-3 cursor-pointer group">
          <input
            type="checkbox"
            checked={suppressContextMenu}
            onchange={(e) => toggleContextMenu(e.currentTarget.checked)}
            class={checkboxClass}
          />
          <div class="flex flex-col">
            <span class="text-xs text-zinc-200 group-hover:text-white"
              >Ocultar Menú AG Grid</span
            >
            <span class="text-[10px] text-zinc-500"
              >Usar menú nativo del navegador</span
            >
          </div>
        </label>
      </div>

      <div class={groupClass}>
        <div class="flex items-center gap-2 mb-2">
          <AlertTriangle size={12} class="text-yellow-500" />
          <span class="text-xs font-medium text-zinc-300">Confirmaciones</span>
        </div>

        <label class="flex items-center gap-3 cursor-pointer group">
          <input
            type="checkbox"
            checked={confirmDelete}
            disabled={dontAskAgain}
            onchange={(e) => handleConfirmDelete(e.currentTarget.checked)}
            class={checkboxClass}
          />
          <span
            class="text-xs text-zinc-200 group-hover:text-white {dontAskAgain
              ? 'opacity-50'
              : ''}">Eliminar registros</span
          >
        </label>

        <label class="flex items-center gap-3 cursor-pointer group">
          <input
            type="checkbox"
            checked={confirmBulk}
            disabled={dontAskAgain}
            onchange={(e) => handleConfirmBulk(e.currentTarget.checked)}
            class={checkboxClass}
          />
          <span
            class="text-xs text-zinc-200 group-hover:text-white {dontAskAgain
              ? 'opacity-50'
              : ''}">Operaciones masivas</span
          >
        </label>

        <label
          class="flex items-center gap-3 cursor-pointer group pt-1 border-t border-white/5"
        >
          <input
            type="checkbox"
            checked={dontAskAgain}
            onchange={(e) => handleDontAskAgain(e.currentTarget.checked)}
            class={checkboxClass}
          />
          <span class="text-xs text-yellow-400 group-hover:text-yellow-300"
            >Desactivar todas (No preguntar)</span
          >
        </label>
      </div>
    </div>

    <!-- Column 2: Performance & Backup -->
    <div class="space-y-4">
      <!-- Performance -->
      <div>
        <h3
          class="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2 flex items-center gap-2"
        >
          <Zap size={12} /> Rendimiento
        </h3>
        <div class={groupClass}>
          <div class="space-y-2">
            <span class={labelClass}>Buffer de Filas</span>
            <div
              class="grid grid-cols-4 gap-1"
              role="group"
              aria-label="Buffer de filas"
            >
              {#each rowBufferOptions as option}
                <button
                  type="button"
                  onclick={() => setRowBuffer(option)}
                  class="py-1 text-[10px] font-medium rounded transition-all border
                            {rowBuffer === option
                    ? 'bg-blue-600/20 text-blue-400 border-blue-500/50'
                    : 'bg-black/20 text-zinc-400 border-white/10 hover:border-white/20 hover:text-white'}"
                >
                  {option}
                </button>
              {/each}
            </div>
          </div>

          <label class="flex items-center gap-3 cursor-pointer group mt-2">
            <input
              type="checkbox"
              checked={debounceScroll}
              onchange={(e) => toggleDebounceScroll(e.currentTarget.checked)}
              class={checkboxClass}
            />
            <div class="flex flex-col">
              <span class="text-xs text-zinc-200 group-hover:text-white"
                >Suavizar Scroll</span
              >
              <span class="text-[10px] text-zinc-500"
                >Reduce parpadeo al scrollear</span
              >
            </div>
          </label>
        </div>
      </div>

      <!-- Import/Export -->
      <div>
        <h3
          class="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2 flex items-center gap-2"
        >
          <Download size={12} /> Respaldo
        </h3>

        <div class="space-y-2">
          <!-- Actions -->
          <div class="grid grid-cols-2 gap-2">
            <label
              class="flex items-center justify-center gap-2 py-2 rounded-md bg-black/20 border border-white/10 text-xs text-zinc-400 hover:text-white hover:bg-white/5 cursor-pointer transition-all"
            >
              <FileUp size={14} />
              Cargar
              <input
                type="file"
                accept=".json"
                onchange={handleFileUpload}
                class="hidden"
              />
            </label>
            <button
              onclick={exportConfig}
              class="flex items-center justify-center gap-2 py-2 rounded-md bg-black/20 border border-white/10 text-xs text-zinc-400 hover:text-white hover:bg-white/5 transition-all"
            >
              <FileDown size={14} />
              Generar
            </button>
          </div>

          <!-- TextArea Area (Shared for Export/Import view) -->
          <div class="relative">
            <textarea
              value={importJson || exportedConfig}
              oninput={(e) => (importJson = e.currentTarget.value)}
              placeholder="Configuración JSON..."
              class="w-full h-24 p-2 text-[10px] font-mono bg-black/30 border border-white/10 rounded-lg text-zinc-300 resize-none focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20"
            ></textarea>
            {#if exportedConfig && !importJson}
              <div class="absolute bottom-2 right-2 flex gap-1">
                <button
                  onclick={copyToClipboard}
                  class="p-1 px-2 rounded bg-zinc-800 text-[10px] text-white hover:bg-zinc-700 flex items-center gap-1"
                >
                  {#if copySuccess}
                    <Check size={10} />
                  {:else}
                    <Copy size={10} />
                  {/if}
                  {copySuccess ? "Copiado" : "Copiar"}
                </button>
                <button
                  onclick={downloadConfig}
                  class="p-1 px-2 rounded bg-blue-600 text-[10px] text-white hover:bg-blue-500"
                >
                  Descargar
                </button>
              </div>
            {/if}
            {#if importJson}
              <button
                onclick={importConfig}
                disabled={!importJson.trim()}
                class="absolute bottom-2 right-2 px-3 py-1 rounded bg-green-600 text-[10px] text-white font-medium hover:bg-green-500 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Aplicar Importación
              </button>
            {/if}
          </div>

          {#if importError}
            <p class="text-[10px] text-red-500 flex items-center gap-1">
              <AlertTriangle size={10} />
              {importError}
            </p>
          {/if}
          {#if importSuccess}
            <p class="text-[10px] text-green-500 flex items-center gap-1">
              <Check size={10} /> Importado correctamente
            </p>
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
