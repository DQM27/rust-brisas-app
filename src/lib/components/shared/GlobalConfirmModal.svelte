<script lang="ts">
	import { fade, scale } from 'svelte/transition';
	import { AlertTriangle, X, ShieldAlert, BadgeInfo } from 'lucide-svelte';
	import { confirmState, closeConfirm } from '$lib/stores/confirm.svelte';
	import { shortcutRegistry } from '$lib/shortcuts';

	// Derived state for aesthetics
	const { isOpen, options } = $derived(confirmState);

	const headerColor = $derived(
		options.type === 'danger'
			? 'text-red-400'
			: options.type === 'warning'
				? 'text-amber-400'
				: 'text-blue-400'
	);

	const iconColor = $derived(
		options.type === 'danger'
			? 'text-red-500'
			: options.type === 'warning'
				? 'text-amber-500'
				: 'text-blue-500'
	);

	const btnColor = $derived(
		options.type === 'danger'
			? 'bg-red-600 hover:bg-red-700 shadow-red-500/20'
			: options.type === 'warning'
				? 'bg-amber-600 hover:bg-amber-700 shadow-amber-500/20'
				: 'bg-blue-600 hover:bg-blue-700 shadow-blue-500/20'
	);

	let loading = $state(false);

	async function handleConfirm() {
		try {
			loading = true;
			await options.onConfirm();
		} finally {
			loading = false;
			closeConfirm();
		}
	}

	// Activar scope modal
	$effect(() => {
		if (confirmState.isOpen) {
			shortcutRegistry.pushScope('modal');
			return () => shortcutRegistry.popScope();
		}
	});
</script>

{#if confirmState.isOpen}
	<div
		class="fixed inset-0 z-[9999] flex items-center justify-center bg-black/80 backdrop-blur-sm p-4"
		transition:fade={{ duration: 200 }}
		onclick={closeConfirm}
		role="button"
		tabindex="0"
		onkeydown={(e) => e.key === 'Escape' && closeConfirm()}
	>
		<div
			class="w-full max-w-md overflow-hidden rounded-xl border border-surface bg-surface-2 shadow-2xl relative"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
			role="alertdialog"
			aria-modal="true"
			tabindex="-1"
			onkeydown={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div class="flex items-center justify-between px-6 py-4 border-b border-surface bg-surface-2">
				<h3 class="text-lg font-semibold {headerColor}">
					{options.title}
				</h3>
				<button
					onclick={closeConfirm}
					class="text-secondary hover:text-primary transition-colors p-1.5 rounded-lg hover:bg-surface-3"
					disabled={loading}
				>
					<X size={20} />
				</button>
			</div>

			<!-- Body -->
			<div class="px-6 py-8">
				<p class="text-secondary leading-relaxed text-[15px]">
					{options.message}
				</p>
			</div>

			<!-- Footer -->
			<div class="flex justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1">
				<button
					onclick={closeConfirm}
					type="button"
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-surface/80 hover:text-primary text-sm"
					disabled={loading}
				>
					{options.cancelText}
				</button>
				<button
					onclick={handleConfirm}
					type="button"
					class="flex items-center gap-2 px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 text-sm hover:bg-surface-3 disabled:opacity-50 disabled:cursor-not-allowed {options.type ===
					'danger'
						? 'hover:!border-red-500 hover:!text-red-500'
						: options.type === 'warning'
							? 'hover:!border-amber-500 hover:!text-amber-500'
							: 'hover:!border-blue-500 hover:!text-blue-500'}"
					disabled={loading}
				>
					{#if loading}
						<div
							class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"
						></div>
					{/if}
					{options.confirmText}
				</button>
			</div>
		</div>
	</div>
{/if}
