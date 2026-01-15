<!-- BlacklistContratistaInfo.svelte -->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import type { BlockCheckResponse } from '$lib/types/listaNegra';

	interface ContratistaData {
		id: string;
		nombreCompleto: string;
		cedula: string;
		empresaNombre?: string;
	}

	interface Props {
		contratista: ContratistaData;
		blockInfo: BlockCheckResponse | null;
		checkingBlock: boolean;
		onClear: () => void;
	}

	let { contratista, blockInfo, checkingBlock, onClear }: Props = $props();
</script>

<div class="space-y-5" in:fly={{ y: -10, duration: 300 }}>
	{#if checkingBlock}
		<div class="card-base bg-surface-2 border-emphasis p-4">
			<div class="flex items-center gap-2">
				<svg
					class="animate-spin h-5 w-5 text-secondary"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
					></path>
				</svg>
				<span class="text-sm text-primary">Verificando estado...</span>
			</div>
		</div>
	{:else if blockInfo?.isBlocked}
		<div class="card-base bg-error border-error p-4">
			<div class="flex items-start gap-3">
				<svg
					class="h-6 w-6 text-error flex-shrink-0 mt-0.5"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
					/>
				</svg>
				<div class="flex-1">
					<h4 class="text-sm font-semibold text-primary mb-2">⚠️ Persona Ya Bloqueada</h4>
					<div class="space-y-1.5 text-sm">
						<div class="flex items-center gap-2">
							<span class="text-secondary">Nombre:</span>
							<span class="text-primary font-medium">{contratista.nombreCompleto}</span>
						</div>
						<div class="flex items-center gap-2">
							<span class="text-secondary">Cédula:</span>
							<span class="text-primary">{contratista.cedula}</span>
						</div>
						{#if contratista.empresaNombre}
							<div class="flex items-center gap-2">
								<span class="text-secondary">Empresa:</span>
								<span class="text-primary">{contratista.empresaNombre}</span>
							</div>
						{/if}
						<div class="pt-2 mt-2 border-t border-emphasis">
							<div class="flex items-start gap-2">
								<span class="text-secondary">Motivo:</span>
								<span class="text-primary">{blockInfo.motivo}</span>
							</div>
							<div class="flex items-center gap-2 mt-1">
								<span class="text-secondary">Bloqueado por:</span>
								<span class="text-primary">{blockInfo.bloqueadoPor}</span>
							</div>
							{#if blockInfo.bloqueadoDesde}
								<div class="flex items-center gap-2 mt-1">
									<span class="text-secondary">Desde:</span>
									<span class="text-primary"
										>{new Date(blockInfo.bloqueadoDesde).toLocaleDateString('es-ES')}</span
									>
								</div>
							{/if}
						</div>
					</div>
				</div>
				<button
					type="button"
					onclick={onClear}
					class="text-error hover:text-error-hover transition-colors"
					title="Cambiar selección"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>
		</div>
	{:else}
		<div class="card-base bg-info border-info p-3">
			<div class="flex items-start justify-between">
				<div class="flex-1">
					<div class="flex items-center gap-2 mb-1.5">
						<svg class="h-4 w-4 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
							/>
						</svg>
						<h4 class="text-xs font-semibold text-primary">Contratista Seleccionado</h4>
					</div>
					<div class="space-y-1 text-xs">
						<div class="flex items-center gap-2">
							<span class="text-secondary font-medium">{contratista.nombreCompleto}</span>
							<span class="text-secondary opacity-75">|</span>
							<span class="text-secondary">Cédula: {contratista.cedula}</span>
						</div>
						{#if contratista.empresaNombre}
							<div class="text-secondary opacity-90">
								Empresa: {contratista.empresaNombre}
							</div>
						{/if}
					</div>
				</div>
				<button
					type="button"
					onclick={onClear}
					class="text-info hover:text-info-hover transition-colors"
					title="Cambiar selección"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>
		</div>
	{/if}
</div>
