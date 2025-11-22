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
  class="group relative flex h-[42px] w-full items-center justify-center border-none bg-transparent text-[#bbb]
         cursor-pointer transition-all duration-200 ease-in-out
         hover:bg-[#3a3a3a] hover:text-white
         focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 focus-visible:ring-offset-[#2d2d2d]
         {isActive ? 'bg-[#3c3c3c] text-white' : ''}"
  on:click={handleClick}
  on:keydown={handleKeydown}
  tabindex="0"
>
  <svelte:component
    this={item.icon}
    size={22}
    class="transition-transform duration-200 group-hover:scale-110"
  />

  <span
    class="absolute left-[52px] z-[1000] hidden whitespace-nowrap rounded bg-[#3a3a3a]
               px-2 py-1 text-[11.5px] shadow-lg
               animate-in fade-in slide-in-from-left-1 duration-150
               group-hover:block"
  >
    {item.label}
  </span>
</button>
