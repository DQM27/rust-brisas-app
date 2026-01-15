<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';

	import type { ComponentType } from 'svelte';
	export let tabs: { id: string; label: string; icon?: ComponentType }[] = [];
	export let activeTab: string;

	const dispatch = createEventDispatcher();

	function selectTab(id: string) {
		if (activeTab === id) return;
		dispatch('change', id);
	}
</script>

<div class="flex items-center gap-1 border-b border-surface-2 bg-surface-1 px-4 mb-0">
	{#each tabs as tab}
		<button
			class="group relative flex items-center gap-2 px-4 py-3 text-sm font-medium transition-colors outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset
      {activeTab === tab.id ? 'text-primary' : 'text-secondary hover:text-primary-hover'}"
			onclick={() => selectTab(tab.id)}
		>
			{#if tab.icon}
				<svelte:component
					this={tab.icon}
					size={16}
					class={activeTab === tab.id
						? 'text-primary'
						: 'text-secondary group-hover:text-primary-hover'}
				/>
			{/if}
			<span>{tab.label}</span>

			<!-- Active Indicator (GitHub Style Underline) -->
			{#if activeTab === tab.id}
				<div
					class="absolute bottom-0 left-0 h-[2px] w-full bg-primary"
					transition:fade={{ duration: 150 }}
				></div>
			{/if}
		</button>
	{/each}
</div>
