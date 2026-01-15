<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { modulesStore } from '$lib/stores/modules';
	import { currentUser } from '$lib/stores/auth';
	import { toast } from 'svelte-5-french-toast';
	import { Package, Construction, EyeOff, Wrench, CheckCircle2 } from 'lucide-svelte';

	// Estados posibles y sus configuraciones visuales
	const STATUS_CONFIG = {
		active: {
			label: 'Activo',
			icon: CheckCircle2,
			class: 'text-green-400 bg-green-400/10 border-green-400/20'
		},
		hidden: {
			label: 'Oculto',
			icon: EyeOff,
			class: 'text-gray-400 bg-gray-400/10 border-gray-400/20'
		},
		development: {
			label: 'En Construcción',
			icon: Construction,
			class: 'text-yellow-400 bg-yellow-400/10 border-yellow-400/20'
		},
		maintenance: {
			label: 'Mantenimiento',
			icon: Wrench,
			class: 'text-orange-400 bg-orange-400/10 border-orange-400/20'
		}
	};

	let loading = false;
	let searchTerm = '';

	// Derivamos la lista de módulos del store
	$: modulesList = Object.values($modulesStore);

	// Filtramos por búsqueda
	$: filteredModules = modulesList.filter(
		(m) =>
			m.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
			m.key.toLowerCase().includes(searchTerm.toLowerCase())
	);

	async function handleStatusChange(key: string, newStatus: string) {
		loading = true;
		try {
			// Validación Frontend (UX): Si intenta poner Dev/Maint y no es GOD
			const restricted = ['development', 'maintenance'];
			if (restricted.includes(newStatus) && !$currentUser?.isSuperuser) {
				toast.error('Solo el Super Usuario puede activar modos de ingeniería.');
				// Revertir selección visualmente (el store no cambió aún)
				return;
			}

			await modulesStore.updateStatus(key, newStatus as any);
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		modulesStore.load();
	});
</script>

<div class="h-full flex flex-col bg-surface-1 p-6" in:fade>
	<!-- Header -->
	<div class="mb-8 flex items-start justify-between">
		<div>
			<h2 class="text-2xl font-bold text-white flex items-center gap-3">
				<Package class="text-primary-400" />
				Gestión de Módulos
			</h2>
			<p class="text-gray-400 mt-1 max-w-2xl">
				Controla la disponibilidad del sistema.
				{#if $currentUser?.isSuperuser}
					<span class="text-purple-400 font-medium"
						>Modo Dios activo: Puedes poner módulos en Mantenimiento o Construcción.</span
					>
				{:else}
					<span class="text-gray-500"
						>Solo lectura. El estado de los módulos es gestionado por Ingeniería.</span
					>
				{/if}
			</p>
		</div>
	</div>

	<!-- Search -->
	<div class="mb-6">
		<input
			type="text"
			bind:value={searchTerm}
			placeholder="Buscar módulo..."
			class="w-full max-w-md bg-surface-2 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-primary-500 transition-colors"
		/>
	</div>

	<!-- Grid -->
	<div
		class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-y-auto pb-4 custom-scrollbar"
	>
		{#each filteredModules as module (module.key)}
			{@const config = STATUS_CONFIG[module.status] || STATUS_CONFIG.active}

			<div
				class="bg-surface-2 border border-white/5 rounded-xl p-5 hover:border-white/10 transition-colors group relative overflow-hidden"
			>
				<!-- Status Indicator line -->
				<div
					class="absolute left-0 top-0 bottom-0 w-1 {config.class.split(' ')[1] ||
						'bg-gray-500/20'}"
				></div>

				<div class="flex justify-between items-start mb-4 pl-2">
					<div>
						<h3 class="font-semibold text-lg text-white">{module.name}</h3>
						<code class="text-xs text-gray-500 font-mono">{module.key}</code>
					</div>

					<div class="p-2 rounded-lg {config.class}">
						<svelte:component this={config.icon} size={20} />
					</div>
				</div>

				<!-- Controls -->
				<div class="pl-2 mt-4">
					<label
						for="status-{module.key}"
						class="text-xs text-gray-400 mb-1.5 block uppercase tracking-wider font-semibold"
						>Estado</label
					>
					<div class="relative">
						<select
							id="status-{module.key}"
							value={module.status}
							on:change={(e) => handleStatusChange(module.key, e.currentTarget.value)}
							class="w-full bg-surface-3 border border-white/10 text-white rounded-lg px-3 py-2 text-sm appearance-none cursor-pointer hover:border-white/20 focus:border-primary-500 focus:outline-none transition-colors"
							disabled={loading}
						>
							<option value="active">🟢 Operación Normal</option>

							<!-- Modos de Ingeniería (GOD) -->
							<option
								value="development"
								disabled={!$currentUser?.isSuperuser}
								class={!$currentUser?.isSuperuser ? 'opacity-30 text-gray-600' : 'text-yellow-400'}
							>
								🚧 En Construcción {!$currentUser?.isSuperuser ? '(Solo GOD)' : ''}
							</option>
							<option
								value="maintenance"
								disabled={!$currentUser?.isSuperuser}
								class={!$currentUser?.isSuperuser ? 'opacity-30 text-gray-600' : 'text-orange-400'}
							>
								🔧 En Mantenimiento {!$currentUser?.isSuperuser ? '(Solo GOD)' : ''}
							</option>
						</select>

						<!-- Chevron -->
						<div
							class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none text-gray-400"
						>
							<svg
								width="12"
								height="12"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"><path d="m6 9 6 6 6-6" /></svg
							>
						</div>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.custom-scrollbar {
		scrollbar-width: thin;
		scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
	}
	.custom-scrollbar::-webkit-scrollbar {
		width: 6px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background-color: rgba(255, 255, 255, 0.1);
		border-radius: 20px;
	}
</style>
