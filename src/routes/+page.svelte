<script lang="ts">
	// @ts-ignore
	import { onMount } from 'svelte';
	import { isAuthenticated as authStore } from '$lib/stores/auth';
	import { tabsStore, openTab } from '$lib/stores/tabs';
	// @ts-ignore
	import { get, type Writable } from 'svelte/store';
	import LoginPage from './LoginPage.svelte';
	import Main from './MainContent.svelte';

	const isAuthenticated = authStore as unknown as Writable<boolean>;

	// Cuando se autentica, inicializar tabs
	// @ts-ignore
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

<!-- @ts-ignore -->
{#if !$isAuthenticated}
	<LoginPage />
{:else}
	<Main />
{/if}
