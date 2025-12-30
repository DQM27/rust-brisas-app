<script lang="ts">
  import {
    activeTabId,
    closeTab,
    closeAllTabs,
    closeOtherTabs,
    closeTabsToRight,
  } from "$lib/stores/tabs";
  import type { HydratedTab } from "$lib/types/tab";
  import { X } from "lucide-svelte";

  import { generalSettings } from "$lib/stores/settingsStore";

  interface Props {
    tabs: HydratedTab[];
  }

  let { tabs }: Props = $props();

  let items = $derived(tabs.map((tab) => ({ id: tab.id, tab })));

  // Context menu state
  let contextMenu = $state<{
    visible: boolean;
    x: number;
    y: number;
    tabId: string;
    tabIndex: number;
  }>({
    visible: false,
    x: 0,
    y: 0,
    tabId: "",
    tabIndex: -1,
  });

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

  // Context menu handlers
  function handleContextMenu(e: MouseEvent, tabId: string, tabIndex: number) {
    e.preventDefault();
    e.stopPropagation();
    contextMenu = {
      visible: true,
      x: e.clientX,
      y: e.clientY,
      tabId,
      tabIndex,
    };
  }

  function closeContextMenu() {
    contextMenu.visible = false;
  }

  function handleCloseTab() {
    closeTab(contextMenu.tabId);
    closeContextMenu();
  }

  function handleCloseOthers() {
    closeOtherTabs(contextMenu.tabId);
    closeContextMenu();
  }

  function handleCloseAll() {
    closeAllTabs();
    closeContextMenu();
  }

  function handleCloseToRight() {
    closeTabsToRight(contextMenu.tabId);
    closeContextMenu();
  }

  // Close context menu when clicking outside
  function handleWindowClick(e: MouseEvent) {
    if (contextMenu.visible) {
      closeContextMenu();
    }
  }

  // Check if there are tabs to the right
  $effect(() => {
    // Reactivity for tabs count
    tabs.length;
  });
</script>

<svelte:window onclick={handleWindowClick} />

<div class="flex h-full flex-col overflow-hidden">
  <!-- Tabs bar -->
  {#if !$generalSettings.isKioskMode}
    <div class="tab-bar scrollbar-hide" role="tablist">
      {#each items as { id, tab }, index (id)}
        {@const isActive = $activeTabId === id}

        <div
          class="tab-item {isActive ? 'active' : ''}"
          role="tab"
          tabindex="0"
          aria-selected={isActive}
          oncontextmenu={(e) => handleContextMenu(e, id, index)}
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
  {/if}

  <!-- Content area - KEEP ALIVE: Components mount once, hide with CSS -->
  <div class="tab-content-area">
    {#each tabs as tab (tab.id)}
      {@const Component = tab.component}
      <div
        class="tab-panel"
        class:hidden={$activeTabId !== tab.id}
        role="tabpanel"
        aria-hidden={$activeTabId !== tab.id}
      >
        <Component
          tabId={tab.id}
          data={tab.data}
          isActive={$activeTabId === tab.id}
        />
      </div>
    {/each}
  </div>
</div>

<!-- Context Menu -->
{#if contextMenu.visible}
  <div
    class="context-menu"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
    role="menu"
  >
    <button class="context-menu-item" onclick={handleCloseTab} role="menuitem">
      Cerrar
    </button>
    <button
      class="context-menu-item"
      onclick={handleCloseOthers}
      role="menuitem"
      disabled={tabs.length <= 1}
    >
      Cerrar otras
    </button>
    <button
      class="context-menu-item"
      onclick={handleCloseToRight}
      role="menuitem"
      disabled={contextMenu.tabIndex >= tabs.length - 1}
    >
      Cerrar a la derecha
    </button>
    <div class="context-menu-separator"></div>
    <button class="context-menu-item" onclick={handleCloseAll} role="menuitem">
      Cerrar todas
    </button>
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 1000;
    min-width: 160px;
    padding: 4px 0;
    background: var(--color-surface-secondary, #1e1e1e);
    border: 1px solid var(--color-border-subtle, #3c3c3c);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .context-menu-item {
    display: block;
    width: 100%;
    padding: 6px 12px;
    text-align: left;
    font-size: 13px;
    color: var(--color-text-primary, #cccccc);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: background-color 0.1s;
  }

  .context-menu-item:hover:not(:disabled) {
    background: var(--color-surface-hover, #2a2d2e);
  }

  .context-menu-item:disabled {
    color: var(--color-text-tertiary, #6e6e6e);
    cursor: not-allowed;
  }

  .context-menu-separator {
    height: 1px;
    margin: 4px 0;
    background: var(--color-border-subtle, #3c3c3c);
  }

  /* Keep-alive pattern: hidden tabs take no space */
  .tab-panel.hidden {
    display: none;
  }
</style>
