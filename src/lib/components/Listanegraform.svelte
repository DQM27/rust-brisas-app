<script lang="ts">
	import { onMount } from "svelte";
	import { fly, fade } from "svelte/transition";
	import { submitFetchActiveContratistas } from "$lib/logic/contratista/submitFetchContratistas";
	import { listaNegra } from "$lib/api/listaNegra";
	import { currentUser } from "$lib/stores/auth";
	import type { BlockCheckResponse } from "$lib/types/listaNegra";

	export let loading = false;
	export let onSubmit: (data: any) => void;
	export let onUnblock: ((data: { id: string; motivoDesbloqueo: string; observaciones?: string }) => void) | undefined = undefined;

	// --- ESTADO DEL FORMULARIO ---
	let modoRegistro: "existente" | "manual" = "existente";
	
	// Modo existente
	let contratistaId = "";
	let searchTerm = "";
	let showDropdown = false;
	let selectedContratista: { id: string; nombreCompleto: string; cedula: string; empresaNombre?: string } | null = null;
	let highlightedIndex = -1;
	
	// Verificación de bloqueo
	let checkingBlock = false;
	let blockInfo: BlockCheckResponse | null = null;
	let bloqueadoId: string | null = null;
	
	// Modo manual
	let cedula = "";
	let nombre = "";
	let apellido = "";
	
	// Datos del bloqueo/desbloqueo (Misma variable, label condicional)
	let motivoBloqueo = "";
	let observaciones = "";

	// Modal de confirmación
	let showConfirmModal = false;

	// --- ESTADOS UI ---
	let contratistas: { id: string; nombreCompleto: string; cedula: string; empresaNombre?: string }[] = [];
	let loadingContratistas = false;

	// Filtrado de contratistas
	$: filteredContratistas = searchTerm.trim() 
		? contratistas.filter(c => 
			c.nombreCompleto.toLowerCase().includes(searchTerm.toLowerCase()) ||
			c.cedula.includes(searchTerm)
		).slice(0, 5)
		: [];

	// --- CARGA INICIAL ---
	onMount(async () => {
		loadingContratistas = true;
		const resultado = await submitFetchActiveContratistas();
		if (resultado.ok) {
			contratistas = resultado.contratistas.map(c => ({
				id: c.id,
				nombreCompleto: `${c.nombre} ${c.apellido}`,
				cedula: c.cedula,
				empresaNombre: c.empresaNombre
			}));
		}
		loadingContratistas = false;
	});

	export function reset() {
		modoRegistro = "existente";
		contratistaId = "";
		searchTerm = "";
		selectedContratista = null;
		showDropdown = false;
		blockInfo = null;
		bloqueadoId = null;
		checkingBlock = false;
		highlightedIndex = -1;
		cedula = "";
		nombre = "";
		apellido = "";
		motivoBloqueo = ""; // Limpiar campos de acción
		observaciones = ""; // Limpiar campos de acción
		showConfirmModal = false;
	}

	async function selectContratista(contratista: typeof contratistas[0]) {
		selectedContratista = contratista;
		contratistaId = contratista.id;
		searchTerm = contratista.nombreCompleto;
		showDropdown = false;
		
		// Resetear campos de acción al seleccionar uno nuevo
		motivoBloqueo = "";
		observaciones = "";
		
		// Verificar si está bloqueado
		await checkIfBlocked(contratista.cedula);
	}

	async function checkIfBlocked(cedulaToCheck: string) {
		checkingBlock = true;
		blockInfo = null;
		bloqueadoId = null;
		
		try {
			const check = await listaNegra.checkIsBlocked(cedulaToCheck);
			blockInfo = check;
			
			if (check.isBlocked) {
				// Obtener el ID del bloqueo
				const bloqueado = await listaNegra.getByCedula(cedulaToCheck);
				if (bloqueado) {
					bloqueadoId = bloqueado.id;
				}
			}
		} catch (error) {
			console.error("Error al verificar bloqueo:", error);
		}
		
		checkingBlock = false;
	}

	function clearSelection() {
		selectedContratista = null;
		contratistaId = "";
		searchTerm = "";
		blockInfo = null;
		bloqueadoId = null;
		motivoBloqueo = ""; // Limpiar campos de acción al deseleccionar
		observaciones = ""; // Limpiar campos de acción al deseleccionar
	}

function handleUnblock() {
	showConfirmModal = true;
}

function cancelUnblock() {
	showConfirmModal = false;
}

function confirmUnblock() {
	if (bloqueadoId && onUnblock) {
		// Antes: onUnblock({ id: bloqueadoId, ... });
		
		// Ahora, debe asegurarse de pasar el motivo y las observaciones al manejador:
		onUnblock({
			id: bloqueadoId,
			motivoDesbloqueo: motivoBloqueo, // Asumiendo que 'motivoBloqueo' contiene el motivo de desbloqueo
			observaciones: observaciones.trim() || undefined
		});
	}
	showConfirmModal = false;
}

	function handleKeyDown(event: KeyboardEvent) {
		if (!showDropdown || filteredContratistas.length === 0) return;

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				highlightedIndex = Math.min(highlightedIndex + 1, filteredContratistas.length - 1);
				break;
			case 'ArrowUp':
				event.preventDefault();
				highlightedIndex = Math.max(highlightedIndex - 1, -1);
				break;
			case 'Enter':
			case 'Tab':
				event.preventDefault();
				if (highlightedIndex >= 0 && highlightedIndex < filteredContratistas.length) {
					selectContratista(filteredContratistas[highlightedIndex]);
				} else if (filteredContratistas.length > 0) {
					selectContratista(filteredContratistas[0]);
				}
				break;
			case 'Escape':
				event.preventDefault();
				showDropdown = false;
				highlightedIndex = -1;
				break;
		}
	}

	// Reset highlighted index cuando cambia el término de búsqueda
	$: if (searchTerm) {
		highlightedIndex = -1;
	}

	function handleSubmit(e: Event) {
		e.preventDefault();
		
		const usuario = $currentUser;
		if (!usuario) {
			console.error("No hay usuario autenticado");
			return;
		}

		const bloqueadoPor = `${usuario.nombre} ${usuario.apellido}`;
		
		const data: any = {
			motivoBloqueo,
			bloqueadoPor,
			observaciones: observaciones.trim() || undefined,
		};

		if (modoRegistro === "existente") {
			data.contratistaId = contratistaId;
		} else {
			data.cedula = cedula;
			data.nombre = nombre;
			data.apellido = apellido;
		}

		onSubmit(data);
	}

	// VALIDACIÓN DEL FORMULARIO
	$: isFormValid = 
		motivoBloqueo.trim() &&
		(
			(modoRegistro === "existente" && contratistaId.trim() && !checkingBlock) || // Para existente: Debe estar seleccionado y la verificación debe haber terminado
			(modoRegistro === "manual" && cedula.trim() && nombre.trim() && apellido.trim())
		);
</script>

<div class="flex min-h-full items-center justify-center p-6">
	
	<div class="relative z-10 w-full max-w-2xl rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10">
		
		<div class="border-b border-white/10 px-8 py-5">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-red-500/10">
					<svg class="h-6 w-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
				</div>
				<div>
					<h2 class="text-xl font-semibold text-gray-100">Agregar a Lista Negra</h2>
					<p class="mt-1 text-sm text-gray-400">Bloquear acceso a una persona del sistema.</p>
				</div>
			</div>
		</div>

		<form on:submit={handleSubmit} class="p-8">
			
			<div class="mb-6 space-y-3">
				<label class="text-sm font-medium text-gray-300">Modo de Registro</label>
				<div class="grid grid-cols-2 gap-3">
					<button
						type="button"
						on:click={() => { modoRegistro = "existente"; clearSelection(); }}
						disabled={loading}
						class="flex items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all {modoRegistro === 'existente' ? 'border-red-500 bg-red-500/10 text-red-400' : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
					>
						<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
						</svg>
						<span>Contratista Existente</span>
					</button>
					<button
						type="button"
						on:click={() => { modoRegistro = "manual"; clearSelection(); }}
						disabled={loading}
						class="flex items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all {modoRegistro === 'manual' ? 'border-red-500 bg-red-500/10 text-red-400' : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
					>
						<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
						</svg>
						<span>Registro Manual</span>
					</button>
				</div>
			</div>

			<div class="space-y-5">
				
				{#if modoRegistro === "existente"}
					<div class="space-y-5" in:fly={{ x: -20, duration: 300 }} out:fade={{ duration: 200 }}>
						<div class="space-y-1.5">
							<label for="searchContratista" class="text-sm font-medium text-gray-300">Buscar Contratista</label>
							<div class="relative">
								<input
									id="searchContratista"
									type="text"
									bind:value={searchTerm}
									on:focus={() => showDropdown = true}
									on:blur={() => setTimeout(() => showDropdown = false, 200)}
									on:keydown={handleKeyDown}
									disabled={loading || loadingContratistas}
									placeholder="Buscar por nombre o cédula..."
									class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 pr-10 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none disabled:opacity-50"
								/>
								
								<div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-gray-400">
									{#if loadingContratistas}
										<svg class="animate-spin h-4 w-4 text-red-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
											<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
											<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
										</svg>
									{:else}
										<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
										</svg>
									{/if}
								</div>

								{#if showDropdown && searchTerm.trim() && filteredContratistas.length > 0}
									<div class="absolute z-10 mt-1 w-full rounded-lg border border-white/10 bg-[#2d2d2d] shadow-xl max-h-60 overflow-y-auto">
										{#each filteredContratistas as contratista, index}
											<button
												type="button"
												on:mousedown|preventDefault={() => selectContratista(contratista)}
												on:mouseenter={() => highlightedIndex = index}
												class="w-full px-3 py-2.5 text-left border-b border-white/5 last:border-b-0 transition-colors {highlightedIndex === index ? 'bg-red-500/10' : 'hover:bg-[#3d3d3d]'}"
											>
												<div class="text-sm text-white font-medium">{contratista.nombreCompleto}</div>
												<div class="text-xs text-gray-400">Cédula: {contratista.cedula}</div>
												{#if contratista.empresaNombre}
													<div class="text-xs text-gray-500">{contratista.empresaNombre}</div>
												{/if}
											</button>
										{/each}
									</div>
								{/if}

								{#if showDropdown && searchTerm.trim() && filteredContratistas.length === 0}
									<div class="absolute z-10 mt-1 w-full rounded-lg border border-white/10 bg-[#2d2d2d] shadow-xl p-3">
										<p class="text-sm text-gray-400 text-center">No se encontraron contratistas</p>
									</div>
								{/if}
							</div>
						</div>

						{#if selectedContratista}
							<div class="space-y-5" in:fly={{ y: -10, duration: 300 }}>
								
								{#if checkingBlock}
									<div class="rounded-lg bg-gray-500/10 border border-gray-500/20 p-4">
										<div class="flex items-center gap-2">
											<svg class="animate-spin h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
												<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
												<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
											</svg>
											<span class="text-sm text-gray-300">Verificando estado...</span>
										</div>
									</div>
								{:else if blockInfo?.isBlocked}
									<div class="rounded-lg bg-red-500/10 border border-red-500/20 p-4">
										<div class="flex items-start gap-3">
											<svg class="h-6 w-6 text-red-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
											</svg>
											<div class="flex-1">
												<h4 class="text-sm font-semibold text-red-200 mb-2">⚠️ Persona Ya Bloqueada</h4>
												<div class="space-y-1.5 text-sm">
													<div class="flex items-center gap-2">
														<span class="text-gray-400">Nombre:</span>
														<span class="text-white font-medium">{selectedContratista.nombreCompleto}</span>
													</div>
													<div class="flex items-center gap-2">
														<span class="text-gray-400">Cédula:</span>
														<span class="text-white">{selectedContratista.cedula}</span>
													</div>
													{#if selectedContratista.empresaNombre}
														<div class="flex items-center gap-2">
															<span class="text-gray-400">Empresa:</span>
															<span class="text-white">{selectedContratista.empresaNombre}</span>
														</div>
													{/if}
													<div class="pt-2 mt-2 border-t border-red-500/20">
														<div class="flex items-start gap-2">
															<span class="text-gray-400">Motivo:</span>
															<span class="text-red-200">{blockInfo.motivo}</span>
														</div>
														<div class="flex items-center gap-2 mt-1">
															<span class="text-gray-400">Bloqueado por:</span>
															<span class="text-red-200">{blockInfo.bloqueadoPor}</span>
														</div>
														{#if blockInfo.bloqueadoDesde}
															<div class="flex items-center gap-2 mt-1">
																<span class="text-gray-400">Desde:</span>
																<span class="text-red-200">{new Date(blockInfo.bloqueadoDesde).toLocaleDateString('es-ES')}</span>
															</div>
														{/if}
													</div>
												</div>
											</div>
											<button
												type="button"
												on:click={clearSelection}
												class="text-red-400 hover:text-red-300 transition-colors"
												title="Cambiar selección"
											>
												<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
												</svg>
											</button>
										</div>
									</div>

									<div class="space-y-1.5">
										<label for="motivoBloqueo" class="text-sm font-medium text-gray-300">Motivo del Desbloqueo <span class="text-red-500">*</span></label>
										<textarea id="motivoBloqueo" bind:value={motivoBloqueo} rows="3" disabled={loading} placeholder="Detalle la razón de la desactivación del bloqueo (ej: Cumplió sanción, revisión de caso, etc.)"
											class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
									</div>

									<div class="space-y-1.5">
										<label for="observaciones" class="text-sm font-medium text-gray-300">Observaciones de Desbloqueo (Opcional)</label>
										<textarea id="observaciones" bind:value={observaciones} rows="2" disabled={loading} placeholder="Notas internas sobre el desbloqueo."
											class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
									</div>
								
								{:else}
									<div class="rounded-lg bg-blue-500/10 border border-blue-500/20 p-4">
										<div class="flex items-start justify-between">
											<div class="flex-1">
												<div class="flex items-center gap-2 mb-2">
													<svg class="h-5 w-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
														<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
													</svg>
													<h4 class="text-sm font-semibold text-blue-200">Contratista Seleccionado (No Bloqueado)</h4>
												</div>
												<div class="space-y-1.5 text-sm">
													<div class="flex items-center gap-2">
														<span class="text-gray-400">Nombre:</span>
														<span class="text-white font-medium">{selectedContratista.nombreCompleto}</span>
													</div>
													<div class="flex items-center gap-2">
														<span class="text-gray-400">Cédula:</span>
														<span class="text-white">{selectedContratista.cedula}</span>
													</div>
													{#if selectedContratista.empresaNombre}
														<div class="flex items-center gap-2">
															<span class="text-gray-400">Empresa:</span>
															<span class="text-white">{selectedContratista.empresaNombre}</span>
														</div>
													{/if}
												</div>
											</div>
											<button
												type="button"
												on:click={clearSelection}
												class="text-blue-400 hover:text-blue-300 transition-colors"
												title="Cambiar selección"
											>
												<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
												</svg>
											</button>
										</div>
									</div>

									<div class="space-y-1.5">
										<label for="motivoBloqueo" class="text-sm font-medium text-gray-300">Motivo del Bloqueo <span class="text-red-500">*</span></label>
										<textarea id="motivoBloqueo" bind:value={motivoBloqueo} rows="3" disabled={loading} placeholder="Detalle la razón del bloqueo (ej: Agresión verbal a personal de seguridad, incumplimiento grave de normas, etc.)"
											class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
									</div>

									<div class="space-y-1.5">
										<label for="observaciones" class="text-sm font-medium text-gray-300">Observaciones Adicionales (Opcional)</label>
										<textarea id="observaciones" bind:value={observaciones} rows="2" disabled={loading} placeholder="Notas internas sobre el incidente."
											class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
									</div>
								{/if}

							</div>
						{/if}
					</div>
				{/if}

				{#if modoRegistro === "manual"}
					<div class="space-y-5" in:fly={{ x: -20, duration: 300 }} out:fade={{ duration: 200 }}>
						<div class="space-y-1.5">
							<label for="cedula" class="text-sm font-medium text-gray-300">Cédula</label>
							<input id="cedula" type="text" bind:value={cedula} placeholder="1-2345-6789" disabled={loading}
								class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all" />
						</div>

						<div class="grid grid-cols-2 gap-4">
							<div class="space-y-1.5">
								<label for="nombre" class="text-sm font-medium text-gray-300">Nombre</label>
								<input id="nombre" type="text" bind:value={nombre} placeholder="Juan" disabled={loading}
									class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none" />
							</div>
							<div class="space-y-1.5">
								<label for="apellido" class="text-sm font-medium text-gray-300">Apellido</label>
								<input id="apellido" type="text" bind:value={apellido} placeholder="Pérez" disabled={loading}
									class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none" />
							</div>
						</div>

						<div class="space-y-1.5">
							<label for="motivoBloqueo" class="text-sm font-medium text-gray-300">Motivo del Bloqueo <span class="text-red-500">*</span></label>
							<textarea id="motivoBloqueo" bind:value={motivoBloqueo} rows="3" disabled={loading} placeholder="Detalle la razón del bloqueo (ej: Agresión verbal a personal de seguridad, incumplimiento grave de normas, etc.)"
								class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
						</div>

						<div class="space-y-1.5">
							<label for="observaciones" class="text-sm font-medium text-gray-300">Observaciones Adicionales (Opcional)</label>
							<textarea id="observaciones" bind:value={observaciones} rows="2" disabled={loading} placeholder="Notas internas sobre el incidente."
								class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
						</div>
					</div>
				{/if}
			</div>

			<div class="mt-8 flex justify-end gap-3">
				{#if selectedContratista && blockInfo?.isBlocked && !checkingBlock}
					<button
						type="button"
						on:click={handleUnblock}
						disabled={loading || !motivoBloqueo.trim()}
						class="rounded-lg px-4 py-2 text-sm font-medium transition-colors bg-green-600 hover:bg-green-700 text-white disabled:bg-green-600/50"
					>
						Desbloquear
					</button>
				{/if}

				{#if (modoRegistro === "manual" && !loading) || (selectedContratista && !blockInfo?.isBlocked && !checkingBlock)}
					<button
						type="submit"
						disabled={loading || !isFormValid}
						class="rounded-lg px-4 py-2 text-sm font-medium transition-colors bg-red-600 hover:bg-red-700 text-white disabled:bg-red-600/50"
					>
						{#if loading}
							<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white inline-block" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Bloqueando...
						{:else}
							Bloquear Persona
						{/if}
					</button>
				{/if}
			</div>
		</form>
	</div>
</div>

{#if showConfirmModal}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-70 transition-opacity" transition:fade>
		<div class="bg-[#1e1e1e] rounded-xl shadow-2xl max-w-sm w-full p-6" transition:fly={{ y: 20, duration: 300 }}>
			<h3 class="text-lg font-semibold text-gray-100 mb-4">Confirmar Desbloqueo</h3>
			<p class="text-sm text-gray-300 mb-6">
				Está a punto de desactivar el bloqueo para **{selectedContratista?.nombreCompleto}**. 
				El motivo y las observaciones para esta acción serán registradas en el historial de la persona.
			</p>
			
			<div class="space-y-4">
				<div class="space-y-1.5">
					<label for="unblockMotivo" class="text-sm font-medium text-gray-300">Motivo del Desbloqueo <span class="text-red-500">*</span></label>
					<textarea id="unblockMotivo" bind:value={motivoBloqueo} rows="2" placeholder="Motivo de la desactivación del bloqueo (ej: Cumplió sanción, revisión de caso, etc.)"
						class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
				</div>
				<div class="space-y-1.5">
					<label for="unblockObservaciones" class="text-sm font-medium text-gray-300">Observaciones (Opcional)</label>
					<textarea id="unblockObservaciones" bind:value={observaciones} rows="2" placeholder="Notas internas."
						class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none transition-all resize-y"></textarea>
				</div>
			</div>

			<div class="mt-6 flex justify-end gap-3">
				<button type="button" on:click={cancelUnblock} class="px-4 py-2 text-sm font-medium text-gray-400 hover:text-white transition-colors">
					Cancelar
				</button>
				<button 
					type="button" 
					on:click={confirmUnblock} 
					disabled={!motivoBloqueo.trim()}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-colors bg-green-600 hover:bg-green-700 text-white disabled:bg-green-600/50"
				>
					Confirmar Desbloqueo
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
/* Aquí puedes poner estilos específicos si los tienes */
</style>