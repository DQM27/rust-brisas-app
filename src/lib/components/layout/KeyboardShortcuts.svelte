<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { shortcutRegistry } from '$lib/logic/shortcuts/registry';
	import { DEFAULT_SHORTCUTS } from '$lib/logic/shortcuts/definitions';

	onMount(() => {
		// Inicializar el registro con los atajos por defecto
		// En el futuro, esto podría fusionarse con configuraciones de usuario
		shortcutRegistry.loadDefinitions(DEFAULT_SHORTCUTS);
		shortcutRegistry.init();
	});

	onDestroy(() => {
		shortcutRegistry.destroy();
	});
</script>
