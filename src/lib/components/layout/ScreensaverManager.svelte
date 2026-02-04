<!-- src/lib/components/layout/ScreensaverManager.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import {
		isScreensaverActive,
		awaitingScreensaverPassword,
		attemptExitScreensaver
	} from '$lib/stores/sessionStore';
	import { generalSettings } from '$lib/stores/settingsStore';

	let screensaverActive = $derived($isScreensaverActive);

	// Efecto reactivo para manejar el modo screensaver FULLSCREEN
	$effect(() => {
		(async () => {
			if (screensaverActive) {
				try {
					const { getCurrentWindow } = await import('@tauri-apps/api/window');
					const appWindow = getCurrentWindow();

					const isMinimized = await appWindow.isMinimized();
					if (isMinimized) {
						await appWindow.unminimize();
						await new Promise((resolve) => setTimeout(resolve, 100));
					}

					const isMaximized = await appWindow.isMaximized();
					if (isMaximized) {
						await appWindow.unmaximize();
						await new Promise((resolve) => setTimeout(resolve, 100));
					}

					await appWindow.setAlwaysOnTop(true);
					await appWindow.setFocus();

					// Force kiosk mode FIRST to hide UI elements
					generalSettings.update((s) => ({ ...s, isKioskMode: true }));

					await appWindow.setFullscreen(true);
					console.log('[Screensaver] Activated - entering fullscreen');
				} catch (e) {
					console.error('[Screensaver] Error entering fullscreen:', e);
					generalSettings.update((s) => ({ ...s, isKioskMode: true }));
				}
			} else if (screensaverActive === false) {
				try {
					const { getCurrentWindow } = await import('@tauri-apps/api/window');
					const appWindow = getCurrentWindow();
					const isCurrentlyFullscreen = await appWindow.isFullscreen();

					if (isCurrentlyFullscreen) {
						await appWindow.setFullscreen(false);
						await appWindow.setAlwaysOnTop(false);
						generalSettings.update((s) => ({ ...s, isKioskMode: false }));
						console.log('[Screensaver] Deactivated - exiting fullscreen');
					}
				} catch (e) {
					console.error('[Screensaver] Error exiting fullscreen:', e);
					generalSettings.update((s) => ({ ...s, isKioskMode: false }));
				}
			}
		})();
	});

	// Handle interaction when screensaver is active
	onMount(() => {
		let ignoreInteractions = false;

		const handleInteraction = () => {
			if (ignoreInteractions) return;

			if ($isScreensaverActive && !$awaitingScreensaverPassword) {
				attemptExitScreensaver();
			}
		};

		const events = ['mousedown', 'keydown', 'touchstart'];
		events.forEach((event) => {
			window.addEventListener(event, handleInteraction, {
				once: false,
				capture: true
			});
		});

		const unsubscribe = isScreensaverActive.subscribe((active) => {
			if (active) {
				ignoreInteractions = true;
				setTimeout(() => {
					ignoreInteractions = false;
				}, 500);
			}
		});

		return () => {
			events.forEach((event) => {
				window.removeEventListener(event, handleInteraction, { capture: true });
			});
			unsubscribe();
		};
	});
</script>
