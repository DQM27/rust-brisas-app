<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { shortcutRegistry } from '$lib/shortcuts';
	import { getUserShortcuts } from '$lib/shortcuts/userShortcutsService';

	onMount(async () => {
		try {
			// Cargar personalizaciones del usuario
			const customs = await getUserShortcuts();
			shortcutRegistry.init(customs);
		} catch (e) {
			console.error('[Shortcuts] Error loading user customizations', e);
			// Inicializar con defaults si falla
			shortcutRegistry.init();
		}
	});

	onDestroy(() => {
		shortcutRegistry.destroy();
	});
</script>
