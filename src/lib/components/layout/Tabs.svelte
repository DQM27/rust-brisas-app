<script lang="ts">
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';
  import { activeTabId, closeTab, reorderTabs } from '$lib/stores/tabs';
  import type { HydratedTab } from '$lib/types/tabs';
  import { X } from 'lucide-svelte';

  export let tabs: HydratedTab[] = [];

  const flipDurationMs = 200;

  // Preparar items para drag & drop
  $: items = tabs.map(tab => ({ 
    id: tab.id, 
    tab 
  }));

  function handleDndConsider(e: CustomEvent) {
    items = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent) {
    items = e.detail.items;
    const newOrder = items.map(item => item.id);
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
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      e.stopPropagation();
      closeTab(id);
    }
  }
</script>

<div class="tabs-container">
  <div 
    class="tabs" 
    role="tablist"
    use:dndzone={{ items, flipDurationMs, type: 'tabs' }}
    on:consider={handleDndConsider}
    on:finalize={handleDndFinalize}
  >
    {#each items as { id, tab } (id)}
      <div
        class="tab"
        class:active={$activeTabId === id}
        class:dirty={tab.isDirty}
        role="tab"
        aria-selected={$activeTabId === id}
        animate:flip={{ duration: flipDurationMs }}
      >
        <button
          class="tab-button"
          on:click={() => setActive(id)}
          tabindex="0"
        >
          {#if tab.isDirty}
            <span class="dirty-indicator" title="Cambios sin guardar">●</span>
          {/if}
          
          <span class="tab-title">{tab.title}</span>
        </button>

        <button
          class="close-btn"
          type="button"
          aria-label="Cerrar tab"
          on:click={(e) => handleClose(e, id)}
          on:keydown={(e) => handleKeyboardClose(e, id)}
        >
          <X size={14} />
        </button>
      </div>
    {/each}
  </div>

  <div class="content">
    {#each tabs as tab (tab.id)}
      {#if $activeTabId === tab.id}
        <div class="tab-content" role="tabpanel">
          <svelte:component this={tab.component} tabId={tab.id} data={tab.data} />
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .tabs-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .tabs {
    display: flex;
    background: #252526;
    border-bottom: 1px solid #3c3c3c;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: thin;
  }

  .tabs::-webkit-scrollbar {
    height: 4px;
  }

  .tabs::-webkit-scrollbar-thumb {
    background: #555;
    border-radius: 2px;
  }

  .tab {
    position: relative;
    display: flex;
    align-items: center;
    gap: 4px;
    background: #2d2d2d;
    border-right: 1px solid #3c3c3c;
    transition: background-color 0.15s;
  }

  .tab:hover {
    background: #3a3a3a;
  }

  .tab.active {
    background: #1e1e1e;
    color: #fff;
  }

  .tab-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    flex: 1;
    cursor: pointer;
    font-size: 13px;
    background: none;
    border: none;
    color: inherit;
    white-space: nowrap;
    outline: none;
  }

  .tab-button:focus-visible {
    box-shadow: inset 0 0 0 1px #007acc;
  }

  .dirty-indicator {
    color: #fff;
    font-size: 16px;
    line-height: 1;
    opacity: 0.8;
  }

  .tab-title {
    user-select: none;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px 8px;
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    border-radius: 2px;
    opacity: 0.6;
    transition: opacity 0.15s, background-color 0.15s;
  }

  .close-btn:hover {
    opacity: 1;
    background: rgba(255, 255, 255, 0.1);
  }

  .tab.active .close-btn:hover {
    color: #e81123;
  }

  .content {
    flex: 1;
    background: #1e1e1e;
    overflow: hidden;
    position: relative;
  }

  .tab-content {
    width: 100%;
    height: 100%;
    overflow: auto;
  }
</style>