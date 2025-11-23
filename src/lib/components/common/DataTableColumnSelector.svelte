<script lang="ts">
  import { X, Eye, EyeOff } from "lucide-svelte";
  import {
    toggleColumnVisibility,
    showAllColumns,
    resetTablePreferences,
  } from "$lib/stores/dataTableStore";
  import type { DataTableColumn } from "$lib/types/dataTable";
  import type { Writable } from "svelte/store";
  import type { TablePreferences } from "$lib/types/dataTable";

  interface Props {
    columns: DataTableColumn<any>[];
    preferencesStore: Writable<TablePreferences>;
    onClose: () => void;
  }

  let { columns, preferencesStore, onClose }: Props = $props();

  let columnVisibility = $state($preferencesStore.columnVisibility);

  $effect(() => {
    columnVisibility = $preferencesStore.columnVisibility;
  });

  function handleToggle(field: string) {
    toggleColumnVisibility(preferencesStore, field);
  }

  function handleShowAll() {
    showAllColumns(preferencesStore);
  }

  function handleReset() {
    const defaultVisibility = columns.reduce(
      (acc, col) => ({ ...acc, [String(col.field)]: !col.hide }),
      {},
    );
    resetTablePreferences(preferencesStore, defaultVisibility);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="column-selector-overlay" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="column-selector" onclick={(e) => e.stopPropagation()}>
    <div class="column-selector-header">
      <h3>Configurar Columnas</h3>
      <button onclick={onClose} class="close-button" title="Cerrar">
        <X size={18} />
      </button>
    </div>

    <div class="column-selector-body">
      {#each columns as column}
        {@const field = String(column.field)}
        {@const isVisible = columnVisibility[field]}
        {@const Icon = isVisible ? Eye : EyeOff}
        <label class="column-option">
          <input
            type="checkbox"
            checked={isVisible}
            onchange={() => handleToggle(field)}
          />
          <span class="column-label">
            <Icon
              size={14}
              class={isVisible ? "text-blue-400" : "text-gray-500"}
            />
            {column.headerName}
          </span>
        </label>
      {/each}
    </div>

    <div class="column-selector-footer">
      <button onclick={handleShowAll} class="action-button">
        Mostrar todas
      </button>
      <button onclick={handleReset} class="action-button"> Restablecer </button>
    </div>
  </div>
</div>

<style>
  .column-selector-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s;
  }

  .column-selector {
    background-color: #252526;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    width: 420px;
    max-width: 90vw;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
    animation: slideUp 0.3s;
  }

  .column-selector-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .column-selector-header h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: #ffffff;
  }

  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    color: #a0a0a0;
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-button:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }

  .column-selector-body {
    padding: 16px 20px;
    overflow-y: auto;
    flex: 1;
  }

  .column-option {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 0;
    cursor: pointer;
    user-select: none;
  }

  .column-option input[type="checkbox"] {
    width: 16px;
    height: 16px;
    cursor: pointer;
    accent-color: #007acc;
  }

  .column-label {
    display: flex;
    align-items: center;
    gap: 8px;
    color: #d4d4d4;
    font-size: 13px;
  }

  .column-option:hover .column-label {
    color: #ffffff;
  }

  .column-selector-footer {
    padding: 16px 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    gap: 8px;
  }

  .action-button {
    flex: 1;
    padding: 8px 16px;
    background-color: #1e1e1e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #ffffff;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-button:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
</style>
