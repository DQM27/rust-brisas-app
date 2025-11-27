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

<div
  class="flex justify-between items-center gap-3 p-4 bg-[#252526] border-b border-white/10"
>
  <!-- Sección izquierda: Acciones principales -->
  <div class="flex items-center gap-2">
    {#if toolbarConfig.showColumnSelector}
      <button
        onclick={onOpenColumnSelector}
        class="flex items-center gap-1.5 px-3 py-2 bg-[#1e1e1e] border border-white/10 rounded-md text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20 whitespace-nowrap"
        title="Configurar columnas"
      >
        <Settings size={16} />
        Columnas ({visibleCount}/{totalColumns})
      </button>
    {/if}

    {#if toolbarConfig.showExport}
      <button
        onclick={onExport}
        class="flex items-center gap-1.5 px-3 py-2 bg-[#1e1e1e] border border-white/10 rounded-md text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20 disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
        title="Exportar a CSV"
        disabled={!gridReady}
      >
        <Download size={16} />
        Exportar CSV
      </button>
    {/if}

    {#if toolbarConfig.showAutoSize}
      <div class="w-px h-6 bg-white/10"></div>

      <button
        onclick={onAutoSize}
        class="flex items-center gap-1 px-2 py-2 bg-[#1e1e1e] border border-white/10 rounded-md text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20 disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
        title="Ajustar columnas al contenido"
        disabled={!gridReady}
      >
        <Maximize2 size={14} />
        Ajustar
      </button>

      <button
        onclick={onSizeToFit}
        class="flex items-center gap-1 px-2 py-2 bg-[#1e1e1e] border border-white/10 rounded-md text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20 disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
        title="Ajustar al ancho disponible"
        disabled={!gridReady}
      >
        <Minimize2 size={14} />
        Llenar
      </button>
    {/if}

    {#if toolbarConfig.customButtons}
      <div class="w-px h-6 bg-white/10"></div>
      {#each toolbarConfig.customButtons as button}
        <button
          onclick={button.onClick}
          class="flex items-center gap-1.5 px-3 py-2 bg-[#1e1e1e] border border-white/10 rounded-md text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20 disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
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

  <!-- Sección derecha: Acciones de selección -->
  {#if selectedRows.length > 0}
    <div
      class="flex items-center gap-2 bg-blue-500/10 border border-blue-500/20 rounded-md px-3 py-2"
    >
      <span class="text-blue-400 text-xs font-medium mr-2">
        {selectedRows.length} seleccionada(s)
      </span>

      <button
        onclick={onCopySelected}
        class="flex items-center gap-1 px-2 py-1.5 bg-[#1e1e1e] border border-white/10 rounded text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20"
        title="Copiar seleccionadas"
      >
        <Copy size={14} />
      </button>

      <button
        onclick={onDeselectAll}
        class="flex items-center gap-1 px-2 py-1.5 bg-[#1e1e1e] border border-white/10 rounded text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white-20"
        title="Deseleccionar todas"
      >
        <X size={14} />
      </button>
    </div>
  {:else if gridReady}
    <div class="flex items-center gap-2">
      <button
        onclick={onSelectAll}
        class="flex items-center gap-1 px-2 py-2 bg-[#1e1e1e] border border-white/10 rounded-md text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20 whitespace-nowrap"
        title="Seleccionar todas las visibles"
      >
        <CheckSquare size={14} />
        Seleccionar todas
      </button>
    </div>
  {/if}
</div>
