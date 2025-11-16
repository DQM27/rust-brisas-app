<script lang="ts">
  import { dndzone } from 'svelte-dnd-action';
  import { activeTabId, closeTab, reorderTabs } from '$lib/stores/tabs';
  import type { HydratedTab } from '$lib/types/tab';
  import { X } from 'lucide-svelte';

  interface Props {
    tabs: HydratedTab[];
  }

  let { tabs }: Props = $props();

  const flipDurationMs = 200;

  let items = $derived(tabs.map(tab => ({ id: tab.id, tab })));

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

<div class="flex h-full flex-col overflow-hidden">
  <!-- Tabs bar mejorado -->
  <div 
    class="flex overflow-x-auto overflow-y-hidden border-b border-[#3c3c3c] bg-[#252526] shadow-sm"
    role="tablist"
    use:dndzone={{ items, flipDurationMs, type: 'tabs' }}
    onconsider={handleDndConsider}
    onfinalize={handleDndFinalize}
  >
    {#each items as { id, tab } (id)}
      {@const isActive = $activeTabId === id}
      
      <div
        class="group relative flex items-center gap-1 border-r border-[#3c3c3c] transition-all duration-200
               {isActive 
                 ? 'bg-[#1e1e1e] text-white shadow-md' 
                 : 'bg-[#2d2d2d] text-gray-400 hover:bg-[#3a3a3a] hover:text-gray-200'}"
        role="tab"
        aria-selected={isActive}
      >
        <!-- Indicador activo superior -->
        {#if isActive}
          <div class="absolute left-0 right-0 top-0 h-0.5 bg-gradient-to-r from-[#007acc] to-[#0098ff]"></div>
        {/if}

        <!-- Tab button -->
        <button
          class="flex flex-1 items-center gap-2 whitespace-nowrap px-4 py-2.5 text-sm font-medium transition-all
                 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-[#007acc]
                 {isActive ? '' : 'hover:translate-x-0.5'}"
          onclick={() => setActive(id)}
          tabindex="0"
        >
          {#if tab.isDirty}
            <span 
              class="relative flex h-2 w-2" 
              title="Cambios sin guardar"
            >
              <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-[#007acc] opacity-75"></span>
              <span class="relative inline-flex h-2 w-2 rounded-full bg-[#007acc]"></span>
            </span>
          {/if}
          
          <span class="select-none">{tab.title}</span>
        </button>

        <!-- Close button mejorado -->
        <button
          class="mr-1 flex items-center justify-center rounded p-1.5 opacity-0 transition-all
                 group-hover:opacity-70 hover:!opacity-100
                 {isActive 
                   ? 'hover:bg-red-500/20 hover:text-red-400' 
                   : 'hover:bg-white/10 hover:text-white'}"
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

  <!-- Content area con transición -->
  <div class="relative flex-1 overflow-hidden bg-[#1e1e1e]">
    {#each tabs as tab (tab.id)}
      {#if $activeTabId === tab.id}
        {@const Component = tab.component}
        <div class="h-full w-full overflow-auto animate-fade-in" role="tabpanel">
          <Component tabId={tab.id} data={tab.data} />
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  /* Scrollbar personalizada */
  div[role="tablist"]::-webkit-scrollbar {
    height: 4px;
  }

  div[role="tablist"]::-webkit-scrollbar-thumb {
    background: #555;
    border-radius: 2px;
    transition: background 0.2s;
  }

  div[role="tablist"]::-webkit-scrollbar-thumb:hover {
    background: #666;
  }

  div[role="tablist"]::-webkit-scrollbar-track {
    background: transparent;
  }

  /* Animación de fade para contenido */
  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .animate-fade-in {
    animation: fade-in 0.15s ease-out;
  }
</style>