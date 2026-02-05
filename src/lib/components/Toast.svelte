<script lang="ts">
	import { Toaster } from 'svelte-5-french-toast';
	import { generalSettings } from '$lib/stores/settingsStore';
	import CustomToast from './shared/CustomToast.svelte';
</script>

<Toaster
	position={$generalSettings.toastPosition}
	toastOptions={{
		duration: 4000
	}}
	gutter={8}
/>
