<script lang="ts">
  import type { GridId } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { X } from "lucide-svelte";
  import AGGridSettingsVisual from "./settings/AGGridSettingsVisual.svelte";
  import AGGridSettingsColumns from "./settings/AGGridSettingsColumns.svelte";
  import AGGridSettingsButtons from "./settings/AGGridSettingsButtons.svelte";
  import AGGridSettingsAdvanced from "./settings/AGGridSettingsAdvanced.svelte";

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

  type TabId = "visual" | "columns" | "buttons" | "advanced";

  let activeTab = $state<TabId>("visual");
  let isHidden = $state(false);
  let isClosing = $state(false);

  const tabs = [
    { id: "visual" as TabId, label: "Visual", icon: "🎨" },
    { id: "columns" as TabId, label: "Columnas", icon: "📊" },
    { id: "buttons" as TabId, label: "Botones", icon: "🔘" },
    { id: "advanced" as TabId, label: "Avanzado", icon: "⚙️" },
  ];

  function handleClose() {
    isClosing = true;
    setTimeout(() => {
      onClose();
    }, 150);
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget && !isHidden) {
      handleClose();
    }
  }

  function handleStartOrganizing() {
    isHidden = true;
  }

  function handleEndOrganizing() {
    isHidden = false;
  }
</script>

<div
  class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4 transition-opacity duration-200 {isClosing
    ? 'opacity-0'
    : 'opacity-100'}"
  class:hidden={isHidden}
  onclick={handleBackdropClick}
  role="presentation"
>
  <div
    class="bg-[#1e1e1e] border border-white/10 rounded-lg shadow-2xl w-full max-w-3xl flex flex-col max-h-[85vh] transition-all duration-200 {isClosing
      ? 'scale-95 opacity-0'
      : 'scale-100 opacity-100'}"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-white/10">
      <div>
        <h2 class="text-lg font-semibold text-white">Configuración de Grid</h2>
        <p class="text-xs text-gray-400 mt-0.5">
          Personaliza la apariencia y comportamiento
        </p>
      </div>
      <button
        onclick={handleClose}
        class="p-1.5 rounded-md text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
        aria-label="Cerrar"
      >
        <X size={18} />
      </button>
    </div>

    <!-- Tabs -->
    <div class="flex border-b border-white/10 bg-[#252526] relative">
      <div
        class="absolute bottom-0 h-0.5 bg-blue-500 transition-all duration-200 ease-out"
        style="left: {activeTab === 'visual'
          ? '0%'
          : activeTab === 'columns'
            ? '25%'
            : activeTab === 'buttons'
              ? '50%'
              : '75%'}; width: 25%;"
      ></div>

      {#each tabs as tab}
        <button
          onclick={() => (activeTab = tab.id)}
          class="flex-1 px-4 py-2.5 text-sm font-medium transition-colors relative
            {activeTab === tab.id
            ? 'text-blue-400 bg-[#1e1e1e]'
            : 'text-gray-400 hover:text-white hover:bg-white/5'}"
        >
          <span class="flex items-center justify-center gap-2">
            <span class="text-base">{tab.icon}</span>
            <span>{tab.label}</span>
          </span>
        </button>
      {/each}
    </div>

    <!-- Content -->
    <div
      class="flex-1 overflow-y-auto p-5 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-white/10 hover:scrollbar-thumb-white/20"
    >
      <div class="transition-opacity duration-200">
        {#if activeTab === "visual"}
          <AGGridSettingsVisual {gridId} {gridApi} />
        {:else if activeTab === "columns"}
          <AGGridSettingsColumns {gridId} {gridApi} />
        {:else if activeTab === "buttons"}
          <AGGridSettingsButtons {gridId} {gridApi} {customButtons} />
        {:else if activeTab === "advanced"}
          <AGGridSettingsAdvanced {gridId} {gridApi} />
        {/if}
      </div>
    </div>

    <!-- Footer -->
    <div
      class="flex items-center justify-end gap-2 p-4 border-t border-white/10"
    >
      <button
        onclick={handleClose}
        class="px-3 py-1.5 rounded-md text-sm font-medium text-gray-300 hover:text-white hover:bg-white/5 transition-colors"
      >
        Cancelar
      </button>
      <button
        onclick={handleClose}
        class="px-4 py-1.5 rounded-md text-sm font-medium bg-blue-500 text-white hover:bg-blue-600 transition-colors"
      >
        Guardar
      </button>
    </div>
  </div>
</div>

<style>
  .scrollbar-thin::-webkit-scrollbar {
    width: 6px;
  }

  .scrollbar-track-transparent::-webkit-scrollbar-track {
    background: transparent;
  }

  .scrollbar-thumb-white\/10::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
  }

  .scrollbar-thumb-white\/10:hover::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
  }
</style>
