<!-- src/lib/components/layout/sidebar/SidebarPanel.svelte -->
<script lang="ts">
  import { cubicOut, cubicIn } from "svelte/easing";
  import type { SidebarItem } from "../../../types/Sidebar";

  function slideHorizontal(
    node: HTMLElement,
    { duration, easing, opacity = 0 }: any,
  ) {
    const style = getComputedStyle(node);
    const width_value = parseFloat(style.width);

    return {
      duration,
      easing,
      css: (t: number) => {
        const eased = easing(t); // This is redundant if easing is passed to css, but css takes t which is already eased if easing is provided? No, t goes 0->1.
        // Actually svelte transition css function receives t (0..1) *after* easing if easing is in options? No, the `css` function receives `t` which is the eased value if `easing` is provided in the return object?
        // Wait, standard svelte transition: `css: (t, u) => css_string`. `t` is 0..1 (eased).

        return `
          width: ${t * 250}px;
          opacity: ${t};
          transform: translateX(${(1 - t) * -20}px);
        `;
      },
    };
  }

  export let item: SidebarItem;
  export let onClose: () => void;
</script>

<div
  class="flex w-[250px] flex-col border-r border-[#1f1f1f] bg-[#252526] overflow-hidden"
  in:slideHorizontal={{ duration: 500, easing: cubicOut }}
  out:slideHorizontal={{ duration: 400, easing: cubicIn }}
>
  <div
    class="flex items-center justify-between border-b border-[#2d2d2d]
             bg-[#2d2d2d] px-[15px] py-3 text-[13px] font-semibold text-[#cccccc]"
  >
    <span>{item.label}</span>
    <button
      class="flex h-5 w-5 items-center justify-center rounded border-none
               bg-transparent p-0 text-base text-[#858585] cursor-pointer
               transition-all duration-150
               hover:bg-[#3c3c3c] hover:text-[#cccccc] hover:rotate-90
               focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500"
      on:click={onClose}
      title="Cerrar panel"
    >
      ×
    </button>
  </div>
  <div class="flex-grow overflow-y-auto py-2">
    <svelte:component this={item.panelComponent} />
  </div>
</div>
