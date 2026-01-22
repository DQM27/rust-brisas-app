<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { CreatePreRegistroInput } from '$lib/types/ingreso-nuevos';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import { slide, fly } from 'svelte/transition';
	import { X, CalendarDays, Save, PersonStanding, Car, Loader2, Check } from 'lucide-svelte';

	export let isOpen = false;
	const dispatch = createEventDispatcher();

	let loading = false;
	let error = '';

	// Form data
	let cedula = '';
	let nombre = '';
	let apellido = '';
	let empresaNombre = '';
	let fechaEsperadaDisplay = formatDateForDisplay(new Date().toISOString().split('T')[0]);
	let anfitrion = '';
	let areaVisitada = '';
	let motivo = '';
	let modoIngreso = 'caminando';
	let showModoIngresoDropdown = false;
	let observaciones = '';

	// Clases estándar según ui-patterns.md
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const selectClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white focus:outline-none disabled:opacity-50 transition-all cursor-pointer appearance-none';
	const labelClass = 'block text-xs font-medium text-secondary mb-1 uppercase tracking-wide';

	// Helpers para formato de fecha (DD/MM/YYYY ↔ YYYY-MM-DD)
	function formatDateForDisplay(isoDate: string): string {
		if (!isoDate) return '';
		const [year, month, day] = isoDate.split('T')[0].split('-');
		return `${day}/${month}/${year}`;
	}

	function formatDateForBackend(displayDate: string): string {
		if (!displayDate || displayDate.length !== 10) return '';
		const [day, month, year] = displayDate.split('/');
		return `${year}-${month}-${day}`;
	}

	async function handleSubmit() {
		if (!cedula || !nombre || !apellido || !anfitrion || !areaVisitada || !motivo) {
			error = 'Por favor complete los campos obligatorios (*)';
			return;
		}

		loading = true;
		error = '';

		try {
			const input: CreatePreRegistroInput = {
				cedula,
				nombre,
				apellido,
				empresaNombre,
				fechaEsperada: formatDateForBackend(fechaEsperadaDisplay),
				anfitrion,
				areaVisitada,
				motivo,
				modoIngreso,
				observaciones
			};

			await preRegistroVisitaService.create(input);
			dispatch('success');
			close();
		} catch (e: any) {
			error = e.message || 'Error al crear pre-registro';
			console.error(e);
		} finally {
			loading = false;
		}
	}

	function close() {
		resetForm();
		dispatch('close');
	}

	function resetForm() {
		cedula = '';
		nombre = '';
		apellido = '';
		empresaNombre = '';
		error = '';
	}

	function handleDateInput(e: Event) {
		const input = e.target as HTMLInputElement;
		let value = input.value.replace(/[^\d/]/g, ''); // Solo números y /
		if (value.length >= 3 && value[2] !== '/') {
			value = value.slice(0, 2) + '/' + value.slice(2);
		}
		if (value.length >= 6 && value[5] !== '/') {
			value = value.slice(0, 5) + '/' + value.slice(5);
		}
		value = value.slice(0, 10);
		fechaEsperadaDisplay = value;
		input.value = value;
	}
</script>

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
		transition:slide|local
	>
		<!-- Modal Container -->
		<div
			class="bg-surface-2 w-full max-w-[700px] rounded-xl shadow-2xl border border-surface overflow-hidden flex flex-col max-h-[95vh]"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header Estándar -->
			<div
				class="flex-none flex items-center justify-between px-5 py-4 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-xl font-semibold text-primary flex items-center gap-2">
					<CalendarDays size={20} class="text-accent" />
					Nuevo Pre-Registro de Visita
				</h2>
				<button
					onclick={close}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Body con Card de Inputs -->
			<div class="flex-1 p-6 space-y-4 overflow-y-auto">
				{#if error}
					<div class="p-3 bg-error/10 border border-error/20 text-error rounded-lg text-sm">
						{error}
					</div>
				{/if}

				<!-- Card de Inputs -->
				<div class="bg-surface-1 rounded-lg border border-surface p-6">
					<form
						onsubmit={(e) => {
							e.preventDefault();
							handleSubmit();
						}}
						class="space-y-5"
					>
						<!-- Fila 1: Datos Personales -->
						<div class="grid grid-cols-1 lg:grid-cols-3 gap-5">
							<div>
								<label class={labelClass}>Cédula <span class="text-red-500">*</span></label>
								<input
									type="text"
									bind:value={cedula}
									class={inputClass}
									placeholder="ID Visitante"
									autofocus
								/>
							</div>
							<div>
								<label class={labelClass}>Nombre <span class="text-red-500">*</span></label>
								<input type="text" bind:value={nombre} class={inputClass} placeholder="Nombre" />
							</div>
							<div>
								<label class={labelClass}>Apellido <span class="text-red-500">*</span></label>
								<input
									type="text"
									bind:value={apellido}
									class={inputClass}
									placeholder="Apellido"
								/>
							</div>
						</div>

						<!-- Fila 2: Empresa y Fecha -->
						<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
							<div>
								<label class={labelClass}>Empresa (Opcional)</label>
								<input
									type="text"
									bind:value={empresaNombre}
									class={inputClass}
									placeholder="Empresa visitante"
								/>
							</div>
							<div>
								<label class={labelClass}>Fecha Esperada <span class="text-red-500">*</span></label>
								<input
									type="text"
									value={fechaEsperadaDisplay}
									oninput={handleDateInput}
									class={inputClass}
									placeholder="DD/MM/YYYY"
									maxlength="10"
								/>
							</div>
						</div>

						<!-- Separador con Título de Sección -->
						<div class="border-t border-surface pt-5">
							<h3 class="text-sm font-semibold text-primary mb-4">Datos de la Visita</h3>
						</div>

						<!-- Fila 3: Detalles Visita -->
						<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
							<div>
								<label class={labelClass}>Anfitrión <span class="text-red-500">*</span></label>
								<input
									type="text"
									bind:value={anfitrion}
									class={inputClass}
									placeholder="¿A quién visita?"
								/>
							</div>
							<div>
								<label class={labelClass}>Área Visitada <span class="text-red-500">*</span></label>
								<input
									type="text"
									bind:value={areaVisitada}
									class={inputClass}
									placeholder="Ej. Administración, Planta..."
								/>
							</div>
						</div>

						<!-- Fila 4: Motivo y Modo -->
						<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
							<div>
								<label class={labelClass}>Motivo <span class="text-red-500">*</span></label>
								<input
									type="text"
									bind:value={motivo}
									class={inputClass}
									placeholder="Ej. Reunión, Entrega..."
								/>
							</div>
							<div>
								<label class={labelClass}>Modo Ingreso</label>
								<div class="relative">
									<!-- Trigger Button -->
									<button
										type="button"
										onclick={() => (showModoIngresoDropdown = !showModoIngresoDropdown)}
										class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
										class:!border-blue-500={showModoIngresoDropdown}
									>
										<span class="flex items-center gap-2">
											{#if modoIngreso === 'caminando'}
												<PersonStanding size={16} class="text-secondary" />
												<span>Caminando</span>
											{:else}
												<Car size={16} class="text-secondary" />
												<span>Vehículo</span>
											{/if}
										</span>
										<!-- Icono dinámico -->
										{#if modoIngreso === 'caminando'}
											<PersonStanding size={16} class="text-secondary" />
										{:else}
											<Car size={16} class="text-secondary" />
										{/if}
									</button>

									{#if showModoIngresoDropdown}
										<!-- Backdrop -->
										<div
											class="fixed inset-0 z-40"
											onclick={() => (showModoIngresoDropdown = false)}
											role="presentation"
										></div>

										<!-- Menú Flotante -->
										<div
											class="absolute z-50 w-full bottom-[calc(100%+4px)] bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-bottom"
											transition:fly={{ y: 5, duration: 200 }}
										>
											<button
												type="button"
												onclick={() => {
													modoIngreso = 'caminando';
													showModoIngresoDropdown = false;
												}}
												class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
											>
												<span class="flex items-center gap-2">
													<PersonStanding size={16} />
													Caminando
												</span>
												{#if modoIngreso === 'caminando'}
													<Check size={14} class="text-white" />
												{/if}
											</button>
											<button
												type="button"
												onclick={() => {
													modoIngreso = 'vehiculo';
													showModoIngresoDropdown = false;
												}}
												class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
											>
												<span class="flex items-center gap-2">
													<Car size={16} />
													Vehículo
												</span>
												{#if modoIngreso === 'vehiculo'}
													<Check size={14} class="text-white" />
												{/if}
											</button>
										</div>
									{/if}
								</div>
							</div>
						</div>
					</form>
				</div>
			</div>

			<!-- Footer Estándar con Botones Reactivos -->
			<div
				class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<!-- Cancelar -->
				<button
					onclick={close}
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
				>
					Cancelar
				</button>

				<!-- Guardar -->
				<button
					onclick={handleSubmit}
					disabled={loading}
					class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
				>
					{#if loading}
						<Loader2 size={16} class="animate-spin" />
						<span>Guardando...</span>
					{:else}
						<Save size={16} />
						<span>Guardar Pre-Registro</span>
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Autofill Fix (Evita fondo blanco de Chrome) */
	input:-webkit-autofill,
	textarea:-webkit-autofill {
		-webkit-text-fill-color: white !important;
		-webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
		transition: background-color 5000s ease-in-out 0s;
	}

	/* Focus Override Global */
	input:focus,
	textarea:focus,
	select:focus {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
		outline: none !important;
	}

	/* Fix para el select en dark mode */
	select {
		color-scheme: dark;
	}
</style>
