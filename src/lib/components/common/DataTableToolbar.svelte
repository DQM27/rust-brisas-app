<script lang="ts">
  import {
    Settings,
    Download,
    CheckSquare,
    X,
    Copy,
    Maximize2,
    Minimize2,
  } from "lucide-svelte";
  import type { DataTableToolbarConfig } from "$lib/types/dataTable";

  interface Props {
    toolbarConfig: DataTableToolbarConfig;
    selectedRows: any[];
    visibleCount: number;
    totalColumns: number;
    gridReady: boolean;
    onOpenColumnSelector: () => void;
    onExport: () => void;
    onAutoSize: () => void;
    onSizeToFit: () => void;
    onSelectAll: () => void;
    onDeselectAll: () => void;
    onCopySelected: () => void;
  }

  let {
    toolbarConfig,
    selectedRows,
    visibleCount,
    totalColumns,
    gridReady,
    onOpenColumnSelector,
    onExport,
    onAutoSize,
    onSizeToFit,
    onSelectAll,
    onDeselectAll,
    onCopySelected,
  }: Props = $props();
</script>

<div class="toolbar">
  <div class="toolbar-section">
    {#if toolbarConfig.showColumnSelector}
      <button
        onclick={onOpenColumnSelector}
        class="toolbar-button"
        title="Configurar columnas"
      >
        <Settings size={16} />
        Columnas ({visibleCount}/{totalColumns})
      </button>
    {/if}

    {#if toolbarConfig.showExport}
      <button
        onclick={onExport}
        class="toolbar-button"
        title="Exportar a CSV"
        disabled={!gridReady}
      >
        <Download size={16} />
        Exportar CSV
      </button>
    {/if}

    {#if toolbarConfig.showAutoSize}
      <div class="toolbar-divider"></div>

      <button
        onclick={onAutoSize}
        class="toolbar-button compact"
        title="Ajustar columnas al contenido"
        disabled={!gridReady}
      >
        <Maximize2 size={14} />
        Ajustar
      </button>

      <button
        onclick={onSizeToFit}
        class="toolbar-button compact"
        title="Ajustar al ancho disponible"
        disabled={!gridReady}
      >
        <Minimize2 size={14} />
        Llenar
      </button>
    {/if}

    {#if toolbarConfig.customButtons}
      <div class="toolbar-divider"></div>
      {#each toolbarConfig.customButtons as button}
        <button
          onclick={button.onClick}
          class="toolbar-button"
          disabled={button.disabled}
          title={button.label}
        >
          {#if button.icon}
            {@const Icon = button.icon}
            <Icon size={16} />
          {/if}
          {button.label}
        </button>
      {/each}
    {/if}
  </div>

  {#if selectedRows.length > 0}
    <div class="toolbar-section selection-actions">
      <span class="selection-count">
        {selectedRows.length} seleccionada(s)
      </span>

      <button
        onclick={onCopySelected}
        class="toolbar-button compact"
        title="Copiar seleccionadas"
      >
        <Copy size={14} />
      </button>

      <button
        onclick={onDeselectAll}
        class="toolbar-button compact"
        title="Deseleccionar todas"
      >
        <X size={14} />
      </button>
    </div>
  {:else if gridReady}
    <div class="toolbar-section">
      <button
        onclick={onSelectAll}
        class="toolbar-button compact"
        title="Seleccionar todas las visibles"
      >
        <CheckSquare size={14} />
        Seleccionar todas
      </button>
    </div>
  {/if}
</div>

<style>
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background-color: #252526;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .toolbar-section {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toolbar-divider {
    width: 1px;
    height: 24px;
    background-color: rgba(255, 255, 255, 0.1);
  }

  .selection-actions {
    background-color: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 6px;
    padding: 6px 12px;
  }

  .selection-count {
    color: #60a5fa;
    font-size: 12px;
    font-weight: 500;
    margin-right: 8px;
  }

  .toolbar-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background-color: #1e1e1e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #ffffff;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .toolbar-button:hover:not(:disabled) {
    background-color: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .toolbar-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toolbar-button.compact {
    padding: 6px 8px;
    font-size: 11px;
  }
</style>
