<script lang="ts">
  import type { GridId, RowHeight } from '$lib/types/agGrid';
  import type { GridApi } from '@ag-grid-community/core';
  import { agGridSettings } from '$lib/stores/agGridSettings.svelte';
  import { Ruler, Hash, AlertTriangle } from 'lucide-svelte';

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  // Estado local
  let rowHeight = $state<RowHeight>(agGridSettings.getRowHeight(gridId));
  let paginationSize = $state(agGridSettings.getPaginationSize(gridId));
  let confirmations = $state(agGridSettings.getConfirmations(gridId));

  // Opciones
  const rowHeightOptions: { value: RowHeight; label: string; px: number }[] = [
    { value: 'compact', label: 'Compacto', px: 32 },
    { value: 'normal', label: 'Normal', px: 40 },
    { value: 'comfortable', label: 'Cómodo', px: 48 }
  ];

  const paginationOptions = [20, 50, 100, 200, 500];

  // Handlers
  function handleRowHeightChange(height: RowHeight) {
    rowHeight = height;
    agGridSettings.setRowHeight(gridId, height);
    
    // Aplicar a la grid
    if (gridApi) {
      const px = rowHeightOptions.find(opt => opt.value === height)?.px || 40;
      gridApi.setGridOption('rowHeight', px);
      gridApi.resetRowHeights();
    }
  }

  function handlePaginationChange(size: number) {
    paginationSize = size;
    agGridSettings.setPaginationSize(gridId, size);
    
    // Aplicar a la grid usando setGridOption (AG Grid v32)
    if (gridApi) {
      gridApi.setGridOption('paginationPageSize', size);
    }
  }

  function handleConfirmationsChange() {
    agGridSettings.setConfirmations(gridId, confirmations);
  }
</script>

<div class="space-y-6">
  <!-- Altura de Fila -->
  <div>
    <div class="flex items-center gap-2 mb-3">
      <Ruler size={18} class="text-purple-400" />
      <h3 class="text-sm font-semibold text-white">Altura de Fila</h3>
    </div>
    
    <div class="space-y-2">
      {#each rowHeightOptions as option}
        <label class="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-all
          {rowHeight === option.value
            ? 'bg-purple-500/10 border-purple-500/30'
            : 'bg-[#252526] border-white/10 hover:border-white/20'}">
          <input
            type="radio"
            name="rowHeight"
            value={option.value}
            checked={rowHeight === option.value}
            onchange={() => handleRowHeightChange(option.value)}
            class="w-4 h-4 text-purple-500 bg-[#1e1e1e] border-white/20 focus:ring-purple-500 focus:ring-2"
          />
          <span class="flex-1 text-sm text-white">{option.label}</span>
          <span class="text-xs text-gray-500">{option.px}px</span>
          {#if rowHeight === option.value}
            <div class="w-2 h-2 rounded-full bg-purple-500"></div>
          {/if}
        </label>
      {/each}
    </div>
  </div>

  <!-- Paginación -->
  <div>
    <div class="flex items-center gap-2 mb-3">
      <Hash size={18} class="text-blue-400" />
      <h3 class="text-sm font-semibold text-white">Tamaño de Página</h3>
    </div>
    
    <select
      bind:value={paginationSize}
      onchange={(e) => handlePaginationChange(Number(e.currentTarget.value))}
      class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      {#each paginationOptions as size}
        <option value={size}>{size} registros</option>
      {/each}
    </select>
  </div>

  <!-- Confirmaciones -->
  <div>
    <div class="flex items-center gap-2 mb-3">
      <AlertTriangle size={18} class="text-amber-400" />
      <h3 class="text-sm font-semibold text-white">Confirmaciones</h3>
    </div>
    
    <div class="space-y-3">
      <label class="flex items-center gap-3 p-3 bg-[#252526] border border-white/10 rounded-lg cursor-pointer hover:border-white/20 transition-all">
        <input
          type="checkbox"
          bind:checked={confirmations.deleteRecords}
          onchange={handleConfirmationsChange}
          class="w-4 h-4 text-red-500 bg-[#1e1e1e] border-white/20 rounded focus:ring-red-500 focus:ring-2"
        />
        <div class="flex-1">
          <div class="text-sm text-white font-medium">Confirmar eliminaciones</div>
          <div class="text-xs text-gray-400 mt-0.5">Mostrar diálogo de confirmación al eliminar registros</div>
        </div>
      </label>

      {#if confirmations.deleteRecords}
        <label class="flex items-center gap-3 p-3 ml-6 bg-[#252526] border border-white/10 rounded-lg cursor-pointer hover:border-white/20 transition-all">
          <input
            type="checkbox"
            bind:checked={confirmations.dontAskAgain}
            onchange={handleConfirmationsChange}
            class="w-4 h-4 text-blue-500 bg-[#1e1e1e] border-white/20 rounded focus:ring-blue-500 focus:ring-2"
          />
          <div class="flex-1">
            <div class="text-sm text-white font-medium">No volver a preguntar</div>
            <div class="text-xs text-gray-400 mt-0.5">Recordar mi elección y no mostrar el diálogo nuevamente</div>
          </div>
        </label>
      {/if}
    </div>
  </div>

  <!-- Info -->
  <div class="p-4 bg-blue-500/10 border border-blue-500/20 rounded-lg">
    <p class="text-xs text-blue-400">
      <strong>Nota:</strong> Los cambios en esta sección se aplican inmediatamente a la grid.
    </p>
  </div>
</div>