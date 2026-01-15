<!-- src/lib/components/layout/sidebar/SidebarPanel.svelte -->
<script lang="ts">
	import { cubicOut, cubicIn } from 'svelte/easing';
	import type { SidebarItem } from '../../../types/Sidebar';

	function slideHorizontal(_node: HTMLElement, { duration, easing }: any) {
		return {
			duration,
			easing,
			css: (t: number) => {
				return `
          width: ${t * 250}px;
          opacity: ${t};
          transform: translateX(${(1 - t) * -20}px);
        `;
			}
		};
	}

	export let item: SidebarItem;
	export let onClose: () => void;
</script>

<div
	class="sidebar-panel"
	in:slideHorizontal={{ duration: 500, easing: cubicOut }}
	out:slideHorizontal={{ duration: 400, easing: cubicIn }}
>
	<div class="sidebar-panel-header">
		<span>{item.label}</span>
		<button class="sidebar-panel-close" on:click={onClose} title="Cerrar panel"> × </button>
	</div>
	<div class="sidebar-panel-content">
		<svelte:component this={item.panelComponent} />
	</div>
</div>
