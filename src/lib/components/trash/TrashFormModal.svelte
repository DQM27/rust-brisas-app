<script lang="ts">
	import { X, RotateCcw, Trash2 } from 'lucide-svelte';

	interface TrashItem {
		id: string;
		[key: string]: any;
	}

	interface Props {
		show: boolean;
		item: TrashItem | null;
		action: 'restore' | 'delete' | null;
		entityName?: string;
		loading?: boolean;
		onConfirm: () => void;
		onClose: () => void;
	}

	let {
		show = false,
		item = null,
		action = null,
		entityName = 'Elemento',
		loading = false,
		onConfirm,
		onClose
	}: Props = $props();

	// Derived content based on action
	const title = $derived(
		action === 'restore' ? `Restaurar ${entityName}` : `Eliminar Permanentemente`
	);
	const confirmLabel = $derived(action === 'restore' ? 'Restaurar' : 'Eliminar');
	const confirmVariant = $derived(
		action === 'restore' ? 'bg-blue-600 hover:bg-blue-700' : 'bg-red-600 hover:bg-red-700'
	);
	const Icon = $derived(action === 'restore' ? RotateCcw : Trash2);
</script>

{#if show}
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-0">
		<!-- Backdrop -->
		<div
			class="absolute inset-0 bg-black/50 backdrop-blur-sm transition-opacity"
			aria-hidden="true"
			onclick={onClose}
		></div>

		<!-- Modal Panel -->
		<div
			class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-auto overflow-hidden animate-in fade-in zoom-in-95 duration-200"
		>
			<!-- Header -->
			<div
				class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center"
			>
				<h3 class="text-lg font-medium text-gray-900 dark:text-gray-100 flex items-center gap-2">
					<Icon class="w-5 h-5 {action === 'restore' ? 'text-blue-500' : 'text-red-500'}" />
					{title}
				</h3>
				<button
					onclick={onClose}
					class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 transition-colors"
				>
					<X class="w-5 h-5" />
				</button>
			</div>

			<!-- Content -->
			<div class="px-6 py-6">
				{#if item}
					<p class="text-sm text-gray-600 dark:text-gray-300">
						¿Estás seguro de que deseas {action === 'restore'
							? 'restaurar'
							: 'eliminar permanentemente'} a
						<span class="font-semibold text-gray-900 dark:text-white">
							{item.nombre || item.nombreCompleto || 'este elemento'}
						</span>?
					</p>
					{#if action === 'restore'}
						<p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
							El elemento volverá a estar visible en la lista principal.
						</p>
					{:else}
						<p class="mt-2 text-xs text-red-500 dark:text-red-400 font-medium">
							Esta acción no se puede deshacer.
						</p>
					{/if}
				{/if}
			</div>

			<!-- Footer -->
			<div class="px-6 py-4 bg-gray-50 dark:bg-gray-900/50 flex justify-end gap-3 rounded-b-lg">
				<button
					onclick={onClose}
					disabled={loading}
					class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700"
				>
					Cancelar
				</button>
				<button
					onclick={onConfirm}
					disabled={loading}
					class="px-4 py-2 text-sm font-medium text-white rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 {confirmVariant} flex items-center gap-2"
				>
					{#if loading}
						<svg class="animate-spin h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						Procesando...
					{:else}
						{confirmLabel}
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}
