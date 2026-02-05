<!-- src/lib/components/layout/AppShell.svelte -->
<!-- 
  AppShell: The productive shell of the application.
  Only rendered when user is authenticated.
  Contains: Sidebar, StatusBar, and the Tab system.
-->
<script lang="ts">
	import Sidebar from './sidebar/Sidebar.svelte';
	import StatusBar from './StatusBar.svelte';
	import Tabs from './Tabs.svelte';
	import { tabsStore, openTab } from '$lib/stores/tabs';
	import { generalSettings } from '$lib/stores/settingsStore';
	import { get } from 'svelte/store';

	// Initialize welcome tab if no tabs exist
	$effect(() => {
		const tabs = get(tabsStore);
		if (tabs.length === 0) {
			openTab({
				componentKey: 'welcome',
				title: 'Bienvenida',
				id: 'welcome'
			});
		}
	});
</script>

<div class="flex flex-col h-full bg-surface-1 text-primary overflow-hidden font-sans">
	<div class="flex flex-1 w-full overflow-hidden md:flex-row flex-col">
		{#if !$generalSettings.isKioskMode}
			<Sidebar />
		{/if}

		<div class="flex-1 bg-surface-1 overflow-auto relative flex flex-col">
			<Tabs tabs={$tabsStore} />
		</div>
	</div>

	{#if !$generalSettings.isKioskMode}
		<StatusBar />
	{/if}
</div>
