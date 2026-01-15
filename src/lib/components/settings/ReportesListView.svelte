<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { toast } from 'svelte-5-french-toast';
	import {
		Bug,
		Lightbulb,
		Sparkles,
		CheckCircle2,
		XCircle,
		Clock,
		RefreshCw,
		Paperclip,
		ChevronDown,
		ChevronRight,
		Filter
	} from 'lucide-svelte';

	interface Reporte {
		id: string;
		tipo: string;
		asunto: string;
		mensaje: string;
		contacto: string | null;
		tieneAdjunto: boolean;
		nombreAdjunto: string | null;
		estado: string;
		errorEnvio: string | null;
		enviadoAt: string | null;
		createdAt: string;
		updatedAt: string;
	}

	interface ReporteListResponse {
		reportes: Reporte[];
		total: number;
		enviados: number;
		pendientes: number;
		fallidos: number;
	}

	let data: ReporteListResponse | null = $state(null);
	let loading = $state(true);
	let retrying = $state<string | null>(null);
	let expandedId = $state<string | null>(null);
	let filterTipo = $state<string>('todos');

	const tipoIcons: Record<string, typeof Bug> = {
		error: Bug,
		sugerencia: Lightbulb,
		mejora: Sparkles
	};

	const tipoColors: Record<string, string> = {
		error: 'text-red-500',
		sugerencia: 'text-blue-500',
		mejora: 'text-purple-500'
	};

	const tipoBgColors: Record<string, string> = {
		error: 'bg-red-500/10 border-red-500/20',
		sugerencia: 'bg-blue-500/10 border-blue-500/20',
		mejora: 'bg-purple-500/10 border-purple-500/20'
	};

	const estadoConfig: Record<string, { icon: typeof CheckCircle2; color: string; label: string }> =
		{
			enviado: { icon: CheckCircle2, color: 'text-green-500', label: 'Enviado' },
			pendiente: { icon: Clock, color: 'text-yellow-500', label: 'Pendiente' },
			fallido: { icon: XCircle, color: 'text-red-500', label: 'Fallido' }
		};

	async function loadReportes() {
		loading = true;
		try {
			data = await invoke<ReporteListResponse>('get_all_reportes');
		} catch (error) {
			console.error('Error loading reportes:', error);
			toast.error('Error al cargar reportes');
		} finally {
			loading = false;
		}
	}

	async function handleRetry(id: string) {
		retrying = id;
		try {
			await invoke('retry_reporte', { id });
			toast.success('Reporte reenviado exitosamente');
			await loadReportes();
		} catch (error) {
			console.error('Error retrying:', error);
			toast.error('Error al reenviar');
		} finally {
			retrying = null;
		}
	}

	function toggleExpand(id: string) {
		expandedId = expandedId === id ? null : id;
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('es-ES', {
			day: '2-digit',
			month: 'short',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getFilteredReportes(): Reporte[] {
		if (!data) return [];
		if (filterTipo === 'todos') return data.reportes;
		return data.reportes.filter((r) => r.tipo === filterTipo);
	}

	onMount(() => {
		loadReportes();
	});
</script>

<div class="h-full overflow-auto bg-surface-1 p-6">
	<div class="max-w-4xl mx-auto space-y-6">
		<!-- Header -->
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-xl font-semibold text-gray-900 dark:text-gray-100 flex items-center gap-2">
					Historial de Reportes
				</h1>
				<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
					Seguimiento de errores y sugerencias enviadas
				</p>
			</div>
			<button
				onclick={loadReportes}
				disabled={loading}
				class="flex items-center gap-2 px-3 py-1.5 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-[#21262d] border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-[#30363d] transition-colors disabled:opacity-50"
			>
				<RefreshCw class="w-4 h-4 {loading ? 'animate-spin' : ''}" />
				Actualizar
			</button>
		</div>

		<!-- Stats Cards -->
		{#if data}
			<div class="grid grid-cols-4 gap-4">
				<div
					class="rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-[#0d1117] p-4"
				>
					<div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
						{data.total}
					</div>
					<div class="text-xs text-gray-500 dark:text-gray-400 mt-1">Total</div>
				</div>
				<div
					class="rounded-md border border-green-200 dark:border-green-800/30 bg-green-50 dark:bg-green-900/20 p-4"
				>
					<div class="text-2xl font-bold text-green-600 dark:text-green-400">
						{data.enviados}
					</div>
					<div class="text-xs text-green-600 dark:text-green-400 mt-1">Enviados</div>
				</div>
				<div
					class="rounded-md border border-yellow-200 dark:border-yellow-800/30 bg-yellow-50 dark:bg-yellow-900/20 p-4"
				>
					<div class="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
						{data.pendientes}
					</div>
					<div class="text-xs text-yellow-600 dark:text-yellow-400 mt-1">Pendientes</div>
				</div>
				<div
					class="rounded-md border border-red-200 dark:border-red-800/30 bg-red-50 dark:bg-red-900/20 p-4"
				>
					<div class="text-2xl font-bold text-red-600 dark:text-red-400">
						{data.fallidos}
					</div>
					<div class="text-xs text-red-600 dark:text-red-400 mt-1">Fallidos</div>
				</div>
			</div>
		{/if}

		<!-- Filter -->
		<div class="flex items-center gap-2">
			<Filter class="w-4 h-4 text-gray-500" />
			<select
				bind:value={filterTipo}
				class="text-sm bg-white dark:bg-[#0d1117] border border-gray-300 dark:border-gray-600 rounded-md px-3 py-1.5 focus:outline-none focus:ring-2 focus:ring-blue-500"
			>
				<option value="todos">Todos los tipos</option>
				<option value="error">Errores</option>
				<option value="sugerencia">Sugerencias</option>
				<option value="mejora">Mejoras</option>
			</select>
		</div>

		<!-- List -->
		<div
			class="rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden"
		>
			{#if loading}
				<div class="p-8 text-center text-gray-500">
					<RefreshCw class="w-6 h-6 mx-auto animate-spin mb-2" />
					Cargando reportes...
				</div>
			{:else if !data || data.reportes.length === 0}
				<div class="p-8 text-center text-gray-500">
					<Lightbulb class="w-8 h-8 mx-auto mb-2 opacity-50" />
					<p>No hay reportes registrados</p>
					<p class="text-xs mt-1">Los reportes que envies apareceran aqui</p>
				</div>
			{:else}
				<div class="divide-y divide-gray-200 dark:divide-gray-700">
					{#each getFilteredReportes() as reporte (reporte.id)}
						{@const Icon = tipoIcons[reporte.tipo] || Lightbulb}
						{@const estado = estadoConfig[reporte.estado] || estadoConfig.pendiente}
						{@const EstadoIcon = estado.icon}
						{@const isExpanded = expandedId === reporte.id}

						<div class="hover:bg-gray-50 dark:hover:bg-[#161b22] transition-colors">
							<!-- Row Header -->
							<button
								class="w-full px-4 py-3 flex items-center gap-3 text-left"
								onclick={() => toggleExpand(reporte.id)}
							>
								<!-- Expand Icon -->
								<div class="text-gray-400">
									{#if isExpanded}
										<ChevronDown class="w-4 h-4" />
									{:else}
										<ChevronRight class="w-4 h-4" />
									{/if}
								</div>

								<!-- Type Badge -->
								<div
									class="flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium border {tipoBgColors[
										reporte.tipo
									] || ''} {tipoColors[reporte.tipo] || ''}"
								>
									<Icon class="w-3 h-3" />
									{reporte.tipo}
								</div>

								<!-- Subject -->
								<div class="flex-1 min-w-0">
									<span class="font-medium text-gray-900 dark:text-gray-100 truncate block">
										{reporte.asunto}
									</span>
								</div>

								<!-- Attachment indicator -->
								{#if reporte.tieneAdjunto}
									<span title={reporte.nombreAdjunto || 'Adjunto'}>
										<Paperclip class="w-4 h-4 text-gray-400" />
									</span>
								{/if}

								<!-- Status Badge -->
								<div class="flex items-center gap-1.5 {estado.color}">
									<EstadoIcon class="w-4 h-4" />
									<span class="text-xs font-medium">{estado.label}</span>
								</div>

								<!-- Date -->
								<span class="text-xs text-gray-500 dark:text-gray-400 whitespace-nowrap">
									{formatDate(reporte.createdAt)}
								</span>
							</button>

							<!-- Expanded Content -->
							{#if isExpanded}
								<div
									class="px-4 pb-4 pt-0 ml-7 border-l-2 border-gray-200 dark:border-gray-700 ml-6"
								>
									<div class="space-y-3 text-sm">
										<!-- Message -->
										<div>
											<span
												class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide"
												>Mensaje</span
											>
											<p
												class="mt-1 text-gray-700 dark:text-gray-300 whitespace-pre-wrap bg-gray-50 dark:bg-[#161b22] rounded-md p-3 border border-gray-200 dark:border-gray-700"
											>
												{reporte.mensaje}
											</p>
										</div>

										<!-- Contact -->
										{#if reporte.contacto}
											<div>
												<span
													class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide"
													>Contacto</span
												>
												<p class="mt-1 text-gray-700 dark:text-gray-300">
													{reporte.contacto}
												</p>
											</div>
										{/if}

										<!-- Error if failed -->
										{#if reporte.estado === 'fallido' && reporte.errorEnvio}
											<div
												class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/30 rounded-md p-3"
											>
												<span class="text-xs font-semibold text-red-600 dark:text-red-400"
													>Error de envio</span
												>
												<p class="mt-1 text-red-700 dark:text-red-300 text-xs">
													{reporte.errorEnvio}
												</p>
											</div>
										{/if}

										<!-- Sent at -->
										{#if reporte.enviadoAt}
											<div class="text-xs text-gray-500">
												Enviado: {formatDate(reporte.enviadoAt)}
											</div>
										{/if}

										<!-- Retry button for failed -->
										{#if reporte.estado === 'fallido'}
											<button
												onclick={() => handleRetry(reporte.id)}
												disabled={retrying === reporte.id}
												class="flex items-center gap-2 px-3 py-1.5 text-xs font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md transition-colors disabled:opacity-50"
											>
												<RefreshCw
													class="w-3 h-3 {retrying === reporte.id ? 'animate-spin' : ''}"
												/>
												{retrying === reporte.id ? 'Reenviando...' : 'Reintentar envio'}
											</button>
										{/if}
									</div>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>
