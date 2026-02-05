<script lang="ts">
	import { CheckCircle2, XCircle, Loader2 } from 'lucide-svelte';

	interface Props {
		toast: any;
		icon?: any;
		message?: any;
		position?: string;
	}

	let { toast, icon, message, position }: Props = $props();
</script>

<div
	class="whatsapp-toast-inner"
	data-type={toast.type}
	data-position={position}
	data-exit={toast.visible ? undefined : ''}
>
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
		background: var(--color-surface-secondary, #252526);
		color: var(--color-text-primary, #e8e8e8);
		padding: 10px 14px;
		border-radius: 8px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
		border: 1px solid var(--color-border-subtle, rgba(255, 255, 255, 0.05));
		min-width: 260px;
		max-width: 350px;
		margin: 0;
		animation: slideInWhatsApp 0.25s cubic-bezier(0.2, -0.1, 0.1, 1.2);
	}

	/* Alineación manual basada en la posición configurada para evitar centrado por defecto */
	.whatsapp-toast-inner[data-position^='top-left'],
	.whatsapp-toast-inner[data-position^='bottom-left'] {
		margin-right: auto;
	}

	.whatsapp-toast-inner[data-position^='top-right'],
	.whatsapp-toast-inner[data-position^='bottom-right'] {
		margin-left: auto;
	}

	.whatsapp-toast-inner[data-position$='-center'] {
		margin-left: auto;
		margin-right: auto;
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
