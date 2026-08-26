<!-- src/lib/components/gafete/modals/ResolveAlertModal.svelte -->
<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { X } from '@lucide/svelte';

	interface Props {
		show: boolean;
		gafeteNumero: string;
		nombrePersona: string;
		cedulaPersona?: string;
		empresaPersona?: string;
		fechaReporte: string;
		tipoAsignacion?: string;
		reportadoPor?: string;
		loading?: boolean;
		onResolve: (notas: string) => void;
		onCancel: () => void;
	}

	let {
		show,
		gafeteNumero,
		nombrePersona,
		cedulaPersona = '-',
		empresaPersona = '-',
		fechaReporte,
		tipoAsignacion = 'General',
		reportadoPor = 'Sistema',
		loading = false,
		onResolve,
		onCancel
	}: Props = $props();

	let notas = $state('');

	// Estilos basados en ui-patterns.md
	const labelClass = 'block text-xs font-medium text-secondary mb-1';

	function formatDisplayDate(val: string) {
		if (!val) return '-';
		try {
			// Limpiar formato d'...' de SurrealDB si viene así
			let cleanVal = val;
			if (cleanVal.startsWith("d'") || cleanVal.startsWith('d"')) {
				cleanVal = cleanVal.substring(2, cleanVal.length - 1);
			}

			const date = new Date(cleanVal);
			if (isNaN(date.getTime())) return val;

			const datePart = date.toLocaleDateString('es-PA', {
				day: '2-digit',
				month: '2-digit',
				year: 'numeric'
			});
			const timePart = date.toLocaleTimeString('es-PA', {
				hour: '2-digit',
				minute: '2-digit',
				hour12: false
			});

			return `${datePart} ${timePart}`;
		} catch {
			return val;
		}
	}
</script>

{#if show}
	<div
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="relative z-10 w-full max-w-[500px] bg-surface-2 shadow-2xl border border-surface rounded-xl flex flex-col max-h-[95vh] overflow-hidden"
			transition:fly={{ y: 20, duration: 250 }}
		>
			<!-- Header Maestro -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-xl font-semibold text-primary">Resolver Alerta de Gafete</h2>
				<button
					onclick={onCancel}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Contenido Scrollable -->
			<div class="flex-1 p-6 space-y-6 overflow-y-auto bg-surface-2">
				<!-- Context Card - Pattern Alertas -->
				<div class="bg-yellow-900/10 border border-yellow-700/30 rounded-lg p-5">
					<div class="space-y-4">
						<div>
							<p class="text-sm text-primary font-medium tracking-tight">Incidencia Detectada</p>
							<p class="text-xs text-secondary mt-1 leading-relaxed">
								Se está gestionando la resolución del gafete <span class="font-bold text-yellow-500"
									>#{gafeteNumero}</span
								> adeudado por:
							</p>
							<div class="mt-2 flex flex-col">
								<span class="font-bold text-primary text-sm">{nombrePersona}</span>
								<div class="flex items-center gap-2 mt-0.5">
									<span class="text-[10px] bg-white/5 px-1.5 rounded text-secondary font-mono"
										>{cedulaPersona}</span
									>
									<span class="text-[10px] text-tertiary uppercase font-medium tracking-tight"
										>{empresaPersona}</span
									>
								</div>
							</div>
						</div>

						<!-- Badge Info Grid -->
						<div class="grid grid-cols-2 gap-4 pt-3 border-t border-yellow-700/20">
							<div class="space-y-1">
								<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
									>Tipo</span
								>
								<p class="text-xs font-semibold text-primary">{tipoAsignacion}</p>
							</div>
							<div class="space-y-1">
								<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
									>Reportado</span
								>
								<p class="text-xs font-semibold text-primary">
									{formatDisplayDate(fechaReporte)}
								</p>
							</div>
							<div class="col-span-2 space-y-1">
								<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
									>Registrado por</span
								>
								<p class="text-xs font-semibold text-primary uppercase tracking-tight">
									{reportadoPor}
								</p>
							</div>
						</div>
					</div>
				</div>

				<!-- Card de Inputs -->
				<div class="bg-surface-1 rounded-lg border border-surface p-6 shadow-inner">
					<div class="space-y-2">
						<label for="notasResolucion" class={labelClass}>
							Notas de Resolución <span class="text-red-500">*</span>
						</label>
						<!-- Textarea Container Pattern -->
						<div
							class="w-full bg-black/20 border border-white/10 rounded-lg transition-all focus-within:!border-blue-500/50 focus-within:!ring-1 focus-within:!ring-blue-500/20"
						>
							<textarea
								id="notasResolucion"
								bind:value={notas}
								rows="4"
								disabled={loading}
								placeholder="Ej: El visitante devolvió el gafete o se procedió con el cobro..."
								class="w-full bg-transparent px-3 py-2.5 text-sm text-white placeholder:text-gray-500 focus:outline-none resize-none border-none outline-none appearance-none ring-0 min-h-[100px]"
							></textarea>
						</div>
						<p class="text-[11px] text-tertiary">
							Describa brevemente el motivo de la resolución del incidente.
						</p>
					</div>
				</div>
			</div>

			<!-- Footer de Acciones Binarias -->
			<div
				class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<button
					type="button"
					onclick={onCancel}
					disabled={loading}
					class="px-5 py-2 rounded-lg border-2 border-surface text-secondary font-bold uppercase tracking-wider transition-all duration-200 hover:border-white/60 hover:text-white/80 text-[11px]"
				>
					Cancelar
				</button>

				<button
					type="button"
					onclick={() => onResolve(notas)}
					disabled={loading || !notas.trim()}
					class="inline-flex items-center justify-center gap-2 px-6 py-2 rounded-lg border-2 border-surface text-secondary font-bold uppercase tracking-wider transition-all duration-200 hover:border-success hover:text-success text-[11px] disabled:opacity-30 disabled:cursor-not-allowed"
				>
					{#if loading}
						<div class="loading loading-spinner loading-xs"></div>
						<span>Procesando...</span>
					{:else}
						<span>Marcar como Resuelto</span>
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Fix text-fill for autocomplete and general textarea focus */
	textarea:focus {
		outline: none !important;
	}
</style>
