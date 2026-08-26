<!-- src/routes/+layout.svelte -->
<!-- 
  System Shell: Bootstrap services and provide global providers.
  This layout does NOT handle UI structure - that's AppShell's job.
-->
<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { isAuthenticated } from '$lib/stores/auth';
	import { initNetworkMonitor } from '$lib/stores/network';
	import { setupWizardVisible } from '$lib/stores/ui';
	import { needsSetup } from '$lib/logic/keyring/keyringService';
	import { windowService } from '$lib/logic/system/windowService';
	import { appApi } from '$lib/api/app';

	// Global Providers (invisible components)
	import Toast from '$lib/components/Toast.svelte';
	import SetupWizard from '$lib/components/setup/SetupWizard.svelte';
	import GlobalUIProviders from '$lib/components/layout/GlobalUIProviders.svelte';
	import ScreensaverManager from '$lib/components/layout/ScreensaverManager.svelte';

	let { children } = $props();

	// Reactive states
	let authenticated = $derived($isAuthenticated);
	let showSetupWizard = $derived($setupWizardVisible);
	let checkingSetup = $state(true);

	// Bootstrap App
	onMount(() => {
		(async () => {
			try {
				const setupRes = await needsSetup();
				$setupWizardVisible = setupRes.ok ? setupRes.data : false;
			} catch (e) {
				console.error('[Layout] Bootstrap error:', e);
			} finally {
				checkingSetup = false;
			}
		})();

		const cleanupNetwork = initNetworkMonitor();

		// Show window when frontend ready
		appApi.showMainWindow().catch(console.error);

		return () => {
			cleanupNetwork();
		};
	});

	// Dynamic Window Management
	$effect(() => {
		if (checkingSetup) return;

		if (showSetupWizard) {
			windowService.setLauncherMode(true);
		} else if (!authenticated) {
			windowService.setLauncherMode(false);
		} else {
			windowService.setAppMode();
		}
	});

	async function handleSetupComplete() {
		$setupWizardVisible = false;
		await windowService.setAppMode();
	}
</script>

{#if checkingSetup}
	<div class="h-screen w-screen bg-blue-600 flex items-center justify-center">
		<span class="text-white text-xl">Verificando configuración...</span>
	</div>
{:else if showSetupWizard}
	<SetupWizard onComplete={handleSetupComplete} />
{:else}
	<Toast />
	<div class="flex flex-col h-screen bg-surface-1 text-primary overflow-hidden font-sans">
		{@render children()}

		<!-- Global Managers & Providers -->
		<ScreensaverManager />
		<GlobalUIProviders {authenticated} />
	</div>
{/if}

<style>
	/* Global layout resets if needed */
</style>
