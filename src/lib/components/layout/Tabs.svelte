<script lang="ts">
  import { dndzone } from "svelte-dnd-action";
  import { activeTabId, closeTab, reorderTabs } from "$lib/stores/tabs";
  import type { HydratedTab } from "$lib/types/tab";
  import { X } from "lucide-svelte";

  interface Props {
    tabs: HydratedTab[];
  }

  let { tabs }: Props = $props();

  const flipDurationMs = 200;

  let items = $derived(tabs.map((tab) => ({ id: tab.id, tab })));

  function handleDndConsider(e: CustomEvent) {
    items = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent) {
    items = e.detail.items;
    const newOrder = items.map((item) => item.id);
    reorderTabs(newOrder);
  }

  function setActive(id: string) {
    activeTabId.set(id);
  }

  function handleClose(e: Event, id: string) {
    e.stopPropagation();
    closeTab(id);
  }

  function handleKeyboardClose(e: KeyboardEvent, id: string) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      e.stopPropagation();
      closeTab(id);
    }
  }
</script>

<div class="flex h-full flex-col overflow-hidden">
  <!-- Tabs bar -->
  <div
    class="tab-bar"
    role="tablist"
    use:dndzone={{ items, flipDurationMs, type: "tabs" }}
    onconsider={handleDndConsider}
    onfinalize={handleDndFinalize}
  >
    {#each items as { id, tab } (id)}
      {@const isActive = $activeTabId === id}

      <div
        class="tab-item {isActive ? 'active' : ''}"
        role="tab"
        aria-selected={isActive}
      >
        <!-- Indicador activo superior -->
        {#if isActive}
          <div class="tab-active-indicator"></div>
        {/if}

        <!-- Tab button -->
        <button
          class="tab-button {isActive ? 'active' : ''}"
          onclick={() => setActive(id)}
          tabindex="0"
        >
          {#if tab.isDirty}
            <span class="tab-dirty-indicator" title="Cambios sin guardar">
              <span class="tab-dirty-ping"></span>
              <span class="tab-dirty-dot"></span>
            </span>
          {/if}

          <span class="select-none">{tab.title}</span>
        </button>

        <!-- Close button -->
        <button
          class="tab-close-btn {isActive ? 'active' : ''}"
          type="button"
          aria-label="Cerrar tab"
          onclick={(e) => handleClose(e, id)}
          onkeydown={(e) => handleKeyboardClose(e, id)}
        >
          <X size={14} strokeWidth={2.5} />
        </button>
      </div>
    {/each}
  </div>

  <!-- Content area -->
  <div class="tab-content-area">
    {#each tabs as tab (tab.id)}
      {#if $activeTabId === tab.id}
        {@const Component = tab.component}
        <div class="tab-panel" role="tabpanel">
          <Component tabId={tab.id} data={tab.data} />
        </div>
      {/if}
    {/each}
  </div>
</div>