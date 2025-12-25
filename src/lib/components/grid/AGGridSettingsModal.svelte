<script lang="ts">
  import type { GridId, SettingsTab } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { X, RotateCcw } from "lucide-svelte";
  import { slide, fade } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";

  import AGGridSettingsAppearance from "./settings/AGGridSettingsAppearance.svelte";
  import AGGridSettingsColumns from "./settings/AGGridSettingsColumns.svelte";
  import AGGridSettingsToolbar from "./settings/AGGridSettingsToolbar.svelte";
  import AGGridSettingsData from "./settings/AGGridSettingsData.svelte";
  import AGGridSettingsAdvanced from "./settings/AGGridSettingsAdvanced.svelte";
  import { Palette, Columns, Wrench, ClipboardType, Cpu } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
    customButtons?: {
      default?: any[];
      singleSelect?: any[];
      multiSelect?: any[];
    };
    onClose: () => void;
  }

  let { gridId, gridApi, customButtons, onClose }: Props = $props();

  let activeTab = $state<SettingsTab>("appearance");
  let isClosing = $state(false);
  let showResetConfirm = $state(false);

  const tabs: { id: SettingsTab; label: string; icon: any }[] = [
    { id: "appearance", label: "Apariencia", icon: Palette },
    { id: "columns", label: "Columnas", icon: Columns },
    { id: "toolbar", label: "Toolbar", icon: Wrench },
    { id: "data", label: "Datos", icon: ClipboardType },
    { id: "advanced", label: "Avanzado", icon: Cpu },
  ];

  function handleClose() {
    isClosing = true;
    setTimeout(onClose, 150);
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      handleClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (showResetConfirm) {
        showResetConfirm = false;
      } else {
        handleClose();
      }
    }
  }

  function handleResetAll() {
    agGridSettings.resetToDefaults(gridId);
    if (gridApi) {
      gridApi.resetColumnState();
      gridApi.setFilterModel(null);
    }
    showResetConfirm = false;
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4
    {isClosing ? 'opacity-0' : 'opacity-100'} transition-opacity duration-150"
  onclick={handleBackdropClick}
  role="dialog"
  aria-modal="true"
  tabindex="-1"
  onkeydown={(e) => e.key === "Escape" && handleClose()}
>
  <!-- Backdrop -->
  <div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>

  <!-- Modal -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="relative w-full max-w-4xl max-h-[85vh] flex flex-col
      bg-[#1a1a1b] border border-white/10 rounded-xl shadow-2xl
      {isClosing ? 'scale-95' : 'scale-100'} transition-transform duration-150"
    onclick={(e) => e.stopPropagation()}
    role="document"
    tabindex="-1"
    onkeydown={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between px-5 py-4 border-b border-white/10"
    >
      <div>
        <h2 class="text-base font-semibold text-white">
          Configuración de Grid
        </h2>
        <p class="text-xs text-gray-500 mt-0.5">
          Personaliza la tabla a tu gusto
        </p>
      </div>
      <div class="flex items-center gap-2">
        <button
          onclick={() => (showResetConfirm = true)}
          class="flex items-center gap-1.5 px-3 py-1.5 text-xs text-gray-400
            hover:text-white hover:bg-white/5 rounded-lg transition-colors"
          title="Restaurar valores por defecto"
        >
          <RotateCcw size={14} />
          <span class="hidden sm:inline">Reset</span>
        </button>
        <button
          onclick={handleClose}
          class="p-1.5 text-gray-400 hover:text-white hover:bg-white/5 rounded-lg transition-colors"
        >
          <X size={18} />
        </button>
      </div>
    </div>

    <!-- Content -->
    <div class="flex flex-1 overflow-hidden">
      <!-- Sidebar Tabs -->
      <div
        class="w-48 border-r border-white/5 bg-black/40 backdrop-blur-md p-3 flex flex-col gap-1"
      >
        {#each tabs as tab}
          <button
            onclick={() => (activeTab = tab.id)}
            class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-left transition-all group
              {activeTab === tab.id
              ? 'bg-blue-600 text-white shadow-lg shadow-blue-900/20'
              : 'text-gray-400 hover:text-white hover:bg-white/5'}"
          >
            <tab.icon
              size={18}
              class={activeTab === tab.id
                ? "text-white"
                : "text-gray-500 group-hover:text-gray-300"}
            />
            <span class="text-sm font-medium">{tab.label}</span>
          </button>
        {/each}
      </div>

      <!-- Tab Content -->
      <div class="flex-1 overflow-y-auto p-5">
        {#if activeTab === "appearance"}
          <AGGridSettingsAppearance {gridId} {gridApi} />
        {:else if activeTab === "columns"}
          <AGGridSettingsColumns {gridId} {gridApi} />
        {:else if activeTab === "toolbar"}
          <AGGridSettingsToolbar {gridId} {gridApi} {customButtons} />
        {:else if activeTab === "data"}
          <AGGridSettingsData {gridId} {gridApi} />
        {:else if activeTab === "advanced"}
          <AGGridSettingsAdvanced {gridId} {gridApi} />
        {/if}
      </div>
    </div>

    <!-- Footer -->
    <div
      class="flex items-center justify-between px-5 py-3 border-t border-white/10 bg-black/20"
    >
      <p class="text-xs text-gray-500">
        Los cambios se aplican automáticamente
      </p>
      <button
        onclick={handleClose}
        class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-500
          rounded-lg transition-colors"
      >
        Listo
      </button>
    </div>
  </div>

  <!-- Reset Confirmation -->
  {#if showResetConfirm}
    <div
      transition:fade={{ duration: 100 }}
      class="absolute inset-0 z-10 flex items-center justify-center bg-black/50"
      onclick={() => (showResetConfirm = false)}
      role="button"
      tabindex="0"
      onkeydown={(e) => e.key === "Escape" && (showResetConfirm = false)}
    >
      <div
        transition:slide={{ duration: 150, easing: cubicOut }}
        class="bg-[#252526] border border-white/10 rounded-xl p-5 max-w-sm mx-4 shadow-2xl"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        tabindex="-1"
        onkeydown={(e) => e.stopPropagation()}
      >
        <h3 class="text-sm font-semibold text-white mb-2">
          ¿Restaurar configuración?
        </h3>
        <p class="text-xs text-gray-400 mb-4">
          Esto restablecerá todas las opciones de esta grid a sus valores por
          defecto. Esta acción no se puede deshacer.
        </p>
        <div class="flex justify-end gap-2">
          <button
            onclick={() => (showResetConfirm = false)}
            class="px-3 py-1.5 text-sm text-gray-400 hover:text-white transition-colors"
          >
            Cancelar
          </button>
          <button
            onclick={handleResetAll}
            class="px-3 py-1.5 text-sm font-medium text-white bg-red-600 hover:bg-red-500
              rounded-lg transition-colors"
          >
            Restaurar
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
