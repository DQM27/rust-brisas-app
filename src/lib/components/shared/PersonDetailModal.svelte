<!-- src/lib/components/shared/PersonDetailModal.svelte -->
<script lang="ts">
	import {
		X,
		HardHat,
		Truck,
		UserCircle,
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
				minute: '2-digit',
				hour12: false
			});
		} catch (e) {
			return dateStr;
		}
	}

	// UI Patterns Classes
	const labelClass = 'block text-[10px] font-bold text-secondary uppercase tracking-widest mb-1';
	const containerClass = 'bg-surface-1 rounded-lg border border-surface p-4';
</script>

{#if show && person}
	<!-- Backdrop con Blur (UI-Pattern 1.80) -->
	<div
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 200 }}
		onclick={handleClose}
		onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && handleClose()}
		role="button"
		tabindex="0"
	>
		<!-- Contenedor Principal (UI-Pattern 2.76) -->
		<div
			class="relative w-full max-w-[550px] bg-surface-2 border border-surface rounded-xl shadow-2xl overflow-hidden flex flex-col max-h-[95vh]"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="presentation"
		>
			<!-- Header Estándar (UI-Pattern 2.82) -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 bg-surface-2 border-b border-surface"
			>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 rounded-full bg-surface-1 border border-surface flex items-center justify-center text-secondary"
					>
						{#if person.tipoIngreso === 'contratista'}
							<HardHat size={20} />
						{:else if person.tipoIngreso === 'proveedor'}
							<Truck size={20} />
						{:else}
							<UserCircle size={20} />
						{/if}
					</div>
					<div>
						<h2 class="text-lg font-bold text-primary leading-tight uppercase">
							{person.nombreCompleto}
						</h2>
						<span
							class="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-wider {person.estaAdentro
								? 'text-green-400'
								: 'text-gray-500'}"
						>
							<span
								class="w-1.5 h-1.5 rounded-full {person.estaAdentro
									? 'bg-green-400'
									: 'bg-gray-400'}"
							></span>
							{person.estaAdentro ? 'Dentro del Recinto' : 'Fuera del Recinto'}
						</span>
					</div>
				</div>
				<button
					onclick={handleClose}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Content (UI-Pattern 3.106) -->
			<div class="flex-1 p-6 space-y-6 overflow-y-auto">
				<!-- Grid de Información en Card (UI-Pattern 3.107) -->
				<div class={containerClass}>
					<div class="grid grid-cols-2 gap-6">
						<div>
							<span class={labelClass}>Identificación</span>
							<p
								class="text-sm font-mono text-white bg-black/20 px-2 py-1 rounded border border-white/5 inline-block"
							>
								{person.cedula}
							</p>
						</div>

						<div>
							<span class={labelClass}>Empresa</span>
							<p class="text-sm font-medium text-primary">
								{person.empresaNombre || 'N/A'}
							</p>
						</div>

						<div>
							<span class={labelClass}>Autorización</span>
							<p class="text-sm text-blue-400 font-medium">
								{person.tipoAutorizacionDisplay}
							</p>
						</div>

						<div>
							<span class={labelClass}>Modo Ingreso</span>
							<p class="text-sm text-primary capitalize font-medium">
								{person.modoIngresoDisplay}
							</p>
						</div>
					</div>
				</div>

				<!-- Historial de Último Movimiento (UI-Pattern 3.107) -->
				<div class="space-y-4">
					<h4 class={labelClass}>Registro de Actividad</h4>

					<div class="grid grid-cols-1 gap-3">
						<!-- Entrada -->
						<div class="flex items-start gap-4 p-3 rounded-lg border border-surface transition-all">
							<div class="mt-1 text-secondary">
								<Calendar size={18} />
							</div>
							<div class="flex-1">
								<div class="flex justify-between items-start">
									<p class="text-[10px] text-gray-500 font-bold uppercase tracking-widest">
										Entrada
									</p>
									<p class="text-xs text-gray-400 font-mono">
										{formatDate(person.fechaHoraIngreso)}
									</p>
								</div>
								<p class="text-sm text-primary mt-1 italic">
									Registrado por: <span class="font-semibold text-gray-300"
										>{person.usuarioIngresoNombre}</span
									>
								</p>
							</div>
						</div>

						<!-- Salida (si aplica) -->
						{#if person.fechaHoraSalida}
							<div
								class="flex items-start gap-4 p-3 rounded-lg border border-surface transition-all"
							>
								<div class="mt-1 text-secondary">
									<LogOut size={18} />
								</div>
								<div class="flex-1">
									<div class="flex justify-between items-start">
										<p class="text-[10px] text-gray-500 font-bold uppercase tracking-widest">
											Salida
										</p>
										<p class="text-xs text-gray-400 font-mono">
											{formatDate(person.fechaHoraSalida)}
										</p>
									</div>
									<p class="text-sm text-primary mt-1 italic">
										Registrado por: <span class="font-semibold text-gray-300"
											>{person.usuarioSalidaNombre || 'Sistema'}</span
										>
									</p>
								</div>
							</div>
						{/if}
					</div>

					<!-- Tiempo de Permanencia (UI-Pattern 3.204) -->
					{#if person.tiempoPermanenciaTexto}
						<div class="flex items-center gap-3 bg-white/5 p-3 rounded-lg border border-white/5">
							<Clock size={16} class="text-secondary" />
							<div class="flex-1 flex justify-between items-center">
								<span class="text-[10px] text-gray-500 font-bold uppercase tracking-widest"
									>Tiempo Total</span
								>
								<span
									class="text-xs text-primary font-mono font-bold bg-white/5 px-2 py-0.5 rounded"
									>{person.tiempoPermanenciaTexto}</span
								>
							</div>
						</div>
					{/if}
				</div>

				<!-- Observaciones (UI-Pattern 4.315) -->
				{#if person.observaciones}
					<div class="bg-white/5 border border-white/10 p-4 rounded-lg flex gap-3">
						<Briefcase size={18} class="text-secondary shrink-0" />
						<div>
							<span class="text-[10px] text-gray-500 font-bold uppercase tracking-widest block mb-1"
								>Observaciones de Registro</span
							>
							<p class="text-xs text-secondary leading-relaxed italic">
								"{person.observaciones}"
							</p>
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer Estándar (UI-Pattern 9.240) -->
			<div
				class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<button
					class="px-5 py-2 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
					onclick={handleClose}
				>
					Cerrar Detalle
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Fix para scroll suave */
	div {
		scrollbar-width: thin;
		scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
	}
</style>
