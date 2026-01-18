<!-- src/lib/components/shared/PersonDetailModal.svelte -->
<script lang="ts">
	import {
		X,
		User,
		Briefcase,
		ShieldCheck,
		Clock,
		CreditCard,
		MapPin,
		LogOut,
		Calendar
	} from 'lucide-svelte';
	import type { IngresoResponse } from '$lib/types/ingreso';
	import { fade, scale } from 'svelte/transition';

	let {
		show = $bindable(false),
		person = null as IngresoResponse | null,
		onClose = () => {}
	} = $props();

	function handleClose() {
		show = false;
		onClose();
	}

	function formatDate(dateStr: string | undefined) {
		if (!dateStr) return '-';
		try {
			const d = new Date(dateStr);
			return d.toLocaleString('es-PA', {
				day: '2-digit',
				month: 'long',
				year: 'numeric',
				hour: '2-digit',
				minute: '2-digit'
			});
		} catch (e) {
			return dateStr;
		}
	}
</script>

{#if show && person}
	<div
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 200 }}
		onclick={handleClose}
	>
		<div
			class="relative w-full max-w-lg bg-surface-2 border border-white/10 rounded-xl shadow-2xl overflow-hidden"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div
				class="relative h-32 bg-gradient-to-br from-blue-600/20 to-purple-600/20 p-6 flex items-end"
			>
				<button
					class="absolute top-4 right-4 p-2 text-gray-400 hover:text-white bg-black/20 hover:bg-black/40 rounded-full transition-all"
					onclick={handleClose}
				>
					<X size={20} />
				</button>

				<div class="flex items-center gap-4">
					<div
						class="w-16 h-16 rounded-full bg-surface-1 border-2 border-blue-500/50 flex items-center justify-center text-blue-400 shadow-lg"
					>
						<User size={32} />
					</div>
					<div>
						<h3 class="text-xl font-bold text-white leading-tight uppercase tracking-tight">
							{person.nombreCompleto}
						</h3>
						<span
							class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider {person.estaAdentro
								? 'bg-green-500/10 text-green-400 border border-green-500/20'
								: 'bg-gray-500/10 text-gray-400 border border-gray-500/20'}"
						>
							<span
								class="w-1.5 h-1.5 rounded-full {person.estaAdentro
									? 'bg-green-400 animate-pulse'
									: 'bg-gray-400'}"
							></span>
							{person.estaAdentro ? 'Dentro del Recinto' : 'Fuera del Recinto'}
						</span>
					</div>
				</div>
			</div>

			<!-- Body -->
			<div class="p-6 space-y-6">
				<!-- Grid de Información -->
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1">
						<span
							class="text-[10px] text-gray-500 font-bold uppercase tracking-widest flex items-center gap-1.5"
						>
							<CreditCard size={12} /> Identificación
						</span>
						<p class="text-sm font-mono text-white bg-surface-3/50 px-2 py-1 rounded">
							{person.cedula}
						</p>
					</div>

					<div class="space-y-1">
						<span
							class="text-[10px] text-gray-500 font-bold uppercase tracking-widest flex items-center gap-1.5"
						>
							<Briefcase size={12} /> Empresa
						</span>
						<p class="text-sm font-medium text-gray-200">
							{person.empresaNombre || 'N/A'}
						</p>
					</div>

					<div class="space-y-1">
						<span
							class="text-[10px] text-gray-500 font-bold uppercase tracking-widest flex items-center gap-1.5"
						>
							<ShieldCheck size={12} /> Autorización
						</span>
						<p class="text-sm text-blue-300 font-medium">
							{person.tipoAutorizacionDisplay}
						</p>
					</div>

					<div class="space-y-1">
						<span
							class="text-[10px] text-gray-500 font-bold uppercase tracking-widest flex items-center gap-1.5"
						>
							<MapPin size={12} /> Modo Ingreso
						</span>
						<p class="text-sm text-gray-300 capitalize font-medium">
							{person.modoIngresoDisplay}
						</p>
					</div>
				</div>

				<hr class="border-white/5" />

				<!-- Historial de Último Movimiento -->
				<div class="space-y-4">
					<div class="flex items-start gap-3">
						<div class="mt-1 p-1.5 bg-blue-500/10 text-blue-400 rounded">
							<Calendar size={14} />
						</div>
						<div class="flex-1">
							<p class="text-[10px] text-gray-500 font-bold uppercase tracking-widest">
								Entrada Registrada
							</p>
							<p class="text-sm text-gray-300 mt-0.5">{formatDate(person.fechaHoraIngreso)}</p>
							<p class="text-[11px] text-gray-500 mt-0.5 italic">
								Por: {person.usuarioIngresoNombre}
							</p>
						</div>
					</div>

					{#if person.fechaHoraSalida}
						<div class="flex items-start gap-3">
							<div class="mt-1 p-1.5 bg-orange-500/10 text-orange-400 rounded">
								<LogOut size={14} />
							</div>
							<div class="flex-1">
								<p class="text-[10px] text-gray-500 font-bold uppercase tracking-widest">
									Salida Registrada
								</p>
								<p class="text-sm text-gray-300 mt-0.5">{formatDate(person.fechaHoraSalida)}</p>
								<p class="text-[11px] text-gray-500 mt-0.5 italic">
									Por: {person.usuarioSalidaNombre || 'Sistema'}
								</p>
							</div>
						</div>
					{/if}

					{#if person.tiempoPermanenciaTexto}
						<div
							class="flex items-center gap-3 bg-surface-3/30 p-3 rounded-lg border border-white/5"
						>
							<Clock size={16} class="text-indigo-400" />
							<div>
								<p class="text-[10px] text-gray-500 font-bold uppercase tracking-widest">
									Tiempo de Permanencia
								</p>
								<p class="text-xs text-indigo-300 font-mono">{person.tiempoPermanenciaTexto}</p>
							</div>
						</div>
					{/if}
				</div>

				{#if person.observaciones}
					<div class="bg-yellow-500/5 border border-yellow-500/10 p-3 rounded-lg">
						<span class="text-[10px] text-yellow-500/70 font-bold uppercase tracking-widest"
							>Observaciones</span
						>
						<p class="text-xs text-gray-400 mt-1 italic">
							"{person.observaciones}"
						</p>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="p-4 bg-surface-3 border-t border-white/5 flex justify-end">
				<button
					class="px-5 py-2 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-lg border border-white/10 transition-all"
					onclick={handleClose}
				>
					Cerrar Detalle
				</button>
			</div>
		</div>
	</div>
{/if}
