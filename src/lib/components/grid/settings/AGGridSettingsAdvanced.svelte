<!-- src/lib/components/grid/settings/AGGridSettingsAdvanced.svelte -->
<script lang="ts">
  // @ts-nocheck
  import type { GridApi } from "@ag-grid-community/core";
  import type { GridId } from "$lib/types/agGrid";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import {
    AlertTriangle,
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
    Info,
  } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  // ============================================
  // Estado de Confirmaciones
  // ============================================
  let confirmDelete = $state(agGridSettings.getConfirmDelete?.() ?? true);
  let confirmBulk = $state(agGridSettings.getConfirmBulkOperations?.() ?? true);
  let dontAskAgain = $state(false);

  // ============================================
  // Estado de Undo/Redo
  // ============================================
  let enableUndoRedo = $derived(
    agGridSettings.getEnableUndoRedo?.(gridId) ?? false,
  );

  // ============================================
  // Estado de Performance
  // ============================================
  let rowBuffer = $derived(agGridSettings.getRowBuffer?.(gridId) ?? 10);
  let debounceScroll = $derived(
    agGridSettings.getDebounceScroll?.(gridId) ?? true,
  );

  const rowBufferOptions = [5, 10, 20, 50];

  // ============================================
  // Estado de Export/Import
  // ============================================
  let exportedConfig = $state("");
  let importJson = $state("");
  let copySuccess = $state(false);
  let importError = $state("");
  let importSuccess = $state(false);

  // Placeholder para el textarea de importar
  const importPlaceholder = '{"gridId": "...", "theme": "...", ...}';

  // ============================================
  // Handlers de Confirmaciones
  // ============================================
  function toggleConfirmDelete() {
    confirmDelete = !confirmDelete;
    agGridSettings.setConfirmDelete?.(confirmDelete);
  }

  function toggleConfirmBulk() {
    confirmBulk = !confirmBulk;
    agGridSettings.setConfirmBulkOperations?.(confirmBulk);
  }

  function toggleDontAskAgain() {
    dontAskAgain = !dontAskAgain;
    if (dontAskAgain) {
      confirmDelete = false;
      confirmBulk = false;
      agGridSettings.setConfirmDelete?.(false);
      agGridSettings.setConfirmBulkOperations?.(false);
    }
  }

  // ============================================
  // Handlers de Undo/Redo
  // ============================================
  function toggleUndoRedo() {
    enableUndoRedo = !enableUndoRedo;
    agGridSettings.setEnableUndoRedo?.(gridId, enableUndoRedo);

    if (gridApi) {
      gridApi.setGridOption("undoRedoCellEditing", enableUndoRedo);
      if (enableUndoRedo) {
        gridApi.setGridOption("undoRedoCellEditingLimit", 20);
      }
    }
  }

  // ============================================
  // Handlers de Performance
  // ============================================
  function setRowBuffer(value: number) {
    rowBuffer = value;
    agGridSettings.setRowBuffer?.(gridId, value);

    if (gridApi) {
      gridApi.setGridOption("rowBuffer", value);
    }
  }

  function toggleDebounceScroll() {
    debounceScroll = !debounceScroll;
    agGridSettings.setDebounceScroll?.(gridId, debounceScroll);

    if (gridApi) {
      gridApi.setGridOption("debounceVerticalScrollbar", debounceScroll);
    }
  }

  // ============================================
  // Handlers de Export/Import
  // ============================================
  function exportConfig() {
    const config = agGridSettings.exportSettings?.(gridId);
    if (config) {
      exportedConfig = JSON.stringify(config, null, 2);
    }
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
      importError = "Por favor, ingresa o pega una configuración JSON";
      return;
    }

    try {
      const config = JSON.parse(importJson);

      // Validar estructura básica
      if (!config || typeof config !== "object") {
        throw new Error("Configuración inválida");
      }

      agGridSettings.importSettings?.(gridId, config);
      importSuccess = true;
      importJson = "";

      // Refrescar la grid
      if (gridApi) {
        gridApi.refreshCells();
      }

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
      reader.onload = (e) => {
        importJson = (e.target?.result as string) || "";
      };
      reader.readAsText(file);
    }
  }
</script>

<div class="space-y-6">
  <!-- ============================================ -->
  <!-- Sección: Confirmaciones -->
  <!-- ============================================ -->
  <div class="space-y-3">
    <div class="flex items-center gap-2 text-white text-sm font-medium">
      <AlertTriangle size={16} class="text-amber-400" />
      Confirmaciones
    </div>

    <div class="space-y-2 pl-6">
      <!-- Confirmar eliminación -->
      <button
        onclick={toggleConfirmDelete}
        class="w-full flex items-center justify-between p-3 bg-[#1e1e1e] border border-white/10 rounded-lg
          hover:border-white/20 transition-colors group"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-red-500/10">
            <Trash2 size={16} class="text-red-400" />
          </div>
          <div class="text-left">
            <div class="text-white text-sm">Confirmar eliminación</div>
            <div class="text-gray-500 text-xs">
              Pedir confirmación antes de eliminar registros
            </div>
          </div>
        </div>
        <div
          class={`w-10 h-6 rounded-full transition-colors ${confirmDelete ? "bg-blue-500" : "bg-gray-600"}`}
        >
          <div
            class={`w-4 h-4 mt-1 rounded-full bg-white transition-transform ${confirmDelete ? "translate-x-5" : "translate-x-1"}`}
          ></div>
        </div>
      </button>

      <!-- Confirmar operaciones masivas -->
      <button
        onclick={toggleConfirmBulk}
        class="w-full flex items-center justify-between p-3 bg-[#1e1e1e] border border-white/10 rounded-lg
          hover:border-white/20 transition-colors group"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-amber-500/10">
            <Layers size={16} class="text-amber-400" />
          </div>
          <div class="text-left">
            <div class="text-white text-sm">Confirmar operaciones masivas</div>
            <div class="text-gray-500 text-xs">
              Pedir confirmación para acciones que afecten múltiples registros
            </div>
          </div>
        </div>
        <div
          class={`w-10 h-6 rounded-full transition-colors ${confirmBulk ? "bg-blue-500" : "bg-gray-600"}`}
        >
          <div
            class={`w-4 h-4 mt-1 rounded-full bg-white transition-transform ${confirmBulk ? "translate-x-5" : "translate-x-1"}`}
          ></div>
        </div>
      </button>

      <!-- No volver a preguntar -->
      <div class="pl-4 pt-1">
        <button
          onclick={toggleDontAskAgain}
          class="flex items-center gap-2 text-gray-400 text-xs hover:text-gray-300 transition-colors"
        >
          <div
            class={`w-4 h-4 rounded border flex items-center justify-center transition-colors
            ${dontAskAgain ? "bg-blue-500 border-blue-500" : "border-gray-500"}`}
          >
            {#if dontAskAgain}
              <Check size={12} class="text-white" />
            {/if}
          </div>
          No volver a preguntar (desactiva todas las confirmaciones)
        </button>
      </div>
    </div>
  </div>

  <!-- ============================================ -->
  <!-- Sección: Undo/Redo -->
  <!-- ============================================ -->
  <div class="space-y-3">
    <div class="flex items-center gap-2 text-white text-sm font-medium">
      <Undo2 size={16} class="text-purple-400" />
      Deshacer / Rehacer
    </div>

    <div class="pl-6">
      <button
        onclick={toggleUndoRedo}
        class="w-full flex items-center justify-between p-3 bg-[#1e1e1e] border border-white/10 rounded-lg
          hover:border-white/20 transition-colors"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-purple-500/10">
            <Undo2 size={16} class="text-purple-400" />
          </div>
          <div class="text-left">
            <div class="text-white text-sm">Habilitar Ctrl+Z / Ctrl+Y</div>
            <div class="text-gray-500 text-xs">
              Permite deshacer y rehacer cambios en celdas editables
            </div>
          </div>
        </div>
        <div
          class={`w-10 h-6 rounded-full transition-colors ${enableUndoRedo ? "bg-blue-500" : "bg-gray-600"}`}
        >
          <div
            class={`w-4 h-4 mt-1 rounded-full bg-white transition-transform ${enableUndoRedo ? "translate-x-5" : "translate-x-1"}`}
          ></div>
        </div>
      </button>
    </div>
  </div>

  <!-- ============================================ -->
  <!-- Sección: Performance -->
  <!-- ============================================ -->
  <div class="space-y-3">
    <div class="flex items-center gap-2 text-white text-sm font-medium">
      <Zap size={16} class="text-yellow-400" />
      Rendimiento
    </div>

    <div class="space-y-3 pl-6">
      <!-- Row Buffer -->
      <div class="p-3 bg-[#1e1e1e] border border-white/10 rounded-lg">
        <div class="flex items-center justify-between mb-3">
          <div>
            <div class="text-white text-sm">Buffer de filas</div>
            <div class="text-gray-500 text-xs">
              Filas pre-renderizadas fuera de la vista
            </div>
          </div>
          <div class="text-blue-400 text-sm font-medium">{rowBuffer}</div>
        </div>
        <div class="flex gap-2">
          {#each rowBufferOptions as option}
            <button
              onclick={() => setRowBuffer(option)}
              class={`flex-1 py-2 text-xs rounded transition-colors
                ${
                  rowBuffer === option
                    ? "bg-blue-500 text-white"
                    : "bg-[#252526] text-gray-400 hover:bg-[#2a2a2b] hover:text-white"
                }`}
            >
              {option}
            </button>
          {/each}
        </div>
        <div class="mt-2 flex items-start gap-1.5 text-gray-500 text-xs">
          <Info size={12} class="mt-0.5 shrink-0" />
          <span
            >Valores más altos mejoran el scroll suave pero usan más memoria</span
          >
        </div>
      </div>

      <!-- Debounce Scroll -->
      <button
        onclick={toggleDebounceScroll}
        class="w-full flex items-center justify-between p-3 bg-[#1e1e1e] border border-white/10 rounded-lg
          hover:border-white/20 transition-colors"
      >
        <div class="text-left">
          <div class="text-white text-sm">Suavizar scroll vertical</div>
          <div class="text-gray-500 text-xs">
            Reduce el parpadeo durante el scroll rápido
          </div>
        </div>
        <div
          class={`w-10 h-6 rounded-full transition-colors ${debounceScroll ? "bg-blue-500" : "bg-gray-600"}`}
        >
          <div
            class={`w-4 h-4 mt-1 rounded-full bg-white transition-transform ${debounceScroll ? "translate-x-5" : "translate-x-1"}`}
          ></div>
        </div>
      </button>
    </div>
  </div>

  <!-- ============================================ -->
  <!-- Sección: Backup -->
  <!-- ============================================ -->
  <div class="space-y-3">
    <div class="flex items-center gap-2 text-white text-sm font-medium">
      <Download size={16} class="text-cyan-400" />
      Respaldo de Configuración
    </div>

    <div class="space-y-4 pl-6">
      <!-- Exportar -->
      <div class="p-3 bg-[#1e1e1e] border border-white/10 rounded-lg space-y-3">
        <div class="flex items-center justify-between">
          <div>
            <div class="text-white text-sm">Exportar configuración</div>
            <div class="text-gray-500 text-xs">
              Guarda tu configuración actual como JSON
            </div>
          </div>
          <button
            onclick={exportConfig}
            class="flex items-center gap-1.5 px-3 py-1.5 bg-cyan-500/10 border border-cyan-500/20
              rounded text-cyan-400 text-xs hover:bg-cyan-500/20 transition-colors"
          >
            <Download size={14} />
            Generar
          </button>
        </div>

        {#if exportedConfig}
          <div class="space-y-2">
            <textarea
              readonly
              value={exportedConfig}
              class="w-full h-32 p-2 text-xs font-mono bg-[#252526] border border-white/10 rounded
                text-gray-300 resize-none focus:outline-none"
            ></textarea>
            <div class="flex gap-2">
              <button
                onclick={copyToClipboard}
                class="flex-1 flex items-center justify-center gap-1.5 py-2 bg-[#252526] border border-white/10
                  rounded text-xs text-gray-300 hover:bg-[#2a2a2b] transition-colors"
              >
                {#if copySuccess}
                  <Check size={14} class="text-green-400" />
                  <span class="text-green-400">Copiado!</span>
                {:else}
                  <Copy size={14} />
                  Copiar
                {/if}
              </button>
              <button
                onclick={downloadConfig}
                class="flex-1 flex items-center justify-center gap-1.5 py-2 bg-[#252526] border border-white/10
                  rounded text-xs text-gray-300 hover:bg-[#2a2a2b] transition-colors"
              >
                <FileDown size={14} />
                Descargar
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Importar -->
      <div class="p-3 bg-[#1e1e1e] border border-white/10 rounded-lg space-y-3">
        <div>
          <div class="text-white text-sm">Importar configuración</div>
          <div class="text-gray-500 text-xs">
            Restaura una configuración previamente guardada
          </div>
        </div>

        <textarea
          bind:value={importJson}
          placeholder={importPlaceholder}
          class="w-full h-32 p-2 text-xs font-mono bg-[#252526] border border-white/10 rounded
            text-gray-300 resize-none focus:outline-none focus:border-white/30
            placeholder:text-gray-600"
        ></textarea>

        {#if importError}
          <div class="flex items-center gap-2 text-red-400 text-xs">
            <AlertTriangle size={14} />
            {importError}
          </div>
        {/if}

        {#if importSuccess}
          <div class="flex items-center gap-2 text-green-400 text-xs">
            <Check size={14} />
            Configuración importada correctamente
          </div>
        {/if}

        <div class="flex gap-2">
          <label
            class="flex-1 flex items-center justify-center gap-1.5 py-2 bg-[#252526] border border-white/10
              rounded text-xs text-gray-300 hover:bg-[#2a2a2b] transition-colors cursor-pointer"
          >
            <FileUp size={14} />
            Cargar archivo
            <input
              type="file"
              accept=".json"
              onchange={handleFileUpload}
              class="hidden"
            />
          </label>
          <button
            onclick={importConfig}
            disabled={!importJson.trim()}
            class="flex-1 flex items-center justify-center gap-1.5 py-2 bg-blue-500/10 border border-blue-500/20
              rounded text-xs text-blue-400 hover:bg-blue-500/20 transition-colors
              disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Upload size={14} />
            Importar
          </button>
        </div>
      </div>
    </div>
  </div>
</div>
