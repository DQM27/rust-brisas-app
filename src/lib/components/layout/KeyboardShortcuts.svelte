<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { shortcutRegistry } from '$lib/shortcuts';

	onMount(() => {
		// Inicializar el registro con los atajos por defecto
		// En el futuro, esto podría cargar personalizaciones del usuario
		shortcutRegistry.init();
	});

	onDestroy(() => {
		shortcutRegistry.destroy();
	});
</script>



