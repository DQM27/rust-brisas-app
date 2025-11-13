<!-- src/lib/components/layout/sidebar/SidebarIcon.svelte -->
<script lang="ts">
  import type { SidebarItem } from './types';

  export let item: SidebarItem;
  export let isActive: boolean = false;
  export let onSelect: (item: SidebarItem) => void;

  function handleClick() {
    onSelect(item);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onSelect(item);
    }
  }
</script>

<button
  class:active={isActive}
  on:click={handleClick}
  on:keydown={handleKeydown}
  title={item.label}
  tabindex="0"
>
  <svelte:component this={item.icon} size={22} />
  <span class="tooltip">{item.label}</span>
</button>

<style>
  button {
    background: none;
    border: none;
    color: #bbb;
    width: 100%;
    height: 42px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    cursor: pointer;
  }

  button.active {
    color: #fff;
    background: #3c3c3c;
  }

  button:hover {
    background: #3a3a3a;
    color: #fff;
  }

  .tooltip {
    position: absolute;
    left: 52px;
    background: #3a3a3a;
    white-space: nowrap;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 11.5px;
    display: none;
    z-index: 1000;
  }

  button:hover .tooltip {
    display: block;
  }
</style>