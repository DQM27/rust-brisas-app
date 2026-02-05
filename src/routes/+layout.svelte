<!-- src/routes/+layout.svelte -->
<!-- 
  System Shell: Bootstrap services and provide global providers.
  This layout does NOT handle UI structure - that's AppShell's job.
-->
<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { isAuthenticated, currentUser } from '$lib/stores/auth';
	import { initNetworkMonitor } from '$lib/stores/network';
	import { setupWizardVisible } from '$lib/stores/ui';
	import { modulesStore } from '$lib/stores/modules';
	import { needsSetup } from '$lib/logic/keyring/keyringService';
	import { windowService } from '$lib/logic/system/windowService';
	import { auditService } from '$lib/logic/audit/auditService';
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
				await modulesStore.load();
				const setupRes = await needsSetup();
				$setupWizardVisible = setupRes.ok ? setupRes.data : false;
			} catch (e) {
				console.error('[Layout] Bootstrap error:', e);
			} finally {
				checkingSetup = false;
			}
		})();

		const cleanupNetwork = initNetworkMonitor();
		setupCloseHandler();

		// Show window when frontend ready
		appApi.showMainWindow().catch(console.error);

		return () => {
			cleanupNetwork();
		};
	});

	/**
	 * Configura el cierre de la app para registrar auditoría
	 */
	async function setupCloseHandler() {
		const { getCurrentWindow } = await import('@tauri-apps/api/window');
		await getCurrentWindow().onCloseRequested(async () => {
			const auth = get(isAuthenticated);
			const user = get(currentUser);

			if (auth && user) {
				const { getCurrentSessionDuration } = await import('$lib/stores/sessionStore');
				await auditService.log(
					'LOGOUT',
					user.nombreCompleto,
					'Application Exit (Manual)',
					getCurrentSessionDuration()
				);
			}
		});
	}

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
	<div class="flex flex-col h-screen bg-surface-1 text-primary overflow-hidden font-sans">
		<Toast />
		{@render children()}

		<!-- Global Managers & Providers -->
		<ScreensaverManager />
		<GlobalUIProviders {authenticated} />
	</div>
{/if}

<style>
	/* Global layout resets if needed */
</style>
