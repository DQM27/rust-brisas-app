<!-- src/lib/components/shared/PersonaQuickCardModal.svelte -->
<script lang="ts">
	import {
		X,
		IdCard,
		Building2,
		AlertTriangle,
		ShieldCheck,
		Clock,
		Calendar,
		History,
		ShieldX,
		User,
		Users,
		HardHat,
		Truck,
		UserCircle,
		FileText
	} from 'lucide-svelte';
	import { personaQuickView } from '$lib/stores/ui';
	import { openTab } from '$lib/stores/tabs';
	import { toast } from 'svelte-5-french-toast';
	import { fade, scale, slide } from 'svelte/transition';
	import { currentUser } from '$lib/stores/auth';
	import { validarIngreso } from '$lib/logic/ingreso/ingresoService';
	import type { ValidacionIngresoResult } from '$lib/logic/ingreso/types';

	// State
	let show = $derived($personaQuickView !== null);
	let data = $derived($personaQuickView);
	let loading = $state(false);
	let validationResult = $state<ValidacionIngresoResult | null>(null);

	// Fetch data when modal opens
	$effect(() => {
		if (data) {
			loadData(data.id, data.type);
		} else {
			validationResult = null;
		}
	});

	async function loadData(id: string, type: string) {
		loading = true;
		try {
			validationResult = await validarIngreso(type as any, id);
		} catch (e) {
			console.error('Error al cargar datos de persona:', e);
			toast.error('Error al cargar información detallada');
		} finally {
			loading = false;
		}
	}

	function handleClose() {
		personaQuickView.set(null);
	}

	function handleOpenFullRecord() {
		if (!data || !validationResult?.persona) return;

		const type = data.type;
		const id = data.id;

		let componentKey = 'contratista-list';
		let title = 'Lista Contratistas';

		if (type === 'proveedor') {
			componentKey = 'proveedor-list';
			title = 'Lista Proveedores';
		} else if (type === 'visitante' || type === 'visita') {
			componentKey = 'visitante-list';
			title = 'Lista Visitantes';
		}

		openTab({
			componentKey: componentKey as any,
			title: title,
			id: componentKey,
			focusOnOpen: true,
			data: { search: validationResult.persona.cedula || id }
		});

		handleClose();
	}

	// UI Patterns Classes
	const labelClass = 'block text-[10px] font-bold text-secondary uppercase tracking-widest mb-1';
	const containerClass = 'bg-surface-1 rounded-lg border border-surface p-4';

	function formatDays(days: number) {
		if (days < 0) return `Venció hace ${Math.abs(days)} días`;
		if (days === 0) return `Vence hoy`;
		return `Vence en ${days} días`;
	}

	function formatDate(dateStr: string | undefined) {
		if (!dateStr || dateStr === 'N/A') return 'N/A';
		try {
			// Limpiar decoradores de SurrealDB si existen: d'2024-01-01...' -> 2024-01-01...
			const cleanStr = dateStr.replace(/^d'/, '').replace(/'$/, '');
			const d = new Date(cleanStr);
			if (isNaN(d.getTime())) return 'N/A';
			return d.toLocaleString('es-PA', {
				day: '2-digit',
				month: 'long',
				year: 'numeric',
				hour: '2-digit',
				minute: '2-digit',
				hour12: false
			});
		} catch (e) {
			return 'N/A';
		}
	}
</script>

<svelte:window
	onkeydown={(e) => {
		if (show && e.key === 'Escape') handleClose();
	}}
/>

{#if show}
	<!-- Backdrop con Blur (UI-Pattern 1.80) -->
	<div
		class="fixed inset-0 z-[110] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 200 }}
		onclick={handleClose}
		role="button"
		tabindex="-1"
		onkeydown={(e) => e.key === 'Escape' && handleClose()}
	>
		<!-- Contenedor Principal (UI-Pattern 2.76) -->
		<div
			class="relative w-full max-w-[550px] bg-surface-2 border border-surface rounded-xl shadow-2xl overflow-hidden flex flex-col max-h-[95vh]"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
			role="presentation"
		>
			<!-- Header Estándar (UI-Pattern 2.82) -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 bg-surface-2 border-b border-surface"
			>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 rounded-full bg-surface-1 border border-surface flex items-center justify-center text-secondary shadow-sm"
					>
						{#if data?.type === 'contratista'}
							<HardHat size={20} />
						{:else if data?.type === 'proveedor'}
							<Truck size={20} />
						{:else if data?.type === 'visita' || data?.type === 'visitante'}
							<UserCircle size={20} />
						{:else}
							<User size={20} />
						{/if}
					</div>
					<div>
						<h2 class="text-lg font-bold text-primary leading-tight uppercase tracking-tight">
							{loading ? 'Cargando...' : validationResult?.persona?.nombreCompleto || 'Persona'}
						</h2>
						<span
							class="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-wider {validationResult?.tieneIngresoAbierto
								? 'text-blue-400'
								: validationResult?.puedeIngresar
									? 'text-green-400'
									: 'text-error'}"
						>
							<span
								class="w-1.5 h-1.5 rounded-full {validationResult?.tieneIngresoAbierto
									? 'bg-blue-400'
									: validationResult?.puedeIngresar
										? 'bg-green-400'
										: 'bg-error'} {loading ? 'animate-pulse' : ''}"
							></span>
							{#if loading}
								Validando...
							{:else if validationResult}
								<span
									class="text-[10px] font-bold uppercase tracking-widest {validationResult?.tieneIngresoAbierto
										? 'text-blue-400'
										: validationResult?.puedeIngresar
											? 'text-emerald-400'
											: 'text-rose-400'}"
								>
									{#if validationResult?.tieneIngresoAbierto}
										Ingreso Activo
									{:else}
										{validationResult?.puedeIngresar ? 'Acceso Autorizado' : 'Acceso Denegado'}
									{/if}
								</span>
							{:else}
								{data?.type?.toUpperCase() || 'Registro'}
							{/if}
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
			<div class="flex-1 p-6 space-y-6 overflow-y-auto scrollbar-none">
				{#if loading}
					<div class="py-12 flex flex-col items-center justify-center gap-4">
						<div
							class="w-8 h-8 border-2 border-primary/20 border-t-primary rounded-full animate-spin"
						></div>
						<p class="text-xs text-secondary font-medium uppercase tracking-widest">
							Obteniendo perfil de seguridad...
						</p>
					</div>
				{:else if validationResult}
					<!-- Card de Estado -->
					{#if validationResult.tieneIngresoAbierto}
						<div
							class="bg-blue-500/10 border border-blue-500/20 p-4 rounded-lg flex items-start gap-4"
							transition:slide
						>
							<div
								class="w-8 h-8 rounded-full bg-blue-500/20 flex items-center justify-center text-blue-400"
							>
								<Clock size={18} />
							</div>
							<div>
								<p class="text-[10px] font-bold text-blue-400 uppercase tracking-widest mb-0.5">
									Ingreso Activo
								</p>
								<p class="text-sm text-blue-400/90 font-medium leading-relaxed font-mono">
									{formatDate(validationResult.ingresoAbierto?.fechaHoraIngreso)}
								</p>
							</div>
						</div>
					{:else if !validationResult.puedeIngresar}
						<div
							class="bg-error/10 border border-error/20 p-4 rounded-lg flex items-start gap-4"
							transition:slide
						>
							<div
								class="w-8 h-8 rounded-full bg-error/20 flex items-center justify-center text-error"
							>
								<ShieldX size={18} />
							</div>
							<div>
								<p class="text-[10px] font-bold text-error uppercase tracking-widest mb-0.5">
									Motivo de Restricción
								</p>
								<p class="text-sm text-error/90 font-medium leading-relaxed">
									{validationResult.motivoRechazo || 'No cumple con los requisitos de acceso.'}
								</p>
							</div>
						</div>
					{/if}

					<!-- Grid de Información (UI-Pattern 3.107) -->
					<div class={containerClass}>
						<div class="grid grid-cols-2 gap-6">
							<div>
								<span class={labelClass}>Identificación</span>
								<p
									class="text-sm font-mono text-white bg-black/20 px-2 py-0.5 rounded border border-white/5 inline-block"
								>
									{validationResult.persona?.cedula ||
										validationResult.contratista?.cedula ||
										'N/A'}
								</p>
							</div>

							<div>
								<span class={labelClass}>Empresa / Origen</span>
								<p class="text-sm font-medium text-primary truncate">
									{validationResult.persona?.empresa ||
										validationResult.contratista?.empresaNombre ||
										validationResult.proveedor?.empresaNombre ||
										validationResult.visitante?.empresaNombre ||
										'Independiente'}
								</p>
							</div>

							<div>
								<span class={labelClass}>Estado</span>
								<div class="flex items-center gap-1.5">
									<div
										class="w-1.5 h-1.5 rounded-full {validationResult.tieneIngresoAbierto
											? 'bg-blue-400'
											: validationResult.puedeIngresar
												? 'bg-green-400'
												: 'bg-error'}"
									></div>
									<p
										class="text-sm font-medium {validationResult.tieneIngresoAbierto
											? 'text-blue-400'
											: validationResult.puedeIngresar
												? 'text-green-400'
												: 'text-error'}"
									>
										{#if validationResult.tieneIngresoAbierto}
											En Planta
										{:else}
											{validationResult.puedeIngresar ? 'Activo' : 'Restringido'}
										{/if}
									</p>
								</div>
							</div>

							<div>
								<span class={labelClass}>Tipo Registro</span>
								<p class="text-sm text-primary capitalize font-medium">
									{data?.type || 'Persona'}
								</p>
							</div>
						</div>
					</div>

					<!-- Alertas de Seguridad -->
					{#if (validationResult.alertas && validationResult.alertas.length > 0) || validationResult.contratista?.praindVencido}
						<div class="space-y-3">
							<h4 class={labelClass}>Observaciones de Seguridad</h4>

							<div class="space-y-2">
								<!-- Gafetes pendientes -->
								{#if validationResult.alertas && validationResult.alertas.length > 0}
									<div
										class="flex items-center gap-3 p-3 bg-warning/10 border border-warning/20 rounded-lg text-warning text-xs"
										transition:slide
									>
										<AlertTriangle size={16} class="shrink-0" />
										<span
											>Debe <strong>{validationResult.alertas.length}</strong> gafete(s) en registros
											anteriores.</span
										>
									</div>
								{/if}

								<!-- PRAIND Status -->
								{#if validationResult.contratista?.praindVencido !== undefined}
									<div
										class="flex items-center gap-3 p-3 {validationResult.contratista.praindVencido
											? 'bg-error/5 border border-error/10 text-error'
											: 'bg-blue-500/5 border border-blue-500/10 text-blue-400'} rounded-lg text-xs"
										transition:slide
									>
										<Calendar size={16} class="shrink-0" />
										<div class="flex-1 flex justify-between items-center">
											<span class="font-bold uppercase tracking-tighter"
												>PRAIND {validationResult.contratista.praindVencido
													? 'Vencido'
													: 'Vigente'}</span
											>
											<span class="font-bold font-mono"
												>{formatDays(validationResult.contratista.diasHastaVencimiento || 0)}</span
											>
										</div>
									</div>
								{/if}
							</div>
						</div>
					{/if}

					<!-- Último Movimiento / Registro (UI-Pattern 4.149) -->
					<div class="space-y-4">
						<h4 class={labelClass}>Último Movimiento</h4>
						<div
							class="flex items-center justify-between p-3 rounded-lg border border-surface bg-surface-1/50"
						>
							<div class="flex items-center gap-3">
								<History size={18} class="text-secondary" />
								<span class="text-xs text-secondary font-medium uppercase tracking-widest"
									>Último Ingreso</span
								>
							</div>
							<span class="text-xs font-mono text-primary font-bold">
								{formatDate(
									validationResult.ultimoIngreso?.fechaHoraIngreso ||
										validationResult.contratista?.createdAt ||
										validationResult.proveedor?.createdAt ||
										validationResult.visitante?.createdAt
								)}
							</span>
						</div>
					</div>
				{:else}
					<div class="py-12 text-center">
						<p class="text-sm text-secondary italic">
							No se pudo cargar la información de seguridad.
						</p>
					</div>
				{/if}
			</div>

			<!-- Footer Estándar (UI-Pattern 9.240) -->
			<div
				class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<button
					class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
					onclick={handleClose}
				>
					Cerrar
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Ocultar scrollbar pero permitir scroll */
	.scrollbar-none::-webkit-scrollbar {
		display: none;
	}
	.scrollbar-none {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}

	div {
		scrollbar-width: thin;
		scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
	}
</style>
