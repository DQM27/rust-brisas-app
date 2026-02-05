<script lang="ts">
	import { Toaster } from 'svelte-5-french-toast';
	import CustomToast from './shared/CustomToast.svelte';
</script>

<div class="toast-container">
	<Toaster
		position="bottom-right"
		toastOptions={{
			duration: 4000
		}}
		gutter={8}
	/>
</div>

<style>
	.toast-container {
		display: contents;
	}

	@media (max-width: 640px) {
		:global(.whatsapp-toast) {
			min-width: 0;
			max-width: calc(100vw - 32px);
			margin: 0 16px;
		}
	}
</style>
