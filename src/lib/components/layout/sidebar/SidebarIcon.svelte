<!-- src/lib/components/layout/sidebar/SidebarIcon.svelte -->
<script lang="ts">
  import type { SidebarItem } from "../../../types/Sidebar";

  export let item: SidebarItem;
  export let isActive: boolean = false;
  export let onSelect: (item: SidebarItem) => void;

  function handleClick() {
    onSelect(item);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      onSelect(item);
    }
  }
</script>

<button
  class="sidebar-icon-btn group {isActive ? 'active' : ''}"
  on:click={handleClick}
  on:keydown={handleKeydown}
  tabindex="0"
>
  <svelte:component
    this={item.icon}
    size={22}
    class="transition-transform duration-200 group-hover:scale-110"
  />

  <span class="sidebar-icon-tooltip">
    {item.label}
  </span>
</button>