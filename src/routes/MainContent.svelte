<script lang="ts">
	import { isAuthenticated } from '$lib/stores/auth';
	import LoginPage from './LoginPage.svelte';
	import Tabs from '$lib/components/layout/Tabs.svelte';
	import { tabsStore, openTab } from '$lib/stores/tabs';
	import { get } from 'svelte/store';

	// Inicializar tabs cuando se autentica
	$effect(() => {
		if ($isAuthenticated) {
			const tabs = get(tabsStore);
			if (tabs.length === 0) {
				openTab({
					componentKey: 'welcome',
					title: 'Bienvenida',
					id: 'welcome'
				});
			}
		}
	});
</script>

{#if !$isAuthenticated}
	<LoginPage />
{:else}
	<!-- App Principal -->
	<div class="h-full bg-surface-1">
		<Tabs tabs={$tabsStore} />
	</div>
{/if}
