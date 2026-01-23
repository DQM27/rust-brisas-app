<script lang="ts">
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';
	import { Clock, MapPin, User, Building2, LogIn, Trash2, Calendar } from 'lucide-svelte';
	import { fade, fly, slide } from 'svelte/transition';

	interface Props {
		pendientes: PreRegistroVisita[];
		onIngreso: (p: PreRegistroVisita) => void;
		onCancel: (id: string) => void;
	}

	let { pendientes, onIngreso, onCancel }: Props = $props();

	// Group and sort by time
	const sortedVisits = $derived(
		[...pendientes].sort((a, b) => {
			const timeA = `${a.fecha_esperada || ''} ${a.hora_esperada || ''}`;
			const timeB = `${b.fecha_esperada || ''} ${b.hora_esperada || ''}`;
			return timeA.localeCompare(timeB);
		})
	);

	function formatTime(timeStr: string) {
		if (!timeStr) return '--:--';
		// Simple cleanup for display
		return timeStr.slice(0, 5);
	}

	function isToday(dateStr: string) {
		const today = new Date().toISOString().split('T')[0];
		return dateStr === today;
	}
</script>

<div class="flex flex-col h-full overflow-hidden bg-black/40 backdrop-blur-sm">
	<div class="flex-1 overflow-y-auto custom-scrollbar p-8">
		<div class="max-w-4xl mx-auto space-y-12">
			<!-- Header / Today Indicator -->
			<div class="flex items-center justify-between border-b border-surface/30 pb-6">
				<div class="space-y-1">
					<h2 class="text-3xl font-bold tracking-tight text-white">Focus Mode</h2>
					<p class="text-secondary text-sm flex items-center gap-2">
						<Calendar size={14} />
						{new Date().toLocaleDateString(undefined, {
							weekday: 'long',
							day: 'numeric',
							month: 'long'
						})}
					</p>
				</div>
				<div class="flex flex-col items-end">
					<span class="text-xs uppercase font-black tracking-widest text-accent/50"
						>Próximas Entradas</span
					>
					<span class="text-2xl font-mono text-accent">{sortedVisits.length}</span>
				</div>
			</div>

			<!-- Timeline -->
			<div class="relative pl-12 space-y-8">
				<!-- Timeline line -->
				<div
					class="absolute left-[23px] top-0 bottom-0 w-0.5 bg-gradient-to-b from-accent/50 via-surface-3 to-transparent"
				></div>

				{#each sortedVisits as visit, i (visit.id)}
					<div class="relative group" in:fly={{ y: 20, delay: i * 50, duration: 400 }}>
						<!-- Dot -->
						<div
							class="absolute -left-[12px] top-6 w-6 h-6 rounded-full bg-surface-1 border-2 border-accent flex items-center justify-center shadow-[0_0_15px_rgba(59,130,246,0.3)] z-10 transition-transform group-hover:scale-125"
						>
							<div class="w-2 h-2 rounded-full bg-accent animate-pulse"></div>
						</div>

						<!-- Card -->
						<div
							class="bg-surface-2/40 hover:bg-surface-2/60 border border-surface/50 hover:border-accent/40 rounded-2xl p-6 transition-all duration-300 backdrop-blur-md shadow-xl hover:shadow-accent/5 flex items-center gap-6"
						>
							<!-- Time Column -->
							<div
								class="flex flex-col items-center justify-center min-w-[70px] border-r border-surface/50 pr-6"
							>
								<span class="text-primary text-xl font-black font-mono tracking-tighter">
									{formatTime(visit.hora_esperada || '')}
								</span>
								<span class="text-[10px] uppercase font-bold text-secondary/60 tracking-widest">
									{visit.hora_esperada?.toLowerCase().includes('pm') ? 'PM' : 'AM'}
								</span>
							</div>

							<!-- Content -->
							<div class="flex-1 space-y-3">
								<div class="flex items-center justify-between">
									<div class="flex items-center gap-3">
										<h4 class="text-lg font-bold text-white tracking-tight">
											{visit.nombre}
											{visit.apellido}
										</h4>
										<span
											class="px-2 py-0.5 rounded-md bg-white/5 border border-white/10 text-[10px] font-mono text-secondary"
										>
											{visit.cedula}
										</span>
									</div>
									<div
										class="px-3 py-1 rounded-full bg-accent/10 border border-accent/20 text-[10px] font-black uppercase tracking-tighter text-accent"
									>
										{visit.modo_ingreso || (visit.placa ? 'Vehículo' : 'Caminando')}
									</div>
								</div>

								<div class="grid grid-cols-2 lg:grid-cols-3 gap-4 text-sm">
									{#if visit.empresa_nombre}
										<div class="flex items-center gap-2 text-secondary/80">
											<Building2 size={14} class="text-accent/70" />
											<span class="truncate">{visit.empresa_nombre}</span>
										</div>
									{/if}
									<div class="flex items-center gap-2 text-secondary/80">
										<User size={14} class="text-accent/70" />
										<span class="truncate">{visit.anfitrion}</span>
									</div>
									<div class="flex items-center gap-2 text-secondary/80 col-span-2 lg:col-span-1">
										<MapPin size={14} class="text-accent/70" />
										<span class="truncate">{visit.area_visitada}</span>
									</div>
								</div>

								{#if visit.motivo}
									<div class="pt-2 border-t border-surface/20">
										<p class="text-xs italic text-secondary/60 line-clamp-1">"{visit.motivo}"</p>
									</div>
								{/if}
							</div>

							<!-- Actions -->
							<div class="flex flex-col gap-2">
								<button
									onclick={() => onIngreso(visit)}
									class="p-3 bg-accent hover:bg-accent-light text-white rounded-xl transition-all shadow-lg shadow-accent/20 hover:-translate-y-0.5 active:translate-y-0 flex items-center justify-center"
									title="Confirmar Ingreso"
								>
									<LogIn size={20} strokeWidth={2.5} />
								</button>
								<button
									onclick={() => onCancel(visit.id)}
									class="p-3 bg-white/5 hover:bg-red-500/10 text-secondary hover:text-red-400 rounded-xl transition-all flex items-center justify-center border border-transparent hover:border-red-500/20"
									title="Cancelar"
								>
									<Trash2 size={20} />
								</button>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 10px;
	}
</style>
