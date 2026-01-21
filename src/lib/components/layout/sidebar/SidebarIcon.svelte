<!-- src/lib/components/layout/sidebar/SidebarIcon.svelte -->
<script lang="ts">
	import type { SidebarItem } from '../../../types/Sidebar';

	interface Props {
		item: SidebarItem;
		isActive?: boolean;
		onSelect: (item: SidebarItem) => void;
	}

	let { item, isActive = false, onSelect }: Props = $props();

	const Icon = $derived(item.icon);

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
	class="sidebar-icon-btn group {isActive ? 'active' : ''}"
	onclick={handleClick}
	onkeydown={handleKeydown}
	tabindex="0"
>
	<Icon size={22} class="transition-transform duration-200 group-hover:scale-110" />

	<span class="sidebar-icon-tooltip">
		{item.label}
	</span>
</button>
