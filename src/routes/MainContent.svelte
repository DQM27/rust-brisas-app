<script lang="ts">
  import { isAuthenticated } from "$lib/stores/auth";
  import LoginPage from "./LoginPage.svelte";
  import { Splitpanes, Pane } from "svelte-splitpanes";
  import Tabs from "$lib/components/layout/Tabs.svelte";
  import { tabsStore, openTab } from "$lib/stores/tabs";
  import { inspectionPanel } from "$lib/stores/ui";
  import { ChevronDown } from "lucide-svelte";
  import { get } from "svelte/store";

  let inspectionContent = $state("27");

  // Inicializar tabs cuando se autentica
  $effect(() => {
    if ($isAuthenticated) {
      const tabs = get(tabsStore);
      if (tabs.length === 0) {
        openTab({
          componentKey: "welcome",
          title: "Bienvenida",
          id: "welcome",
        });
      }
    }
  });

  function handleKeyPress(event: KeyboardEvent, handler: () => void) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      handler();
    }
  }

  function closeInspectionPanel() {
    $inspectionPanel.visible = false;
  }
</script>

{#if !$isAuthenticated}
  <LoginPage />
{:else}
  <!-- App Principal -->
  <div class="h-full bg-[#1e1e1e] relative">
    <Splitpanes horizontal class="default-theme">
      <!-- Contenido principal -->
      <Pane minSize={30} size={$inspectionPanel.visible ? 70 : 100}>
        <div class="h-full bg-[#1e1e1e]">
          <Tabs tabs={$tabsStore} />
        </div>
      </Pane>

      <!-- Panel de inspección -->
      {#if $inspectionPanel.visible}
        <Pane minSize={20} size={30}>
          <div class="flex h-full flex-col bg-[#252526]">
            <div
              class="flex items-center justify-between border-b border-[#3c3c3c] bg-[#2d2d2d] px-3 py-2"
            >
              <h4
                class="text-xs font-semibold uppercase tracking-wide text-gray-300"
              >
                Inspección
              </h4>
              <button
                class="flex items-center justify-center rounded p-1 text-gray-300 hover:bg-[#3c3c3c]"
                onclick={closeInspectionPanel}
                onkeydown={(e) => handleKeyPress(e, closeInspectionPanel)}
                type="button"
                title="Cerrar panel de inspección"
              >
                <ChevronDown size={16} />
              </button>
            </div>
            <div class="flex-1 overflow-y-auto p-3 text-sm text-gray-300">
              {inspectionContent}
            </div>
          </div>
        </Pane>
      {/if}
    </Splitpanes>
  </div>
{/if}

<style>
  :global(.splitpanes__pane) {
    background: transparent;
  }
  :global(.splitpanes__splitter) {
    background: #2d2d2d;
    border: none;
  }
  :global(.splitpanes__splitter:hover) {
    background: #3c3c3c;
  }
  :global(.splitpanes--horizontal .splitpanes__splitter) {
    min-height: 6px;
    border-top: 1px solid #3c3c3c;
    border-bottom: 1px solid #3c3c3c;
  }
</style>
