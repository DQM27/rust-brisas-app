<script lang="ts">
	import { CheckCircle2, XCircle, Loader2 } from 'lucide-svelte';

	interface Props {
		toast: any;
		icon?: any;
		message?: any;
	}

	let { toast, icon, message }: Props = $props();
</script>

<div class="whatsapp-toast-inner" data-type={toast.type} data-exit={toast.visible ? undefined : ''}>
	<div class="toast-icon-wrapper">
		{#if toast.type === 'success'}
			<CheckCircle2 size={20} class="icon-success" />
		{:else if toast.type === 'error'}
			<XCircle size={20} class="icon-error" />
		{:else if toast.type === 'loading'}
			<Loader2 size={20} class="icon-loading" />
		{:else if icon}
			{@render icon()}
		{/if}
	</div>
	<div class="toast-message">
		{#if typeof message === 'function'}
			{@render message()}
		{:else}
			{message}
		{/if}
	</div>
</div>

<style>
	.whatsapp-toast-inner {
		display: flex;
		align-items: center;
		gap: 12px;
		background: var(--surface-2, #2a2f32);
		color: var(--text-primary, #e9edef);
		padding: 12px 16px;
		border-radius: 12px;
		box-shadow:
			0 10px 15px -3px rgba(0, 0, 0, 0.2),
			0 4px 6px -2px rgba(0, 0, 0, 0.1);
		border: 1px solid var(--border-emphasis, rgba(255, 255, 255, 0.1));
		min-width: 280px;
		max-width: 400px;
		animation: slideInWhatsApp 0.25s cubic-bezier(0.2, -0.1, 0.1, 1.2);
	}

	.whatsapp-toast-inner[data-exit] {
		animation: slideOutWhatsApp 0.2s ease-out forwards;
	}

	.toast-icon-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.toast-message {
		flex: 1;
		line-height: 1.4;
	}

	:global(.icon-success) {
		color: #25d366;
	}

	:global(.icon-error) {
		color: #dc4a3e;
	}

	:global(.icon-loading) {
		color: #34b7f1;
		animation: spin 1s linear infinite;
	}

	@keyframes slideInWhatsApp {
		from {
			transform: translateY(20px) scale(0.95);
			opacity: 0;
		}
		to {
			transform: translateY(0) scale(1);
			opacity: 1;
		}
	}

	@keyframes slideOutWhatsApp {
		from {
			transform: translateY(0) scale(1);
			opacity: 1;
		}
		to {
			transform: translateY(10px) scale(0.95);
			opacity: 0;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
