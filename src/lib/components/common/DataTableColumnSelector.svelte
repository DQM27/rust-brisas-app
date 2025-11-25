<script lang="ts">
  import { X, Eye, EyeOff } from "lucide-svelte";
  import {
    toggleColumnVisibility,
    showAllColumns,
    resetTablePreferences,
    hideAllColumns,
  } from "$lib/stores/dataTableStore";
  import type { DataTableColumn } from "$lib/types/dataTable";
  import type { Writable } from "svelte/store";
  import type { TablePreferences, ColumnVisibilityConfig } from "$lib/types/dataTable";

  interface Props {
    columns: DataTableColumn<any>[];
    preferencesStore: Writable<TablePreferences>;
    onClose: () => void;
  }

  let { columns, preferencesStore, onClose }: Props = $props();

  let columnVisibility = $state($preferencesStore.columnVisibility);
  let defaultVisibility = $state<ColumnVisibilityConfig>({});

  $effect(() => {
    columnVisibility = $preferencesStore.columnVisibility;
    defaultVisibility = columns.reduce(
      (acc, col) => ({ ...acc, [String(col.field)]: !col.hide }),
      {} as ColumnVisibilityConfig
    );
  });

  function handleToggle(field: string) {
    toggleColumnVisibility(preferencesStore, field);
  }

  function handleShowAll() {
    showAllColumns(preferencesStore);
  }

  function handleHideAll() {
    hideAllColumns(preferencesStore);
  }

  function handleReset() {
    resetTablePreferences(preferencesStore, defaultVisibility);
  }

  let hasChanges = $derived(
    Object.keys(columnVisibility).some(
      field => columnVisibility[field] !== defaultVisibility[field]
    )
  );

  let visibleCount = $derived(Object.values(columnVisibility).filter(Boolean).length);
  let totalCount = $derived(columns.length);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 animate-in fade-in duration-200"
  on:click={onClose}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    class="bg-[#252526] border border-white/10 rounded-lg w-full max-w-md max-h-[80vh] flex flex-col shadow-2xl animate-in slide-in-from-bottom-4 duration-300"
    on:click={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-5 border-b border-white/10">
      <div>
        <h3 class="text-lg font-semibold text-white m-0">
          Configurar Columnas
        </h3>
        <p class="text-sm text-gray-400 mt-1">
          {visibleCount} de {totalCount} columnas visibles
          {#if hasChanges}
            <span class="text-amber-400 ml-2">• Modificado</span>
          {/if}
        </p>
      </div>
      <button 
        on:click={onClose}
        class="flex items-center justify-center w-7 h-7 bg-transparent border-none rounded text-gray-400 hover:bg-white/10 hover:text-white transition-colors cursor-pointer"
        title="Cerrar"
      >
        <X size={18} />
      </button>
    </div>

    <!-- Body -->
    <div class="flex-1 p-5 overflow-y-auto">
      <div class="space-y-3">
        {#each columns as column}
          {@const field = String(column.field)}
          {@const isVisible = columnVisibility[field]}
          {@const isDefault = defaultVisibility[field]}
          <label class="flex items-center gap-3 py-2.5 cursor-pointer user-select-none group">
            <input
              type="checkbox"
              checked={isVisible}
              on:change={() => handleToggle(field)}
              class="w-4 h-4 cursor-pointer accent-blue-500"
            />
            <span class="flex items-center gap-2 text-gray-300 group-hover:text-white transition-colors text-sm flex-1">
              {#if isVisible}
                <Eye size={14} class="text-blue-400" />
              {:else}
                <EyeOff size={14} class="text-gray-500" />
              {/if}
              <span class="flex-1">{column.headerName}</span>
              {#if isVisible !== isDefault}
                <span class="text-xs text-amber-400 font-medium">Modificado</span>
              {/if}
            </span>
          </label>
        {/each}
      </div>
    </div>

    <!-- Footer -->
    <div class="flex gap-2 p-4 border-t border-white/10">
      <button 
        on:click={handleShowAll}
        class="flex-1 px-4 py-2 bg-blue-500/10 border border-blue-500/20 rounded-md text-blue-400 text-xs font-medium cursor-pointer transition-colors hover:bg-blue-500/20"
        title="Mostrar todas las columnas"
      >
        Mostrar todas
      </button>
      <button 
        on:click={handleHideAll}
        class="flex-1 px-4 py-2 bg-gray-500/10 border border-gray-500/20 rounded-md text-gray-400 text-xs font-medium cursor-pointer transition-colors hover:bg-gray-500/20"
        title="Ocultar todas las columnas"
      >
        Ocultar todas
      </button>
      <button 
        on:click={handleReset}
        disabled={!hasChanges}
        class="flex-1 px-4 py-2 bg-amber-500/10 border border-amber-500/20 rounded-md text-amber-400 text-xs font-medium cursor-pointer transition-colors hover:bg-amber-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
        title="Volver a la configuración original"
      >
        Restablecer
      </button>
    </div>
  </div>
</div>