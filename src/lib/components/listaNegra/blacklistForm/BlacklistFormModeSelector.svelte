<!-- BlacklistFormModeSelector.svelte -->
<script lang="ts">
	interface Props {
		modoRegistro: 'existente' | 'manual';
		loading?: boolean;
		onModeChange: (mode: 'existente' | 'manual') => void;
	}

	let { modoRegistro, loading = false, onModeChange }: Props = $props();
</script>

<div class="border-b border-border-subtle mb-6">
	<nav class="-mb-px flex space-x-8" aria-label="Tabs">
		<button
			type="button"
			onclick={() => onModeChange('existente')}
			disabled={loading}
			class="group inline-flex items-center gap-2 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium transition-colors {modoRegistro ===
			'existente'
				? 'border-error text-error'
				: 'border-transparent text-tertiary hover:border-border-emphasis hover:text-primary'}"
		>
			<svg
				class="h-5 w-5 {modoRegistro === 'existente'
					? 'text-error'
					: 'text-tertiary group-hover:text-primary'}"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
				/>
			</svg>
			<span>Contratista Existente</span>
		</button>

		<button
			type="button"
			onclick={() => onModeChange('manual')}
			disabled={loading}
			class="group inline-flex items-center gap-2 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium transition-colors {modoRegistro ===
			'manual'
				? 'border-error text-error'
				: 'border-transparent text-tertiary hover:border-border-emphasis hover:text-primary'}"
		>
			<svg
				class="h-5 w-5 {modoRegistro === 'manual'
					? 'text-error'
					: 'text-tertiary group-hover:text-primary'}"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
				/>
			</svg>
			<span>Registro Manual</span>
		</button>
	</nav>
</div>
